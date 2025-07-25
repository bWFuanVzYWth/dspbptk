use dspbptk::{
    blueprint::data::{content::Content, header::Header},
    dspbptk_blueprint::{
        Building,
        editor::{belt::connect_belts, fix_uuid::fix_dspbptk_buildings_index},
        generator::tesselation::{Module, module::receiver_1i1o},
        uuid::some_new_uuid as new_some_uuid,
    },
    error::DspbptkError::{self, UnexpectBuildingsCount},
    item::Item,
    planet::unit_conversion::{arc_from_grid, grid_from_arc, local_offset_to_direction},
    workflow::{BlueprintKind, LegalBlueprintFileType, process::process_back_end},
};
use nalgebra::Vector3;
use std::{cmp::Ordering::Equal, f64::consts::TAU};

// FIXME 改用tesselation::Row
#[derive(Debug)]
struct Row {
    pub y: f64, // 这一行建筑坐标的中心
    pub n: i64, // 这一行建筑的数量
}

// 当error=0时，期望输出2920锅；然后在锅不减少的情况下试出最大的error(0.00019, 0.00020)
// 考虑行星尺寸与IEEE754标准，至少要让ERROR > 2^-15 (约0.00003)
const ERROR: f64 = 0.00019;
// 锅的尺寸数据由src/bin/test_ray_receiver_size测出
const GRID_A: f64 = receiver_1i1o::GRID_A + ERROR;
const GRID_B: f64 = receiver_1i1o::GRID_B + ERROR;
const HALF_GRID_A: f64 = GRID_A / 2.0;
const HALF_GRID_B: f64 = GRID_B / 2.0;

const ARC_A: f64 = arc_from_grid(GRID_A);
const ARC_B: f64 = arc_from_grid(GRID_B);
const HALF_ARC_A: f64 = arc_from_grid(HALF_GRID_A);
const HALF_ARC_B: f64 = arc_from_grid(HALF_GRID_B);

fn calculate_layout() -> Vec<Row> {
    let module = Module::new(GRID_A, GRID_B);

    let mut rows = Vec::new();

    // 生成贴着赤道的一圈
    let row_0 = Row {
        y: HALF_ARC_A,
        n: (TAU / ARC_A).floor() as i64,
    };
    rows.push(row_0);

    loop {
        // 尝试直接偏移一行
        let row_try_offset = Row {
            y: rows.last().unwrap().y + ARC_A,
            n: rows.last().unwrap().n,
        };

        let row_next = if (row_try_offset.y + ARC_B / 2.0).cos() < row_try_offset.n as f64 * ARC_A {
            // 如果直接偏移太挤了
            let Some(y_fixed) = module.calculate_next_y(rows.last().unwrap().y + HALF_ARC_A) else {
                break;
            };
            let n = ((y_fixed + HALF_ARC_B).cos() * (TAU / ARC_A)).floor() as i64;
            Row { y: y_fixed, n }
        } else {
            // 如果直接偏移放得下
            if row_try_offset.y > TAU {
                break;
            }
            row_try_offset
        };

        rows.push(row_next);
    }

    rows
}

fn find_nearest(buildings: &[Building], reference_local_offset: Vector3<f64>) -> &Building {
    buildings
        .iter()
        .max_by(|a, b| {
            let ref_direction = local_offset_to_direction(reference_local_offset);
            let a_direction = local_offset_to_direction(a.local_offset);
            let b_direction = local_offset_to_direction(b.local_offset);
            let cos_arc_a = ref_direction.dot(&a_direction);
            let cos_arc_b = ref_direction.dot(&b_direction);
            cos_arc_a.partial_cmp(&cos_arc_b).unwrap_or(Equal)
        })
        .expect("fatal error: can not find nearest buildings")
}

fn receivers_with_io(
    row: &Row,
    lens_belts: &[Building],
    photons_belts: &[Building],
) -> Vec<Building> {
    (0..row.n)
        .map(|i| {
            let local_offset = Vector3::new(
                1000.0 * (i as f64 + 0.5) / (row.n as f64),
                grid_from_arc(row.y),
                0.0,
            );

            let output_to_slot = if local_offset[1] > photons_belts[0].local_offset[1] {
                2
            } else {
                3
            };

            let nearest_lens_belt = find_nearest(lens_belts, local_offset);
            let nearest_photons_belt = find_nearest(photons_belts, local_offset);

            receiver_1i1o::new(
                local_offset,
                nearest_lens_belt,
                -1,
                nearest_photons_belt,
                output_to_slot,
            )
        })
        .collect::<Vec<_>>()
        .concat()
}

fn main_belts(row: &Row) -> Vec<Building> {
    const BELT_GRID: f64 = 1.83;
    const BELT_ARC: f64 = arc_from_grid(BELT_GRID);

    // 生成传送带点位
    let y = row.y - HALF_ARC_A;
    let x_protect = arc_from_grid(1.0);
    let x_from = x_protect / y.cos();
    let x_to = TAU - x_from;
    let x_arc = x_to - x_from;
    let belts_count = (y.cos() * (x_arc / BELT_ARC)).ceil() as u64;

    (0..=belts_count)
        .map(|i| Building {
            uuid: new_some_uuid(),
            item_id: Item::极速传送带 as i16,
            model_index: Item::极速传送带.model().default_value(),
            local_offset: Vector3::new(
                grid_from_arc(x_arc.mul_add((i as f64) / (belts_count as f64), x_from)),
                grid_from_arc(y),
                0.0,
            ),
            ..Default::default()
        })
        .collect::<Vec<_>>()
}

fn layout_to_buildings(rows: &[Row]) -> Vec<Building> {
    // 生成主干传送带
    let belts_in_rows = rows
        .iter()
        .map(|row| {
            let row_of_belt = main_belts(row);
            connect_belts(&row_of_belt, None, 0, None, 0)
        })
        .collect::<Vec<_>>();

    // 生成所有锅盖
    let receivers_in_rows = rows
        .iter()
        .take(rows.len() - 1) // 跳过最后一行
        .enumerate()
        .map(|(i, row)| {
            let (lens_belts, photons_belts) = if i.is_multiple_of(2) {
                (&belts_in_rows[i + 1], &belts_in_rows[i])
            } else {
                (&belts_in_rows[i], &belts_in_rows[i + 1])
            };
            receivers_with_io(row, lens_belts, photons_belts)
        })
        .collect::<Vec<_>>();

    // 整合所有种类的建筑
    let all_buildings_in_rows = [belts_in_rows, receivers_in_rows].concat();

    let all_buildings = all_buildings_in_rows.concat();

    fix_dspbptk_buildings_index(all_buildings)
}

fn main() -> Result<(), DspbptkError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let header_data = Header::default();
    let zopfli_options = zopfli::Options::default();

    // 先计算布局
    let rows = calculate_layout();

    // 再转换为建筑列表
    let buildings = layout_to_buildings(&rows);

    let content_data = Content {
        buildings_length: u32::try_from(buildings.len()).map_err(UnexpectBuildingsCount)?,
        buildings: buildings
            .into_iter()
            .map(|dspbptk_building| dspbptk_building.try_into().unwrap())
            .collect(),
        ..Default::default()
    };

    if let BlueprintKind::Txt(blueprint) = process_back_end(
        &header_data,
        &content_data,
        &zopfli_options,
        &LegalBlueprintFileType::Txt,
    )? {
        // cargo run --bin photon --release > "C:\Users\%USERNAME%\Documents\Dyson Sphere Program\Blueprint\receiver2920.txt"
        print!("{blueprint}");
    }

    Ok(())
}
