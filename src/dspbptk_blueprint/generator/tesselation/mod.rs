pub mod module;

use crate::planet::unit_conversion::arc_from_grid;
use arrayvec::ArrayVec;
use std::cmp::Ordering;
use std::f64::consts::FRAC_PI_2;

const MODULE_TYPE_COUNT: usize = 6;
const MAX_ROW_COUNT: usize = 44;

/// 列向量，表示了一组排列
type ColumnVector = ArrayVec<u8, MAX_ROW_COUNT>;

// const MODULE_TYPE_COUNT: usize = 6;
// type TotalModule = [usize; MODULE_TYPE_COUNT];
// type NeedModule = [f64; MODULE_TYPE_COUNT];

// TODO 密铺排列计算
// TODO 重构，为不同的模块impl对应的方法

#[derive(Debug)]
pub struct Module {
    pub scale: f64,
    pub theta_down: f64,
}

impl Module {
    #[must_use]
    pub fn new(grid_a: f64, grid_b: f64) -> Self {
        let half_grid_a: f64 = grid_a / 2.0;
        let half_grid_b: f64 = grid_b / 2.0;

        let half_arc_a: f64 = arc_from_grid(half_grid_a);
        let half_arc_b: f64 = arc_from_grid(half_grid_b);

        let half_arc_b_tan: f64 = half_arc_b.tan();
        let half_arc_a_tan: f64 = half_arc_a.tan();
        let half_arc_b_tan_pow2: f64 = half_arc_b_tan.powi(2);
        let half_arc_a_tan_pow2: f64 = half_arc_a_tan.powi(2);
        let norm_sq: f64 = half_arc_b_tan_pow2 + half_arc_a_tan_pow2 + 1.0;
        let scale: f64 = (1.0 - (half_arc_b_tan_pow2 / norm_sq)).sqrt();
        let theta_down: f64 = ((half_arc_a_tan / norm_sq.sqrt()).sin() / scale).asin();

        Self { scale, theta_down }
    }

    // TODO 补文档，越详细越好，包括背后的数学原理
    // 根据下一行模块尺寸计算中心y，使得模块的最低点高于edge_y
    // 不完整，用法参考photon.rs
    #[must_use]
    pub fn calculate_next_y(&self, edge_y: f64) -> Option<f64> {
        let z_max_of_this_row = edge_y.sin();
        let theta_up_sin = z_max_of_this_row / self.scale;
        if theta_up_sin >= 1.0 {
            return None;
        }
        let theta_up = theta_up_sin.asin();
        if theta_up >= FRAC_PI_2 {
            return None;
        }
        Some(theta_up + self.theta_down)
    }
}

/// 状态空间中
#[derive(Clone)]
struct Node {
    rows: ColumnVector,
    score: f64,
    y_max: f64,
}

const _: () = assert!(std::mem::size_of::<Node>() == 64);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.total_cmp(&other.score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.rows.cmp(&other.rows) == Ordering::Equal
    }
}

/// 找出最缺的建筑，将其相对需求的倍率作为分数
fn score(
    each_type_module: &ArrayVec<f64, MODULE_TYPE_COUNT>,
    need: &ArrayVec<f64, MODULE_TYPE_COUNT>,
) -> Option<f64> {
    // 除零将会产生`positive quiet NaN`，大于所有的数字
    each_type_module
        .iter()
        .zip(need.iter())
        .map(|(module, need)| module / need)
        .min_by(f64::total_cmp)
}



impl Node {
    /// 获取从起点到当前节点的准确距离
    pub fn g(&self, need: &ArrayVec<f64, MODULE_TYPE_COUNT>) -> f64 {
        self.
    }
}
