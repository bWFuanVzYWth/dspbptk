use std::f64::consts::{FRAC_PI_2, PI};

use crate::item::Item;

#[derive(Debug)]
pub struct Row {
    pub t: Item,
    pub y: f64, // 这一行建筑坐标的中心
    pub n: u64, // 这一行建筑的数量
}

// TODO 重构，为不同的模块impl对应的方法
// 根据下一行模块尺寸计算中心y，使得模块的最低点高于edge_y
#[must_use]
pub fn calculate_next_y(edge_y: f64, scale: f64, theta_down: f64) -> Option<f64> {
    let z_max_of_this_row = edge_y.sin();
    let theta_up_sin = z_max_of_this_row / scale;
    if theta_up_sin >= 1.0 {
        return None;
    }
    let theta_up = theta_up_sin.asin();
    if theta_up >= FRAC_PI_2 {
        return None;
    }
    Some(theta_up + theta_down)
}
