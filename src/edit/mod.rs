pub mod belt;
pub mod tesselation;
pub mod unit_conversion;

use std::f64::consts::PI;

use approx::abs_diff_eq;
use log::warn;
use nalgebra::{geometry::Quaternion, Vector3};

use crate::blueprint::content::building;

pub const EARTH_R: f64 = 200.0;
pub const HALF_EQUATORIAL_GRID: f64 = 500.0;

pub fn sort_buildings(buildings: &mut Vec<building::BuildingData>) {
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

fn calculate_quaternion_between_vectors(
    from: &Vector3<f64>,
    to: &Vector3<f64>,
) -> (Quaternion<f64>, Quaternion<f64>) {
    // FIXME 这里不一定要检查？
    assert!(abs_diff_eq!(from.norm_squared(), 1.0, epsilon = 1e-6));
    assert!(abs_diff_eq!(to.norm_squared(), 1.0, epsilon = 1e-6));

    let cos_theta = from.dot(to).clamp(-1.0, 1.0);
    let cos_theta = cos_theta.clamp(-1.0, 1.0);

    if cos_theta >= 1.0 - f64::EPSILON {
        return (
            Quaternion::new(1.0, 0.0, 0.0, 0.0),
            Quaternion::new(1.0, 0.0, 0.0, 0.0),
        );
    }

    let cross = from.cross(to);
    let sin_theta = cross.norm();

    // 处理接近共线的情况
    if sin_theta < 1e-6 {
        return handle_colinear_case(from);
    }

    let axis = cross / sin_theta; // cross.normalize()
    let theta = cos_theta.acos();
    let (sin_half, cos_half) = (theta / 2.0).sin_cos();

    let quaternion = Quaternion::new(
        cos_half,
        axis.x * sin_half,
        axis.y * sin_half,
        axis.z * sin_half,
    );

    (quaternion, quaternion.conjugate())
}

// 处理接近共线的情况，选择与 from 正交的轴
fn handle_colinear_case(v: &Vector3<f64>) -> (Quaternion<f64>, Quaternion<f64>) {
    let axis = select_orthogonal_axis(v);
    let quaternion = Quaternion::new(0.0, axis.x, axis.y, axis.z).normalize();
    (quaternion, quaternion.conjugate())
}

// 选择与给定向量正交的单位向量
fn select_orthogonal_axis(v: &Vector3<f64>) -> Vector3<f64> {
    if v.x.abs() < 1e-6 {
        return Vector3::new(1.0, 0.0, 0.0).normalize();
    }
    Vector3::new(-v.y - v.z, v.x + v.z, v.x + v.y).normalize()
}

// 使用给定的四元数旋转一个向量
pub fn compute_3d_rotation_vector(
    from: &Vector3<f64>,
    (quaternion, inverse_quaternion): (Quaternion<f64>, Quaternion<f64>),
) -> Vector3<f64> {
    let from_quat = Quaternion::new(0.0, from.x, from.y, from.z);
    let to_quat = quaternion * from_quat * inverse_quaternion;
    Vector3::new(to_quat.i, to_quat.j, to_quat.k)
}

// FIXME 把warn改结构化
// 将局部偏移转换为方向向量
pub fn local_offset_to_direction(local_offset: [f64; 3]) -> Vector3<f64> {
    const ANGLE_SCALE: f64 = PI / HALF_EQUATORIAL_GRID;

    if local_offset[0] > 500.0
        || local_offset[0] < -500.0
        || local_offset[1] > 250.0
        || local_offset[1] < -250.0
    {
        warn!(
            "Non-standard local_offset: ({}, {})",
            local_offset[0], local_offset[1]
        );
    }
    let theta_x = (local_offset[0] as f64) * ANGLE_SCALE;
    let theta_y = (local_offset[1] as f64) * ANGLE_SCALE;

    let z = theta_y.sin();
    let radius = (1.0 - z * z).sqrt();

    let (sin_theta_x, cos_theta_x) = theta_x.sin_cos();

    Vector3::new(radius * cos_theta_x, radius * sin_theta_x, z).normalize()
}

// FIXME 把warn改结构化
// 将方向向量转换为局部偏移
pub fn direction_to_local_offset(direction: &Vector3<f64>, z: f64) -> [f64; 3] {
    const ANGLE_SCALE: f64 = HALF_EQUATORIAL_GRID / PI;

    if direction.norm_squared() == 0.0 {
        warn!("Zero direction!");
        return [0.0, 0.0, 0.0];
    }

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

    if !x.is_finite() || !y.is_finite() {
        warn!("Lost precision: x = {:?}, y = {:?}", x, y);
    }

    [x, y, z]
}
