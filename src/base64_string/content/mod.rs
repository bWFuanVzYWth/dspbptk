pub mod area;
pub mod building;

use nom::{
    bytes::complete::{tag, take},
    multi::length_count,
    number::complete::{le_f32, le_i16, le_i32, le_i8},
    sequence::tuple,
    IResult,
};

// TODO 规范写法和命名
pub fn decode_base64(string: &str) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::prelude::*;
    BASE64_STANDARD.decode(string)
}

pub fn decompress_gzip(bin: Vec<u8>) -> Result<Vec<u8>, miniz_oxide::inflate::DecompressError> {
    use miniz_oxide::inflate::*;
    decompress_to_vec(bin.as_slice())
}

pub struct Data {
    version: i64,
    cursor_offset: [i64; 2],
    cursor_target_area: i64,
    drag_box_size: [i64; 2],
    primary_area_idx: i64,
    areas: Vec<area::BlueprintArea>,
    buildings: Vec<building::BlueprintBuilding>,
}

fn le_i8_to_usize(bin: &[u8]) -> IResult<&[u8], usize> {
    let (unknown, num) = le_i8(bin)?;
    Ok((unknown, num as usize))
}

fn le_i32_to_usize(bin: &[u8]) -> IResult<&[u8], usize> {
    let (unknown, num) = le_i32(bin)?;
    Ok((unknown, num as usize))
}

fn tag_num_neg100(bin: &[u8]) -> IResult<&[u8], &[u8]> {
    let (unknown, num_neg100) = tag((-1_i32).to_le_bytes())(bin)?;
    Ok((unknown, num_neg100))
}

// TODO 规范化命名

// fn parse_building(bin: &[u8]) -> IResult<&[u8], Building> {
//     alt(
//         tuple((

//         )),
//         tuple((

//         ))
//     )
// }

// pub fn parse(bin: &[u8]) -> IResult<&[u8], Data> {
//     match tuple((
//         le_i32,
//         le_i32,
//         le_i32,
//         le_i32,
//         le_i32,
//         le_i32,
//         le_i32,
//         length_count(le_i8_to_usize, area::parse),
//         length_count(le_i32_to_usize, building::parse),
//     ))(bin)
//     {
//         Ok(result) => Ok((
//             result.0,
//             Data {
//                 version: result.1 .0 as i64,
//                 cursor_offset: [result.1 .1 as i64, result.1 .2 as i64],
//                 cursor_target_area: result.1 .3 as i64,
//                 drag_box_size: [result.1 .4 as i64, result.1 .5 as i64],
//                 primary_area_idx: result.1 .6 as i64,
//                 areas: result.1 .7,
//                 buildings: Vec::new(), // FIXME 写好建筑解析之后接入
//             },
//         )),
//         Err(why) => Err(why),
//     }
// }
