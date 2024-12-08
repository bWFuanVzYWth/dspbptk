/// 在蓝图编辑中有通用性的工具箱

use std::f64::consts::PI;

use log::warn;
use nalgebra::{geometry::Quaternion, Vector3};

use crate::blueprint::content::building;

pub const EARTH_R: f64 = 200.0;
pub const HALF_EQUATORIAL_GRID: f64 = 500.0;

pub fn sort_buildings(buildings: &mut Vec<building::BuildingData>) {
    use std::cmp::Ordering::{Equal, Greater, Less};

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

        let item_id_order = a.item_id.cmp(&b.item_id);
        if item_id_order != Equal {
            return item_id_order;
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

    let mut index_lut = HashMap::new();
    buildings.iter().enumerate().for_each(|(index, building)| {
        index_lut.insert(building.index, index as i32);
    });
    buildings.iter_mut().for_each(|building| {
        building.index = *index_lut
            .get(&building.index)
            .unwrap(/* impossible */);
        building.temp_output_obj_idx = *index_lut
            .get(&building.temp_output_obj_idx)
            .unwrap_or(&building::INDEX_NULL);
        building.temp_input_obj_idx = *index_lut
            .get(&building.temp_input_obj_idx)
            .unwrap_or(&building::INDEX_NULL);
    });
}

fn shortest_arc(from: &Vector3<f64>, to: &Vector3<f64>) -> Quaternion<f64> {
    let k_cos_theta = from.dot(to);
    let k = (from.norm_squared() * to.norm_squared()).sqrt();
    if k_cos_theta / k == -1.0 {
        // FIXME 随便选一个正交向量
        let from_cross_to = from.cross(to);
        Quaternion::new(
            k_cos_theta + k,
            from_cross_to.x,
            from_cross_to.y,
            from_cross_to.z,
        )
    } else {
        let from_cross_to = from.cross(to);
        Quaternion::new(
            k_cos_theta + k,
            from_cross_to.x,
            from_cross_to.y,
            from_cross_to.z,
        )
    }
}

fn rotation_vector3(from: &Vector3<f64>, quaternion: Quaternion<f64>) -> Vector3<f64> {
    let from_quaternion = Quaternion::new(0.0, from.x, from.y, from.z);
    // 防呆不防傻，希望用户自行检查四元数是否有正确的数学意义
    let inverse_quaternion = quaternion
        .try_inverse()
        .expect("Fatal error: quaternion must != 0");
    let to_quaternion = quaternion * from_quaternion * inverse_quaternion;
    Vector3::new(to_quaternion.i, to_quaternion.j, to_quaternion.k)
}

// back:x+ right:y+ up:z+
pub fn local_offset_to_direction(local_offset_x: f32, local_offset_y: f32) -> Vector3<f64> {
    let z_normal = (local_offset_y as f64 * (PI / HALF_EQUATORIAL_GRID)).sin();
    let r_normal = (1.0 - z_normal * z_normal).sqrt();
    let y_normal = r_normal * (local_offset_x as f64 * (PI / HALF_EQUATORIAL_GRID)).sin();
    let x_normal = r_normal * (local_offset_x as f64 * (PI / HALF_EQUATORIAL_GRID)).cos();

    let x = x_normal;
    let y = y_normal;
    let z = z_normal;

    Vector3::new(x, y, z)
}

// back:x+ right:y+ up:z+
pub fn direction_to_local_offset(direction: &Vector3<f64>) -> (f32, f32) {
    let x = (direction.x / (1.0 - direction.z * direction.z).sqrt()).acos()
        * if direction.x >= 0.0 {
            HALF_EQUATORIAL_GRID / PI
        } else {
            -HALF_EQUATORIAL_GRID / PI
        };
    let y = direction.z.asin() * (HALF_EQUATORIAL_GRID / PI);

    let x_fix = if x.is_finite() {
        x
    } else {
        if direction.y >= 0.0 {
            0.0
        } else {
            -500.0
        }
    };
    let y_fix = if y.is_finite() {
        y
    } else {
        if direction.z >= 0.0 {
            250.0
        } else {
            -250.0
        }
    };

    if !x.is_finite() || !y.is_finite() {
        warn!("Lost precision: {}, {} -> {}, {}", x, y, x_fix, y_fix);
    }

    (x_fix as f32, y_fix as f32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_direction_to_local_offset_x() {
        let local_offset_origin = direction_to_local_offset(&Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(local_offset_origin.0.abs(), 0.0);
        assert_eq!(local_offset_origin.1.abs(), 0.0);
    }

    #[test]
    fn test_direction_to_local_offset_y() {
        let local_offset_origin = direction_to_local_offset(&Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(local_offset_origin.0.abs(), 250.0);
        assert_eq!(local_offset_origin.1.abs(), 0.0);
    }

    #[test]
    fn test_direction_to_local_offset_z() {
        let local_offset_north_pole = direction_to_local_offset(&Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(local_offset_north_pole.0.abs(), 0.0);
        assert_eq!(local_offset_north_pole.1.abs(), 250.0);
    }

    // TODO 多加几个测试
}
