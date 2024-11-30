use nom::{
    number::complete::{le_i16, le_i8},
    IResult,
};

#[derive(Debug)]
pub struct BlueprintArea {
    pub index: i8,
    pub parent_index: i8,
    pub tropic_anchor: i16,
    pub area_segments: i16,
    pub anchor_local_offset_x: i16,
    pub anchor_local_offset_y: i16,
    pub width: i16,
    pub height: i16,
}

pub fn parse(memory_stream: &[u8]) -> IResult<&[u8], BlueprintArea> {
    let unknown = memory_stream;

    let (unknown, index) = le_i8(unknown)?;
    let (unknown, parent_index) = le_i8(unknown)?;
    let (unknown, tropic_anchor) = le_i16(unknown)?;
    let (unknown, area_segments) = le_i16(unknown)?;
    let (unknown, anchor_local_offset_x) = le_i16(unknown)?;
    let (unknown, anchor_local_offset_y) = le_i16(unknown)?;
    let (unknown, width) = le_i16(unknown)?;
    let (unknown, height) = le_i16(unknown)?;

    Ok((
        unknown,
        BlueprintArea {
            index: index,
            parent_index: parent_index,
            tropic_anchor: tropic_anchor,
            area_segments: area_segments,
            anchor_local_offset_x: anchor_local_offset_x,
            anchor_local_offset_y: anchor_local_offset_y,
            width: width,
            height: height,
        },
    ))
}

pub fn serialization(area: &BlueprintArea) -> Vec<u8> {
    let mut memory_stream = Vec::new();
    memory_stream.extend_from_slice(&area.index.to_le_bytes());
    memory_stream.extend_from_slice(&area.parent_index.to_le_bytes());
    memory_stream.extend_from_slice(&area.tropic_anchor.to_le_bytes());
    memory_stream.extend_from_slice(&area.area_segments.to_le_bytes());
    memory_stream.extend_from_slice(&area.anchor_local_offset_x.to_le_bytes());
    memory_stream.extend_from_slice(&area.anchor_local_offset_y.to_le_bytes());
    memory_stream.extend_from_slice(&area.width.to_le_bytes());
    memory_stream.extend_from_slice(&area.height.to_le_bytes());
    memory_stream
}

#[cfg(test)]
mod test {
    use super::*;
    // TODO test
}
