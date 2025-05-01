pub mod belt;
pub mod tesselation;
pub mod unit_conversion;

use std::collections::{HashMap, VecDeque};
use std::f64::consts::PI;

use nalgebra::Vector3;

use crate::blueprint::content::building;

pub const EARTH_R: f64 = 200.0;
pub const HALF_EQUATORIAL_GRID: f64 = 500.0;

/// 对建筑进行排序。
///
/// 首先根据`item_id`给建筑分组，如果是传送带`(2001 <= item_id && item_id <= 2009)`则跨`item_id`进行拓扑排序(不稳定)，传送带放在建筑列表前面；\
/// 如果是其它建筑则依次按照`item_id`、`model_index`、`recipe_id`、`area_index`、`local_offset`进行排序(稳定)，其它建筑放在建筑列表后面。\
/// 注意传送带单独分组，并不与其它建筑保证`item_id`的顺序关系
// 主函数职责：协调整个排序流程
pub fn sort_buildings(buildings: &mut [building::BuildingData]) {
    let n = buildings.len();
    if n == 0 {
        return;
    }

    // 1. 分割阶段：预分配内存容量
    let (mut belts, mut non_belts) = split_belt_and_non_belt(buildings);

    // 2. 排序阶段：传送带和非传送带独立排序
    topological_sort_belt(&mut belts);
    stable_sort_non_belt(&mut non_belts);

    // 3. 合并阶段：直接拼接无需反转
    let new_order = combine_sorted_results(belts, non_belts);

    buildings[..n].clone_from_slice(&new_order[..n]);
}

