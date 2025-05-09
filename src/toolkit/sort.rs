use crate::blueprint::content::building::{self, BuildingData};

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
    let (mut belts, mut non_belts) = split_belt_and_non_belt(buildings);

    // 2. 排序阶段：传送带和非传送带独立排序
    topological_sort_belt(&mut belts);
    stable_sort_non_belt(&mut non_belts);

    // 3. 合并阶段：合并排序结果
    combine_sorted_results(belts, non_belts, reserved)
}

fn split_belt_and_non_belt(buildings: &[BuildingData]) -> (Vec<BuildingData>, Vec<BuildingData>) {
    let mut belt = Vec::new();
    let mut non_belt = Vec::new();

    for building in buildings {
        if (2001..=2009).contains(&building.item_id) {
            belt.push(building.clone());
        } else {
            non_belt.push(building.clone());
        }
    }

    (belt, non_belt)
}

fn stable_sort_non_belt(non_belts: &mut [BuildingData]) {
    // 使用Schwartzian transform优化多字段排序
    // 修改后的排序逻辑（替换原stable_sort_non_belt中的sort_by部分）
    non_belts.sort_by(|a, b| {
        // 先比较前四个整型字段
        let order = (a.item_id, a.model_index, a.recipe_id, a.area_index).cmp(&(
            b.item_id,
            b.model_index,
            b.recipe_id,
            b.area_index,
        ));

        if order != std::cmp::Ordering::Equal {
            return order;
        }

        // 单独处理浮点数比较
        let score_a = calculate_offset_score(a);
        let score_b = calculate_offset_score(b);

        // 处理NaN情况，保持排序稳定性
        score_a
            .partial_cmp(&score_b)
            .map_or(std::cmp::Ordering::Equal, |ord| ord)
    });
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
/// 实现步骤：
/// 1. 构建以`temp_output_obj_idx`为边的有向图
/// 2. 将每个SCC收缩为单个节点
/// 3. 对生成的DAG进行拓扑排序，然后把收缩后的节点原地展开为SCC，得到最终结果
///
/// 在保持拓扑序的前提下，应尽量优化内存布局，使线性链节点连续存储
pub fn topological_sort_belt(buildings: &mut [BuildingData]) {}
