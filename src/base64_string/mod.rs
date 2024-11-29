pub mod content;
pub mod header;

use nom::{
    bytes::complete::{tag, take, take_till},
    sequence::{preceded, tuple},
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
pub struct BlueprintData<'blueprint> {
    pub header: &'blueprint str,
    pub content: &'blueprint str,
    pub md5f: &'blueprint str,
}

pub fn parse(string: &str) -> IResult<&str, BlueprintData> {
    let (unknown, blueprint) = tuple((
        take_till_quote,
        preceded(tag_quote, take_till_quote),
        preceded(tag_quote, take_32),
    ))(string)?;

    Ok((
        unknown,
        BlueprintData {
            header: blueprint.0,
            content: blueprint.1,
            md5f: blueprint.2,
        },
    ))
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
