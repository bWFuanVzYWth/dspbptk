mod parser {
    use nom::{
        bytes::complete::{tag, take, take_till},
        sequence::tuple,
        IResult,
    };

    #[derive(Debug, PartialEq)]
    pub struct Parser<'parser> {
        pub head: &'parser str,
        pub data: &'parser str,
        pub md5f: &'parser str,
    }

    fn take_till_quote(string: &str) -> IResult<&str, &str> {
        take_till(|c| c == '"')(string)
    }

    fn tag_quote(string: &str) -> IResult<&str, &str> {
        tag("\"")(string)
    }

    fn take_md5f(string: &str) -> IResult<&str, &str> {
        take(32usize)(string)
    }

    fn parser(string: &str) -> IResult<&str, Parser> {
        let (unknown, (head, _, data, _, md5f)) = tuple((
            take_till_quote,
            tag_quote,
            take_till_quote,
            tag_quote,
            take_md5f,
        ))(string)?;
        Ok((unknown, Parser { head, data, md5f }))
    }

    #[test]
    fn test_parser() {
        assert_eq!(
        parser("BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,\"H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA\"E4E5A1CF28F1EC611E33498CBD0DF02B\n\0"),
        Ok((
            "\n\0",
            Parser {
                head:"BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,",
                data:"H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA",
                md5f:"E4E5A1CF28F1EC611E33498CBD0DF02B"
            }
        ))
    );
    }
}

mod head_parser {}

mod data_parser {
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

    pub struct Blueprint<'blueprint> {
        //head
        layout: i64,
        icons: [i64; 5],
        time: i64,
        game_version: &'blueprint str,
        short_desc: &'blueprint str,
        desc: &'blueprint str,
        //data
        version: i64,
        cursor_offset: [i64; 2],
        cursor_target_area: i64,
        drag_box_size: [i64; 2],
        primary_area_idx: i64,
        areas: BTreeMap<i64, Area>,
        buildings: BTreeMap<i64, Building>,
        //md5f
        md5f: [u32; 4],
    }

    impl<'blueprint> Blueprint<'blueprint> {
        pub fn new() -> Blueprint<'blueprint> {
            Blueprint {
                layout: 0,
                icons: [0, 0, 0, 0, 0],
                time: 0,
                game_version: "",
                short_desc: "",
                desc: "",
                version: 0,
                cursor_offset: [0, 0],
                cursor_target_area: 0,
                drag_box_size: [0, 0],
                primary_area_idx: 0,
                areas: BTreeMap::new(),
                buildings: BTreeMap::new(),
                md5f: [0, 0, 0, 0],
            }
        }
    }
}
