pub mod area;
pub mod building;

use crate::{
    blueprint::{Content, Version},
    error::{
        DspbptkError::{self, BrokenBase64, BrokenContent, BrokenGzip, CanNotCompressGzip},
        DspbptkWarn::{self, FewUnknownAfterContent, LotUnknownAfterContent},
    },
};
use base64::prelude::*;
use flate2::read::GzDecoder;
use nom::{
    Finish, IResult, Parser,
    multi::count,
    number::complete::{le_i32, le_u8, le_u32},
};
use std::io::Read;

impl Content {
    /// # Errors
    /// 可能的原因：
    /// * content编码错误，或者编码不受支持
    pub fn from_bin(bin: &'_ [u8]) -> Result<(Self, Vec<DspbptkWarn>), DspbptkError> {
        let (unknown, content) = deserialization_non_finish(bin)
            .finish()
            .map_err(|e| BrokenContent(e.clone().into()))?;
        let unknown_length = content.unknown.len();
        let warns = match unknown_length {
            10.. => vec![LotUnknownAfterContent(unknown_length)],
            1..=9 => vec![FewUnknownAfterContent(unknown.to_vec())],
            _ => Vec::new(),
        };
        Ok((content, warns))
    }

    // TODO 性能优化，当前实现存在多次数组拓容。
    #[must_use]
    pub fn to_bin(&self) -> Vec<u8> {
        let mut bin = Vec::new();
        bin.extend_from_slice(&self.patch.to_le_bytes());
        bin.extend_from_slice(&self.cursor_offset_x.to_le_bytes());
        bin.extend_from_slice(&self.cursor_offset_y.to_le_bytes());
        bin.extend_from_slice(&self.cursor_target_area.to_le_bytes());
        bin.extend_from_slice(&self.drag_box_size_x.to_le_bytes());
        bin.extend_from_slice(&self.drag_box_size_y.to_le_bytes());
        bin.extend_from_slice(&self.primary_area_idx.to_le_bytes());
        bin.extend_from_slice(&self.areas_length.to_le_bytes());
        self.areas
            .iter()
            .for_each(|area_data| area::serialization(&mut bin, area_data));
        bin.extend_from_slice(&self.buildings_length.to_le_bytes());
        self.buildings.iter().for_each(|building_data| {
            building::serialization(&mut bin, building_data, &Version::Neg101);
        });
        bin
    }
}

fn deserialization_non_finish(bin: &[u8]) -> IResult<&[u8], Content> {
    let unknown = bin;

    let (unknown, patch) = le_i32(unknown)?;
    let (unknown, cursor_offset_x) = le_i32(unknown)?;
    let (unknown, cursor_offset_y) = le_i32(unknown)?;
    let (unknown, cursor_target_area) = le_i32(unknown)?;
    let (unknown, drag_box_size_x) = le_i32(unknown)?;
    let (unknown, drag_box_size_y) = le_i32(unknown)?;
    let (unknown, primary_area_idx) = le_i32(unknown)?;
    let (unknown, areas_length) = le_u8(unknown)?;
    let (unknown, areas) =
        count(area::deserialization, usize::from(areas_length)).parse(unknown)?;
    let (unknown, buildings_length) = le_u32(unknown)?;
    let (unknown, buildings) =
        count(building::deserialization, buildings_length as usize).parse(unknown)?;

    Ok((
        unknown,
        Content {
            patch,
            cursor_offset_x,
            cursor_offset_y,
            cursor_target_area,
            drag_box_size_x,
            drag_box_size_y,
            primary_area_idx,
            areas_length,
            areas,
            buildings_length,
            buildings,
            unknown: unknown.to_vec(),
        },
    ))
}

fn gzip_from_string(string: &'_ str) -> Result<Vec<u8>, DspbptkError> {
    match BASE64_STANDARD.decode(string) {
        Ok(bin) => Ok(bin),
        Err(why) => Err(BrokenBase64(why)),
    }
}

fn string_from_gzip(gzip: &[u8]) -> String {
    BASE64_STANDARD.encode(gzip)
}

fn bin_from_gzip(gzip: &[u8]) -> Result<Vec<u8>, DspbptkError> {
    let mut bin = Vec::new();
    let mut decoder = GzDecoder::new(gzip);
    decoder.read_to_end(&mut bin).map_err(BrokenGzip)?;
    Ok(bin)
}

fn compress_gzip_zopfli(
    bin: &[u8],
    zopfli_options: &zopfli::Options,
) -> Result<Vec<u8>, DspbptkError> {
    let mut gzip = Vec::new();
    zopfli::compress(*zopfli_options, zopfli::Format::Gzip, bin, &mut gzip)
        .map_err(CanNotCompressGzip)?;
    Ok(gzip)
}

fn gzip_from_bin(bin: &[u8], zopfli_options: &zopfli::Options) -> Result<Vec<u8>, DspbptkError> {
    compress_gzip_zopfli(bin, zopfli_options)
}

/// # Errors
/// 可能的原因：
/// * base64解码错误，说明数据已损坏
/// * gzip解压错误，说明数据已损坏
pub fn bin_from_string(string: &str) -> Result<Vec<u8>, DspbptkError> {
    let gzip = gzip_from_string(string)?;
    bin_from_gzip(&gzip)
}

/// # Errors
/// 可能的原因：
/// * gzip压缩错误，这个错误通常不该出现，万一真炸了得去看zopfli的文档
pub fn string_from_data(
    data: &Content,
    zopfli_options: &zopfli::Options,
) -> Result<String, DspbptkError> {
    let bin = data.to_bin();
    let gzip = gzip_from_bin(&bin, zopfli_options)?;
    Ok(string_from_gzip(&gzip))
}
