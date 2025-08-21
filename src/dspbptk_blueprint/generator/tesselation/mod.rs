pub mod module;

use crate::planet::unit_conversion::arc_from_grid;
use std::f64::consts::FRAC_PI_2;

// TODO 考虑内存布局，在不牺牲性能的前提下尽量优化下面的几个值，如果有必要换成常规vec
/// 最多允许多少行建筑
const MAX_ROW_COUNT: usize = 64;

/// 最多允许计算多少种基础模块
const MAX_MODULE_TYPE_COUNT: usize = 6;

#[derive(Debug)]
pub struct Module {
    pub arc_x: f64,
    pub arc_y: f64,
    pub scale: f64,
    pub theta_down: f64,
}

impl Module {
    #[expect(clippy::similar_names)]
    #[must_use]
    pub fn new(grid_x: f64, grid_y: f64) -> Self {
        let arc_x = arc_from_grid(grid_x);
        let arc_y = arc_from_grid(grid_y);

        let half_arc_x: f64 = arc_x * 0.5;
        let half_arc_y: f64 = arc_y * 0.5;

        let half_arc_x_tan: f64 = half_arc_x.tan();
        let half_arc_y_tan: f64 = half_arc_y.tan();
        let half_arc_x_tan_pow2: f64 = half_arc_x_tan.powi(2);
        let half_arc_y_tan_pow2: f64 = half_arc_y_tan.powi(2);
        let norm_sq: f64 = half_arc_x_tan_pow2 + half_arc_y_tan_pow2 + 1.0;
        let scale: f64 = (1.0 - (half_arc_x_tan_pow2 / norm_sq)).sqrt();
        let theta_down: f64 = ((half_arc_y_tan / norm_sq.sqrt()).sin() / scale).asin();

        Self {
            arc_x,
            arc_y,
            scale,
            theta_down,
        }
    }

    // TODO 补文档，越详细越好，包括背后的数学原理
    // 不完整，用法参考photon.rs
    /// 根据下一行模块尺寸计算中心y，使得模块的最低点高于`edge_y`\
    /// 下一行可能已经超出了纬度限制，不一定存在
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

/// 代表了一行的建筑数据，
struct Row<'a> {
    /// 这一行的建筑类型，也就是建筑在输入数组中对应的下标
    module_type: &'a Module,

    /// 这一行模块的数量，注意是浮点数。不过浮点数可以精确的表示整数所以不用担心误差
    count: f64,

    /// 这一行模块的几何中心的高度，单位是弧度
    center_y: f64,
}

/// 代表了一个缓存了重要数据的中间布局
struct Draft<'a> {
    rows: Vec<Row<'a>>,
    each_type_count: Vec<f64>,
    score: Option<f64>,
}

impl Draft<'_> {
    // TODO 推入一排建筑，包括计算新行的y，数量之类的，更新评分之类的
    pub fn push(self, module_type: &Module) -> Self {
        todo!()
    }
}

/// 这个函数不检查y是否超标，超标解应该放在流程控制中排除
/// 找出最缺的建筑，将其相对需求的倍率作为分数
pub fn score(
    each_type_count: &Vec<f64>,
    need: &Vec<f64>,
) -> Option<f64> {
    // 除零将会产生`positive quiet NaN`，大于所有的数字
    each_type_count
        .iter()
        .zip(need.iter())
        .map(|(module, need)| module / need)
        .min_by(f64::total_cmp)
}

// TODO 基于排列生成具体的蓝图，这个函数必须抽象出去，不然难以保证基于相同的基因生成的蓝图结果一致
// 而且好像还需要考虑渐进式生成，根据已有排列和缓存信息生成下一列

// 输入每种模块的需求比例，纬度限制，输出排列
#[must_use]
pub fn tesselation(modules: &[Module], need: &[f64]) -> Vec<usize> {
    todo!()
}
