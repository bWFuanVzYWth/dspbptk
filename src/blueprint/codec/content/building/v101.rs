use nom::{
    IResult, Parser,
    bytes::complete::tag,
    multi::count,
    number::complete::{le_f32, le_i8, le_i16, le_i32, le_u16},
};

use crate::blueprint::data::content::building::{Building, Version};

// FIXME 这种硬编码常量应该放在这里吗？
const BELT_LOW: i16 = 2001;
const BELT_HIGH: i16 = 2009;
const SORTER_LOW: i16 = 2011;
const SORTER_HIGH: i16 = 2019;

// 定义过于复杂的类型，避免使用时不小心写错
pub type F32x12 = (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32);

#[expect(clippy::similar_names)]
pub fn deserialization(bin: &[u8]) -> IResult<&[u8], Building> {
    let unknown = bin;

    let (unknown, _version) = tag(i32::from(Version::Neg101).to_le_bytes().as_slice())(unknown)?;
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
        SORTER_LOW..=SORTER_HIGH => parse_sorter(unknown)?,
        BELT_LOW..=BELT_HIGH => parse_belt(unknown)?,
        _ => parse_default(unknown)?,
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
            yaw,
            tilt,
            pitch,
            local_offset_x2,
            local_offset_y2,
            local_offset_z2,
            yaw2,
            tilt2,
            pitch2,
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
    bin.extend_from_slice(&i32::from(Version::Neg101).to_le_bytes());
    bin.extend_from_slice(&data.index.to_le_bytes());
    bin.extend_from_slice(&data.item_id.to_le_bytes());
    bin.extend_from_slice(&data.model_index.to_le_bytes());
    bin.extend_from_slice(&data.area_index.to_le_bytes());

    match data.item_id {
        SORTER_LOW..=SORTER_HIGH => {
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
        BELT_LOW..=BELT_HIGH => {
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
    data.parameters
        .iter()
        .for_each(|x| bin.extend_from_slice(&x.to_le_bytes()));
}

#[expect(clippy::similar_names)]
fn parse_sorter(input: &[u8]) -> IResult<&[u8], F32x12> {
    let (input, local_offset_x) = le_f32(input)?;
    let (input, local_offset_y) = le_f32(input)?;
    let (input, local_offset_z) = le_f32(input)?;
    let (input, yaw) = le_f32(input)?;
    let (input, tilt) = le_f32(input)?;
    let (input, pitch) = le_f32(input)?;
    let (input, local_offset_x2) = le_f32(input)?;
    let (input, local_offset_y2) = le_f32(input)?;
    let (input, local_offset_z2) = le_f32(input)?;
    let (input, yaw2) = le_f32(input)?;
    let (input, tilt2) = le_f32(input)?;
    let (input, pitch2) = le_f32(input)?;

    Ok((
        input,
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
    ))
}

fn parse_belt(input: &[u8]) -> IResult<&[u8], F32x12> {
    let (input, local_offset_x) = le_f32(input)?;
    let (input, local_offset_y) = le_f32(input)?;
    let (input, local_offset_z) = le_f32(input)?;
    let (input, yaw) = le_f32(input)?;
    let (input, tilt) = le_f32(input)?;

    Ok((
        input,
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
            yaw,
            tilt,
            0.0,
        ),
    ))
}

fn parse_default(input: &[u8]) -> IResult<&[u8], F32x12> {
    let (input, local_offset_x) = le_f32(input)?;
    let (input, local_offset_y) = le_f32(input)?;
    let (input, local_offset_z) = le_f32(input)?;
    let (input, yaw) = le_f32(input)?;

    Ok((
        input,
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
    ))
}

#[allow(clippy::cognitive_complexity)]
#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_serialization_version_neg101_default() {
        let bin_expected: Vec<u8> = vec![
            155, 255, 255, 255, 1, 0, 0, 0, 15, 0, 16, 0, 2, 102, 102, 70, 64, 51, 51, 131, 64, 51,
            51, 163, 64, 51, 51, 195, 64, 17, 0, 0, 0, 18, 0, 0, 0, 19, 20, 21, 22, 23, 24, 25, 0,
            26, 0, 4, 0, 27, 0, 0, 0, 28, 0, 0, 0, 29, 0, 0, 0, 30, 0, 0, 0,
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
    fn test_deserialization_version_neg101_default() {
        let data_expected = Building {
            index: 1,
            area_index: 2,
            local_offset_x: 3.1,
            local_offset_y: 4.1,
            local_offset_z: 5.1,
            yaw: 6.1,
            tilt: 0.0,            // 注意不是 7.1
            pitch: 0.0,           // 注意不是 8.1
            local_offset_x2: 0.0, // 注意不是 9.1
            local_offset_y2: 0.0, // 注意不是 10.1
            local_offset_z2: 0.0, // 注意不是 11.1
            yaw2: 6.1,            // 注意不是 12.1，也不是 0.0
            tilt2: 0.0,           // 注意不是 13.2
            pitch2: 0.0,          // 注意不是 14.2
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
            155, 255, 255, 255, 1, 0, 0, 0, 15, 0, 16, 0, 2, 102, 102, 70, 64, 51, 51, 131, 64, 51,
            51, 163, 64, 51, 51, 195, 64, 17, 0, 0, 0, 18, 0, 0, 0, 19, 20, 21, 22, 23, 24, 25, 0,
            26, 0, 4, 0, 27, 0, 0, 0, 28, 0, 0, 0, 29, 0, 0, 0, 30, 0, 0, 0,
        ];

        let test = deserialization(&bin_test).finish();

        assert_eq!(test, Ok(([].as_slice(), data_expected)));
    }

    #[test]
    fn test_serialization_version_neg101_belt() {
        let bin_expected: Vec<u8> = vec![
            155, 255, 255, 255, 1, 0, 0, 0, 209, 7, 16, 0, 2, 102, 102, 70, 64, 51, 51, 131, 64,
            51, 51, 163, 64, 51, 51, 195, 64, 51, 51, 227, 64, 17, 0, 0, 0, 18, 0, 0, 0, 19, 20,
            21, 22, 23, 24, 25, 0, 26, 0, 4, 0, 27, 0, 0, 0, 28, 0, 0, 0, 29, 0, 0, 0, 30, 0, 0, 0,
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
            item_id: 2001, // 2001 黄带
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
    fn test_deserialization_version_neg101_belt() {
        let data_expected = Building {
            index: 1,
            area_index: 2,
            local_offset_x: 3.1,
            local_offset_y: 4.1,
            local_offset_z: 5.1,
            yaw: 6.1,
            tilt: 7.1,
            pitch: 0.0,           // 注意不是 8.1
            local_offset_x2: 0.0, // 注意不是 9.1
            local_offset_y2: 0.0, // 注意不是 10.1
            local_offset_z2: 0.0, // 注意不是 11.1
            yaw2: 6.1,            // 注意不是 12.1，也不是 0.0
            tilt2: 7.1,           // 注意不是 13.2，也不是 0.0
            pitch2: 0.0,          // 注意不是 14.2
            item_id: 2001,        // 2001 黄带
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
            155, 255, 255, 255, 1, 0, 0, 0, 209, 7, 16, 0, 2, 102, 102, 70, 64, 51, 51, 131, 64,
            51, 51, 163, 64, 51, 51, 195, 64, 51, 51, 227, 64, 17, 0, 0, 0, 18, 0, 0, 0, 19, 20,
            21, 22, 23, 24, 25, 0, 26, 0, 4, 0, 27, 0, 0, 0, 28, 0, 0, 0, 29, 0, 0, 0, 30, 0, 0, 0,
        ];

        let test = deserialization(&bin_test).finish();

        assert_eq!(test, Ok(([].as_slice(), data_expected)));
    }

    #[test]
    fn test_serialization_version_neg101_sorter() {
        let bin_expected: Vec<u8> = vec![
            155, 255, 255, 255, 1, 0, 0, 0, 219, 7, 16, 0, 2, 102, 102, 70, 64, 51, 51, 131, 64,
            51, 51, 163, 64, 51, 51, 195, 64, 51, 51, 227, 64, 154, 153, 1, 65, 51, 51, 19, 65, 51,
            51, 35, 65, 51, 51, 51, 65, 51, 51, 67, 65, 51, 51, 83, 65, 51, 51, 99, 65, 17, 0, 0,
            0, 18, 0, 0, 0, 19, 20, 21, 22, 23, 24, 25, 0, 26, 0, 4, 0, 27, 0, 0, 0, 28, 0, 0, 0,
            29, 0, 0, 0, 30, 0, 0, 0,
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
            item_id: 2011, // 黄爪
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
    fn test_deserialization_version_neg101_sorter() {
        let data_expected = Building {
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
            item_id: 2011, // 黄爪
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
            155, 255, 255, 255, 1, 0, 0, 0, 219, 7, 16, 0, 2, 102, 102, 70, 64, 51, 51, 131, 64,
            51, 51, 163, 64, 51, 51, 195, 64, 51, 51, 227, 64, 154, 153, 1, 65, 51, 51, 19, 65, 51,
            51, 35, 65, 51, 51, 51, 65, 51, 51, 67, 65, 51, 51, 83, 65, 51, 51, 99, 65, 17, 0, 0,
            0, 18, 0, 0, 0, 19, 20, 21, 22, 23, 24, 25, 0, 26, 0, 4, 0, 27, 0, 0, 0, 28, 0, 0, 0,
            29, 0, 0, 0, 30, 0, 0, 0,
        ];

        let test = deserialization(&bin_test).finish();

        assert_eq!(test, Ok(([].as_slice(), data_expected)));
    }
}
