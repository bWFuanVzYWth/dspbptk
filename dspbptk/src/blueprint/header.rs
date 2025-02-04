use nom::{
    bytes::complete::{tag, take_till},
    sequence::preceded,
    Finish, IResult,
};

use crate::error::DspbptkError;
use crate::error::DspbptkError::*;

#[derive(Debug, PartialEq)]
pub struct HeadData<'a> {
    pub layout: &'a str,
    pub icons_0: &'a str,
    pub icons_1: &'a str,
    pub icons_2: &'a str,
    pub icons_3: &'a str,
    pub icons_4: &'a str,
    pub time: &'a str,
    pub game_version: &'a str,
    pub short_desc: &'a str,
    pub desc: &'a str,
    pub unknown: &'a str,
}

fn tag_blueprint(string: &str) -> IResult<&str, &str> {
    tag("BLUEPRINT:0,")(string)
}

fn tag_zero(string: &str) -> IResult<&str, &str> {
    tag(",0,")(string)
}

fn tag_comma(string: &str) -> IResult<&str, &str> {
    tag(",")(string)
}

fn take_till_comma(string: &str) -> IResult<&str, &str> {
    take_till(|c| c == ',')(string)
}

pub fn parse_non_finish(string: &str) -> IResult<&str, HeadData> {
    let unknown = string;

    let (unknown, layout) = preceded(tag_blueprint, take_till_comma)(unknown)?;
    let (unknown, icons_0) = preceded(tag_comma, take_till_comma)(unknown)?;
    let (unknown, icons_1) = preceded(tag_comma, take_till_comma)(unknown)?;
    let (unknown, icons_2) = preceded(tag_comma, take_till_comma)(unknown)?;
    let (unknown, icons_3) = preceded(tag_comma, take_till_comma)(unknown)?;
    let (unknown, icons_4) = preceded(tag_comma, take_till_comma)(unknown)?;
    let (unknown, time) = preceded(tag_zero, take_till_comma)(unknown)?;
    let (unknown, game_version) = preceded(tag_comma, take_till_comma)(unknown)?;
    let (unknown, short_desc) = preceded(tag_comma, take_till_comma)(unknown)?;
    let (unknown, desc) = preceded(tag_comma, take_till_comma)(unknown)?;

    Ok((
        unknown,
        HeadData {
            layout: layout,
            icons_0: icons_0,
            icons_1: icons_1,
            icons_2: icons_2,
            icons_3: icons_3,
            icons_4: icons_4,
            time: time,
            game_version: game_version,
            short_desc: short_desc,
            desc: desc,
            unknown: unknown,
        },
    ))
}

pub fn parse(string: &str) -> Result<HeadData, DspbptkError> {
    Ok(parse_non_finish(string)
        .finish()
        .map_err(|e| BrokenHeader(e))?
        .1)
}

pub fn serialization(data: &HeadData) -> String {
    format!(
        "BLUEPRINT:0,{},{},{},{},{},{},0,{},{},{},{}",
        data.layout,
        data.icons_0,
        data.icons_1,
        data.icons_2,
        data.icons_3,
        data.icons_4,
        data.time,
        data.game_version,
        data.short_desc,
        data.desc,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let string = "BLUEPRINT:0,9,0,1,2,3,4,0,5,6.7.8.9,,";

        let result = parse(string);

        assert_eq!(
            result.ok(),
            Some(HeadData {
                layout: "9",
                icons_0: "0",
                icons_1: "1",
                icons_2: "2",
                icons_3: "3",
                icons_4: "4",
                time: "5",
                game_version: "6.7.8.9",
                short_desc: "",
                desc: "",
                unknown: "",
            })
        );
    }
}
