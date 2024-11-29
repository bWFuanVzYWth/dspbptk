pub struct BlueprintBuilding {
    version: i32,
    index: i32,
    area_index: i8,
    local_offset_x: f32,
    local_offset_y: f32,
    local_offset_z: f32,
    local_offset_x2: f32,
    local_offset_y2: f32,
    local_offset_z2: f32,
    yaw: f32,
    yaw2: f32,
    tilt: f32,
    item_id: i16,
    model_index: i16,
    temp_output_obj_idx: i32,
    temp_input_obj_idx: i32,
    output_to_slot: i8,
    input_from_slot: i8,
    output_from_slot: i8,
    input_to_slot: i8,
    output_offset: i8,
    input_offset: i8,
    recipe_id: i16,
    filter_id: i16,
    parameters_length: i16,
    parameters: Vec<u32>,
}

// pub fn parse(memory_stream: &[u8]) -> IResult<&[u8], BlueprintBuilding> {

// }