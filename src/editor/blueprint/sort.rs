use std::collections::{HashMap, HashSet};

use crate::blueprint::data::content::building::{self, Building};

use petgraph::{
    graph::{Graph, NodeIndex},
    visit::EdgeRef,
};

/// 对建筑进行排序。
///
/// 首先根据`item_id`给建筑分组，如果是传送带`(2001 <= item_id && item_id <= 2009)`则跨`item_id`进行拓扑排序(不稳定)，传送带放在建筑列表前面；\
/// 如果是其它建筑则依次按照`item_id`、`model_index`、`recipe_id`、`area_index`、`local_offset`进行排序(稳定)，其它建筑放在建筑列表后面。\
/// 注意传送带单独分组，并不与其它建筑保证`item_id`的顺序关系
#[must_use]
pub fn sort_buildings(buildings: Vec<Building>, reserved: bool) -> Vec<Building> {
    // 0. 空建筑列表提前返回
    if buildings.is_empty() {
        return Vec::new();
    }

    // 1. 分组阶段：根据item_id进行分类
    let (belts, non_belts) = split_belt_and_non_belt(buildings);

    // 2. 排序阶段：传送带和非传送带独立排序
    let sorted_belt = topological_sort_belt(&belts);
    let sorted_non_belt = stable_sort_non_belt(non_belts);

    // 3. 合并阶段：合并排序结果
    let mut sorted = combine_sorted_results(sorted_belt, sorted_non_belt);

    // 4. 游戏内的建造顺序与蓝图顺序相反，故提供选项是否翻转
    if reserved {
        sorted.reverse();
    }

    sorted
}

fn split_belt_and_non_belt(buildings: Vec<Building>) -> (Vec<Building>, Vec<Building>) {
    buildings
        .into_iter()
        .partition(|building| (2001..=2009).contains(&building.item_id))
}

fn stable_sort_non_belt(mut buildings: Vec<Building>) -> Vec<Building> {
    buildings.sort_by_cached_key(|building| {
        // 预计算排序键，实现Schwartzian transform优化
        (
            building.item_id,
            building.model_index,
            building.recipe_id,
            building.area_index,
            calculate_offset_score(building).to_bits(),
        )
    });

    buildings
}

fn calculate_offset_score(b: &Building) -> f64 {
    let (x, y, z) = (
        f64::from(b.local_offset_x),
        f64::from(b.local_offset_y),
        f64::from(b.local_offset_z),
    );
    y.mul_add(256.0, x).mul_add(1024.0, z)
}

fn combine_sorted_results(
    belt_buildings: Vec<Building>,
    non_belt_buildings: Vec<Building>,
) -> Vec<Building> {
    belt_buildings
        .into_iter()
        .chain(non_belt_buildings)
        .collect::<Vec<_>>()
}

#[must_use]
pub fn fix_buildings_index(buildings: Vec<Building>) -> Vec<Building> {
    let lut = buildings
        .iter()
        .zip(0..=i32::MAX)
        .map(|(building, index)| (building.index, index))
        .collect::<HashMap<_, _>>();

    buildings
        .into_iter()
        .map(|building| Building {
            index: *lut.get(&building.index).unwrap_or(&building::INDEX_NULL),
            temp_output_obj_idx: lut
                .get(&building.temp_output_obj_idx)
                .copied()
                .unwrap_or(building::INDEX_NULL),
            temp_input_obj_idx: lut
                .get(&building.temp_input_obj_idx)
                .copied()
                .unwrap_or(building::INDEX_NULL),
            ..building
        })
        .collect()
}

