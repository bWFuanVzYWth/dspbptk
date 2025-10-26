use crate::blueprint::Area;
use nom::{
    IResult,
    number::complete::{le_i8, le_i16},
};

/// # Errors
/// 可能的原因：
/// * content的area部分已经损坏，或者编码不受支持
pub fn deserialization(bin: &[u8]) -> IResult<&[u8], Area> {
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
        Area {
            index,
            parent_index,
            tropic_anchor,
            area_segments,
            anchor_local_offset_x,
            anchor_local_offset_y,
            width,
            height,
        },
    ))
}

pub fn serialization(mut bin: Vec<u8>, data: &Area) -> Vec<u8> {
    bin.extend_from_slice(&data.index.to_le_bytes());
    bin.extend_from_slice(&data.parent_index.to_le_bytes());
    bin.extend_from_slice(&data.tropic_anchor.to_le_bytes());
    bin.extend_from_slice(&data.area_segments.to_le_bytes());
    bin.extend_from_slice(&data.anchor_local_offset_x.to_le_bytes());
    bin.extend_from_slice(&data.anchor_local_offset_y.to_le_bytes());
    bin.extend_from_slice(&data.width.to_le_bytes());
    bin.extend_from_slice(&data.height.to_le_bytes());

    bin
}
