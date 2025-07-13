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
    if belts.is_empty() {
        return vec![];
    }

    let first = belts.first().cloned();

    let next_info = belts
        .iter()
        .skip(1)
        .map(|b| (b.uuid, 1))
        .chain(std::iter::once((temp_output_obj_idx, output_to_slot)));

    let last_info = std::iter::once((temp_input_obj_idx, input_from_slot)).chain(
        std::iter::repeat((DspbptkBuildingData::default().temp_input_obj_idx, DspbptkBuildingData::default().input_from_slot))
    );

    belts
        .into_iter()
        .zip(next_info)
        .zip(last_info)
        .map(|belt|)
    // belts
    //     .into_iter()
    //     .zip(next_info)
    //     .map(|(belt, (next_output_obj, next_output_slot))| {
    //         match first.as_ref() {
    //             // 首元素：注入输入参数
    //             Some(first_belt) if belt.uuid == first_belt.uuid => DspbptkBuildingData {
    //                 temp_input_obj_idx,
    //                 input_from_slot,
    //                 input_to_slot: 1,
    //                 temp_output_obj_idx: next_output_obj,
    //                 output_to_slot: next_output_slot,
    //                 ..belt
    //             },
    //             // 其他元素：仅更新输出配置
    //             _ => DspbptkBuildingData {
    //                 temp_output_obj_idx: next_output_obj,
    //                 output_to_slot: next_output_slot,
    //                 ..belt
    //             },
    //         }
    //     })
    //     .collect()
}
