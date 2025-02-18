use uuid::Uuid;

use crate::{blueprint::content::building::DspbptkBuildingData, item::Item};

fn new_receiver(local_offset: [f64; 3]) -> DspbptkBuildingData {
    DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::射线接收站 as i16,
        model_index: Item::射线接收站.model()[0],
        local_offset: local_offset,
        parameters: vec![1208],
        ..Default::default()
    }
}

pub fn new(direction: [f64; 3]) -> Vec<DspbptkBuildingData> {
    // TODO 从bin/photons把生成代码扣过来
    todo!()
}
