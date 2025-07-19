use nalgebra::Vector3;

#[derive(Debug, Clone)]
pub struct Building {
    pub uuid: Option<u128>,
    pub area_index: i8,
    pub local_offset: Vector3<f64>,
    pub yaw: f64,
    pub tilt: f64,
    pub pitch: f64,
    pub local_offset_2: Vector3<f64>,
    pub yaw2: f64,
    pub tilt2: f64,
    pub pitch2: f64,
    pub item_id: i16,
    pub model_index: i16,
    pub temp_output_obj_idx: Option<u128>,
    pub temp_input_obj_idx: Option<u128>,
    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,
    pub output_offset: i8,
    pub input_offset: i8,
    pub recipe_id: i16,
    pub filter_id: i16,
    pub parameters: Vec<i32>,
}

impl Default for Building {
    fn default() -> Self {
        Self {
            uuid: None,
            area_index: 0,
            local_offset: Vector3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            tilt: 0.0,
            pitch: 0.0,
            local_offset_2: Vector3::new(0.0, 0.0, 0.0),
            yaw2: 0.0,
            tilt2: 0.0,
            pitch2: 0.0,
            item_id: 0,
            model_index: 0,
            temp_output_obj_idx: None,
            temp_input_obj_idx: None,
            output_to_slot: 0,
            input_from_slot: 0,
            output_from_slot: 0,
            input_to_slot: 0,
            output_offset: 0,
            input_offset: 0,
            recipe_id: 0,
            filter_id: 0,
            parameters: Vec::new(),
        }
    }
}
