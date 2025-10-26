use std::f64::consts::PI;

use nalgebra::Vector3;

// TODO 跨纬度的坐标计算

/// 纬线长度(格)
pub const EQUATORIAL_GRID: f64 = 1000.0;

/// 经线长度(格)
pub const HALF_EQUATORIAL_GRID: f64 = 500.0;

/// 蓝图高度0对应的行星半径(M)
pub const EARTH_R: f64 = 200.2;

/// 蓝图高度抬高一格对应的距离(M)
pub const UNIT_Z: f64 = 4.0 / 3.0;

#[must_use]
pub fn arc_from_m(m: f64, y_offset: f64) -> f64 {
    let r = EARTH_R + y_offset;
    let rr2 = r * r * 2.0;
    (m.mul_add(-m, rr2) / rr2).acos()
}

#[must_use]
pub const fn arc_from_grid(grid: f64) -> f64 {
    grid * (PI / HALF_EQUATORIAL_GRID)
}

#[must_use]
pub const fn grid_from_arc(arc: f64) -> f64 {
    arc * (HALF_EQUATORIAL_GRID / PI)
}

// TODO 建筑旋转后的角度修复

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

    let theta_x = local_offset.x * ANGLE_SCALE;
    let theta_y = local_offset.y * ANGLE_SCALE;

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
