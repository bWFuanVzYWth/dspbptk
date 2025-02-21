pub mod area;
pub mod building;

use crate::error::{
    DspbptkError,
    DspbptkError::{BrokenBase64, BrokenContent, BrokenGzip, CanNotCompressGzip},
    DspbptkWarn,
    DspbptkWarn::{FewUnknownAfterContent, LotUnknownAfterContent},
};

use nom::{
    multi::count,
    number::complete::{le_i32, le_u32, le_u8},
    IResult,
};

#[derive(Debug, Clone)]
pub struct ContentData {
    pub patch: i32,
    pub cursor_offset_x: i32,
    pub cursor_offset_y: i32,
    pub cursor_target_area: i32,
    pub drag_box_size_x: i32,
    pub drag_box_size_y: i32,
    pub primary_area_idx: i32,
    pub areas_length: u8,
    pub areas: Vec<area::AreaData>,
    pub buildings_length: u32,
    pub buildings: Vec<building::BuildingData>,
    pub unknown: Vec<u8>,
}

impl ContentData {
    pub fn from_bin(bin: &[u8]) -> Result<(Self, Vec<DspbptkWarn>), DspbptkError> {
        use nom::Finish;
        let (unknown, content) = deserialization_non_finish(bin)
            .finish()
            .map_err(BrokenContent)?;
        let unknown_length = content.unknown.len();
        let warns = match unknown_length {
            10.. => vec![LotUnknownAfterContent(unknown_length)],
            1..=9 => vec![FewUnknownAfterContent(unknown.to_vec())],
            _ => Vec::new(),
        };
        Ok((content, warns))
    }

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
        self.buildings
            .iter()
            .for_each(|building_data| building::serialization(&mut bin, building_data));
        bin
    }
}

impl Default for ContentData {
    fn default() -> Self {
        Self {
            patch: 0,
            cursor_offset_x: 0,
            cursor_offset_y: 0,
            cursor_target_area: 0,
            drag_box_size_x: 1,
            drag_box_size_y: 1,
            primary_area_idx: 0,
            areas_length: 1, // 默认一个区域
            areas: vec![area::AreaData::default()],
            buildings_length: 0,
            buildings: Vec::new(),
            unknown: Vec::new(),
        }
    }
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
    let (unknown, areas_length) = le_u8(unknown)?;
    let (unknown, areas) = count(area::deserialization, usize::from(areas_length))(unknown)?;
    let (unknown, buildings_length) = le_u32(unknown)?;
    let (unknown, buildings) = count(
        building::deserialization,
        usize::try_from(buildings_length).expect("Fatal error: can not casting `u32` to `usize`"),
    )(unknown)?;

    Ok((
        unknown,
        ContentData {
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

fn gzip_from_string(string: &str) -> Result<Vec<u8>, DspbptkError> {
    use base64::prelude::*;
    match BASE64_STANDARD.decode(string) {
        Ok(bin) => Ok(bin),
        Err(why) => Err(BrokenBase64(why)),
    }
}

fn string_from_gzip(gzip: &[u8]) -> String {
    use base64::prelude::*;
    BASE64_STANDARD.encode(gzip)
}

fn bin_from_gzip<'a>(bin: &mut Vec<u8>, gzip: &[u8]) -> Result<(), DspbptkError<'a>> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    let mut decoder = GzDecoder::new(gzip);
    decoder.read_to_end(bin).map_err(BrokenGzip)?;
    Ok(())
}

fn compress_gzip_zopfli<'a>(
    bin: &[u8],
    zopfli_options: &zopfli::Options,
) -> Result<Vec<u8>, DspbptkError<'a>> {
    let mut gzip = Vec::new();
    zopfli::compress(*zopfli_options, zopfli::Format::Gzip, bin, &mut gzip)
        .map_err(CanNotCompressGzip)?;
    Ok(gzip)
}

fn gzip_from_bin<'a>(
    bin: &[u8],
    zopfli_options: &zopfli::Options,
) -> Result<Vec<u8>, DspbptkError<'a>> {
    compress_gzip_zopfli(bin, zopfli_options)
}

pub fn bin_from_string<'a>(
    content_bin: &mut Vec<u8>,
    string: &'a str,
) -> Result<(), DspbptkError<'a>> {
    let gzip = gzip_from_string(string)?;
    bin_from_gzip(content_bin, &gzip)?;
    Ok(())
}

pub fn string_from_data<'a>(
    data: &ContentData,
    zopfli_options: &zopfli::Options,
) -> Result<String, DspbptkError<'a>> {
    let bin = data.to_bin();
    let gzip = gzip_from_bin(&bin, zopfli_options)?;
    Ok(string_from_gzip(&gzip))
}

#[cfg(test)]
mod test {
    // TODO test
}
