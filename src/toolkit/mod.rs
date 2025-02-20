pub mod belt;
pub mod tesselation;
pub mod unit_conversion;

use std::f64::consts::PI;

use nalgebra::Vector3;

use crate::blueprint::content::building;

pub const EARTH_R: f64 = 200.0;
pub const HALF_EQUATORIAL_GRID: f64 = 500.0;

pub fn sort_buildings(buildings: &mut [building::BuildingData]) {
    buildings.sort_by(|a, b| {
        a.item_id
            .cmp(&b.item_id)
            .then(a.model_index.cmp(&b.model_index))
            .then(a.recipe_id.cmp(&b.recipe_id))
            .then(a.area_index.cmp(&b.area_index))
            .then({
                const KY: f64 = 256.0;
                const KX: f64 = 1024.0;
                let score_a = (a.local_offset_y as f64 * KY + a.local_offset_x as f64) * KX
                    + a.local_offset_z as f64;
                let score_b = (b.local_offset_y as f64 * KY + b.local_offset_x as f64) * KX
                    + b.local_offset_z as f64;
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });
}

pub fn fix_buildings_index(buildings: Vec<building::BuildingData>) -> Vec<building::BuildingData> {
    use std::collections::HashMap;

    let index_lut: HashMap<_, _> = buildings
        .iter()
        .enumerate()
        .map(|(index, building)| (building.index, index as i32))
        .collect();

    buildings
        .into_iter()
        .map(|building| {
            building::BuildingData {
                // 这里panic是安全的，因为理论上所有的building.index都应该在index_lut里
                index: *index_lut
                    .get(&building.index)
                    .expect("Fatal error: unknown building index"),
                temp_output_obj_idx: index_lut
                    .get(&building.temp_output_obj_idx)
                    .copied()
                    .unwrap_or(building::INDEX_NULL),
                temp_input_obj_idx: index_lut
                    .get(&building.temp_input_obj_idx)
                    .copied()
                    .unwrap_or(building::INDEX_NULL),
                ..building
            }
        })
        .collect()
}

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
        .map(|building| {
            building::DspbptkBuildingData {
                // 这里panic是安全的，因为理论上所有的building.index都应该在index_lut里
                uuid: *uuid_lut
                    .get(&building.uuid)
                    .expect("Fatal error: unknown dspbptk building uuid"),
                temp_output_obj_idx: uuid_lut
                    .get(&building.temp_output_obj_idx)
                    .copied()
                    .unwrap_or(None),
                temp_input_obj_idx: uuid_lut
                    .get(&building.temp_input_obj_idx)
                    .copied()
                    .unwrap_or(None),
                ..building
            }
        })
        .collect()
}

// 将局部偏移转换为方向向量
pub fn local_offset_to_direction(local_offset: [f64; 3]) -> Vector3<f64> {
    const ANGLE_SCALE: f64 = PI / HALF_EQUATORIAL_GRID;

    let theta_x = (local_offset[0]) * ANGLE_SCALE;
    let theta_y = (local_offset[1]) * ANGLE_SCALE;

    let z = theta_y.sin();
    let radius = (1.0 - z * z).sqrt();

    let (sin_theta_x, cos_theta_x) = theta_x.sin_cos();

    Vector3::new(radius * cos_theta_x, radius * sin_theta_x, z).normalize()
}

// 将方向向量转换为局部偏移
pub fn direction_to_local_offset(direction: &Vector3<f64>, z: f64) -> [f64; 3] {
    const ANGLE_SCALE: f64 = HALF_EQUATORIAL_GRID / PI;

    let theta_x = direction.y.atan2(direction.x);
    let x = theta_x * ANGLE_SCALE;

    let theta_z = direction.z.asin();
    let y = theta_z * ANGLE_SCALE;

    // 修复非有限值的情况
    fn fix_value(value: f64, component: f64, default_positive: f64, default_negative: f64) -> f64 {
        if !value.is_finite() {
            return if component >= 0.0 {
                default_positive
            } else {
                default_negative
            };
        }
        value
    }

    let x = fix_value(x, direction.x, 500.0, -500.0);
    let y = fix_value(y, direction.z, 250.0, -250.0);

    [x, y, z]
}
