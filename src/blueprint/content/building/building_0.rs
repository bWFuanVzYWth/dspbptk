use nom::{
    multi::count,
    number::complete::{le_f32, le_i16, le_i32, le_i8},
    IResult,
};

use super::BuildingData;

const ZERO: i32 = 0; // 00 00 00 00

#[allow(clippy::similar_names, clippy::cast_sign_loss)]
pub fn deserialization_version_0(bin: &[u8]) -> IResult<&[u8], BuildingData> {
    let unknown = bin;

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
        BuildingData {
            _version: ZERO,
            index,
            area_index,
            local_offset_x,
            local_offset_y,
            local_offset_z,
            local_offset_x2,
            local_offset_y2,
            local_offset_z2,
            pitch: 0.0,
            pitch2: 0.0,
            yaw,
            yaw2,
            tilt: 0.0,
            tilt2: 0.0,
            item_id,
            model_index,
            temp_output_obj_idx,
            temp_input_obj_idx,
            output_to_slot,
            input_from_slot,
            output_from_slot,
            input_to_slot,
            output_offset,
            input_offset,
            recipe_id,
            filter_id,
            parameters_length,
            parameters,
        },
    ))
}

pub fn _serialization_version_0(bin: &mut Vec<u8>, data: &BuildingData) {
    bin.extend_from_slice(&data.index.to_le_bytes());
    bin.extend_from_slice(&data.area_index.to_le_bytes());
    bin.extend_from_slice(&data.local_offset_x.to_le_bytes());
    bin.extend_from_slice(&data.local_offset_y.to_le_bytes());
    bin.extend_from_slice(&data.local_offset_z.to_le_bytes());
    bin.extend_from_slice(&data.local_offset_x2.to_le_bytes());
    bin.extend_from_slice(&data.local_offset_y2.to_le_bytes());
    bin.extend_from_slice(&data.local_offset_z2.to_le_bytes());
    bin.extend_from_slice(&data.yaw.to_le_bytes());
    bin.extend_from_slice(&data.yaw2.to_le_bytes());
    bin.extend_from_slice(&data.item_id.to_le_bytes());
    bin.extend_from_slice(&data.model_index.to_le_bytes());
    bin.extend_from_slice(&data.temp_output_obj_idx.to_le_bytes());
    bin.extend_from_slice(&data.temp_input_obj_idx.to_le_bytes());
    bin.extend_from_slice(&data.output_to_slot.to_le_bytes());
    bin.extend_from_slice(&data.input_from_slot.to_le_bytes());
    bin.extend_from_slice(&data.output_from_slot.to_le_bytes());
    bin.extend_from_slice(&data.input_to_slot.to_le_bytes());
    bin.extend_from_slice(&data.output_offset.to_le_bytes());
    bin.extend_from_slice(&data.input_offset.to_le_bytes());
    bin.extend_from_slice(&data.recipe_id.to_le_bytes());
    bin.extend_from_slice(&data.filter_id.to_le_bytes());
    bin.extend_from_slice(&data.parameters_length.to_le_bytes());
    data.parameters
        .iter()
        .for_each(|x| bin.extend_from_slice(&x.to_le_bytes()));
}
