pub mod content;
pub mod header;
pub mod md5;

use crate::error::{
    DspbptkError,
    DspbptkError::BrokenBlueprint,
    DspbptkWarn,
    DspbptkWarn::{FewUnknownAfterBlueprint, LotUnknownAfterBlueprint},
};

use nom::{
    bytes::complete::{tag, take, take_till},
    sequence::preceded,
    Finish, IResult,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BlueprintData<'a> {
    pub header: &'a str,
    pub content: &'a str,
    pub md5f: &'a str,
    pub unknown: &'a str,
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
            header,
            content,
            md5f,
            unknown,
        },
    ))
}

pub fn parse(string: &str) -> Result<(BlueprintData, Vec<DspbptkWarn>), DspbptkError> {
    let (unknown, data) = parse_non_finish(string).finish().map_err(BrokenBlueprint)?;
    let unknown_length = unknown.len();
    let warns = match unknown.len() {
        10.. => vec![LotUnknownAfterBlueprint(unknown_length)],
        1..=9 => vec![FewUnknownAfterBlueprint(unknown.to_string())],
        _ => Vec::new(),
    };
    Ok((data, warns))
}

#[must_use]
pub fn serialization(header: &str, content: &str) -> String {
    let mut header_content = format!("{header}\"{content}");
    let md5f = md5::compute_md5f_string(&header_content);
    header_content.push('"');
    header_content.push_str(&md5f);
    header_content
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let string = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,\"H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA\"E4E5A1CF28F1EC611E33498CBD0DF02B\n\0";
        let result = parse(string);

        assert_eq!(
            result.ok(),
            Some((
                BlueprintData {
                    header: "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,",
                    content: "H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA",
                    md5f: "E4E5A1CF28F1EC611E33498CBD0DF02B",
                    unknown: "\n\0",
                },
                vec![FewUnknownAfterBlueprint("\n\0".to_string())]
            ))
        );
    }
}
