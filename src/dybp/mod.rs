mod header;

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
pub struct DysonBlueprintData<'bp> {
    pub header: &'bp str,
    pub content: &'bp str,
    pub md5f: &'bp str,
}

pub fn parse(string: &str) -> IResult<&str, DysonBlueprintData> {
    let unknown = string;

    let (unknown, header) = take_till_quote(unknown)?;
    let (unknown, content) = preceded(tag_quote, take_till_quote)(unknown)?;
    let (unknown, md5f) = preceded(tag_quote, take_32)(unknown)?;
    Ok((
        unknown,
        DysonBlueprintData {
            header: header,
            content: content,
            md5f: md5f,
        },
    ))
}
