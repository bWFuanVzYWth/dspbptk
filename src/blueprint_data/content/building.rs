use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::count,
    number::complete::{le_f32, le_i16, le_i32, le_i8},
    IResult,
};

#[derive(Debug)]
pub struct BlueprintBuilding {
    version: i32,

    index: i32,
    area_index: i8,

    local_offset_x: f32,
    local_offset_y: f32,
    local_offset_z: f32,
    yaw: f32,
    tilt: f32,
    pitch: f32,

    local_offset_x2: f32,
    local_offset_y2: f32,
    local_offset_z2: f32,
    yaw2: f32,
    tilt2: f32,
    pitch2: f32,

    item_id: i16,
    model_index: i16,

    temp_output_obj_idx: i32,
    temp_input_obj_idx: i32,

    output_to_slot: i8,
    input_from_slot: i8,
    output_from_slot: i8,
    input_to_slot: i8,

    output_offset: i8,
    input_offset: i8,

    recipe_id: i16,
    filter_id: i16,

    parameters_length: i16,
    parameters: Vec<i32>,
}

fn parse_version_neg101(memory_stream: &[u8]) -> IResult<&[u8], BlueprintBuilding> {
    let unknown = memory_stream;

    let (unknown, _version) = tag((-101_i32).to_le_bytes())(unknown)?;
    let (unknown, index) = le_i32(unknown)?;
    let (unknown, item_id) = le_i16(unknown)?;
    let (unknown, model_index) = le_i16(unknown)?;
    let (unknown, area_index) = le_i8(unknown)?;

    let (
        unknown,
        (
            local_offset_x,
            local_offset_y,
            local_offset_z,
            yaw,
            tilt,
            pitch,
            local_offset_x2,
            local_offset_y2,
            local_offset_z2,
            yaw2,
            tilt2,
            pitch2,
        ),
    ) = match item_id {
        2011..2021 => {
            // 分拣器
            let (unknown, local_offset_x) = le_f32(unknown)?;
            let (unknown, local_offset_y) = le_f32(unknown)?;
            let (unknown, local_offset_z) = le_f32(unknown)?;
            let (unknown, yaw) = le_f32(unknown)?;
            let (unknown, tilt) = le_f32(unknown)?;
            let (unknown, pitch) = le_f32(unknown)?;
            let (unknown, local_offset_x2) = le_f32(unknown)?;
            let (unknown, local_offset_y2) = le_f32(unknown)?;
            let (unknown, local_offset_z2) = le_f32(unknown)?;
            let (unknown, yaw2) = le_f32(unknown)?;
            let (unknown, tilt2) = le_f32(unknown)?;
            let (unknown, pitch2) = le_f32(unknown)?;
            (
                unknown,
                (
                    local_offset_x,
                    local_offset_y,
                    local_offset_z,
                    yaw,
                    tilt,
                    pitch,
                    local_offset_x2,
                    local_offset_y2,
                    local_offset_z2,
                    yaw2,
                    tilt2,
                    pitch2,
                ),
            )
        }
        2001..2011 => {
            // 传送带
            let (unknown, local_offset_x) = le_f32(unknown)?;
            let (unknown, local_offset_y) = le_f32(unknown)?;
            let (unknown, local_offset_z) = le_f32(unknown)?;
            let (unknown, yaw) = le_f32(unknown)?;
            let (unknown, tilt) = le_f32(unknown)?;
            (
                unknown,
                (
                    local_offset_x,
                    local_offset_y,
                    local_offset_z,
                    yaw,
                    tilt,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ),
            )
        }
        _ => {
            let (unknown, local_offset_x) = le_f32(unknown)?;
            let (unknown, local_offset_y) = le_f32(unknown)?;
            let (unknown, local_offset_z) = le_f32(unknown)?;
            let (unknown, yaw) = le_f32(unknown)?;
            (
                unknown,
                (
                    local_offset_x,
                    local_offset_y,
                    local_offset_z,
                    yaw,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ),
            )
        }
    };

    let (unknown, temp_output_obj_idx) = le_i32(unknown)?;
    let (unknown, temp_input_obj_idx) = le_i32(unknown)?;
    let (unknown, output_to_slot) = le_i8(unknown)?;
    let (unknown, input_from_slot) = le_i8(unknown)?;
    let (unknown, output_from_slot) = le_i8(unknown)?;
    let (unknown, input_to_slot) = le_i8(unknown)?;
    let (unknown, output_offset) = le_i8(unknown)?;
    let (unknown, input_offset) = le_i8(unknown)?;
    let (unknown, recipe_id) = le_i16(unknown)?;
    let (unknown, filter_id) = le_i16(unknown)?;
    let (unknown, parameters_length) = le_i16(unknown)?;
    let (unknown, parameters) = count(le_i32, parameters_length as usize)(unknown)?;

    Ok((
        unknown,
        BlueprintBuilding {
            version: -100,
            index: index,
            area_index: area_index,
            local_offset_x: local_offset_x,
            local_offset_y: local_offset_y,
            local_offset_z: local_offset_z,
            local_offset_x2: local_offset_x2,
            local_offset_y2: local_offset_y2,
            local_offset_z2: local_offset_z2,
            pitch: pitch,
            pitch2: pitch2,
            yaw: yaw,
            yaw2: yaw2,
            tilt: tilt,
            tilt2: tilt2,
            item_id: item_id,
            model_index: model_index,
            temp_output_obj_idx: temp_output_obj_idx,
            temp_input_obj_idx: temp_input_obj_idx,
            output_to_slot: output_to_slot,
            input_from_slot: input_from_slot,
            output_from_slot: output_from_slot,
            input_to_slot: input_to_slot,
            output_offset: output_offset,
            input_offset: input_offset,
            recipe_id: recipe_id,
            filter_id: filter_id,
            parameters_length: parameters_length,
            parameters: parameters,
        },
    ))
}

