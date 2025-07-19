use nom::{
    IResult, Parser,
    multi::count,
    number::complete::{le_f32, le_i8, le_i16, le_i32, le_u16},
};

use crate::blueprint::data::content::building::Building;

#[expect(clippy::similar_names)]
pub fn deserialization(bin: &[u8]) -> IResult<&[u8], Building> {
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
    let (unknown, parameters_length) = le_u16(unknown)?;
    let (unknown, parameters) = count(le_i32, parameters_length as usize).parse(unknown)?;

    Ok((
        unknown,
        Building {
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

pub fn serialization(bin: &mut Vec<u8>, data: &Building) {
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

#[allow(clippy::cognitive_complexity)]
#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_serialization_version_0() {
        let bin_expected: Vec<u8> = vec![
            1, 0, 0, 0, 2, 102, 102, 70, 64, 51, 51, 131, 64, 51, 51, 163, 64, 51, 51, 19, 65, 51,
            51, 35, 65, 51, 51, 51, 65, 51, 51, 195, 64, 51, 51, 67, 65, 15, 0, 16, 0, 17, 0, 0, 0,
            18, 0, 0, 0, 19, 20, 21, 22, 23, 24, 25, 0, 26, 0, 4, 0, 27, 0, 0, 0, 28, 0, 0, 0, 29,
            0, 0, 0, 30, 0, 0, 0,
        ];

        let data_test = Building {
            index: 1,
            area_index: 2,
            local_offset_x: 3.1,
            local_offset_y: 4.1,
            local_offset_z: 5.1,
            yaw: 6.1,
            tilt: 7.1,
            pitch: 8.1,
            local_offset_x2: 9.2,
            local_offset_y2: 10.2,
            local_offset_z2: 11.2,
            yaw2: 12.2,
            tilt2: 13.2,
            pitch2: 14.2,
            item_id: 15,
            model_index: 16,
            temp_output_obj_idx: 17,
            temp_input_obj_idx: 18,
            output_to_slot: 19,
            input_from_slot: 20,
            output_from_slot: 21,
            input_to_slot: 22,
            output_offset: 23,
            input_offset: 24,
            recipe_id: 25,
            filter_id: 26,
            parameters_length: 4,
            parameters: vec![27, 28, 29, 30],
        };

        let mut bin_test = Vec::new();
        serialization(&mut bin_test, &data_test);

        assert_eq!(bin_test, bin_expected);
    }

    #[test]
    fn test_deserialization_version_0() {
        let data_expected = Building {
            index: 1,
            area_index: 2,
            local_offset_x: 3.1,
            local_offset_y: 4.1,
            local_offset_z: 5.1,
            yaw: 6.1,
            tilt: 0.0,  // 注意不是 7.1
            pitch: 0.0, // 注意不是 8.1
            local_offset_x2: 9.2,
            local_offset_y2: 10.2,
            local_offset_z2: 11.2,
            yaw2: 12.2,
            tilt2: 0.0,  // 注意不是 13.2
            pitch2: 0.0, // 注意不是 14.2
            item_id: 15,
            model_index: 16,
            temp_output_obj_idx: 17,
            temp_input_obj_idx: 18,
            output_to_slot: 19,
            input_from_slot: 20,
            output_from_slot: 21,
            input_to_slot: 22,
            output_offset: 23,
            input_offset: 24,
            recipe_id: 25,
            filter_id: 26,
            parameters_length: 4,
            parameters: vec![27, 28, 29, 30],
        };

        let bin_test: Vec<u8> = vec![
            1, 0, 0, 0, 2, 102, 102, 70, 64, 51, 51, 131, 64, 51, 51, 163, 64, 51, 51, 19, 65, 51,
            51, 35, 65, 51, 51, 51, 65, 51, 51, 195, 64, 51, 51, 67, 65, 15, 0, 16, 0, 17, 0, 0, 0,
            18, 0, 0, 0, 19, 20, 21, 22, 23, 24, 25, 0, 26, 0, 4, 0, 27, 0, 0, 0, 28, 0, 0, 0, 29,
            0, 0, 0, 30, 0, 0, 0,
        ];

        let test = deserialization(&bin_test).finish();

        assert_eq!(test, Ok(([].as_slice(), data_expected)));
    }
}
