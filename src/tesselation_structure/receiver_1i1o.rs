use uuid::Uuid;

use crate::{dspbptk_building::DspbptkBuildingData, item::Item, toolkit::belt::connect_belts};

// 模块尺寸即锅的尺寸，数据由src/bin/test_ray_receiver_size测出
pub const GRID_A: f64 = 7.30726;
pub const GRID_B: f64 = 4.19828;

const RECEIVER_MODEL: i16 = Item::射线接收站.model()[0];
const BELT_MODEL: i16 = Item::极速传送带.model()[0];
const SORTER_MODEL: i16 = Item::分拣器.model()[0];

#[must_use]
pub fn new(
    local_offset: [f64; 3],
    input_obj: &DspbptkBuildingData,
    input_from_slot: i8,
    output_obj: &DspbptkBuildingData,
    output_to_slot: i8,
) -> Vec<DspbptkBuildingData> {
    let (y_scale, sorter_yaw) = if input_obj.local_offset[1] > output_obj.local_offset[1] {
        (1.0_f64, 180.0_f64)
    } else {
        (-1.0_f64, 0.0_f64)
    };

    // 光子锅
    let receiver = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::射线接收站 as i16,
        model_index: RECEIVER_MODEL,
        local_offset,
        parameters: vec![1208],
        ..Default::default()
    };

    // 透镜带
    let belt_lens_from_sorter = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::极速传送带 as i16,
        model_index: BELT_MODEL,
        local_offset: [
            receiver.local_offset[0],
            // receiver.local_offset[1] + y_scale * ((GRID_A / 2.0) * (2.0 / 3.0)),
            y_scale.mul_add((GRID_A / 2.0) * (2.0 / 3.0), receiver.local_offset[1]),
            receiver.local_offset[2],
        ],
        ..Default::default()
    };

    let belt_lens_into_receiver = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::极速传送带 as i16,
        model_index: BELT_MODEL,
        local_offset: [
            receiver.local_offset[0],
            // receiver.local_offset[1] + y_scale * ((GRID_A / 2.0) * (1.0 / 3.0)),
            y_scale.mul_add((GRID_A / 2.0) * (1.0 / 3.0), receiver.local_offset[1]),
            receiver.local_offset[2],
        ],
        ..Default::default()
    };

    // 分流透镜的黄爪
    let sorter_lens_input = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::分拣器 as i16,
        model_index: SORTER_MODEL,
        yaw: sorter_yaw,
        yaw2: sorter_yaw,
        local_offset: [
            receiver.local_offset[0],
            // receiver.local_offset[1] + y_scale * ((GRID_A / 2.0) - 0.25),
            y_scale.mul_add((GRID_A / 2.0) - 0.25, receiver.local_offset[1]),
            receiver.local_offset[2],
        ],
        local_offset_2: belt_lens_from_sorter.local_offset,
        temp_input_obj_idx: input_obj.uuid,
        temp_output_obj_idx: belt_lens_from_sorter.uuid,
        output_to_slot: -1,
        input_from_slot,
        input_to_slot: 1,
        ..Default::default()
    };

    let belts_lens = vec![belt_lens_from_sorter, belt_lens_into_receiver];
    let belts_lens = connect_belts(belts_lens, None, 0, receiver.uuid, 0);

    // 光子带
    let belt_photons_from_receiver = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::极速传送带 as i16,
        model_index: BELT_MODEL,
        local_offset: [
            receiver.local_offset[0],
            // receiver.local_offset[1] - y_scale * ((GRID_A / 2.0) * (1.0 / 3.0)),
            (-y_scale).mul_add((GRID_A / 2.0) * (1.0 / 3.0), receiver.local_offset[1]),
            receiver.local_offset[2],
        ],
        ..Default::default()
    };

    let belt_photons_output = DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::极速传送带 as i16,
        model_index: BELT_MODEL,
        local_offset: [
            receiver.local_offset[0],
            // receiver.local_offset[1] - y_scale * ((GRID_A / 2.0) * (2.0 / 3.0)),
            (-y_scale).mul_add((GRID_A / 2.0) * (2.0 / 3.0), receiver.local_offset[1]),
            receiver.local_offset[2],
        ],
        ..Default::default()
    };

    let belts_photons = vec![belt_photons_from_receiver, belt_photons_output];
    let belts_photons = connect_belts(
        belts_photons,
        receiver.uuid,
        1,
        output_obj.uuid,
        output_to_slot,
    );

    let other_buildings = vec![receiver, sorter_lens_input];

    [other_buildings, belts_lens, belts_photons].concat()
}
