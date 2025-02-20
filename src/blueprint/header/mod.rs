use nom::{
    bytes::complete::{tag, take_till},
    sequence::preceded,
    Finish, IResult,
};

use crate::error::{DspbptkError, DspbptkError::BrokenHeader, DspbptkWarn, DspbptkWarn::UnknownAfterHeader};

#[derive(Debug, Clone, PartialEq)]
pub struct HeaderData {
    pub layout: String,
    pub icons_0: String,
    pub icons_1: String,
    pub icons_2: String,
    pub icons_3: String,
    pub icons_4: String,
    pub time: String,
    pub game_version: String,
    pub short_desc: String,
    pub desc: String,
    pub unknown: String,
}

impl Default for HeaderData {
    fn default() -> Self {
        Self {
            layout: "0".to_string(),
            icons_0: "0".to_string(),
            icons_1: "0".to_string(),
            icons_2: "0".to_string(),
            icons_3: "0".to_string(),
            icons_4: "0".to_string(),
            time: "0".to_string(),
            game_version: String::new(),
            short_desc: String::new(),
            desc: String::new(),
            unknown: String::new(),
        }
    }
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

fn parse_non_finish(string: &str) -> IResult<&str, HeaderData> {
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
        HeaderData {
            layout: layout.to_string(),
            icons_0: icons_0.to_string(),
            icons_1: icons_1.to_string(),
            icons_2: icons_2.to_string(),
            icons_3: icons_3.to_string(),
            icons_4: icons_4.to_string(),
            time: time.to_string(),
            game_version: game_version.to_string(),
            short_desc: short_desc.to_string(),
            desc: desc.to_string(),
            unknown: unknown.to_string(),
        },
    ))
}

pub fn parse(string: &str) -> Result<(HeaderData, Vec<DspbptkWarn>), DspbptkError> {
    let (unknown, data) = parse_non_finish(string).finish().map_err(BrokenHeader)?;
    match unknown.len() {
        0 => Ok((data, Vec::new())),
        _ => Ok((data, vec![UnknownAfterHeader])),
    }
}

pub fn serialization(data: &HeaderData) -> String {
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
            Some((
                HeaderData {
                    layout: "9".to_string(),
                    icons_0: "0".to_string(),
                    icons_1: "1".to_string(),
                    icons_2: "2".to_string(),
                    icons_3: "3".to_string(),
                    icons_4: "4".to_string(),
                    time: "5".to_string(),
                    game_version: "6.7.8.9".to_string(),
                    short_desc: "".to_string(),
                    desc: "".to_string(),
                    unknown: "".to_string(),
                },
                Vec::new()
            ))
        );
    }
}
