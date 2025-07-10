use std::f64::consts::PI;

use nalgebra::Vector3;

pub const EARTH_R: f64 = 200.0;
pub const HALF_EQUATORIAL_GRID: f64 = 500.0;

// 将方向向量转换为局部偏移
#[must_use]
pub fn direction_to_local_offset(direction: &Vector3<f64>, z: f64) -> Vector3<f64> {
    const ANGLE_SCALE: f64 = HALF_EQUATORIAL_GRID / PI;

    let theta_x = direction.y.atan2(direction.x);
    let x = theta_x * ANGLE_SCALE;

    let theta_z = direction.z.asin();
    let y = theta_z * ANGLE_SCALE;

    let x = fix_value(x, direction.x, 500.0, -500.0);
    let y = fix_value(y, direction.z, 250.0, -250.0);

    Vector3::new(x, y, z)
}

// 将局部偏移转换为方向向量
#[must_use]
pub fn local_offset_to_direction(local_offset: Vector3<f64>) -> Vector3<f64> {
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
