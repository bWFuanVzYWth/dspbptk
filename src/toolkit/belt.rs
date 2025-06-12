use std::collections::VecDeque;

use crate::dspbptk_building::DspbptkBuildingData;

/// 把vec中的传送带节点连接成一条整体，注意这个函数并不检查建筑是否为传送带
#[must_use]
pub fn connect_belts(
    belts: Vec<DspbptkBuildingData>,
    temp_input_obj_idx: Option<u128>,
    input_from_slot: i8,
    temp_output_obj_idx: Option<u128>,
    output_to_slot: i8,
) -> Vec<DspbptkBuildingData> {
    let mut queue = VecDeque::from(belts);
    let mut result = Vec::with_capacity(queue.len()); // 预分配容量

    // 处理第一个节点
    if let Some(belt) = queue.pop_front() {
        let (next_output_obj, next_output_slot) =
            compute_next(temp_output_obj_idx, output_to_slot, &queue);
        let first = DspbptkBuildingData {
            temp_input_obj_idx,
            input_from_slot,
            input_to_slot: 1,
            temp_output_obj_idx: next_output_obj,
            output_to_slot: next_output_slot,
            ..belt
        };
        result.push(first);
    }

    // 处理其余节点
    while let Some(belt) = queue.pop_front() {
        let (next_output_obj, next_output_slot) =
            compute_next(temp_output_obj_idx, output_to_slot, &queue);
        let modified = DspbptkBuildingData {
            temp_output_obj_idx: next_output_obj,
            output_to_slot: next_output_slot,
            ..belt
        };
        result.push(modified);
    }

    result
}

fn compute_next(
    temp_output_obj_idx: Option<u128>,
    output_to_slot: i8,
    queue: &VecDeque<DspbptkBuildingData>,
) -> (Option<u128>, i8) {
    queue
        .front()
        .map_or((temp_output_obj_idx, output_to_slot), |next_belt| {
            (next_belt.uuid, 1)
        })
}