/// 根据传送带连接关系，进行广义的拓扑排序。假定所有的建筑都是传送带。
///
/// 传送带节点通过`temp_output_obj_idx`记录输出连接（可为`INDEX_NULL`），输入连接不记录，永远都是`INDEX_NULL`。
/// 每个节点最多有三个输入和一个输出，支持环形连接。
///
/// # 实现步骤
///
/// 1. 构建以`temp_output_obj_idx`为边的有向图
/// 2. 将每个SCC收缩为单个节点
/// 3. 对生成的DAG进行拓扑排序，然后把收缩后的节点原地展开为SCC，得到最终结果
///
/// 在保持拓扑序的前提下，应尽量优化内存布局，使线性链节点连续存储
///
/// # Panics
/// 说明构建缩点后的DAG这一步的实现存在错误，实际生成的并不是DAG。这永远不应该出现。
#[must_use]
pub fn topological_sort_belt(buildings: &[Building]) -> Vec<Building> {
    let graph = build_graph(buildings);

    // 1. 获取所有强连通分量（SCC）
    let sccs = petgraph::algo::kosaraju_scc(&graph);

    // 2. 构建缩点后的DAG
    let mut dag = Graph::<usize, usize>::new();

    // 3. 为每个SCC创建DAG节点（直接使用SCC索引作为节点标识）
    for _ in &sccs {
        dag.add_node(1);
    }
    let dag_node_to_ssc = sccs
        .iter()
        .enumerate()
        .flat_map(|(i, scc)| scc.iter().map(move |&node| (node, i)))
        .collect::<HashMap<_, _>>();

    // 4. 添加DAG中的边（过滤重复边）
    let edge_keys = graph
        .edge_references()
        .filter_map(|edge_ref| {
            let source = edge_ref.source();
            let target = edge_ref.target();

            let scc_source = dag_node_to_ssc.get(&source)?;
            let scc_target = dag_node_to_ssc.get(&target)?;

            if scc_source == scc_target {
                None
            } else {
                Some((*scc_source, *scc_target))
            }
        })
        .collect::<HashSet<_>>();
    for (scc_source, scc_target) in edge_keys {
        dag.add_edge(NodeIndex::new(scc_source), NodeIndex::new(scc_target), 1);
    }

    // 5. 对DAG进行拓扑排序
    let dag_order =
        petgraph::algo::toposort(&dag, None).expect("unreachable: cycle detected in DAG.");

    // 6. 按照拓扑序展开SCC
    dag_order
        .iter()
        .flat_map(|&dag_node_idx| {
            let scc = &sccs[dag_node_idx.index()];
            optimize_scc(scc, buildings)
        })
        .collect()
}

/// 构建建筑依赖关系图
///
/// # 参数
/// * `buildings` - 建筑数据切片，包含建筑索引和输出对象索引等信息
///
/// # 返回值
/// 返回构建完成的有向图结构，节点权重和边权重均为usize类型
fn build_graph(buildings: &[Building]) -> Graph<usize, usize> {
    let mut graph: Graph<usize, usize> = Graph::new();

    // 创建节点
    let index_to_node = buildings
        .iter()
        .map(|building| {
            let node_index = graph.add_node(1);
            (building.index, node_index)
        })
        .collect::<HashMap<_, _>>();

    // 创建边
    for building in buildings {
        if building.temp_output_obj_idx != building::INDEX_NULL
            && let Some(edge_from) = index_to_node.get(&building.index)
            && let Some(edge_to) = index_to_node.get(&building.temp_output_obj_idx)
        {
            graph.add_edge(*edge_from, *edge_to, 1);
        }
    }

    graph
}

/// 优化强连通分量（SCC）中的建筑布局顺序
///
/// # 输入要求
/// * 输入必须为SCC，并且每个节点出度 <= 1
///
/// # 参数
/// * `scc` - SCC中的节点索引列表
/// * `buildings` - 建筑物数据切片
///
/// # 返回值
/// 返回按拓扑顺序排列的建筑物数据向量
///
/// # 算法说明
/// 1. 构建节点间链表关系`next_node`
/// 2. 标记没有输出依赖的起始节点`is_start`
/// 3. 按链表顺序收集结果
///
/// 复杂度: O(n)
fn optimize_scc(scc: &[NodeIndex], buildings: &[Building]) -> Vec<Building> {
    // 创建节点索引映射表: 建筑物ID -> SCC中的位置
    let node_index_map = scc
        .iter()
        .enumerate()
        .map(|(idx, node)| (buildings[node.index()].index, idx))
        .collect::<HashMap<_, _>>();

    // 输入应已经保证可以选择第一个传送带作为起点
    std::iter::successors(Some(0), |&i| {
        let current_node = scc[i];
        let output = buildings[current_node.index()].temp_output_obj_idx;

        if output == building::INDEX_NULL {
            None
        } else {
            node_index_map.get(&output).copied()
        }
    })
    .map(|i| buildings[scc[i].index()].clone())
    .collect()
}
