use lazy_static::lazy_static;
use log::info;

use dspbptk::{
    blueprint::{
        content::{building::BuildingData, ContentData},
        header::HeaderData,
    },
    edit::{
        fix_buildings_index,
        tesselation::Row,
        unit_conversion::{arc_from_grid, grid_from_arc},
    },
    error::DspbptkError,
    io::{BlueprintKind, FileType},
    item::Item,
};

use std::f64::consts::PI;

// 当error=0时，期望输出2920锅；然后在锅不减少的情况下试出最大的error(0.00019, 0.00020)
// 考虑行星尺寸与IEEE754标准，至少要让ERROR > 2^-15 (约0.00003)
const ERROR: f64 = 0.00019;
// 锅的尺寸数据由src/bin/test_ray_receiver_size测出
const GRID_A: f64 = 7.30726 + ERROR;
const GRID_B: f64 = 4.19828 + ERROR;
const HALF_GRID_A: f64 = GRID_A / 2.0;
const HALF_GRID_B: f64 = GRID_B / 2.0;

const ARC_A: f64 = arc_from_grid(GRID_A);
const ARC_B: f64 = arc_from_grid(GRID_B);
const HALF_ARC_A: f64 = arc_from_grid(HALF_GRID_A);
const HALF_ARC_B: f64 = arc_from_grid(HALF_GRID_B);

fn new_receiver(index: i32, local_offset: [f32; 3]) -> BuildingData {
    BuildingData {
        index: index,
        item_id: Item::射线接收站 as i16,
        model_index: Item::射线接收站.model()[0],
        local_offset_x: local_offset[0],
        local_offset_y: local_offset[1],
        local_offset_z: local_offset[2],
        parameters: vec![1208],
        parameters_length: 1,
        ..Default::default()
    }
}

fn new_belt(index: i32, local_offset: [f32; 3]) -> BuildingData {
    BuildingData {
        index: index,
        item_id: Item::极速传送带 as i16,
        model_index: Item::极速传送带.model()[0],
        local_offset_x: local_offset[0],
        local_offset_y: local_offset[1],
        local_offset_z: local_offset[2],
        ..Default::default()
    }
}

fn calculate_y(this_y: f64) -> Option<f64> {
    // 这段代码由我推导出初始的函数后，交给 Mathematica 进行代数化简，再翻译成rust代码
    // 为什么长成这样我也没完全弄明白，但是它算的很快，所以**不要动它**
    lazy_static! {
        static ref half_arc_b_tan: f64 = HALF_ARC_B.tan();
        static ref half_arc_a_tan: f64 = HALF_ARC_A.tan();
        static ref half_arc_b_tan_pow2: f64 = half_arc_b_tan.powi(2);
        static ref half_arc_a_tan_pow2: f64 = half_arc_a_tan.powi(2);
        static ref norm_sq: f64 = *half_arc_b_tan_pow2 + *half_arc_a_tan_pow2 + 1.0;
        static ref scale: f64 = (1.0 - (*half_arc_b_tan_pow2 / *norm_sq)).sqrt();
        static ref theta_down: f64 = ((*half_arc_a_tan / norm_sq.sqrt()).sin() / *scale).asin();
    };

    let z_max_of_this_row = (HALF_ARC_A + this_y).sin();
    let theta_up_sin = z_max_of_this_row / *scale;
    if theta_up_sin >= 1.0 {
        return None;
    }
    let theta_up = theta_up_sin.asin();
    if theta_up >= PI / 2.0 {
        return None;
    }

    Some(theta_up + *theta_down)
}

fn calculate_rows() -> Vec<Row> {
    let mut rows = Vec::new();

    // 生成贴着赤道的一圈
    let row_0 = Row {
        t: Item::射线接收站,
        y: HALF_ARC_A,
        n: ((2.0 * PI) / ARC_A).floor() as u64,
    };
    rows.push(row_0);

    loop {
        // 尝试直接偏移一行
        let row_try_offset = Row {
            t: Item::射线接收站,
            y: rows.last().unwrap().y + ARC_A,
            n: rows.last().unwrap().n,
        };

        let row_next = if (row_try_offset.y + ARC_B / 2.0).cos() < row_try_offset.n as f64 * ARC_A {
            // 如果直接偏移太挤了
            let y_fixed = match calculate_y(rows.last().unwrap().y) {
                Some(num) => num,
                None => break,
            };
            let n = ((y_fixed + HALF_ARC_B).cos() * ((2.0 * PI) / ARC_A)).floor() as u64;
            Row {
                t: Item::射线接收站,
                y: y_fixed,
                n: n,
            }
        } else {
            // 如果直接偏移放得下
            if row_try_offset.y > (2.0 * PI) {
                break;
            }
            row_try_offset
        };

        rows.push(row_next);
    }

    rows
}

fn convert_row_to_receivers(row: &Row) -> Vec<BuildingData> {
    let row_buildings: Vec<_> = (0..row.n)
        .map(|i| {
            new_receiver(
                i as i32,
                [
                    (1000.0 / (row.n as f64) * (i as f64 + 0.5)) as f32,
                    grid_from_arc(row.y) as f32,
                    0.0,
                ],
            )
        })
        .collect();
    row_buildings
}

fn convert_row_to_belts(row: &Row) -> Vec<BuildingData> {
    const BELT_GRID: f64 = 1.83202;
    const BELT_ARC: f64 = arc_from_grid(BELT_GRID);

    // 生成传送带点位
    let y = row.y - HALF_ARC_A;
    let belts_count = (y.cos() * (2.0 * PI / BELT_ARC)).ceil() as u64;
    let row_buildings: Vec<_> = (0..belts_count)
        .map(|i| {
            new_belt(
                i as i32,
                [
                    (1000.0 / (belts_count as f64) * (i as f64)) as f32,
                    grid_from_arc(y) as f32,
                    0.0,
                ],
            )
        })
        .collect();

    //

    row_buildings
}

fn convert_row_to_buildings(rows: Vec<Row>) -> Vec<BuildingData> {
    // 生成所有锅盖
    let receivers_in_rows: Vec<_> = rows
        .iter()
        .map(|row| convert_row_to_receivers(row))
        .collect();
    info!(
        "receiver count = {}",
        receivers_in_rows.iter().map(|row| row.len()).sum::<usize>()
    );

    // 生成传送带
    let belts_in_rows: Vec<_> = rows.iter().map(|row| convert_row_to_belts(row)).collect();
    info!(
        "belt count = {}",
        belts_in_rows.iter().map(|row| row.len()).sum::<usize>()
    );

    // 整合所有种类的建筑
    let all_buildings_in_rows = vec![receivers_in_rows, belts_in_rows].concat();

    let all_buildings: Vec<_> = all_buildings_in_rows.concat();
    let all_buildings = fix_buildings_index(all_buildings);

    all_buildings
}

fn main() -> Result<(), DspbptkError<'static>> {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let header_data = HeaderData::default();
    let zopfli_options = zopfli::Options::default();

    // 先计算布局
    let rows = calculate_rows();

    let buildings = convert_row_to_buildings(rows);

    let content_data = ContentData {
        buildings_length: buildings.len() as i32,
        buildings: buildings,
        ..Default::default()
    };

    if let BlueprintKind::Txt(blueprint) =
        dspbptk::io::process_back_end(&header_data, &content_data, &zopfli_options, &FileType::Txt)?
    {
        // cargo run --bin photon --release > "C:\Users\%USERNAME%\Documents\Dyson Sphere Program\Blueprint\receiver2920.txt"
        print!("{}", blueprint);
    }

    Ok(())
}
