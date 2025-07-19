pub mod content;
pub mod header;
pub mod md5f;

use crate::{
    blueprint::data::Blueprint,
    error::{
        DspbptkError::{self, BrokenBlueprint},
        DspbptkWarn::{self, FewUnknownAfterBlueprint, LotUnknownAfterBlueprint},
    },
};

use nom::{
    Finish, IResult, Parser,
    bytes::complete::{tag, take, take_till},
    sequence::preceded,
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

fn parse_non_finish(string: &'_ str) -> IResult<&'_ str, Blueprint<'_>> {
    let unknown = string;

    let (unknown, header) = take_till_quote(unknown)?;
    let (unknown, content) = preceded(tag_quote, take_till_quote).parse(unknown)?;
    let (unknown, md5f) = preceded(tag_quote, take_32).parse(unknown)?;
    Ok((
        unknown,
        Blueprint {
            header,
            content,
            md5f,
            unknown,
        },
    ))
}

/// # Errors
/// 可能的原因：
/// * 蓝图已损坏，或者编码不受支持
pub fn parse(string: &'_ str) -> Result<(Blueprint<'_>, Vec<DspbptkWarn>), DspbptkError> {
    let (unknown, data) = parse_non_finish(string)
        .finish()
        .map_err(|e| BrokenBlueprint(e.clone().into()))?;
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
    let md5f = md5f::compute_md5f_string(&header_content);
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
                Blueprint {
                    header: "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,",
                    content: "H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA",
                    md5f: "E4E5A1CF28F1EC611E33498CBD0DF02B",
                    unknown: "\n\0",
                },
                vec![FewUnknownAfterBlueprint("\n\0".to_string())]
            ))
        );
    }

    #[test]
    fn test_serialization() {
        let blueprint_data = Blueprint {
            header: "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,",
            content: "H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA",
            md5f: "E4E5A1CF28F1EC611E33498CBD0DF02B",
            unknown: "\n\0",
        };
        assert_eq!(
            serialization(blueprint_data.header, blueprint_data.content),
            "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,\"H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA\"E4E5A1CF28F1EC611E33498CBD0DF02B"
        );
    }
}
