use crate::dspbptk_blueprint::Building;
use std::collections::HashMap;

#[must_use]
pub fn fix_dspbptk_buildings_index(buildings: Vec<Building>) -> Vec<Building> {
    let uuid_lut = buildings
        .iter()
        .enumerate()
        .map(|(uuid, building)| (building.uuid, Some(uuid as u128)))
        .collect::<HashMap<_, _>>();

    buildings
        .into_iter()
        .map(|building| Building {
            uuid: *uuid_lut.get(&building.uuid).unwrap_or(&None),
            temp_output_obj_idx: uuid_lut
                .get(&building.temp_output_obj_idx)
                .copied()
                .unwrap_or(None),
            temp_input_obj_idx: uuid_lut
                .get(&building.temp_input_obj_idx)
                .copied()
                .unwrap_or(None),
            ..building
        })
        .collect()
}
