use nom::{
    bytes::complete::{tag, take_till},
    sequence::preceded,
    IResult,
};

fn tag_dybp(string: &str) -> IResult<&str, &str> {
    tag("DYBP:0,")(string)
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

#[derive(Debug, PartialEq)]
pub struct Head<'head> {
    time: &'head str,
    game_version: &'head str,
    dyson_blueprint_type: &'head str,
    lat_limit: &'head str,
}

pub fn parse(string: &str) -> IResult<&str, Head> {
    let unknown = string;

    let (unknown, time) = preceded(tag_dybp, take_till_comma)(unknown)?;
    let (unknown, game_version) = preceded(tag_comma, take_till_comma)(unknown)?;
    let (unknown, dyson_blueprint_type) = preceded(tag_comma, take_till_comma)(unknown)?;
    let (unknown, lat_limit) = preceded(tag_comma, take_till_comma)(unknown)?;

    Ok((
        unknown,
        Head {
            time: time,
            game_version: game_version,
            dyson_blueprint_type: dyson_blueprint_type,
            lat_limit: lat_limit,
        },
    ))
}
