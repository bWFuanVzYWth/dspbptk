use dspbptk::{
    blueprint::{
        content::{building::BuildingData, ContentData},
        header::HeaderData,
    },
    edit::{
        compute_3d_rotation_vector,
        tesselation::Row,
        unit_conversion::{arc_from_grid, grid_from_arc, EQUATORIAL_CIRCUMFERENCE_GRID},
    },
    error::DspbptkError,
    io::{BlueprintKind, FileType},
    item::Item,
};

use nalgebra::{Quaternion, Vector3};

use std::f64::consts::PI;

const ERROR: f64 = 0.0;
const GRID_A: f64 = 7.30726 + ERROR;
const GRID_B: f64 = 4.19828 + ERROR;

fn new_ray_receiver(index: i32, local_offset: [f32; 3]) -> BuildingData {
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

fn calculate_circumference(y: f64) -> f64 {
    (y * ((PI / 2.0) / (EQUATORIAL_CIRCUMFERENCE_GRID / 4.0))).cos() * EQUATORIAL_CIRCUMFERENCE_GRID
}

fn calculate_rows() -> Vec<Row> {
    const ARC_A: f64 = arc_from_grid(GRID_A);
    const ARC_B: f64 = arc_from_grid(GRID_B);
    const HALF_ARC_A: f64 = arc_from_grid(GRID_A / 2.0);
    const HALF_ARC_B: f64 = arc_from_grid(GRID_B / 2.0);

    // 求出建筑放在赤道上时的底角坐标
    let position_down_1eft = Vector3::new(-HALF_ARC_B.tan(), 1.0, -HALF_ARC_A.tan()).normalize();
    let position_down_right = Vector3::new(HALF_ARC_B.tan(), 1.0, -HALF_ARC_A.tan()).normalize();

    let mut rows = Vec::new();

    // 生成贴着赤道的一圈
    let row_0 = Row {
        t: Item::射线接收站,
        y: HALF_ARC_A,
        n: ((2.0 * PI) / ARC_A).floor() as u64,
    };
    rows.push(row_0);

    loop {
        // 尝试在数量不变的情况下偏移一整行
        let row_try_offset = Row {
            t: Item::射线接收站,
            y: rows.last().unwrap().y + ARC_A,
            n: rows.last().unwrap().n,
        };

        let row_next = if (row_try_offset.y + ARC_B / 2.0).cos()
            < row_try_offset.n as f64 * ARC_A
        {
            // 如果这一行太挤了

            // 把建筑旋转到目标纬度
            // 求旋转角
            let k = (1.0 - position_down_right.x * position_down_right.x).sqrt();
            let theta_up_tmp = (rows.last().unwrap().y + HALF_ARC_A).sin() / k;
            if theta_up_tmp >= 1.0 {
                break;
            }
            let theta_up = theta_up_tmp.asin();
            let theta_down = ((-(position_down_1eft.z)).sin() / k).asin();
            let theta = theta_up + theta_down;

            let half_theta = theta / 2.0;
            let q = Quaternion::new(half_theta.cos(), half_theta.sin(), 0.0, 0.0);
            let inv_q = q.conjugate();

            if theta > PI / 2.0 {
                break;
            };

            let position_down_1eft_rotated =
                compute_3d_rotation_vector(&(position_down_1eft), (q, inv_q));
            let position_down_right_rotated =
                compute_3d_rotation_vector(&(position_down_right), (q, inv_q));

            // 求出建筑底边中心的y
            let tmp = (position_down_1eft_rotated + position_down_right_rotated).normalize();
            let y_fixed = tmp.z.asin() + HALF_ARC_A; // 下一排建筑的中心y
            let n = ((y_fixed + HALF_ARC_B).cos() * ((2.0 * PI) / ARC_A)).floor() as u64;

            let row = Row {
                t: Item::射线接收站,
                y: y_fixed,
                n: n,
            };
            row
        } else {
            // 如果这一行放得下
            if row_try_offset.y > (2.0 * PI) {
                break;
            }
            row_try_offset
        };

        rows.push(row_next);
    }

    rows
}

fn convert_row(row: &Row) -> Vec<BuildingData> {
    let row_buildings: Vec<_> = (0..row.n)
        .map(|i| {
            new_ray_receiver(
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

fn convert_rows(rows: Vec<Row>) -> Vec<BuildingData> {
    let all_buildings_in_rows: Vec<_> = rows.iter().map(|row| convert_row(row)).collect();
    let all_buildings: Vec<_> = all_buildings_in_rows
        .concat()
        .iter()
        .enumerate()
        .map(|(i, building)| {
            let mut building_fixed = building.clone();
            building_fixed.index = i as i32;
            building_fixed
        })
        .collect();
    all_buildings
}

fn main() -> Result<(), DspbptkError<'static>> {
    let header_data = HeaderData::default();
    let zopfli_options = zopfli::Options::default();

    // 先计算布局
    // 记录每一行的中心坐标和锅盖数量
    let rows = calculate_rows();

    let buildings = convert_rows(rows);

    let content_data = ContentData {
        buildings_length: buildings.len() as i32,
        buildings: buildings,
        ..Default::default()
    };

    // println!("{:#?}", content_data);

    if let BlueprintKind::Txt(blueprint) =
        dspbptk::io::process_back_end(&header_data, &content_data, &zopfli_options, &FileType::Txt)?
    {
        print!("{}", blueprint);
    }

    Ok(())
}
