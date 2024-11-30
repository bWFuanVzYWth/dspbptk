pub mod content;
pub mod header;

use nom::{
    bytes::complete::{tag, take, take_till},
    sequence::preceded,
    IResult,
};

fn tag_quote(string: &str) -> IResult<&str, &str> {
    tag("\"")(string)
}

fn take_32(string: &str) -> IResult<&str, &str> {
    take(32_usize)(string)
}

fn take_till_quote(string: &str) -> IResult<&str, &str> {
    take_till(|c| c == '"')(string)
}

#[derive(Debug, PartialEq)]
pub struct BlueprintData<'bp> {
    pub header: &'bp str,
    pub content: &'bp str,
    pub md5f: &'bp str,
}

pub fn parse(string: &str) -> IResult<&str, BlueprintData> {
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
        },
    ))
}

pub fn serialization(blueprint: BlueprintData) -> String {
    format!(
        "{}\"{}\"{}",
        blueprint.header, blueprint.content, blueprint.md5f
    )
}

pub fn compute_md5f_string(header: &str, content: &str) -> String {
    use crate::md5::*;
    let header_content = format!("{}\"{}", header, content);
    MD5::new(Algo::MD5F)
        .process(header_content.as_bytes())
        .iter()
        .map(|x| format!("{:02X}", x))
        .collect::<Vec<String>>()
        .join("")
}

pub fn has_broken_md5f(blueprint: BlueprintData) -> bool {
    let expect_md5f_string = compute_md5f_string(blueprint.header, blueprint.content);
    expect_md5f_string != blueprint.md5f
}

// TODO 异常处理：去掉两个unwrap()
pub fn decode_content(content: &str) -> Vec<u8> {
    use base64::prelude::*;
    use std::io::Read;
    let gzip_stream = BASE64_STANDARD.decode(content).unwrap();
    let mut decoder = flate2::read::GzDecoder::new(gzip_stream.as_slice());
    let mut memory_stream = Vec::new();
    decoder.read_to_end(&mut memory_stream).unwrap();
    memory_stream
}

// TODO 异常处理：去掉两个unwrap()
pub fn encode_content_with_options(
    memory_stream: Vec<u8>,
    iteration_count: u64,
    iterations_without_improvement: u64,
    maximum_block_splits: u16,
) -> String {
    use base64::prelude::*;
    use std::num::NonZero;
    let options = zopfli::Options {
        iteration_count: NonZero::new(iteration_count).unwrap(),
        iterations_without_improvement: NonZero::new(iterations_without_improvement).unwrap(),
        maximum_block_splits: maximum_block_splits,
    };
    let mut gzip_stream = Vec::new();
    zopfli::compress(
        options,
        zopfli::Format::Gzip,
        memory_stream.as_slice(),
        &mut gzip_stream,
    )
    .unwrap();
    BASE64_STANDARD.encode(gzip_stream)
}

pub fn encode_content(memory_stream: Vec<u8>) -> String {
    encode_content_with_options(memory_stream, 64, u64::MAX, 64)
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
            Ok((
                "\n\0",
                BlueprintData {
                    header: "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,",
                    content: "H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA",
                    md5f: "E4E5A1CF28F1EC611E33498CBD0DF02B"
                }
            ))
        );
    }
}
