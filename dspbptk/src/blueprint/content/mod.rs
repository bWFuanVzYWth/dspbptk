pub mod area;
pub mod building;

use log::{error, warn};

use crate::error::DspbptkError;
use crate::error::DspbptkError::*;
use crate::error::DspbptkInfo::*;
use crate::error::DspbptkWarn::*;

use nom::{
    multi::count,
    number::complete::{le_i32, le_i8},
    IResult,
};

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

fn deserialization(bin: &[u8]) -> Option<ContentData> {
    use nom::Finish;
    match deserialization_non_finish(bin).finish() {
        Err(why) => {
            error!("{:?}", BrokenContent(why));
            None
        }
        Ok((unknown, result)) => {
            let unknown_length = unknown.len();
            match unknown_length {
                10.. => warn!("{:?}", LotUnknownAfterContent(unknown_length)),
                1..=9 => warn!("{:?}", FewUnknownAfterContent(unknown)),
                _ => {}
            };
            Some(result)
        }
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

fn decode_base64(string: &str) -> Option<Vec<u8>> {
    use base64::prelude::*;
    match BASE64_STANDARD.decode(string) {
        Ok(bin) => Some(bin),
        Err(why) => {
            error!("{:?}", BrokenBase64(why));
            None
        }
    }
}

fn encode_base64(bin: Vec<u8>) -> String {
    use base64::prelude::*;
    BASE64_STANDARD.encode(bin)
}

fn decompress_gzip<'a>(gzip: Vec<u8>) -> Option<Vec<u8>> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    let mut decoder = GzDecoder::new(&gzip[..]);
    let mut bin = Vec::new();
    match decoder.read_to_end(&mut bin) {
        Ok(_size) => Some(bin),
        Err(why) => {
            error!("{:?}", BrokenGzip(why));
            None
        }
    }
}

fn compress_gzip_zopfli(
    bin: Vec<u8>,
    zopfli_options: &zopfli::Options,
) -> Result<Vec<u8>, DspbptkError> {
    let mut gzip = Vec::new();
    zopfli::compress(
        *zopfli_options,
        zopfli::Format::Gzip,
        bin.as_slice(),
        &mut gzip,
    )
    .map_err(|e| CanNotCompressGzip(e))?;
    Ok(gzip)
}

fn compress_gzip(bin: Vec<u8>, zopfli_options: &zopfli::Options) -> Result<Vec<u8>, DspbptkError> {
    compress_gzip_zopfli(bin, zopfli_options)
}

pub fn gzip_from_string(string: &str) -> Option<Vec<u8>> {
    decode_base64(string)
}

pub fn bin_from_gzip<'a>(gzip: Vec<u8>) -> Option<Vec<u8>> {
    decompress_gzip(gzip)
}

// FIXME 所有权
pub fn data_from_bin<'a>(bin: &'a [u8]) -> Option<ContentData> {
    deserialization(bin)
}

pub fn bin_from_data<'a>(data: ContentData) -> Result<Vec<u8>, DspbptkError<'a>> {
    Ok(serialization(data))
}

pub fn gzip_from_bin(
    bin: Vec<u8>,
    zopfli_options: &zopfli::Options,
) -> Result<Vec<u8>, DspbptkError> {
    Ok(compress_gzip(bin, zopfli_options)?)
}

pub fn string_from_gzip(gzip: Vec<u8>) -> String {
    encode_base64(gzip)
}

pub fn bin_from_string(string: &str) -> Option<Vec<u8>> {
    let gzip = gzip_from_string(string)?;
    bin_from_gzip(gzip)
}

pub fn string_from_data(
    data: ContentData,
    zopfli_options: &zopfli::Options,
) -> Result<String, DspbptkError> {
    let bin = bin_from_data(data)?;
    let gzip = gzip_from_bin(bin, zopfli_options)?;
    Ok(string_from_gzip(gzip))
}

#[cfg(test)]
mod test {
    // TODO test
}