fn parse_version_neg100(memory_stream: &[u8]) -> IResult<&[u8], BlueprintBuilding> {
    let unknown = memory_stream;

    let (unknown, _version) = tag((-100_i32).to_le_bytes())(unknown)?;
    let (unknown, index) = le_i32(unknown)?;
    let (unknown, area_index) = le_i8(unknown)?;
    let (unknown, local_offset_x) = le_f32(unknown)?;
    let (unknown, local_offset_y) = le_f32(unknown)?;
    let (unknown, local_offset_z) = le_f32(unknown)?;
    let (unknown, local_offset_x2) = le_f32(unknown)?;
    let (unknown, local_offset_y2) = le_f32(unknown)?;
    let (unknown, local_offset_z2) = le_f32(unknown)?;
    let (unknown, yaw) = le_f32(unknown)?;
    let (unknown, yaw2) = le_f32(unknown)?;
    let (unknown, tilt) = le_f32(unknown)?;
    let (unknown, item_id) = le_i16(unknown)?;
    let (unknown, model_index) = le_i16(unknown)?;
    let (unknown, temp_output_obj_idx) = le_i32(unknown)?;
    let (unknown, temp_input_obj_idx) = le_i32(unknown)?;
    let (unknown, output_to_slot) = le_i8(unknown)?;
    let (unknown, input_from_slot) = le_i8(unknown)?;
    let (unknown, output_from_slot) = le_i8(unknown)?;
    let (unknown, input_to_slot) = le_i8(unknown)?;
    let (unknown, output_offset) = le_i8(unknown)?;
    let (unknown, input_offset) = le_i8(unknown)?;
    let (unknown, recipe_id) = le_i16(unknown)?;
    let (unknown, filter_id) = le_i16(unknown)?;
    let (unknown, parameters_length) = le_i16(unknown)?;
    let (unknown, parameters) = count(le_i32, parameters_length as usize)(unknown)?;

    Ok((
        unknown,
        BlueprintBuilding {
            version: -100,
            index: index,
            area_index: area_index,
            local_offset_x: local_offset_x,
            local_offset_y: local_offset_y,
            local_offset_z: local_offset_z,
            local_offset_x2: local_offset_x2,
            local_offset_y2: local_offset_y2,
            local_offset_z2: local_offset_z2,
            pitch: 0.0,
            pitch2: 0.0,
            yaw: yaw,
            yaw2: yaw2,
            tilt: tilt,
            tilt2: 0.0,
            item_id: item_id,
            model_index: model_index,
            temp_output_obj_idx: temp_output_obj_idx,
            temp_input_obj_idx: temp_input_obj_idx,
            output_to_slot: output_to_slot,
            input_from_slot: input_from_slot,
            output_from_slot: output_from_slot,
            input_to_slot: input_to_slot,
            output_offset: output_offset,
            input_offset: input_offset,
            recipe_id: recipe_id,
            filter_id: filter_id,
            parameters_length: parameters_length,
            parameters: parameters,
        },
    ))
}

