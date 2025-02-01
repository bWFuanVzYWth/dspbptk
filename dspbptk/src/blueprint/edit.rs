/// 在蓝图编辑中有通用性的工具箱
use std::f64::consts::PI;

use approx::abs_diff_eq;
use log::warn;
use nalgebra::{geometry::Quaternion, Vector3};

use crate::blueprint::content::building;

pub const EARTH_R: f64 = 200.0;
pub const HALF_EQUATORIAL_GRID: f64 = 500.0;

pub fn sort_buildings(buildings: &mut Vec<building::BuildingData>) {
    use std::cmp::Ordering::{Equal, Greater, Less};

    // 按照 item_id, model_index, recipe_id, area_index, local_offset 排序
    buildings.sort_by(|a, b| {
        let item_id_order = a.item_id.cmp(&b.item_id);
        if item_id_order != Equal {
            return item_id_order;
        };

        let model_index_order = a.model_index.cmp(&b.model_index);
        if model_index_order != Equal {
            return model_index_order;
        };

        let recipe_id_order = a.recipe_id.cmp(&b.recipe_id);
        if recipe_id_order != Equal {
            return recipe_id_order;
        };

        let area_index_order = a.area_index.cmp(&b.area_index);
        if area_index_order != Equal {
            return area_index_order;
        };

        const KY: f64 = 256.0;
        const KX: f64 = 1024.0;
        let local_offset_score = |x, y, z| ((y as f64) * KY + (x as f64)) * KX + (z as f64);
        let local_offset_score_a =
            local_offset_score(a.local_offset_x, a.local_offset_y, a.local_offset_z);
        let local_offset_score_b =
            local_offset_score(b.local_offset_x, b.local_offset_y, b.local_offset_z);
        if local_offset_score_a < local_offset_score_b {
            Less
        } else {
            Greater
        }
    });
}

pub fn fix_buildings_index(buildings: &mut Vec<building::BuildingData>) {
    use std::collections::HashMap;

    let mut index_lut = HashMap::with_capacity(buildings.len());
    buildings.iter().enumerate().for_each(|(index, building)| {
        index_lut.insert(building.index, index as i32);
    });
    buildings.iter_mut().for_each(|building| {
        building.index = *index_lut
            .get(&building.index)
            .unwrap(/* impossible */);

        // FIXME 这里静默处理了不存在的索引，至少应该警告
        building.temp_output_obj_idx = *index_lut
            .get(&building.temp_output_obj_idx)
            .unwrap_or(&building::INDEX_NULL);
        building.temp_input_obj_idx = *index_lut
            .get(&building.temp_input_obj_idx)
            .unwrap_or(&building::INDEX_NULL);
    });
}

// 返回两个四元数，第一个是旋转到目标向量的四元数，第二个是它的共轭（即逆）
fn calculate_quaternion_between_vectors(
    from: &Vector3<f64>,
    to: &Vector3<f64>,
) -> (Quaternion<f64>, Quaternion<f64>) {
    // 确保输入向量都是单位向量
    assert!(abs_diff_eq!(from.norm_squared(), 1.0, epsilon = 1e-6));
    assert!(abs_diff_eq!(to.norm_squared(), 1.0, epsilon = 1e-6));

    let cos_theta = from.dot(to);

    // 处理浮点精度问题，确保在[-1.0, 1.0]范围内
    let cos_theta = if cos_theta < -1.0 {
        -1.0
    } else if cos_theta > 1.0 {
        1.0
    } else {
        cos_theta
    };

    // 特殊情况处理：cosθ >= 1.0时，无需旋转
    if cos_theta >= 1.0 {
        return (
            Quaternion::new(1.0, 0.0, 0.0, 0.0),
            Quaternion::new(1.0, 0.0, 0.0, 0.0),
        );
    }

    // 计算叉积得到旋转轴
    let cross = from.cross(to);
    let sin_theta = cross.norm();

    // 当两个向量几乎共线（反向）时，选择一个与from正交的轴
    if sin_theta < 1e-6 {
        let axis = select_orthogonal_axis(from);
        let quaternion = Quaternion::new(0.0, axis.x, axis.y, axis.z).normalize();
        return (quaternion, quaternion.conjugate());
    }

    // 计算旋转轴的单位向量
    let axis = cross.normalize();

    // 计算半角和四元数分量
    let theta = cos_theta.acos();
    let half_theta = theta / 2.0;
    let cos_half = half_theta.cos();
    let sin_half = half_theta.sin();

    let quaternion = Quaternion::new(
        cos_half,
        axis.x * sin_half,
        axis.y * sin_half,
        axis.z * sin_half,
    );

    // 返回四元数及其共轭（即逆）
    (quaternion, quaternion.conjugate())
}

// 选择与给定向量正交的单位向量
fn select_orthogonal_axis(v: &Vector3<f64>) -> Vector3<f64> {
    if v.x.abs() < 1e-6 {
        return Vector3::new(1.0, 0.0, 0.0).normalize();
    }
    let axis = Vector3::new(-v.y - v.z, v.x + v.z, v.x + v.y);
    axis.normalize()
}

// 使用给定的四元数旋转一个向量
fn compute_3d_rotation_vector(
    from: &Vector3<f64>,
    (quaternion, inverse_quaternion): (Quaternion<f64>, Quaternion<f64>),
) -> Vector3<f64> {
    let from_quaternion = Quaternion::new(0.0, from.x, from.y, from.z);

    // 执行旋转变换：q * v * q^(-1)
    let to_quaternion = quaternion * from_quaternion * inverse_quaternion;

    Vector3::new(to_quaternion.i, to_quaternion.j, to_quaternion.k)
}

// back:x+ right:y+ up:z+
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

    let x = radius * cos_theta_x;
    let y = radius * sin_theta_x;

    Vector3::new(x, y, z)
}

// back:x+ right:y+ up:z+
pub fn direction_to_local_offset(direction: &Vector3<f64>) -> (f32, f32) {
    const ANGLE_SCALE: f64 = HALF_EQUATORIAL_GRID / PI;

    if direction.norm_squared() == 0.0 {
        warn!("Zero direction!");
        return (0.0, 0.0);
    }

    // 使用 atan2 计算 theta_x
    let theta_x = direction.y.atan2(direction.x);
    let x = theta_x * ANGLE_SCALE;

    // 计算 theta_z，即 y 偏移的角度
    let theta_z = direction.z.asin();
    let y = theta_z * ANGLE_SCALE;

    // 保持修复逻辑不变
    fn fix_value(
        value: f64,
        direction_component: f64,
        default_positive: f64,
        default_negative: f64,
    ) -> f64 {
        if value.is_finite() {
            value
        } else {
            if direction_component >= 0.0 {
                default_positive
            } else {
                default_negative
            }
        }
    }

    let x_fix = fix_value(x, direction.x, 500.0, -500.0);
    let y_fix = fix_value(y, direction.z, 250.0, -250.0);

    if !x.is_finite() || !y.is_finite() {
        warn!(
            "Lost precision: x = {:?}, y = {:?}, x_fix = {:?}, y_fix = {:?}",
            x, y, x_fix, y_fix
        );
    }

    (x_fix as f32, y_fix as f32)
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
        // 构造一个cosθ=1.0000001的情况
        let to = Vector3::new(1.00000005, 0.0, 0.0).normalize();

        let (q, q_inv) = calculate_quaternion_between_vectors(&from, &to);

        // 预期cosθ被截断为1.0，返回单位四元数
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
