use std::collections::{HashMap, HashSet};

use crate::blueprint::content::building::{self, BuildingData};

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
pub fn sort_buildings(buildings: &[BuildingData], reserved: bool) -> Vec<BuildingData> {
    let buildings_num = buildings.len();
    if buildings_num == 0 {
        return Vec::new();
    }

    // 1. 分组阶段：根据item_id进行分类
    let (belts, non_belts) = split_belt_and_non_belt(buildings);

    // 2. 排序阶段：传送带和非传送带独立排序
    let sorted_belt = topological_sort_belt(&belts);
    let sorted_non_belt = stable_sort_non_belt(&non_belts);

    // 3. 合并阶段：合并排序结果
    combine_sorted_results(sorted_belt, sorted_non_belt, reserved)
}

fn split_belt_and_non_belt(buildings: &[BuildingData]) -> (Vec<BuildingData>, Vec<BuildingData>) {
    buildings
        .iter()
        .cloned()
        .partition(|building| (2001..=2009).contains(&building.item_id))
}

fn stable_sort_non_belt(non_belts: &[BuildingData]) -> Vec<BuildingData> {
    let mut sorted = non_belts.to_vec();

    sorted.sort_by_cached_key(|building| {
        // 预计算排序键，实现Schwartzian transform优化
        let int_key = (
            building.item_id,
            building.model_index,
            building.recipe_id,
            building.area_index,
        );
        let float_score = calculate_offset_score(building);

        // 将浮点数转换为可排序的整数表示（处理NaN）
        let float_key = float_score.to_bits();

        (int_key, float_key)
    });

    sorted
}

fn calculate_offset_score(b: &BuildingData) -> f64 {
    let (x, y, z) = (
        f64::from(b.local_offset_x),
        f64::from(b.local_offset_y),
        f64::from(b.local_offset_z),
    );
    y.mul_add(256.0, x).mul_add(1024.0, z)
}

fn combine_sorted_results(
    belt_buildings: Vec<BuildingData>,
    non_belt_buildings: Vec<BuildingData>,
    reserved: bool,
) -> Vec<BuildingData> {
    let mut result = Vec::with_capacity(belt_buildings.len() + non_belt_buildings.len());
    result.extend(belt_buildings);
    result.extend(non_belt_buildings);
    if reserved {
        result.reverse();
    }
    result
}

#[must_use]
pub fn fix_buildings_index(buildings: Vec<BuildingData>) -> Vec<BuildingData> {
    use std::collections::HashMap;

    let index_lut: HashMap<_, _> = buildings
        .iter()
        .zip(0..=i32::MAX)
        .map(|(building, index)| (building.index, index))
        .collect();

    buildings
        .into_iter()
        .map(|building| BuildingData {
            index: *index_lut
                .get(&building.index)
                .unwrap_or(&building::INDEX_NULL),
            temp_output_obj_idx: index_lut
                .get(&building.temp_output_obj_idx)
                .copied()
                .unwrap_or(building::INDEX_NULL),
            temp_input_obj_idx: index_lut
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
/// # Panic
///
#[must_use]
pub fn topological_sort_belt(buildings: &[BuildingData]) -> Vec<BuildingData> {
    let graph = build_graph(buildings);

    // 1. 获取所有强连通分量（SCC）
    let sccs = petgraph::algo::kosaraju_scc(&graph);

    // 2. 构建缩点后的DAG
    let mut dag = Graph::<usize, usize>::new();
    let mut scc_hashmap = HashMap::<NodeIndex, usize>::new();

    // 3. 为每个SCC创建DAG节点（直接使用SCC索引作为节点标识）
    for (i, scc) in sccs.iter().enumerate() {
        dag.add_node(1);
        for &node in scc {
            scc_hashmap.insert(node, i);
        }
    }

    // 4. 添加DAG中的边（过滤重复边）
    let mut edge_set = HashSet::new();
    for edge_ref in graph.edge_references() {
        let source = edge_ref.source();
        let target = edge_ref.target();

        // 获取源和目标的SCC编号
        if let (Some(scc_source), Some(scc_target)) =
            (scc_hashmap.get(&source), scc_hashmap.get(&target))
            && scc_source != scc_target
        {
            let edge_key = (*scc_source, *scc_target);
            if edge_set.insert(edge_key) {
                dag.add_edge(NodeIndex::new(*scc_source), NodeIndex::new(*scc_target), 1);
            }
        }
    }

    // 5. 对DAG进行拓扑排序
    let dag_order =
        petgraph::algo::toposort(&dag, None).expect("fatal error: Cycle detected in DAG.");

    // 6. 按照拓扑序展开SCC
    let mut result = Vec::with_capacity(buildings.len());
    for &dag_node_idx in &dag_order {
        // 通过反向查找表获取对应的SCC索引
        let scc = &sccs[dag_node_idx.index()];
        // 6.1. 对每个SCC内部进行局部排序（线性链优化）
        let scc_nodes = optimize_scc_layout(scc, buildings);
        // 6.2. 保持SCC内部节点的相对顺序（可扩展为更复杂的优化策略）
        result.extend(scc_nodes);
    }

    result
}

/// 构建建筑依赖关系图
///
/// # 参数
/// * `buildings` - 建筑数据切片，包含建筑索引和输出对象索引等信息
///
/// # 返回值
/// 返回构建完成的有向图结构，节点权重和边权重均为usize类型
fn build_graph(buildings: &[BuildingData]) -> Graph<usize, usize> {
    let mut graph: Graph<usize, usize> = Graph::new();

    // 合并两个查找表为单一结构
    let mut index_to_node = HashMap::new();
    // 直接按building.index存储对应的NodeIndex
    for building in buildings {
        let node_idx = graph.add_node(1);
        index_to_node.insert(building.index, node_idx);
    }

    // 一次查找完成边的建立
    for building in buildings {
        if building.temp_output_obj_idx != building::INDEX_NULL
            && let Some(&edge_to) = index_to_node.get(&building.temp_output_obj_idx)
            && let Some(edge_from) = index_to_node.get(&building.index)
        {
            graph.add_edge(*edge_from, edge_to, 1);
        }
    }

    graph
}

/// 优化强连通分量（SCC）中的建筑布局顺序
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
fn optimize_scc_layout(scc: &[NodeIndex], buildings: &[BuildingData]) -> Vec<BuildingData> {
    let scc_size = scc.len();

    // 利用SCC特性：所有节点强连通，无需处理孤立节点
    // 创建节点索引映射表: 建筑物ID -> SCC中的位置
    let node_index_map: HashMap<i32, usize> = scc
        .iter()
        .enumerate()
        .map(|(idx, node)| (buildings[node.index()].index, idx))
        .collect();

    // 构建节点链表关系
    let mut next_node = vec![None; scc_size];
    for (i, &node) in scc.iter().enumerate() {
        let node_idx = node.index();
        let output = buildings[node_idx].temp_output_obj_idx;

        if output != building::INDEX_NULL {
            // 通过哈希表直接查找目标节点位置
            if let Some(&j) = node_index_map.get(&output) {
                next_node[i] = Some(j);
            }
        }
    }

    // 保证至少有一个起点（SCC特性：可从任意节点遍历整个分量）
    let mut visited = vec![false; scc_size];
    let mut result = Vec::with_capacity(scc_size);

    // 找到第一个可用起点
    for start in 0..scc_size {
        if !visited[start] {
            let mut curr = start;
            while !visited[curr] {
                visited[curr] = true;
                result.push(buildings[scc[curr].index()].clone());
                curr = match next_node[curr] {
                    Some(n) => n,
                    None => break,
                };
            }
        }
    }

    result
}
