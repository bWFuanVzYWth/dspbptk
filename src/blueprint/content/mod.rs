pub mod area;
pub mod building;

use nom::{
    multi::count,
    number::complete::{le_i32, le_i8},
    IResult,
};

#[derive(Debug)]
pub struct Content {
    pub patch: i32,
    pub cursor_offset_x: i32,
    pub cursor_offset_y: i32,
    pub cursor_target_area: i32,
    pub drag_box_size_x: i32,
    pub drag_box_size_y: i32,
    pub primary_area_idx: i32,
    pub areas_length: i8,
    pub areas: Vec<area::BlueprintArea>,
    pub buildings_length: i32,
    pub buildings: Vec<building::BlueprintBuilding>,
}

pub fn parse(memory_stream: &[u8]) -> IResult<&[u8], Content> {
    let unknown = memory_stream;

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
        Content {
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
        },
    ))
}

pub fn serialization(content: Content) -> Vec<u8> {
    let mut memory_stream = Vec::new();
    memory_stream.extend_from_slice(&content.patch.to_le_bytes());
    memory_stream.extend_from_slice(&content.cursor_offset_x.to_le_bytes());
    memory_stream.extend_from_slice(&content.cursor_offset_y.to_le_bytes());
    memory_stream.extend_from_slice(&content.cursor_target_area.to_le_bytes());
    memory_stream.extend_from_slice(&content.drag_box_size_x.to_le_bytes());
    memory_stream.extend_from_slice(&content.drag_box_size_y.to_le_bytes());
    memory_stream.extend_from_slice(&content.primary_area_idx.to_le_bytes());
    memory_stream.extend_from_slice(&content.areas_length.to_le_bytes());
    content
        .areas
        .iter()
        .for_each(|area| memory_stream.extend(area::serialization(area)));
    memory_stream.extend_from_slice(&content.buildings_length.to_le_bytes());
    content.buildings.iter().for_each(|building| {
        memory_stream.extend(building::serialization_version_neg100(building))
    });
    memory_stream
}

pub fn sort_buildings(buildings: &mut Vec<building::BlueprintBuilding>) {
    buildings.sort_by(|a, b| {
        use std::cmp::Ordering::{Equal, Greater, Less};
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

    use std::collections::HashMap;
    let mut index_lut = HashMap::new();
    buildings.iter().enumerate().for_each(|(index, building)| {
        index_lut.insert(building.index, index as i32);
    });
    buildings.iter_mut().for_each(|building| {
        building.index = index_lut
            .get(&building.index)
            .copied()
            .unwrap_or(building::INDEX_NULL);
        building.temp_output_obj_idx = index_lut
            .get(&building.temp_output_obj_idx)
            .copied()
            .unwrap_or(building::INDEX_NULL);
        building.temp_input_obj_idx = index_lut
            .get(&building.temp_input_obj_idx)
            .copied()
            .unwrap_or(building::INDEX_NULL);
    });
}

#[cfg(test)]
mod test {
    use super::*;
    // TODO test
}
