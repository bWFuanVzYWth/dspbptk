use crate::blueprint::Building;
use std::collections::HashMap;

#[must_use]
pub fn fix_buildings_index(buildings: Vec<Building>) -> Vec<Building> {
    let lut = buildings
        .iter()
        .zip(0..=i32::MAX)
        .map(|(building, index)| (building.index, index))
        .collect::<HashMap<_, _>>();

    buildings
        .into_iter()
        .map(|building| Building {
            index: *lut.get(&building.index).unwrap_or(&Building::INDEX_NULL),
            temp_output_obj_idx: lut
                .get(&building.temp_output_obj_idx)
                .copied()
                .unwrap_or(Building::INDEX_NULL),
            temp_input_obj_idx: lut
                .get(&building.temp_input_obj_idx)
                .copied()
                .unwrap_or(Building::INDEX_NULL),
            ..building
        })
        .collect()
}
