use nalgebra::Vector3;
use uuid::Uuid;

use crate::{
    blueprint::content::building::DspbptkBuildingData,
    edit::{belt::connect_belts, direction_to_local_offset, unit_conversion::arc_from_grid},
    item::Item,
};

// 锅的尺寸数据由src/bin/test_ray_receiver_size测出
const GRID_A: f64 = 7.30726;
const GRID_B: f64 = 4.19828;
const HALF_GRID_A: f64 = GRID_A / 2.0;
const HALF_GRID_B: f64 = GRID_B / 2.0;

const ARC_A: f64 = arc_from_grid(GRID_A);
const ARC_B: f64 = arc_from_grid(GRID_B);
const HALF_ARC_A: f64 = arc_from_grid(HALF_GRID_A);
const HALF_ARC_B: f64 = arc_from_grid(HALF_GRID_B);

pub fn new(
    direction: Vector3<f64>,
    upside_down: bool,
    temp_input_obj_idx: Option<u128>,
    input_from_slot: i8,
    temp_output_obj_idx: Option<u128>,
    output_to_slot: i8,
) -> Vec<DspbptkBuildingData> {
    let (y_scale, sorter_yaw) = if upside_down {
        (-1.0, 180.0)
    } else {
        (1.0, 0.0)
    };

    let local_offset = direction_to_local_offset(&direction, 0.0);

    // 光子锅
    let receiver = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::射线接收站 as i16,
        model_index: Item::射线接收站.model()[0],
        local_offset: local_offset,
        parameters: vec![1208],
        ..Default::default()
    };

    // 透镜带
    let belt_lens_from_sorter = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::极速传送带 as i16,
        model_index: Item::极速传送带.model()[0],
        local_offset: [
            receiver.local_offset[0],
            receiver.local_offset[1] + HALF_GRID_A * (2.0 / 3.0),
            receiver.local_offset[2],
        ],
        ..Default::default()
    };

    let belt_lens_into_receiver = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::极速传送带 as i16,
        model_index: Item::极速传送带.model()[0],
        local_offset: [
            receiver.local_offset[0],
            receiver.local_offset[1] + HALF_GRID_A * (1.0 / 3.0),
            receiver.local_offset[2],
        ],
        ..Default::default()
    };

    // 分流透镜的黄爪
    let sorter_lens_input = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::分拣器 as i16,
        model_index: Item::分拣器.model()[0],
        yaw: sorter_yaw,
        yaw2: sorter_yaw,
        local_offset: [
            receiver.local_offset[0],
            receiver.local_offset[1] + y_scale * (HALF_GRID_A - 0.25),
            receiver.local_offset[2],
        ],
        local_offset_2: belt_lens_from_sorter.local_offset,
        temp_input_obj_idx: temp_input_obj_idx,
        temp_output_obj_idx: belt_lens_from_sorter.uuid,
        output_to_slot: -1,
        input_from_slot: input_from_slot,
        input_to_slot: 1,
        ..Default::default()
    };

    let belts_lens = vec![belt_lens_from_sorter, belt_lens_into_receiver];
    let belts_lens = connect_belts(belts_lens, None, 0, receiver.uuid, 0);

    // 光子带
    let belt_photons_from_receiver = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::极速传送带 as i16,
        model_index: Item::极速传送带.model()[0],
        local_offset: [
            receiver.local_offset[0],
            receiver.local_offset[1] - HALF_GRID_A * (1.0 / 3.0),
            receiver.local_offset[2],
        ],
        ..Default::default()
    };

    let belt_photons_output = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::极速传送带 as i16,
        model_index: Item::极速传送带.model()[0],
        local_offset: [
            receiver.local_offset[0],
            receiver.local_offset[1] - HALF_GRID_A * (2.0 / 3.0),
            receiver.local_offset[2],
        ],
        ..Default::default()
    };

    let belts_photons = vec![belt_photons_from_receiver, belt_photons_output];
    let belts_photons = connect_belts(
        belts_photons,
        receiver.uuid,
        1,
        temp_output_obj_idx,
        output_to_slot,
    );

    let other_buildings = vec![receiver, sorter_lens_input];

    vec![other_buildings, belts_lens, belts_photons].concat()
}
