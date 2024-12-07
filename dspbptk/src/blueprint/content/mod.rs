pub mod area;
pub mod building;

use nom::{
    multi::count,
    number::complete::{le_i32, le_i8},
    IResult,
};

use crate::blueprint::error::BlueprintError;
use crate::blueprint::error::BlueprintError::*;

#[derive(Debug)]
pub struct ContentData {
    pub patch: i32,
    pub cursor_offset_x: i32,
    pub cursor_offset_y: i32,
    pub cursor_target_area: i32,
    pub drag_box_size_x: i32,
    pub drag_box_size_y: i32,
    pub primary_area_idx: i32,
    pub areas_length: i8,
    pub areas: Vec<area::AreaData>,
    pub buildings_length: i32,
    pub buildings: Vec<building::BuildingData>,
    pub unknown: Vec<u8>,
}

fn deserialization_non_finish(bin: &[u8]) -> IResult<&[u8], ContentData> {
    let unknown = bin;

    let (unknown, patch) = le_i32(unknown)?;
    let (unknown, cursor_offset_x) = le_i32(unknown)?;
    let (unknown, cursor_offset_y) = le_i32(unknown)?;
    let (unknown, cursor_target_area) = le_i32(unknown)?;
    let (unknown, drag_box_size_x) = le_i32(unknown)?;
    let (unknown, drag_box_size_y) = le_i32(unknown)?;
    let (unknown, primary_area_idx) = le_i32(unknown)?;
    let (unknown, areas_length) = le_i8(unknown)?;
    let (unknown, areas) = count(area::deserialization, areas_length as usize)(unknown)?;
    let (unknown, buildings_length) = le_i32(unknown)?;
    let (unknown, buildings) =
        count(building::deserialization, buildings_length as usize)(unknown)?;

    Ok((
        unknown,
        ContentData {
            patch: patch,
            cursor_offset_x: cursor_offset_x,
            cursor_offset_y: cursor_offset_y,
            cursor_target_area: cursor_target_area,
            drag_box_size_x: drag_box_size_x,
            drag_box_size_y: drag_box_size_y,
            primary_area_idx: primary_area_idx,
            areas_length: areas_length,
            areas: areas,
            buildings_length: buildings_length,
            buildings: buildings,
            unknown: unknown.to_vec(),
        },
    ))
}

fn deserialization(bin: &[u8]) -> Result<ContentData, BlueprintError<String>> {
    use nom::Finish;
    match deserialization_non_finish(bin).finish() {
        Ok((_unknown, data)) => Ok(data),
        Err(why) => Err(CanNotDeserializationContent(format!("{:?}", why))),
    }
}

fn serialization(data: ContentData) -> Vec<u8> {
    let mut bin = Vec::new();
    bin.extend_from_slice(&data.patch.to_le_bytes());
    bin.extend_from_slice(&data.cursor_offset_x.to_le_bytes());
    bin.extend_from_slice(&data.cursor_offset_y.to_le_bytes());
    bin.extend_from_slice(&data.cursor_target_area.to_le_bytes());
    bin.extend_from_slice(&data.drag_box_size_x.to_le_bytes());
    bin.extend_from_slice(&data.drag_box_size_y.to_le_bytes());
    bin.extend_from_slice(&data.primary_area_idx.to_le_bytes());
    bin.extend_from_slice(&data.areas_length.to_le_bytes());
    data.areas
        .iter()
        .for_each(|area_data| area::serialization(&mut bin, area_data));
    bin.extend_from_slice(&data.buildings_length.to_le_bytes());
    data.buildings
        .iter()
        .for_each(|building_data| building::serialization(&mut bin, building_data));
    bin
}

fn decode_base64(string: &str) -> Result<Vec<u8>, BlueprintError<String>> {
    use base64::prelude::*;
    match BASE64_STANDARD.decode(string) {
        Ok(result) => Ok(result),
        Err(why) => Err(ReadBrokenBase64(why.to_string())),
    }
}

fn encode_base64(bin: Vec<u8>) -> String {
    use base64::prelude::*;
    BASE64_STANDARD.encode(bin)
}

fn decompress_gzip(gzip: Vec<u8>) -> Result<Vec<u8>, BlueprintError<String>> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    let mut decoder = GzDecoder::new(gzip.as_slice());
    let mut bin = Vec::new();
    match decoder.read_to_end(&mut bin) {
        Ok(_) => Ok(bin),
        Err(why) => Err(ReadBrokenGzip(why.to_string())),
    }
}

fn compress_gzip_zopfli(
    bin: Vec<u8>,
    iteration_count: u64,
    iterations_without_improvement: u64,
    maximum_block_splits: u16,
) -> Result<Vec<u8>, BlueprintError<std::io::Error>> {
    use std::num::NonZero;
    let options = zopfli::Options {
        // 防呆不防傻，这两个expect只有用户瞎几把输入参数才会炸
        iteration_count: NonZero::new(iteration_count)
            .expect("Fatal error: iteration_count must > 0"),
        iterations_without_improvement: NonZero::new(iterations_without_improvement)
            .expect("Fatal error: iterations_without_improvement must > 0"),
        maximum_block_splits: maximum_block_splits,
    };

    let mut gzip = Vec::new();

    match zopfli::compress(options, zopfli::Format::Gzip, bin.as_slice(), &mut gzip) {
        Ok(_) => Ok(gzip),
        Err(why) => Err(CanNotCompressGzip(why)),
    }
}

fn compress_gzip(bin: Vec<u8>) -> Result<Vec<u8>, BlueprintError<std::io::Error>> {
    compress_gzip_zopfli(bin, 256, u64::MAX, 0)
}

fn gzip_from_string(string: &str) -> Result<Vec<u8>, BlueprintError<String>> {
    decode_base64(string)
}

fn bin_from_gzip(gzip: Vec<u8>) -> Result<Vec<u8>, BlueprintError<String>> {
    decompress_gzip(gzip)
}

fn data_from_bin(bin: Vec<u8>) -> Result<ContentData, BlueprintError<String>> {
    deserialization(bin.as_slice())
}

fn bin_from_data(data: ContentData) -> Result<Vec<u8>, BlueprintError<String>> {
    Ok(serialization(data))
}

fn gzip_from_bin(bin: Vec<u8>) -> Result<Vec<u8>, BlueprintError<String>> {
    match compress_gzip(bin) {
        Ok(gzip) => Ok(gzip),
        Err(why) => Err(CanNotCompressGzip(why.to_string())),
    }
}

fn string_from_gzip(gzip: Vec<u8>) -> Result<String, BlueprintError<String>> {
    Ok(encode_base64(gzip))
}

pub fn bin_from_string(string: &str) -> Result<Vec<u8>, BlueprintError<String>> {
    let gzip = gzip_from_string(string)?;
    bin_from_gzip(gzip)
}

pub fn string_from_bin(bin: Vec<u8>) -> Result<String, BlueprintError<String>> {
    let gzip = gzip_from_bin(bin)?;
    string_from_gzip(gzip)
}

pub fn data_from_string(string: &str) -> Result<ContentData, BlueprintError<String>> {
    let gzip = gzip_from_string(string)?;
    let bin = bin_from_gzip(gzip)?;
    data_from_bin(bin)
}

pub fn string_from_data(data: ContentData) -> Result<String, BlueprintError<String>> {
    let bin = bin_from_data(data)?;
    let gzip = gzip_from_bin(bin)?;
    string_from_gzip(gzip)
}

#[cfg(test)]
mod test {
    // TODO test
}