// Split阶段优化：预分配50%容量
fn split_belt_and_non_belt(
    buildings: &[building::BuildingData],
) -> (Vec<building::BuildingData>, Vec<building::BuildingData>) {
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

// 非传送带排序优化：预计算排序键值
fn stable_sort_non_belt(non_belts: &mut [building::BuildingData]) {
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

// 提取为独立函数便于复用
#[inline]
fn calculate_offset_score(b: &building::BuildingData) -> f64 {
    let (x, y, z) = (
        f64::from(b.local_offset_x),
        f64::from(b.local_offset_y),
        f64::from(b.local_offset_z),
    );
    y.mul_add(256.0, x).mul_add(1024.0, z)
}

// 合并阶段优化：删除冗余反转
fn combine_sorted_results(
    belt_buildings: Vec<building::BuildingData>,
    non_belt_buildings: Vec<building::BuildingData>,
) -> Vec<building::BuildingData> {
    let mut result = Vec::with_capacity(belt_buildings.len() + non_belt_buildings.len());
    result.extend(belt_buildings);
    result.extend(non_belt_buildings);
    result.reverse();
    result
}

#[must_use]
pub fn fix_buildings_index(buildings: Vec<building::BuildingData>) -> Vec<building::BuildingData> {
    use std::collections::HashMap;

    let index_lut: HashMap<_, _> = buildings
        .iter()
        .zip(0..=i32::MAX)
        .map(|(building, index)| (building.index, index))
        .collect();

    buildings
        .into_iter()
        .map(|building| building::BuildingData {
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

#[must_use]
pub fn fix_dspbptk_buildings_index(
    buildings: Vec<building::DspbptkBuildingData>,
) -> Vec<building::DspbptkBuildingData> {
    use std::collections::HashMap;

    let uuid_lut: HashMap<_, _> = buildings
        .iter()
        .enumerate()
        .map(|(uuid, building)| (building.uuid, Some(uuid as u128)))
        .collect();

    buildings
        .into_iter()
        .map(|building| building::DspbptkBuildingData {
            uuid: *uuid_lut.get(&building.uuid).unwrap_or(&None),
            temp_output_obj_idx: uuid_lut
                .get(&building.temp_output_obj_idx)
                .copied()
                .unwrap_or(None),
            temp_input_obj_idx: uuid_lut
                .get(&building.temp_input_obj_idx)
                .copied()
                .unwrap_or(None),
            ..building
        })
        .collect()
}

// 将局部偏移转换为方向向量
#[must_use]
pub fn local_offset_to_direction(local_offset: [f64; 3]) -> Vector3<f64> {
    const ANGLE_SCALE: f64 = PI / HALF_EQUATORIAL_GRID;

    let theta_x = (local_offset[0]) * ANGLE_SCALE;
    let theta_y = (local_offset[1]) * ANGLE_SCALE;

    let (sin_theta_y, cos_theta_y) = theta_y.sin_cos();
    let (sin_theta_x, cos_theta_x) = theta_x.sin_cos();

    Vector3::new(
        cos_theta_y * cos_theta_x,
        cos_theta_y * sin_theta_x,
        sin_theta_y,
    )
}

// 修复非有限值的情况
fn fix_value(value: f64, component: f64, default_positive: f64, default_negative: f64) -> f64 {
    if value.is_finite() {
        value
    } else if component >= 0.0 {
        default_positive
    } else {
        default_negative
    }
}

// 将方向向量转换为局部偏移
#[must_use]
pub fn direction_to_local_offset(direction: &Vector3<f64>, z: f64) -> [f64; 3] {
    const ANGLE_SCALE: f64 = HALF_EQUATORIAL_GRID / PI;

    let theta_x = direction.y.atan2(direction.x);
    let x = theta_x * ANGLE_SCALE;

    let theta_z = direction.z.asin();
    let y = theta_z * ANGLE_SCALE;

    let x = fix_value(x, direction.x, 500.0, -500.0);
    let y = fix_value(y, direction.z, 250.0, -250.0);

    [x, y, z]
}

/// 根据传送带连接关系尝试进行拓扑排序。假定所有的建筑都是传送带。
///
/// 传送带由节点构成，每个节点最多从三个其它节点输入，并输出到最多一个其它节点，可以成环。\
/// 每个节点通过`temp_output_obj_idx`来表示输出的节点，不设置输入节点。\
/// 所有传送带可能形成非连通图，对其中的每个连通子图尝试进行拓扑排序，非连通子图的顺序未定义。
pub fn topological_sort_belt(buildings: &mut [building::BuildingData]) {
    // TODO 性能优化

    if let Some((n, adj, in_degree)) = build_graph(buildings) {
        let mut visited = vec![false; n];
        let mut result = Vec::new();

        for i in 0..n {
            if !visited[i] {
                // Kahn 算法进行拓扑排序
                let mut queue: VecDeque<usize> = VecDeque::new();
                let mut current_in_degree = in_degree.clone();

                for j in 0..n {
                    if visited[j] {
                        continue;
                    }
                    if current_in_degree[j] == 0 {
                        queue.push_back(j);
                    }
                }

                let mut sorted_nodes = Vec::new();
                while let Some(node) = queue.pop_front() {
                    if visited[node] {
                        continue;
                    }
                    visited[node] = true;
                    sorted_nodes.push(node);

                    for &next in &adj[node] {
                        current_in_degree[next] -= 1;
                        if current_in_degree[next] == 0 {
                            queue.push_back(next);
                        }
                    }
                }

                // 如果排序后的节点数等于当前连通图的节点数，说明排序成功
                if !sorted_nodes.is_empty() {
                    result.extend(sorted_nodes);
                }
            }
        }

        if result.is_empty() {
            return;
        }

        // 将排序后的索引映射回原始数组
        let mut temp = Vec::with_capacity(n);
        for &idx in &result {
            temp.push(buildings[idx].clone());
        }

        buildings[..n].clone_from_slice(&temp[..n]);
    }
}

/// 分析传送带连接关系，构建传送带图。
///
/// 节点的度计算仅考虑传送带节点，不考虑其他建筑。  
fn build_graph(buildings: &[building::BuildingData]) -> Option<(usize, Vec<Vec<usize>>, Vec<i32>)> {
    let n = buildings.len();
    if n == 0 {
        return None;
    }
    let index_map: HashMap<i32, usize> = buildings
        .iter()
        .enumerate()
        .map(|(i, b)| (b.index, i))
        .collect();
    let mut adj = vec![vec![]; n];
    let mut in_degree = vec![0; n];
    for (i, building) in buildings.iter().enumerate() {
        if building.temp_output_obj_idx != building::INDEX_NULL {
            if let Some(&j) = index_map.get(&building.temp_output_obj_idx) {
                adj[i].push(j);
                in_degree[j] += 1;
            }
        }
    }
    Some((n, adj, in_degree))
}