fn parse_version_0(memory_stream: &[u8]) -> IResult<&[u8], BlueprintBuilding> {
    let unknown = memory_stream;

    let (unknown, index) = le_i32(unknown)?;
    let (unknown, area_index) = le_i8(unknown)?;
    let (unknown, local_offset_x) = le_f32(unknown)?;
    let (unknown, local_offset_y) = le_f32(unknown)?;
    let (unknown, local_offset_z) = le_f32(unknown)?;
    let (unknown, local_offset_x2) = le_f32(unknown)?;
    let (unknown, local_offset_y2) = le_f32(unknown)?;
    let (unknown, local_offset_z2) = le_f32(unknown)?;
    let (unknown, yaw) = le_f32(unknown)?;
    let (unknown, yaw2) = le_f32(unknown)?;
    let (unknown, item_id) = le_i16(unknown)?;
    let (unknown, model_index) = le_i16(unknown)?;
    let (unknown, temp_output_obj_idx) = le_i32(unknown)?;
    let (unknown, temp_input_obj_idx) = le_i32(unknown)?;
    let (unknown, output_to_slot) = le_i8(unknown)?;
    let (unknown, input_from_slot) = le_i8(unknown)?;
    let (unknown, output_from_slot) = le_i8(unknown)?;
    let (unknown, input_to_slot) = le_i8(unknown)?;
    let (unknown, output_offset) = le_i8(unknown)?;
    let (unknown, input_offset) = le_i8(unknown)?;
    let (unknown, recipe_id) = le_i16(unknown)?;
    let (unknown, filter_id) = le_i16(unknown)?;
    let (unknown, parameters_length) = le_i16(unknown)?;
    let (unknown, parameters) = count(le_i32, parameters_length as usize)(unknown)?;

    Ok((
        unknown,
        BlueprintBuilding {
            version: 0,
            index: index,
            area_index: area_index,
            local_offset_x: local_offset_x,
            local_offset_y: local_offset_y,
            local_offset_z: local_offset_z,
            local_offset_x2: local_offset_x2,
            local_offset_y2: local_offset_y2,
            local_offset_z2: local_offset_z2,
            pitch: 0.0,
            pitch2: 0.0,
            yaw: yaw,
            yaw2: yaw2,
            tilt: 0.0,
            tilt2: 0.0,
            item_id: item_id,
            model_index: model_index,
            temp_output_obj_idx: temp_output_obj_idx,
            temp_input_obj_idx: temp_input_obj_idx,
            output_to_slot: output_to_slot,
            input_from_slot: input_from_slot,
            output_from_slot: output_from_slot,
            input_to_slot: input_to_slot,
            output_offset: output_offset,
            input_offset: input_offset,
            recipe_id: recipe_id,
            filter_id: filter_id,
            parameters_length: parameters_length,
            parameters: parameters,
        },
    ))
}

pub fn parse(memory_stream: &[u8]) -> IResult<&[u8], BlueprintBuilding> {
    let (unknown, building) =
        alt((parse_version_neg101, parse_version_neg100, parse_version_0))(memory_stream)?;
    Ok((unknown, building))
}

#[cfg(test)]
mod test {
    use super::*;
    // TODO test
}
