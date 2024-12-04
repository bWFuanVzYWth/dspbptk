pub mod area;
pub mod building;

use nom::{
    multi::count,
    number::complete::{le_i32, le_i8},
    IResult,
};

use crate::blueprint::error::BlueprintError;
use crate::blueprint::error::BlueprintError::*;

#[derive(Debug)]
pub struct MemoryStreamData<'m> {
    pub patch: i32,
    pub cursor_offset_x: i32,
    pub cursor_offset_y: i32,
    pub cursor_target_area: i32,
    pub drag_box_size_x: i32,
    pub drag_box_size_y: i32,
    pub primary_area_idx: i32,
    pub areas_length: i8,
    pub areas: Vec<area::AreaData>,
    pub buildings_length: i32,
    pub buildings: Vec<building::BuildingData>,
    pub unknown: &'m [u8],
}

fn parse_non_finish(bin: &[u8]) -> IResult<&[u8], MemoryStreamData> {
    let unknown = bin;

    let (unknown, patch) = le_i32(unknown)?;
    let (unknown, cursor_offset_x) = le_i32(unknown)?;
    let (unknown, cursor_offset_y) = le_i32(unknown)?;
    let (unknown, cursor_target_area) = le_i32(unknown)?;
    let (unknown, drag_box_size_x) = le_i32(unknown)?;
    let (unknown, drag_box_size_y) = le_i32(unknown)?;
    let (unknown, primary_area_idx) = le_i32(unknown)?;
    let (unknown, areas_length) = le_i8(unknown)?;
    let (unknown, areas) = count(area::parse, areas_length as usize)(unknown)?;
    let (unknown, buildings_length) = le_i32(unknown)?;
    let (unknown, buildings) = count(building::parse, buildings_length as usize)(unknown)?;

    Ok((
        unknown,
        MemoryStreamData {
            patch: patch,
            cursor_offset_x: cursor_offset_x,
            cursor_offset_y: cursor_offset_y,
            cursor_target_area: cursor_target_area,
            drag_box_size_x: drag_box_size_x,
            drag_box_size_y: drag_box_size_y,
            primary_area_idx: primary_area_idx,
            areas_length: areas_length,
            areas: areas,
            buildings_length: buildings_length,
            buildings: buildings,
            unknown: unknown,
        },
    ))
}

pub fn parse(bin: &[u8]) -> Result<MemoryStreamData, BlueprintError<String>> {
    use nom::Finish;
    match parse_non_finish(bin).finish() {
        Ok((_unknown, data)) => Ok(data),
        Err(why) => Err(CanNotParseMemoryStream(format!("{:?}", why))),
    }
}

pub fn serialization(data: MemoryStreamData) -> Vec<u8> {
    let mut bin = Vec::new();
    bin.extend_from_slice(&data.patch.to_le_bytes());
    bin.extend_from_slice(&data.cursor_offset_x.to_le_bytes());
    bin.extend_from_slice(&data.cursor_offset_y.to_le_bytes());
    bin.extend_from_slice(&data.cursor_target_area.to_le_bytes());
    bin.extend_from_slice(&data.drag_box_size_x.to_le_bytes());
    bin.extend_from_slice(&data.drag_box_size_y.to_le_bytes());
    bin.extend_from_slice(&data.primary_area_idx.to_le_bytes());
    bin.extend_from_slice(&data.areas_length.to_le_bytes());
    data.areas
        .iter()
        .for_each(|area_data| area::serialization(&mut bin, area_data));
    bin.extend_from_slice(&data.buildings_length.to_le_bytes());
    data.buildings
        .iter()
        .for_each(|building_data| building::serialization(&mut bin, building_data));
    bin
}

pub fn fix_buildings_index(buildings: &mut Vec<building::BuildingData>) {
    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::collections::HashMap;

    buildings.sort_by(|a, b| {
        let item_id_order = a.item_id.cmp(&b.item_id);
        if item_id_order != Equal {
            return item_id_order;
        };

        let model_index_order = a.model_index.cmp(&b.model_index);
        if model_index_order != Equal {
            return model_index_order;
        };

        let recipe_id_order = a.recipe_id.cmp(&b.recipe_id);
        if recipe_id_order != Equal {
            return recipe_id_order;
        };

        let area_index_order = a.area_index.cmp(&b.area_index);
        if area_index_order != Equal {
            return area_index_order;
        };

        let item_id_order = a.item_id.cmp(&b.item_id);
        if item_id_order != Equal {
            return item_id_order;
        };

        const KY: f64 = 256.0;
        const KX: f64 = 1024.0;
        let local_offset_score = |x, y, z| ((y as f64) * KY + (x as f64)) * KX + (z as f64);
        let local_offset_score_a =
            local_offset_score(a.local_offset_x, a.local_offset_y, a.local_offset_z);
        let local_offset_score_b =
            local_offset_score(b.local_offset_x, b.local_offset_y, b.local_offset_z);
        if local_offset_score_a < local_offset_score_b {
            Less
        } else {
            Greater
        }
    });

    let mut index_lut = HashMap::new();
    buildings.iter().enumerate().for_each(|(index, building)| {
        index_lut.insert(building.index, index as i32);
    });
    buildings.iter_mut().for_each(|building| {
        building.index = *index_lut
            .get(&building.index)
            .unwrap(/* impossible */);
        building.temp_output_obj_idx = *index_lut
            .get(&building.temp_output_obj_idx)
            .unwrap_or(&building::INDEX_NULL);
        building.temp_input_obj_idx = *index_lut
            .get(&building.temp_input_obj_idx)
            .unwrap_or(&building::INDEX_NULL);
    });
}

#[cfg(test)]
mod test {
    // TODO test
}
