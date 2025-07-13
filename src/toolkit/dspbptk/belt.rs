use crate::dspbptk_building::DspbptkBuildingData;

/// 把vec中的传送带节点连接成一条整体，注意这个函数并不检查建筑是否为传送带
#[must_use]
pub fn connect_belts(
    belts: &[DspbptkBuildingData],
    temp_input_obj_idx: Option<u128>,
    input_from_slot: i8,
    temp_output_obj_idx: Option<u128>,
    output_to_slot: i8,
) -> Vec<DspbptkBuildingData> {
    if belts.is_empty() {
        return vec![];
    }

    let next_info = belts
        .iter()
        .skip(1)
        .map(|b| (b.uuid, 1))
        .chain(std::iter::once((temp_output_obj_idx, output_to_slot)));

    let last_info =
        std::iter::once((temp_input_obj_idx, input_from_slot)).chain(std::iter::repeat((
            DspbptkBuildingData::default().temp_input_obj_idx,
            DspbptkBuildingData::default().input_from_slot,
        )));

    next_info
        .zip(last_info)
        .map(
            |((temp_output_obj_idx, output_to_slot), (temp_input_obj_idx, input_from_slot))| {
                DspbptkBuildingData {
                    temp_input_obj_idx,
                    input_from_slot,
                    temp_output_obj_idx,
                    output_to_slot,
                    ..DspbptkBuildingData::default()
                }
            },
        )
        .collect::<Vec<_>>()
}
