pub mod content;
pub mod error;
pub mod header;

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

fn parse_non_finish(string: &str) -> IResult<&str, BlueprintData> {
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

pub fn parse(string: &str) -> Result<BlueprintData, DspbptkError<String>> {
    use nom::Finish;
    // let tmp = parse_non_finish(string).finish();
    match parse_non_finish(string).finish() {
        Ok((_unknown, data)) => Ok(data),
        Err(why) => Err(CanNotParseBluePrint(why.to_string())),
    }
}

pub fn serialization(header: &str, content: &str) -> String {
    use crate::md5::compute_md5f_string;
    let mut header_content = format!("{}\"{}", header, content);
    let md5f = compute_md5f_string(&header_content);
    header_content.push_str("\"");
    header_content.push_str(&md5f);
    header_content
}

fn decode_base64(base64_string: &str) -> Result<Vec<u8>, DspbptkError<String>> {
    use base64::prelude::*;
    match BASE64_STANDARD.decode(base64_string) {
        Ok(result) => Ok(result),
        Err(why) => Err(ReadBrokenBase64(why.to_string())),
    }
}

fn encode_base64(bin: Vec<u8>) -> String {
    use base64::prelude::*;
    BASE64_STANDARD.encode(bin)
}

fn decompress_gzip(gzip_stream: Vec<u8>) -> Result<Vec<u8>, DspbptkError<String>> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    let mut decoder = GzDecoder::new(gzip_stream.as_slice());
    let mut memory_stream = Vec::new();
    match decoder.read_to_end(&mut memory_stream) {
        Ok(_) => Ok(memory_stream),
        Err(why) => Err(ReadBrokenGzip(why.to_string())),
    }
}

fn compress_gzip_zopfli(
    bin: Vec<u8>,
    iteration_count: u64,
    iterations_without_improvement: u64,
    maximum_block_splits: u16,
) -> Result<Vec<u8>, DspbptkError<std::io::Error>> {
    let options = zopfli::Options {
        // 防呆不防傻，这两个expect只有用户瞎几把输入参数才会炸
        iteration_count: std::num::NonZero::new(iteration_count)
            .expect("Fatal error: iteration_count must greater than 0"),
        iterations_without_improvement: std::num::NonZero::new(iterations_without_improvement)
            .expect("Fatal error: iterations_without_improvement must greater than 0"),
        maximum_block_splits: maximum_block_splits,
    };

    let mut gzip_stream = Vec::new();

    match zopfli::compress(
        options,
        zopfli::Format::Gzip,
        bin.as_slice(),
        &mut gzip_stream,
    ) {
        Ok(_) => Ok(gzip_stream),
        Err(why) => Err(CanNotCompressGzip(why)),
    }
}

fn compress_gzip(bin: Vec<u8>) -> Result<Vec<u8>, DspbptkError<std::io::Error>> {
    compress_gzip_zopfli(bin, 256, u64::MAX, 0)
}

pub fn decode_content(content: &str) -> Result<Vec<u8>, DspbptkError<String>> {
    decompress_gzip(decode_base64(content)?)
}

pub fn encode_content(memory_stream: Vec<u8>) -> Result<String, DspbptkError<String>> {
    match compress_gzip(memory_stream) {
        Ok(gzip_stream) => Ok(encode_base64(gzip_stream)),
        Err(why) => Err(CanNotCompressGzip(why.to_string())),
    }
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
