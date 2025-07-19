pub mod area;
pub mod building;

use crate::blueprint::data::content::area::Area;

#[derive(Debug, Clone, PartialEq)]
pub struct Content {
    pub patch: i32,
    pub cursor_offset_x: i32,
    pub cursor_offset_y: i32,
    pub cursor_target_area: i32,
    pub drag_box_size_x: i32,
    pub drag_box_size_y: i32,
    pub primary_area_idx: i32,
    pub areas_length: u8,
    pub areas: Vec<area::Area>,
    pub buildings_length: u32,
    pub buildings: Vec<building::Building>,
    pub unknown: Vec<u8>,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            patch: 0,
            cursor_offset_x: 0,
            cursor_offset_y: 0,
            cursor_target_area: 0,
            drag_box_size_x: 1,
            drag_box_size_y: 1,
            primary_area_idx: 0,
            areas_length: 1, // 默认一个区域
            areas: vec![Area::default()],
            buildings_length: 0,
            buildings: Vec::new(),
            unknown: Vec::new(),
        }
    }
}
