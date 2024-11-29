use nom::{
    bytes::complete::{tag, take_till},
    sequence::{preceded, tuple},
    IResult,
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

#[derive(Debug, PartialEq)]
pub struct Head<'head> {
    layout: &'head str,
    icons_0: &'head str,
    icons_1: &'head str,
    icons_2: &'head str,
    icons_3: &'head str,
    icons_4: &'head str,
    time: &'head str,
    game_version: &'head str,
    short_desc: &'head str,
    desc: &'head str,
}

pub fn parse(string: &str) -> IResult<&str, Head> {
    let (unknown, head) = tuple((
        preceded(tag_blueprint, take_till_comma),
        preceded(tag_comma, take_till_comma),
        preceded(tag_comma, take_till_comma),
        preceded(tag_comma, take_till_comma),
        preceded(tag_comma, take_till_comma),
        preceded(tag_comma, take_till_comma),
        preceded(tag_zero, take_till_comma),
        preceded(tag_comma, take_till_comma),
        preceded(tag_comma, take_till_comma),
        preceded(tag_comma, take_till_comma),
    ))(string)?;

    Ok((
        unknown,
        Head {
            layout: head.0,
            icons_0: head.1,
            icons_1: head.2,
            icons_2: head.3,
            icons_3: head.4,
            icons_4: head.5,
            time: head.6,
            game_version: head.7,
            short_desc: head.8,
            desc: head.9,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let string = "BLUEPRINT:0,9,0,1,2,3,4,0,5,6.7.8.9,,";

        let result = parse(string);

        assert_eq!(
            result,
            Ok((
                "",
                Head {
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
                }
            ))
        );
    }
}
