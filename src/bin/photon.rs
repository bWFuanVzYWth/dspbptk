use dspbptk::{
    blueprint::{
        content::{building::BuildingData, ContentData},
        header::HeaderData,
    },
    error::DspbptkError,
    io::{BlueprintKind, FileType},
};

use std::f64::consts::PI;

fn new_ray_receiver(index: i32, local_offset: [f32; 3]) -> BuildingData {
    BuildingData {
        index: index,
        item_id: 2208,
        model_index: 73,
        local_offset_x: local_offset[0],
        local_offset_y: local_offset[1],
        local_offset_z: local_offset[2],
        parameters: vec![1208],
        parameters_length: 1,
        ..Default::default()
    }
}

const EQUATORIAL_CIRCUMFERENCE: f64 = 1000.0;
const SIZE_A: f64 = 7.30726;
const SIZE_B: f64 = 4.19828;

struct Row {
    pub y: f64, // 这一行建筑坐标的y
    pub n: u64, // 这一行建筑的数量
}

fn calculate_circumference(y: f64) -> f64 {
    (y * (PI / 2.0) / (EQUATORIAL_CIRCUMFERENCE / 4.0)).cos() * EQUATORIAL_CIRCUMFERENCE
}

fn calculate_rows() -> Vec<Row> {
    let mut rows = Vec::new();

    // 生成贴着赤道的一圈
    let row_0 = Row {
        y: SIZE_A / 2.0,
        n: (EQUATORIAL_CIRCUMFERENCE / SIZE_A).floor() as u64,
    };
    rows.push(row_0);

    loop {
        // 尝试在数量不变的情况下偏移一整行
        let row_try_offset = Row {
            y: rows.last().unwrap().y + SIZE_A,
            n: rows.last().unwrap().n,
        };

        let row_next = if calculate_circumference(row_try_offset.y + SIZE_B / 2.0)
            < row_try_offset.n as f64 * SIZE_A
        {
            // 如果这一行太挤了
            let vector_theta = SIZE_A.atan2(SIZE_B);
            let vector_length = (SIZE_A * SIZE_A + SIZE_B * SIZE_B).sqrt() / 2.0;

            // FIXME 这个补偿有点太多了，不知道怎么算
            // 旋转建筑的对角线计算补偿值
            let max_latitude_error = PI / ((rows.last().unwrap().n - 1) as f64);
            // let max_latitude_error = PI / ((rows.last().unwrap().n * 2) as f64);
            let y_fixed = (vector_theta + max_latitude_error).min(PI / 2.0).sin() * vector_length
                + SIZE_A / 2.0;

            let y = rows.last().unwrap().y + y_fixed;
            let n = (calculate_circumference(y + SIZE_B / 2.0) / SIZE_A).floor() as u64;

            Row { y: y, n: n }
        } else {
            // 如果这一行放得下
            row_try_offset
        };

        if row_next.y > EQUATORIAL_CIRCUMFERENCE / 4.0 {
            break;
        }

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
                    row.y as f32,
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
