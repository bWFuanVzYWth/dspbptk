pub mod area;
pub mod building;

use nom::{
    multi::count,
    number::complete::{le_i32, le_i8},
    IResult,
};

#[derive(Debug)]
pub struct Content {
    patch: i32,
    cursor_offset_x: i32,
    cursor_offset_y: i32,
    cursor_target_area: i32,
    drag_box_size_x: i32,
    drag_box_size_y: i32,
    primary_area_idx: i32,
    areas_length: i8,
    areas: Vec<area::BlueprintArea>,
    buildings_length: i32,
    buildings: Vec<building::BlueprintBuilding>,
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

#[cfg(test)]
mod test {
    use super::*;
    // TODO test
}
