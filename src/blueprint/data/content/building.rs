use num_enum::IntoPrimitive;
use strum_macros::EnumIter;

#[derive(IntoPrimitive, EnumIter)]
#[repr(i32)]
pub enum Version {
    Zero = 0,
    Neg100 = -100,
    Neg101 = -101,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Building {
    pub index: i32,
    pub area_index: i8,
    pub local_offset_x: f32,
    pub local_offset_y: f32,
    pub local_offset_z: f32,
    pub yaw: f32,
    pub tilt: f32,
    pub pitch: f32,
    pub local_offset_x2: f32,
    pub local_offset_y2: f32,
    pub local_offset_z2: f32,
    pub yaw2: f32,
    pub tilt2: f32,
    pub pitch2: f32,
    pub item_id: i16,
    pub model_index: i16,
    pub temp_output_obj_idx: i32,
    pub temp_input_obj_idx: i32,
    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,
    pub output_offset: i8,
    pub input_offset: i8,
    pub recipe_id: i16,
    pub filter_id: i16,
    pub parameters_length: u16,
    pub parameters: Vec<i32>,
}

impl Building {
    pub const INDEX_NULL: i32 = -1;
}

impl Default for Building {
    fn default() -> Self {
        Self {
            index: Self::INDEX_NULL,
            area_index: 0,
            local_offset_x: 0.0,
            local_offset_y: 0.0,
            local_offset_z: 0.0,
            yaw: 0.0,
            tilt: 0.0,
            pitch: 0.0,
            local_offset_x2: 0.0,
            local_offset_y2: 0.0,
            local_offset_z2: 0.0,
            yaw2: 0.0,
            tilt2: 0.0,
            pitch2: 0.0,
            item_id: 0,
            model_index: 0,
            temp_output_obj_idx: Self::INDEX_NULL,
            temp_input_obj_idx: Self::INDEX_NULL,
            output_to_slot: 0,
            input_from_slot: 0,
            output_from_slot: 0,
            input_to_slot: 0,
            output_offset: 0,
            input_offset: 0,
            recipe_id: 0,
            filter_id: 0,
            parameters_length: 0,
            parameters: Vec::new(),
        }
    }
}
