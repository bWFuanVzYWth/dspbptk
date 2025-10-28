use dspbptk::{
    blueprint::data::{content::Content, header::Header},
    dspbptk_blueprint::{
        Building,
        editor::fix_uuid::fix_dspbptk_buildings_index,
        generator::tesselation::{Draft, Module, Row},
        uuid::some_new_uuid,
    },
    error::DspbptkError::{self, UnexpectBuildingsCount},
    item::Item,
    planet::unit_conversion::{arc_from_grid, arc_from_m, grid_from_arc},
    workflow::{BlueprintKind, LegalBlueprintFileType, process::process_back_end},
};
use nalgebra::Vector3;
use std::f64::consts::TAU;

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

fn layout_to_buildings(layout: &Draft) -> Vec<Building> {
    let mut buildings = Vec::new();

    for row in &layout.rows {
        for i in 0..row.count {
            let local_offset = Vector3::new(
                1000.0 / layout.pizza_count * (i as f64 + 0.5) / (row.count as f64),
                grid_from_arc(row.top_y - 0.5 * row.module_type.arc_y),
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

    let grid_pv = grid_from_arc(arc_from_m(3.5, -0.6)) + ERROR;

    let module = Module::new(grid_pv, grid_pv);

    let (mut layout, _) = Draft::new(10.0).push(module);
    while {
        let (layout_new, flag) = layout.push(module);
        layout = layout_new;

        flag
    } {}
    dbg!(layout.rows.len());

    let buildings = fix_dspbptk_buildings_index(layout_to_buildings(&layout));

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
