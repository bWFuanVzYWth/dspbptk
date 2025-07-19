use nom::{
    Finish, IResult, Parser,
    bytes::complete::{tag, take_till},
    sequence::preceded,
};
use crate::{
    blueprint::Header,
    error::{
        DspbptkError::{self, BrokenHeader},
        DspbptkWarn::{self, UnknownAfterHeader},
    },
};

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

fn parse_non_finish(string: &str) -> IResult<&str, Header> {
    let unknown = string;

    let (unknown, layout) = preceded(tag_blueprint, take_till_comma).parse(unknown)?;
    let (unknown, icons_0) = preceded(tag_comma, take_till_comma).parse(unknown)?;
    let (unknown, icons_1) = preceded(tag_comma, take_till_comma).parse(unknown)?;
    let (unknown, icons_2) = preceded(tag_comma, take_till_comma).parse(unknown)?;
    let (unknown, icons_3) = preceded(tag_comma, take_till_comma).parse(unknown)?;
    let (unknown, icons_4) = preceded(tag_comma, take_till_comma).parse(unknown)?;
    let (unknown, time) = preceded(tag_zero, take_till_comma).parse(unknown)?;
    let (unknown, game_version) = preceded(tag_comma, take_till_comma).parse(unknown)?;
    let (unknown, short_desc) = preceded(tag_comma, take_till_comma).parse(unknown)?;
    let (unknown, desc) = preceded(tag_comma, take_till_comma).parse(unknown)?;

    Ok((
        unknown,
        Header {
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

/// # Errors
/// 可能的原因：
/// * 蓝图的header已经损坏，或编码不受支持
pub fn parse(string: &'_ str) -> Result<(Header, Vec<DspbptkWarn>), DspbptkError> {
    let (unknown, data) = parse_non_finish(string)
        .finish()
        .map_err(|e| BrokenHeader(e.clone().into()))?;
    match unknown.len() {
        0 => Ok((data, Vec::new())),
        _ => Ok((data, vec![UnknownAfterHeader])),
    }
}

#[must_use]
pub fn serialization(data: &Header) -> String {
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
    use crate::blueprint::data::header::Header;
    use super::*;

    #[test]
    fn test_parse() {
        let string = "BLUEPRINT:0,9,0,1,2,3,4,0,5,6.7.8.9,,";

        let result = parse(string);

        assert_eq!(
            result.ok(),
            Some((
                Header {
                    layout: "9".to_string(),
                    icons_0: "0".to_string(),
                    icons_1: "1".to_string(),
                    icons_2: "2".to_string(),
                    icons_3: "3".to_string(),
                    icons_4: "4".to_string(),
                    time: "5".to_string(),
                    game_version: "6.7.8.9".to_string(),
                    short_desc: String::new(),
                    desc: String::new(),
                    unknown: String::new(),
                },
                Vec::new()
            ))
        );
    }

    #[test]
    fn test_serialization() {
        let header = Header {
            layout: "9".to_string(),
            icons_0: "0".to_string(),
            icons_1: "1".to_string(),
            icons_2: "2".to_string(),
            icons_3: "3".to_string(),
            icons_4: "4".to_string(),
            time: "5".to_string(),
            game_version: "6.7.8.9".to_string(),
            short_desc: String::new(),
            desc: String::new(),
            unknown: String::new(),
        };

        assert_eq!(
            serialization(&header),
            "BLUEPRINT:0,9,0,1,2,3,4,0,5,6.7.8.9,,"
        );
    }
}
