pub mod content;
pub mod error;
pub mod header;

use log::{debug, error, info, trace, warn};
use nom::{
    bytes::complete::{tag, take, take_till},
    sequence::preceded,
    IResult,
};

use error::DspbptkError;
use error::DspbptkError::*;

#[derive(Debug, PartialEq)]
pub struct BlueprintData<'bp> {
    pub header: &'bp str,
    pub content: &'bp str,
    pub md5f: &'bp str,
    pub unknown: &'bp str,
}

fn tag_quote(string: &str) -> IResult<&str, &str> {
    tag("\"")(string)
}

fn take_32(string: &str) -> IResult<&str, &str> {
    take(32_usize)(string)
}

fn take_till_quote(string: &str) -> IResult<&str, &str> {
    take_till(|c| c == '"')(string)
}

pub fn parse_non_finish(string: &str) -> IResult<&str, BlueprintData> {
    let unknown = string;

    let (unknown, header) = take_till_quote(unknown)?;
    let (unknown, content) = preceded(tag_quote, take_till_quote)(unknown)?;
    let (unknown, md5f) = preceded(tag_quote, take_32)(unknown)?;
    Ok((
        unknown,
        BlueprintData {
            header: header,
            content: content,
            md5f: md5f,
            unknown: unknown,
        },
    ))
}

pub fn parse(string: &str) -> Result<BlueprintData, DspbptkError<&str>> {
    use nom::Finish;
    let tmp = parse_non_finish(string).finish();
    match parse_non_finish(string).finish() {
        Ok((_unknown, data)) => Ok(data),
        Err(why) => {
            error!("{:#?}", why);
            Err(CanNotParseBluePrint)
        }
    }
}

pub fn compute_md5f_string(header_content: &str) -> String {
    use crate::md5::*;
    let md5f = MD5::new(Algo::MD5F).process(header_content.as_bytes());
    format!("{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        md5f[0], md5f[1], md5f[2], md5f[3], md5f[4], md5f[5], md5f[6], md5f[7], md5f[8], md5f[9], md5f[10], md5f[11], md5f[12], md5f[13], md5f[14], md5f[15])
}

pub fn serialization(header: &str, content: &str) -> String {
    let mut header_content = format!("{}\"{}", header, content);
    let md5f = compute_md5f_string(&header_content);
    header_content.push_str("\"");
    header_content.push_str(&md5f);
    header_content
}

pub fn decode_base64(base64_string: &str) -> Result<Vec<u8>, DspbptkError<&str>> {
    use base64::prelude::*;
    match BASE64_STANDARD.decode(base64_string) {
        Ok(result) => Ok(result),
        Err(why) => {
            error!("{:#?}", why);
            Err(ReadBrokenBase64)
        }
    }
}

pub fn encode_base64(bin: Vec<u8>) -> String {
    use base64::prelude::*;
    BASE64_STANDARD.encode(bin)
}

pub fn decompress_gzip(gzip_stream: Vec<u8>) -> Result<Vec<u8>, DspbptkError<&str>> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    let mut decoder = GzDecoder::new(gzip_stream.as_slice());
    let mut memory_stream = Vec::new();
    match decoder.read_to_end(&mut memory_stream) {
        Ok(_) => Ok(memory_stream),
        Err(why) => {
            error!("{:#?}", why);
            Err(ReadBrokenGzip)
        }
    }
}

pub fn compress_gzip_zopfli(
    bin: Vec<u8>,
    iteration_count: u64,
    iterations_without_improvement: u64,
    maximum_block_splits: u16,
) -> Result<Vec<u8>, DspbptkError<&str>> {
    // check options
    if iteration_count == 0 || iterations_without_improvement == 0 {
        return Err(IllegalCompressParameters);
    };

    // set options
    let options = zopfli::Options {
        iteration_count: std::num::NonZero::new(iteration_count).unwrap(/* impossible */),
        iterations_without_improvement: std::num::NonZero::new(iterations_without_improvement).unwrap(/* impossible */),
        maximum_block_splits: maximum_block_splits,
    };

    // create write buffer
    let mut gzip_stream = Vec::new();

    // compress
    match zopfli::compress(
        options,
        zopfli::Format::Gzip,
        bin.as_slice(),
        &mut gzip_stream,
    ) {
        Ok(_) => Ok(gzip_stream),
        Err(why) => {
            error!("{:#?}", why);
            Err(CanNotCompressGzip)
        }
    }
}

pub fn compress_gzip(bin: Vec<u8>) -> Result<Vec<u8>, DspbptkError<&str>> {
    compress_gzip_zopfli(bin, 256, u64::MAX, 0)
}

pub fn decode_content(content: &str) -> Result<Vec<u8>, DspbptkError<&str>> {
    decompress_gzip(decode_base64(content)?)
}

pub fn encode_content(memory_stream: Vec<u8>) -> Result<String, DspbptkError<&str>> {
    Ok(encode_base64(compress_gzip(memory_stream)?))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let string = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,\"H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA\"E4E5A1CF28F1EC611E33498CBD0DF02B\n\0";
        let result = parse(string);

        assert_eq!(
            result,
            Ok(BlueprintData {
                header: "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,",
                content: "H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA",
                md5f: "E4E5A1CF28F1EC611E33498CBD0DF02B",
                unknown: "\n\0",
            })
        );
    }
}
