use nalgebra::Vector3;
use uuid::Uuid;

use crate::{
    blueprint::content::building::DspbptkBuildingData,
    edit::{direction_to_local_offset, unit_conversion::arc_from_grid},
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

pub fn new(direction: Vector3<f64>, upside_down: bool) -> Vec<DspbptkBuildingData> {
    // TODO 从bin/photons把生成代码扣过来
    let y_scale = if upside_down { 1.0 } else { -1.0 };

    let local_offset = direction_to_local_offset(&direction, 0.0);

    // 锅
    let receiver = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::射线接收站 as i16,
        model_index: Item::射线接收站.model()[0],
        local_offset: local_offset,
        parameters: vec![1208],
        ..Default::default()
    };

    let sorter_lens_input = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::分拣器 as i16,
        model_index: Item::分拣器.model()[0],
        yaw: sorter_yaw,
        yaw2: sorter_yaw,
        local_offset: lens_sorter_local_offset,
        local_offset_2: lens_belt_from_sorter.local_offset,
        temp_input_obj_idx: nearest_main_belt_node.uuid,
        temp_output_obj_idx: lens_belt_from_sorter.uuid,
        output_to_slot: -1,
        input_from_slot: -1,
        output_from_slot: 0,
        input_to_slot: 1,
        ..Default::default()
    };

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
}
