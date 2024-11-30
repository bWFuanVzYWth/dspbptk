use nom::{
    number::complete::{le_i16, le_i8},
    IResult,
};

#[derive(Debug)]
pub struct BlueprintArea {
    index: i8,
    parent_index: i8,
    tropic_anchor: i16,
    area_segments: i16,
    anchor_local_offset_x: i16,
    anchor_local_offset_y: i16,
    width: i16,
    height: i16,
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

#[cfg(test)]
mod test {
    use super::*;
// TODO test
}
