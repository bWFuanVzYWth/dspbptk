use nom::{
    number::complete::{le_i16, le_i8},
    IResult,
};

#[derive(Debug)]
pub struct AreaData {
    pub index: i8,
    pub parent_index: i8,
    pub tropic_anchor: i16,
    pub area_segments: i16,
    pub anchor_local_offset_x: i16,
    pub anchor_local_offset_y: i16,
    pub width: i16,
    pub height: i16,
}

pub fn deserialization(bin: &[u8]) -> IResult<&[u8], AreaData> {
    let unknown = bin;

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
        AreaData {
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

pub fn serialization(bin: &mut Vec<u8>, data: &AreaData) {
    bin.extend_from_slice(&data.index.to_le_bytes());
    bin.extend_from_slice(&data.parent_index.to_le_bytes());
    bin.extend_from_slice(&data.tropic_anchor.to_le_bytes());
    bin.extend_from_slice(&data.area_segments.to_le_bytes());
    bin.extend_from_slice(&data.anchor_local_offset_x.to_le_bytes());
    bin.extend_from_slice(&data.anchor_local_offset_y.to_le_bytes());
    bin.extend_from_slice(&data.width.to_le_bytes());
    bin.extend_from_slice(&data.height.to_le_bytes());
}

#[cfg(test)]
mod test {
    // TODO test
}
