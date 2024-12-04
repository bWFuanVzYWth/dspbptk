use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::count,
    number::complete::{le_f32, le_i16, le_i32, le_i8},
    IResult,
};

pub const INDEX_NULL: i32 = -1;

#[derive(Debug)]
pub struct BuildingData {
    // 暂时用不到，但是保留字段
    pub _version: i32,

    pub index: i32,
    pub area_index: i8,

    pub local_offset_x: f32,
    pub local_offset_y: f32,
    pub local_offset_z: f32,
    pub yaw: f32,
    pub tilt: f32,
    pub pitch: f32,

    pub local_offset_x2: f32,
    pub local_offset_y2: f32,
    pub local_offset_z2: f32,
    pub yaw2: f32,
    pub tilt2: f32,
    pub pitch2: f32,

    pub item_id: i16,
    pub model_index: i16,

    pub temp_output_obj_idx: i32,
    pub temp_input_obj_idx: i32,

    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,

    pub output_offset: i8,
    pub input_offset: i8,

    pub recipe_id: i16,
    pub filter_id: i16,

    pub parameters_length: i16,
    pub parameters: Vec<i32>,
}

fn deserialization_version_neg101(bin: &[u8]) -> IResult<&[u8], BuildingData> {
    let unknown = bin;

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
                    yaw,
                    tilt,
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
                    yaw,
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
        BuildingData {
            _version: -100,
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

fn deserialization_version_neg100(bin: &[u8]) -> IResult<&[u8], BuildingData> {
    let unknown = bin;

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
        BuildingData {
            _version: -100,
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

fn deserialization_version_0(bin: &[u8]) -> IResult<&[u8], BuildingData> {
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
            _version: 0,
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

pub fn deserialization(bin: &[u8]) -> IResult<&[u8], BuildingData> {
    let (unknown, data) =
        alt((deserialization_version_neg101, deserialization_version_neg100, deserialization_version_0))(bin)?;
    Ok((unknown, data))
}

fn serialization_version_neg101(bin: &mut Vec<u8>, data: &BuildingData) {
    bin.extend_from_slice(&(-101_i32).to_le_bytes());
    bin.extend_from_slice(&data.index.to_le_bytes());
    bin.extend_from_slice(&data.item_id.to_le_bytes());
    bin.extend_from_slice(&data.model_index.to_le_bytes());
    bin.extend_from_slice(&data.area_index.to_le_bytes());

    match data.item_id {
        2011..2021 => {
            // 分拣器
            bin.extend_from_slice(&data.local_offset_x.to_le_bytes());
            bin.extend_from_slice(&data.local_offset_y.to_le_bytes());
            bin.extend_from_slice(&data.local_offset_z.to_le_bytes());
            bin.extend_from_slice(&data.yaw.to_le_bytes());
            bin.extend_from_slice(&data.tilt.to_le_bytes());
            bin.extend_from_slice(&data.pitch.to_le_bytes());
            bin.extend_from_slice(&data.local_offset_x2.to_le_bytes());
            bin.extend_from_slice(&data.local_offset_y2.to_le_bytes());
            bin.extend_from_slice(&data.local_offset_z2.to_le_bytes());
            bin.extend_from_slice(&data.yaw2.to_le_bytes());
            bin.extend_from_slice(&data.tilt2.to_le_bytes());
            bin.extend_from_slice(&data.pitch2.to_le_bytes());
        }
        2001..2011 => {
            bin.extend_from_slice(&data.local_offset_x.to_le_bytes());
            bin.extend_from_slice(&data.local_offset_y.to_le_bytes());
            bin.extend_from_slice(&data.local_offset_z.to_le_bytes());
            bin.extend_from_slice(&data.yaw.to_le_bytes());
            bin.extend_from_slice(&data.tilt.to_le_bytes());
        }
        _ => {
            bin.extend_from_slice(&data.local_offset_x.to_le_bytes());
            bin.extend_from_slice(&data.local_offset_y.to_le_bytes());
            bin.extend_from_slice(&data.local_offset_z.to_le_bytes());
            bin.extend_from_slice(&data.yaw.to_le_bytes());
        }
    }

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
    data
        .parameters
        .iter()
        .for_each(|x| bin.extend_from_slice(&x.to_le_bytes()));
}

fn _serialization_version_neg100(bin: &mut Vec<u8>, data: &BuildingData) {
    bin.extend_from_slice(&(-100_i32).to_le_bytes());
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
    bin.extend_from_slice(&data.tilt.to_le_bytes());
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
    data
        .parameters
        .iter()
        .for_each(|x| bin.extend_from_slice(&x.to_le_bytes()));
}

fn _serialization_version_0(bin: &mut Vec<u8>, data: &BuildingData) {
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
    data
        .parameters
        .iter()
        .for_each(|x| bin.extend_from_slice(&x.to_le_bytes()));
}

pub fn serialization(bin: &mut Vec<u8>, data: &BuildingData) {
    serialization_version_neg101(bin, data)
}

#[cfg(test)]
mod test {
    // TODO test
}
