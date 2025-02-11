pub mod unit_conversion;
pub mod tesselation;

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

pub fn fix_buildings_index(buildings: &mut Vec<building::BuildingData>) {
    use std::collections::HashMap;

    let mut index_lut = HashMap::with_capacity(buildings.len());
    buildings.iter().enumerate().for_each(|(index, building)| {
        index_lut.insert(building.index, index as i32);
    });
    for building in buildings {
        // 这里panic是安全的，因为理论上所有的building.index都应该在index_lut里
        building.index = *index_lut
            .get(&building.index)
            .expect("Fatal error: unknown building index");

        if let Some(idx) = index_lut.get(&building.temp_output_obj_idx) {
            building.temp_output_obj_idx = *idx;
        } else {
            building.temp_output_obj_idx = building::INDEX_NULL;
        }

        if let Some(idx) = index_lut.get(&building.temp_input_obj_idx) {
            building.temp_input_obj_idx = *idx;
        } else {
            building.temp_input_obj_idx = building::INDEX_NULL;
        }
    }
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

    let axis = cross.normalize();
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

// FIXME 检查所有坐标转换，强制direction是单位向量
// FIXME 把warn改结构化
// 将局部偏移转换为方向向量
pub fn local_offset_to_direction(local_offset_x: f32, local_offset_y: f32) -> Vector3<f64> {
    const ANGLE_SCALE: f64 = PI / HALF_EQUATORIAL_GRID;

    if local_offset_x > 500.0
        || local_offset_x < -500.0
        || local_offset_y > 250.0
        || local_offset_y < -250.0
    {
        warn!(
            "Non-standard local_offset: ({}, {})",
            local_offset_x, local_offset_y
        );
    }
    let theta_x = (local_offset_x as f64) * ANGLE_SCALE;
    let theta_y = (local_offset_y as f64) * ANGLE_SCALE;

    let z = theta_y.sin();
    let radius = (1.0 - z * z).sqrt();

    let (sin_theta_x, cos_theta_x) = theta_x.sin_cos();

    Vector3::new(radius * cos_theta_x, radius * sin_theta_x, z).normalize()
}

// 将方向向量转换为局部偏移
pub fn direction_to_local_offset(direction: &Vector3<f64>) -> (f32, f32) {
    const ANGLE_SCALE: f64 = HALF_EQUATORIAL_GRID / PI;

    if direction.norm_squared() == 0.0 {
        warn!("Zero direction!");
        return (0.0, 0.0);
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

    (x as f32, y as f32)
}

#[cfg(test)]
mod test_coordinate_transformation {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_origin() {
        let offset = (0.0, 0.0);
        let direction = local_offset_to_direction(offset.0, offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(offset.0, converted_offset.0, epsilon = 1e-6);
        assert_abs_diff_eq!(offset.1, converted_offset.1, epsilon = 1e-6);
    }

    #[test]
    fn test_positive_x_axis() {
        let offset = (1.0, 0.0);
        let direction = local_offset_to_direction(offset.0, offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(offset.0, converted_offset.0, epsilon = 1e-6);
        assert_abs_diff_eq!(offset.1, converted_offset.1, epsilon = 1e-6);
    }

    #[test]
    fn test_positive_y_axis() {
        let offset = (0.0, 1.0);
        let direction = local_offset_to_direction(offset.0, offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(offset.0, converted_offset.0, epsilon = 1e-6);
        assert_abs_diff_eq!(offset.1, converted_offset.1, epsilon = 1e-6);
    }

    #[test]
    fn test_negative_x_axis() {
        let offset = (-1.0, 0.0);
        let direction = local_offset_to_direction(offset.0, offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(offset.0, converted_offset.0, epsilon = 1e-6);
        assert_abs_diff_eq!(offset.1, converted_offset.1, epsilon = 1e-6);
    }

    #[test]
    fn test_negative_y_axis() {
        let offset = (0.0, -1.0);
        let direction = local_offset_to_direction(offset.0, offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(offset.0, converted_offset.0, epsilon = 1e-6);
        assert_abs_diff_eq!(offset.1, converted_offset.1, epsilon = 1e-6);
    }

    #[test]
    fn test_45_degree_direction() {
        let offset = (1.0, 1.0);
        let direction = local_offset_to_direction(offset.0, offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(offset.0, converted_offset.0, epsilon = 1e-6);
        assert_abs_diff_eq!(offset.1, converted_offset.1, epsilon = 1e-6);
    }

    #[test]
    fn test_max_local_offset() {
        let max_offset = (HALF_EQUATORIAL_GRID as f32, 0.0);
        let direction = local_offset_to_direction(max_offset.0, max_offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(
            max_offset.0 as f64,
            converted_offset.0 as f64,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            max_offset.1 as f64,
            converted_offset.1 as f64,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_min_local_offset() {
        let min_offset = (-HALF_EQUATORIAL_GRID as f32, 0.0);
        let direction = local_offset_to_direction(min_offset.0, min_offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(
            min_offset.0 as f64,
            converted_offset.0 as f64,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            min_offset.1 as f64,
            converted_offset.1 as f64,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_near_extreme_values() {
        let near_max_offset = (HALF_EQUATORIAL_GRID as f32 - 1e-3, 0.0);
        let direction = local_offset_to_direction(near_max_offset.0, near_max_offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(
            near_max_offset.0 as f64,
            converted_offset.0 as f64,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            near_max_offset.1 as f64,
            converted_offset.1 as f64,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_direction_z_component_1() {
        let direction = Vector3::new(0.0, 0.0, 1.0);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(0.0, converted_offset.0 as f64, epsilon = 1e-6);
        assert_abs_diff_eq!(250.0, converted_offset.1 as f64, epsilon = 1e-6);
    }

    #[test]
    fn test_direction_z_component_neg_1() {
        let direction = Vector3::new(0.0, 0.0, -1.0);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(0.0, converted_offset.0 as f64, epsilon = 1e-6);
        assert_abs_diff_eq!(-250.0, converted_offset.1 as f64, epsilon = 1e-6);
    }

    #[test]
    fn test_non_symmetric_offset() {
        let offset = (2.0, 3.0);
        let direction = local_offset_to_direction(offset.0, offset.1);
        let converted_offset = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(offset.0 as f64, converted_offset.0 as f64, epsilon = 1e-6);
        assert_abs_diff_eq!(offset.1 as f64, converted_offset.1 as f64, epsilon = 1e-6);
    }

    #[test]
    fn test_max_z_component() {
        let direction = Vector3::new(0.0, 0.0, 1.0);
        let (x, y) = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(0.0, x as f64, epsilon = 1e-6);
        assert_abs_diff_eq!(250.0, y as f64, epsilon = 1e-6);

        let converted_direction = local_offset_to_direction(x, y);
        assert_abs_diff_eq!(direction.x, converted_direction.x, epsilon = 1e-6);
        assert_abs_diff_eq!(direction.y, converted_direction.y, epsilon = 1e-6);
        assert_abs_diff_eq!(direction.z, converted_direction.z, epsilon = 1e-6);
    }

    #[test]
    fn test_min_z_component() {
        let direction = Vector3::new(0.0, 0.0, -1.0);
        let (x, y) = direction_to_local_offset(&direction);
        assert_abs_diff_eq!(0.0, x as f64, epsilon = 1e-6);
        assert_abs_diff_eq!(-250.0, y as f64, epsilon = 1e-6);

        let converted_direction = local_offset_to_direction(x, y);
        assert_abs_diff_eq!(direction.x, converted_direction.x, epsilon = 1e-6);
        assert_abs_diff_eq!(direction.y, converted_direction.y, epsilon = 1e-6);
        assert_abs_diff_eq!(direction.z, converted_direction.z, epsilon = 1e-6);
    }

    #[test]
    fn test_180_degree_rotation() {
        let offset = (HALF_EQUATORIAL_GRID as f32, 0.0);
        let direction = local_offset_to_direction(offset.0, offset.1);
        assert_abs_diff_eq!(direction.x, -Vector3::new(1.0, 0.0, 0.0).x, epsilon = 1e-6);
    }

    #[test]
    fn test_arbitrary_direction() {
        let direction = Vector3::new(0.5, 0.5, 0.70710678); // 接近 (45°, 45°)
        let (x, y) = direction_to_local_offset(&direction);
        let converted_direction = local_offset_to_direction(x, y);
        assert_abs_diff_eq!(direction.x, converted_direction.x, epsilon = 1e-6);
        assert_abs_diff_eq!(direction.y, converted_direction.y, epsilon = 1e-6);
        assert_abs_diff_eq!(direction.z, converted_direction.z, epsilon = 1e-6);
    }
}

#[cfg(test)]
mod test_quaternion {
    use super::*;

    #[test]
    fn test_same_vector() {
        let from = Vector3::new(1.0, 0.0, 0.0).normalize();
        let to = from.clone();

        let (q, q_inv) = calculate_quaternion_between_vectors(&from, &to);

        // 预期四元数为单位四元数及其共轭
        assert!(abs_diff_eq!(q.w, 1.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.i, 0.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.j, 0.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.k, 0.0, epsilon = 1e-6));

        // 共轭四元数应与原四元数相同
        assert!(abs_diff_eq!(q_inv.w, 1.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.i, 0.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.j, 0.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.k, 0.0, epsilon = 1e-6));
    }

    // TODO 检查测试用例，查漏补缺

    #[test]
    fn test_orthogonal_vectors() {
        let from = Vector3::new(1.0, 0.0, 0.0).normalize();
        let to = Vector3::new(0.0, 1.0, 0.0).normalize();

        let (q, q_inv) = calculate_quaternion_between_vectors(&from, &to);

        // 预期四元数为绕z轴旋转90度，即w=√2/2 ≈ 0.7071，k=√2/2
        let expected_q = Quaternion::new(0.70710678, 0.0, 0.0, 0.70710678);

        assert!(abs_diff_eq!(q.w, expected_q.w, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.i, expected_q.i, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.j, expected_q.j, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.k, expected_q.k, epsilon = 1e-6));

        // 检查共轭四元数
        let expected_inv = expected_q.conjugate();
        assert!(abs_diff_eq!(q_inv.w, expected_inv.w, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.i, expected_inv.i, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.j, expected_inv.j, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.k, expected_inv.k, epsilon = 1e-6));
    }

    #[test]
    fn test_opposite_vectors() {
        let from = Vector3::new(1.0, 0.0, 0.0).normalize();
        let to = Vector3::new(-1.0, 0.0, 0.0).normalize();

        let (q, q_inv) = calculate_quaternion_between_vectors(&from, &to);

        // 预期四元数：绕某个正交轴旋转180度
        // 根据函数逻辑，会选择与from正交的轴（例如y或z）
        // 选择一个可能的预期值，假设轴为 (0, 1/√2, 1/√2)
        let expected_axis = Vector3::new(0.0, 1.0 / f64::sqrt(2.0), 1.0 / f64::sqrt(2.0));
        let expected_q = Quaternion::new(0.0, 0.0, expected_axis.y, expected_axis.z);

        assert!(abs_diff_eq!(q.w, 0.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.i, 0.0, epsilon = 1e-6));
        // 检查j和k分量是否为±√2/2，并且符号正确
        assert!(abs_diff_eq!((q.j).abs(), expected_axis.y, epsilon = 1e-6));
        assert!(abs_diff_eq!((q.k).abs(), expected_axis.z, epsilon = 1e-6));

        // 检查共轭四元数是否正确
        let expected_inv = expected_q.conjugate();
        assert!(abs_diff_eq!(q_inv.w, expected_inv.w, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.i, expected_inv.i, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.j, expected_inv.j, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.k, expected_inv.k, epsilon = 1e-6));
    }

    #[test]
    fn test_almost_colinear() {
        let from = Vector3::new(1.0, 0.0, 0.0).normalize();
        let to = Vector3::new(0.999999, 0.0, 0.000001).normalize();

        let (q, _) = calculate_quaternion_between_vectors(&from, &to);

        // 预期四元数接近单位四元数，旋转角度很小
        assert!(abs_diff_eq!(q.w, 1.0, epsilon = 1e-3));
        assert!(abs_diff_eq!(q.i, 0.0, epsilon = 1e-3));
        assert!(abs_diff_eq!(q.j, 0.0, epsilon = 1e-3));
        // k分量应非常小
        assert!(abs_diff_eq!(q.k.abs(), 0.0, epsilon = 1e-3));
    }

    #[test]
    fn test_cos_theta_out_of_range() {
        let from = Vector3::new(1.0, 0.0, 0.0).normalize();
        // 构造一个cos θ = 1.0000001的情况
        let to = Vector3::new(1.00000005, 0.0, 0.0).normalize();

        let (q, q_inv) = calculate_quaternion_between_vectors(&from, &to);

        // 预期cos θ 被截断为1.0，返回单位四元数
        assert!(abs_diff_eq!(q.w, 1.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.i, 0.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.j, 0.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q.k, 0.0, epsilon = 1e-6));

        // 检查共轭四元数
        assert!(abs_diff_eq!(q_inv.w, 1.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.i, 0.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.j, 0.0, epsilon = 1e-6));
        assert!(abs_diff_eq!(q_inv.k, 0.0, epsilon = 1e-6));
    }
}
