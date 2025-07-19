
pub const INDEX_NULL: i8 = -1;

#[derive(Debug, Clone)]
pub struct AreaData {
    pub index: i8,
    pub parent_index: i8,
    pub tropic_anchor: i16,
    pub area_segments: i16,
    pub anchor_local_offset_x: i16,
    pub anchor_local_offset_y: i16,
    pub width: i16,
    pub height: i16,
}

impl Default for AreaData {
    fn default() -> Self {
        Self {
            index: 0,
            parent_index: INDEX_NULL,
            tropic_anchor: 0,
            area_segments: 200, // Magic Number
            anchor_local_offset_x: 0,
            anchor_local_offset_y: 0,
            width: 0,
            height: 0,
        }
    }
}
