pub mod blueprint {
    use nom::{
        bytes::complete::{tag, take, take_till},
        sequence::tuple,
        IResult,
    };

    fn take_till_quote(string: &str) -> IResult<&str, &str> {
        take_till(|c| c == '"')(string)
    }

    fn tag_quote(string: &str) -> IResult<&str, &str> {
        tag("\"")(string)
    }

    fn take_32(string: &str) -> IResult<&str, &str> {
        take(32usize)(string)
    }

    #[derive(Debug, PartialEq)]
    pub struct Blueprint<'blueprint> {
        pub head: &'blueprint str,
        pub data: &'blueprint str,
        pub md5f: &'blueprint str,
    }

    pub fn parser(string: &str) -> IResult<&str, Blueprint> {
        let (unknown, (head, _, data, _, md5f)) = tuple((
            take_till_quote,
            tag_quote,
            take_till_quote,
            tag_quote,
            take_32,
        ))(string)?;
        Ok((unknown, Blueprint { head, data, md5f }))
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parser() {
            let string = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,\"H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA\"E4E5A1CF28F1EC611E33498CBD0DF02B\n\0";
            let result = parser(string);

            assert_eq!(
                result,
                Ok((
                    "\n\0",
                    Blueprint {
                        head: "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,",
                        data: "H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA",
                        md5f: "E4E5A1CF28F1EC611E33498CBD0DF02B"
                    }
                ))
            );
        }
    }
}

pub mod head {
    use nom::{
        bytes::complete::{tag, take_till},
        multi::separated_list0,
        sequence::tuple,
        IResult,
    };

    fn tag_blueprint(string: &str) -> IResult<&str, &str> {
        tag("BLUEPRINT:0,")(string)
    }

    fn tag_comma(string: &str) -> IResult<&str, &str> {
        tag(",")(string)
    }

    fn tag_zero(string: &str) -> IResult<&str, &str> {
        tag(",0,")(string)
    }

    fn take_till_comma(string: &str) -> IResult<&str, &str> {
        take_till(|c| c == ',')(string)
    }

    #[derive(Debug, PartialEq)]
    pub struct Head<'head> {
        tag: &'head str,
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

    pub fn parser(string: &str) -> IResult<&str, Head> {
        let (
            unknown,
            (
                tag,
                layout,
                _,
                icons_0,
                _,
                icons_1,
                _,
                icons_2,
                _,
                icons_3,
                _,
                icons_4,
                _,
                time,
                _,
                game_version,
                _,
                short_desc,
                _,
                desc,
            ),
        ) = tuple((
            tag_blueprint,
            take_till_comma,
            tag_comma,
            take_till_comma,
            tag_comma,
            take_till_comma,
            tag_comma,
            take_till_comma,
            tag_comma,
            take_till_comma,
            tag_comma,
            take_till_comma,
            tag_zero,
            take_till_comma,
            tag_comma,
            take_till_comma,
            tag_comma,
            take_till_comma,
            tag_comma,
            take_till_comma,
        ))(string)?;
        Ok((
            unknown,
            Head {
                tag,
                layout,
                icons_0,
                icons_1,
                icons_2,
                icons_3,
                icons_4,
                time,
                game_version,
                short_desc,
                desc,
            },
        ))
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parser() {
            let string = "BLUEPRINT:0,9,0,1,2,3,4,0,5,6.7.8.9,,";

            let result = parser(string);

            assert_eq!(
                result,
                Ok((
                    "",
                    Head {
                        tag: "BLUEPRINT:0,",
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
}

pub mod data {
    use std::collections::BTreeMap;

    pub struct Area {
        index: i64,
        parent_index: i64,
        tropic_anchor: i64,
        area_segments: i64,
        anchor_local_offset: [i64; 2],
        width: i64,
        height: i64,
    }

    pub struct Building {
        magic_number: i64,
        index: i64,
        area_index: i64,
        local_offset: [f64; 4],
        local_offset2: [f64; 4],
        yaw: f64,
        yaw2: f64,
        tilt: f64,
        item_id: i64,
        model_index: i64,
        temp_output_obj_idx: i64,
        temp_input_obj_idx: i64,
        output_to_slot: i64,
        input_from_slot: i64,
        output_from_slot: i64,
        input_to_slot: i64,
        output_offset: i64,
        input_offset: i64,
        recipe_id: i64,
        filter_id: i64,
        parameters: Vec<u32>,
    }

    pub struct Data {
        version: i64,
        cursor_offset: [i64; 2],
        cursor_target_area: i64,
        drag_box_size: [i64; 2],
        primary_area_idx: i64,
        areas: BTreeMap<i64, Area>,
        buildings: BTreeMap<i64, Building>,
    }

    pub fn decode_base64(string: &str) -> Vec<u8> {
        use base64::prelude::*;
        let gzip = BASE64_STANDARD
            .decode(string)
            .expect("Failed to decode base64!");
        gzip
    }

    pub fn decode_gzip(vec: Vec<u8>) -> Vec<u8> {
        use miniz_oxide::inflate;
        let raw = inflate::decompress_to_vec(vec.as_slice()).expect("Failed to decode gzip!");
        raw
    }
}
