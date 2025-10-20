use dspbptk::{
    blueprint::data::{content::Content, header::Header},
    dspbptk_blueprint::{
        Building,
        editor::{belt::connect_belts, fix_uuid::fix_dspbptk_buildings_index},
        generator::tesselation::{Module, module::receiver_1i1o},
        uuid::some_new_uuid,
    },
    error::DspbptkError::{self, UnexpectBuildingsCount},
    item::Item,
    planet::unit_conversion::{
        arc_from_grid, arc_from_m, grid_from_arc, local_offset_to_direction,
    },
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

fn new_pv(local_offset: Vector3<f64>) -> Building {
    Building {
        uuid: some_new_uuid(),
        item_id: Item::太阳能板 as i16,
        model_index: Item::太阳能板.model().default_value(),
        local_offset,
        parameters: vec![],
        ..Default::default()
    }
}

const ERROR: f64 = 0.00000;

fn calculate_layout() -> Vec<Row> {
    let GRID_PV = grid_from_arc(arc_from_m(3.5, -0.6)) + ERROR;
    let ARC_PV = arc_from_grid(GRID_PV);

    let module = Module::new(GRID_PV, GRID_PV);
    let mut rows = Vec::new();

    let row_0 = Row {
        y: ARC_PV / 2.0,
        n: (TAU / ARC_PV).floor() as i64,
    };

    rows.push(row_0);

    loop {
        // 尝试直接偏移一行
        let row_try_offset = Row {
            y: rows.last().unwrap().y + ARC_PV,
            n: rows.last().unwrap().n,
        };

        let row_next = if (row_try_offset.y + ARC_PV / 2.0).cos() < row_try_offset.n as f64 * ARC_PV
        {
            // 如果直接偏移太挤了
            let Some(y_fixed) = module.calculate_next_edge_y(rows.last().unwrap().y + ARC_PV / 2.0)
            else {
                break;
            };
            let n = ((y_fixed + ARC_PV).cos() * (TAU / ARC_PV)).floor() as i64;
            Row { y: y_fixed + ARC_PV / 2.0, n }
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

fn layout_to_buildings(rows: &[Row]) -> Vec<Building> {
    let mut buildings = Vec::new();

    for row in rows {
        for i in 0..row.n {
            let local_offset = Vector3::new(
                1000.0 * (i as f64 + 0.5) / (row.n as f64),
                grid_from_arc(row.y),
                0.0,
            );
            buildings.push(new_pv(local_offset));
        }
    }

    buildings
}

fn main() -> Result<(), DspbptkError> {
    let header_data = Header::default();
    let zopfli_options = zopfli::Options::default();

    let rows = calculate_layout();

    let buildings = fix_dspbptk_buildings_index(layout_to_buildings(&rows));

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
        // cargo run --bin pv --release > "C:\Users\%USERNAME%\Documents\Dyson Sphere Program\Blueprint\pv.txt"
        print!("{blueprint}");
    }

    Ok(())
}
