use crate::blueprint::content::building::DspbptkBuildingData;

/// 把vec中的传送带节点连接成一条整体，注意这个函数并不检查建筑是否为传送带  
pub fn connect_belts(
    belts: Vec<DspbptkBuildingData>,
    temp_input_obj_idx: Option<u128>,
    input_from_slot: i8,
    temp_output_obj_idx: Option<u128>,
    output_to_slot: i8,
) -> Vec<DspbptkBuildingData> {
    let nexts = belts
        .iter()
        .enumerate()
        .map(|(i, _belt)| match belts.get(i + 1) {
            Some(belt) => (belt.uuid, 1),
            None => (temp_output_obj_idx, output_to_slot),
        })
        .collect::<Vec<_>>();

    belts
        .into_iter()
        .enumerate()
        .map(|(i, belt)| match i {
            0 => DspbptkBuildingData {
                temp_input_obj_idx,
                input_from_slot,
                input_to_slot: 1,
                temp_output_obj_idx: nexts[i].0,
                output_to_slot: nexts[i].1,
                ..belt
            },
            _ => DspbptkBuildingData {
                temp_output_obj_idx: nexts[i].0,
                output_to_slot: nexts[i].1,
                ..belt
            },
        })
        .collect()
}
