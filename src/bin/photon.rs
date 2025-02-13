use lazy_static::lazy_static;
use log::info;
use nalgebra::Quaternion;
use uuid::Uuid;

use dspbptk::{
    blueprint::{
        content::{building::DspbptkBuildingData, ContentData},
        header::HeaderData,
    },
    edit::{
        compute_3d_rotation_vector, direction_to_local_offset, fix_dspbptk_buildings_index,
        local_offset_to_direction,
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

fn new_receiver(local_offset: [f64; 3]) -> DspbptkBuildingData {
    DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::射线接收站 as i16,
        model_index: Item::射线接收站.model()[0],
        local_offset: local_offset,
        parameters: vec![1208],
        ..Default::default()
    }
}

fn new_belt(local_offset: [f64; 3]) -> DspbptkBuildingData {
    DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::极速传送带 as i16,
        model_index: Item::极速传送带.model()[0],
        local_offset: local_offset,
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

/// 把vec中的传送带节点连接成一条整体，注意这个函数并不检查建筑是否为传送带  
pub fn connect_belts(
    belts: Vec<DspbptkBuildingData>,
    temp_input_obj_idx: Option<u128>,
    input_from_slot: i8,
    temp_output_obj_idx: Option<u128>,
    output_to_slot: i8,
) -> Vec<DspbptkBuildingData> {
    let nexts = belts
        .iter()
        .enumerate()
        .map(|(i, _belt)| match belts.get(i + 1) {
            Some(belt) => (belt.uuid, 1),
            None => (temp_output_obj_idx, output_to_slot),
        })
        .collect::<Vec<_>>();

    belts
        .into_iter()
        .enumerate()
        .map(|(i, belt)| match i {
            0 => DspbptkBuildingData {
                temp_input_obj_idx: temp_input_obj_idx,
                input_from_slot: input_from_slot,
                input_to_slot: 1,
                temp_output_obj_idx: nexts[i].0,
                output_to_slot: nexts[i].1,
                ..belt
            },
            _ => DspbptkBuildingData {
                temp_output_obj_idx: nexts[i].0,
                output_to_slot: nexts[i].1,
                ..belt
            },
        })
        .collect()
}

/// 沿测地线连接两个传送带节点
pub fn create_belts_path(
    from: DspbptkBuildingData,
    to: &DspbptkBuildingData,
) -> Vec<DspbptkBuildingData> {
    const BELT_GRID: f64 = 1.83;
    const BELT_ARC: f64 = arc_from_grid(BELT_GRID);

    let from_z = from.local_offset[2];
    let to_z = to.local_offset[2];

    let from_direction = local_offset_to_direction(from.local_offset).normalize();
    let to_direction = local_offset_to_direction(to.local_offset).normalize();

    let cos_theta = from_direction.dot(&to_direction);

    // 如果两传送带在球面上相对，此时存在无数条测地线，应拒绝生成并让用户检查
    assert!(cos_theta > -1.0);

    let axis = from_direction.cross(&to_direction).normalize();

    // TODO 优化性能
    // 用四元数球面插值
    let arc_between = cos_theta.acos();
    let belts_count = (arc_between / BELT_ARC).ceil() as i64; // 加上了from，没算to
    let belts = (0..=belts_count)
        .map(|i| {
            if i == 0 {
                from.clone()
            } else if i == belts_count {
                to.clone()
            } else {
                let k = (i as f64) / (belts_count as f64);
                let half_arc_lerp = arc_between * 0.5 * k;
                let q = Quaternion::new(
                    half_arc_lerp.cos(),
                    axis.x * half_arc_lerp.sin(),
                    axis.y * half_arc_lerp.sin(),
                    axis.z * half_arc_lerp.sin(),
                );
                let inv_q = q.conjugate();
                let belt_direction = compute_3d_rotation_vector(&from_direction, (q, inv_q));
                let belt_z = from_z * (1.0 - k) + to_z * k;
                new_belt(direction_to_local_offset(&belt_direction, belt_z))
            }
        })
        .collect::<Vec<_>>();

    connect_belts(belts, None, 0, None, 0)
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

fn row_to_receivers(row: &Row) -> Vec<DspbptkBuildingData> {
    (0..row.n)
        .map(|i| {
            new_receiver([
                (1000.0 / (row.n as f64) * (i as f64 + 0.5)),
                grid_from_arc(row.y),
                0.0,
            ])
        })
        .collect::<Vec<_>>()
}

fn row_to_belts(row: &Row) -> Vec<DspbptkBuildingData> {
    const BELT_GRID: f64 = 1.83;
    const BELT_ARC: f64 = arc_from_grid(BELT_GRID);

    // 生成传送带点位
    let y = row.y - HALF_ARC_A;
    let x_from = HALF_ARC_A / y.cos();
    let x_to = (2.0 * PI) - HALF_ARC_A / y.cos();
    let x_arc = x_to - x_from;
    let belts_count = (y.cos() * (x_arc / BELT_ARC)).ceil() as u64;

    (0..=belts_count)
        .map(|i| {
            new_belt([
                grid_from_arc(x_arc / (belts_count as f64) * (i as f64) + x_from),
                grid_from_arc(y),
                0.0,
            ])
        })
        .collect::<Vec<_>>()
}

pub fn distance_sq(a: &DspbptkBuildingData, b: &DspbptkBuildingData) -> f64 {
    (a.local_offset[0] - b.local_offset[0]).powi(2)
        + (a.local_offset[1] - b.local_offset[1]).powi(2)
        + (a.local_offset[2] - b.local_offset[2]).powi(2)
}

fn photon_belts(
    main_belts: &Vec<DspbptkBuildingData>,
    photon_belt_1: DspbptkBuildingData,
    photon_belt_2: DspbptkBuildingData,
    receiver_uuid: Option<u128>,
    output_to_slot: i8,
) -> Vec<DspbptkBuildingData> {
    let nearest = main_belts
        .iter()
        .min_by(|a, b| {
            distance_sq(a, &photon_belt_1)
                .partial_cmp(&distance_sq(b, &photon_belt_1))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();

    let photon_belts = vec![photon_belt_1, photon_belt_2];
    connect_belts(photon_belts, receiver_uuid, 1, nearest.uuid, output_to_slot)
}

fn receiver_output(
    rows_index: usize,
    rows_index_max: usize,
    row_of_receivers: &Vec<DspbptkBuildingData>,
    belts_in_rows: &Vec<Vec<DspbptkBuildingData>>,
) -> Vec<DspbptkBuildingData> {
    row_of_receivers
        .iter()
        .map(|receiver| {
            if rows_index == rows_index_max {
                Vec::new()
            } else {
                let (y_scale, main_belts, output_to_slot) = if rows_index % 2 == 0 {
                    (-1.0, &belts_in_rows[rows_index], 2)
                } else {
                    (1.0, &belts_in_rows[rows_index + 1], 3)
                };

                let photon_belt_1 = new_belt([
                    receiver.local_offset[0],
                    receiver.local_offset[1] + y_scale * HALF_GRID_A * (1.0 / 3.0),
                    receiver.local_offset[2],
                ]);

                let photon_belt_2 = new_belt([
                    receiver.local_offset[0],
                    receiver.local_offset[1] + y_scale * HALF_GRID_A * (2.0 / 3.0),
                    receiver.local_offset[2],
                ]);

                photon_belts(
                    main_belts,
                    photon_belt_1,
                    photon_belt_2,
                    receiver.uuid,
                    output_to_slot,
                )
            }
        })
        .collect::<Vec<_>>()
        .concat()
}

fn receiver_outputs(
    receivers_in_rows: &Vec<Vec<DspbptkBuildingData>>,
    belts_in_rows: &Vec<Vec<DspbptkBuildingData>>,
) -> Vec<Vec<DspbptkBuildingData>> {
    receivers_in_rows
        .iter()
        .enumerate()
        .map(|(rows_index, row_of_receivers)| {
            receiver_output(
                rows_index,
                receivers_in_rows.len() - 1,
                row_of_receivers,
                belts_in_rows,
            )
        })
        .collect::<Vec<_>>()
}

fn rows_to_buildings(rows: Vec<Row>) -> Vec<DspbptkBuildingData> {
    // 生成传送带
    let belts_in_rows = rows
        .iter()
        .map(|row| {
            let row_of_belt = row_to_belts(row);
            connect_belts(row_of_belt, None, 0, None, 0)
        })
        .collect::<Vec<_>>();
    info!(
        "belt count = {}",
        belts_in_rows.iter().map(|row| row.len()).sum::<usize>()
    );

    // 生成所有锅盖
    let receivers_in_rows = rows
        .iter()
        .map(|row| row_to_receivers(row))
        .collect::<Vec<_>>();
    info!(
        "receiver count = {}",
        receivers_in_rows.iter().map(|row| row.len()).sum::<usize>()
    );

    // 生成锅盖的输入输出传送带
    let receiver_outputs = receiver_outputs(&receivers_in_rows, &belts_in_rows);

    // 整合所有种类的建筑
    let all_buildings_in_rows = vec![receivers_in_rows, belts_in_rows, receiver_outputs].concat();

    let all_buildings = all_buildings_in_rows.concat();
    let all_buildings = fix_dspbptk_buildings_index(all_buildings);

    all_buildings
}

fn main() -> Result<(), DspbptkError<'static>> {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let header_data = HeaderData::default();
    let zopfli_options = zopfli::Options::default();

    // 先计算布局
    let rows = calculate_rows();

    let buildings = rows_to_buildings(rows);

    let content_data = ContentData {
        buildings_length: buildings.len() as i32,
        buildings: buildings
            .iter()
            .map(|dspbptk_building| dspbptk_building.to_building_data())
            .collect(),
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
