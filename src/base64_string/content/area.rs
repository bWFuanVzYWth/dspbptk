use nom::{
    number::complete::{le_i16, le_i8},
    sequence::tuple,
    IResult,
};

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
    let (unknown, area) =
        tuple((le_i8, le_i8, le_i16, le_i16, le_i16, le_i16, le_i16, le_i16))(memory_stream)?;

    Ok((
        unknown,
        BlueprintArea {
            index: area.0,
            parent_index: area.1,
            tropic_anchor: area.2,
            area_segments: area.3,
            anchor_local_offset_x: area.4,
            anchor_local_offset_y: area.5,
            width: area.6,
            height: area.7,
        },
    ))
}
