use lazy_static::lazy_static;
use log::info;
use uuid::Uuid;

use dspbptk::{
    blueprint::{
        content::{building::DspbptkBuildingData, ContentData},
        header::HeaderData,
    },
    edit::{
        belt::connect_belts,
        fix_dspbptk_buildings_index,
        tesselation::{calculate_next_y, Row},
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

lazy_static! {
    static ref half_arc_b_tan: f64 = HALF_ARC_B.tan();
    static ref half_arc_a_tan: f64 = HALF_ARC_A.tan();
    static ref half_arc_b_tan_pow2: f64 = half_arc_b_tan.powi(2);
    static ref half_arc_a_tan_pow2: f64 = half_arc_a_tan.powi(2);
    static ref norm_sq: f64 = *half_arc_b_tan_pow2 + *half_arc_a_tan_pow2 + 1.0;
    static ref scale: f64 = (1.0 - (*half_arc_b_tan_pow2 / *norm_sq)).sqrt();
    static ref theta_down: f64 = ((*half_arc_a_tan / norm_sq.sqrt()).sin() / *scale).asin();
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
            let y_fixed =
                match calculate_next_y(rows.last().unwrap().y + HALF_ARC_A, *scale, *theta_down) {
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
        .map(|i| DspbptkBuildingData {
            uuid: Some(Uuid::new_v4().to_u128_le()),
            item_id: Item::射线接收站 as i16,
            model_index: Item::射线接收站.model()[0],
            local_offset: [
                (1000.0 / (row.n as f64) * (i as f64 + 0.5)),
                grid_from_arc(row.y),
                0.0,
            ],
            parameters: vec![1208],
            ..Default::default()
        })
        .collect::<Vec<_>>()
}

fn row_to_belts(row: &Row) -> Vec<DspbptkBuildingData> {
    const BELT_GRID: f64 = 1.83;
    const BELT_ARC: f64 = arc_from_grid(BELT_GRID);

    // 生成传送带点位
    let y = row.y - HALF_ARC_A;
    let x_protect = arc_from_grid(1.0);
    let x_from = x_protect / y.cos();
    let x_to = (2.0 * PI) - x_protect / y.cos();
    let x_arc = x_to - x_from;
    let belts_count = (y.cos() * (x_arc / BELT_ARC)).ceil() as u64;

    (0..=belts_count)
        .map(|i| DspbptkBuildingData {
            uuid: Some(Uuid::new_v4().to_u128_le()),
            item_id: Item::极速传送带 as i16,
            model_index: Item::极速传送带.model()[0],
            local_offset: [
                grid_from_arc(x_arc / (belts_count as f64) * (i as f64) + x_from),
                grid_from_arc(y),
                0.0,
            ],
            ..Default::default()
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

                let photon_belt_1 = DspbptkBuildingData {
                    uuid: Some(Uuid::new_v4().to_u128_le()),
                    item_id: Item::极速传送带 as i16,
                    model_index: Item::极速传送带.model()[0],
                    local_offset: [
                        receiver.local_offset[0],
                        receiver.local_offset[1] + y_scale * HALF_GRID_A * (1.0 / 3.0),
                        receiver.local_offset[2],
                    ],
                    ..Default::default()
                };

                let photon_belt_2 = DspbptkBuildingData {
                    uuid: Some(Uuid::new_v4().to_u128_le()),
                    item_id: Item::极速传送带 as i16,
                    model_index: Item::极速传送带.model()[0],
                    local_offset: [
                        receiver.local_offset[0],
                        receiver.local_offset[1] + y_scale * HALF_GRID_A * (2.0 / 3.0),
                        receiver.local_offset[2],
                    ],
                    ..Default::default()
                };

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

fn receiver_input(
    rows_index: usize,
    row_of_receivers: &Vec<DspbptkBuildingData>,
    belts_in_rows: &Vec<Vec<DspbptkBuildingData>>,
) -> Vec<DspbptkBuildingData> {
    row_of_receivers
        .iter()
        .map(|receiver| {
            let (y_scale, main_belts, sorter_yaw) = if rows_index % 2 == 0 {
                (1.0, &belts_in_rows[rows_index + 1], 180.0)
            } else {
                (-1.0, &belts_in_rows[rows_index], 0.0)
            };

            let lens_belt_into_receiver = DspbptkBuildingData {
                uuid: Some(Uuid::new_v4().to_u128_le()),
                item_id: Item::极速传送带 as i16,
                model_index: Item::极速传送带.model()[0],
                local_offset: [
                    receiver.local_offset[0],
                    receiver.local_offset[1] + y_scale * HALF_GRID_A * (1.0 / 3.0),
                    receiver.local_offset[2],
                ],
                ..Default::default()
            };

            let lens_belt_from_sorter = DspbptkBuildingData {
                uuid: Some(Uuid::new_v4().to_u128_le()),
                item_id: Item::极速传送带 as i16,
                model_index: Item::极速传送带.model()[0],
                local_offset: [
                    receiver.local_offset[0],
                    receiver.local_offset[1] + y_scale * HALF_GRID_A * (2.0 / 3.0),
                    receiver.local_offset[2],
                ],
                ..Default::default()
            };

            let nearest_main_belt_node = main_belts
                .iter()
                .min_by(|a, b| {
                    distance_sq(a, &lens_belt_from_sorter)
                        .partial_cmp(&distance_sq(b, &lens_belt_from_sorter))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();

            let lens_sorter_local_offset = [
                receiver.local_offset[0],
                receiver.local_offset[1] + y_scale * (HALF_GRID_A - 0.25),
                receiver.local_offset[2],
            ];

            let lens_sorter = DspbptkBuildingData {
                uuid: Some(Uuid::new_v4().to_u128_le()),
                item_id: Item::分拣器 as i16,
                model_index: Item::分拣器.model()[0],
                yaw: sorter_yaw,
                yaw2: sorter_yaw,
                local_offset: lens_sorter_local_offset,
                local_offset_2: lens_belt_from_sorter.local_offset,
                temp_input_obj_idx: nearest_main_belt_node.uuid,
                temp_output_obj_idx: lens_belt_from_sorter.uuid,
                output_to_slot: -1,
                input_from_slot: -1,
                output_from_slot: 0,
                input_to_slot: 1,
                ..Default::default()
            };

            let mut receiver_input = connect_belts(
                vec![lens_belt_from_sorter, lens_belt_into_receiver],
                None,
                0,
                receiver.uuid,
                0,
            );

            receiver_input.push(lens_sorter);
            receiver_input
        })
        .collect::<Vec<_>>()
        .concat()
}

fn receiver_io_belts(
    receivers_in_rows: &Vec<Vec<DspbptkBuildingData>>,
    belts_in_rows: &Vec<Vec<DspbptkBuildingData>>,
) -> Vec<Vec<DspbptkBuildingData>> {
    receivers_in_rows
        .iter()
        .enumerate()
        .map(|(rows_index, row_of_receivers)| {
            let receiver_output = receiver_output(
                rows_index,
                receivers_in_rows.len() - 1,
                row_of_receivers,
                belts_in_rows,
            );

            let receiver_input = receiver_input(rows_index, row_of_receivers, belts_in_rows);

            vec![receiver_output, receiver_input].concat()
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
    let receiver_io_belts = receiver_io_belts(&receivers_in_rows, &belts_in_rows);

    // 整合所有种类的建筑
    let all_buildings_in_rows = vec![receivers_in_rows, belts_in_rows, receiver_io_belts].concat();

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
