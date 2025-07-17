use crate::dspbptk_building::DspbptkBuildingData;

/// 把vec中的传送带节点连接成一条整体，注意这个函数并不检查建筑是否为传送带
#[must_use]
pub fn connect_belts(
    belts: &[DspbptkBuildingData],
    module_temp_input_obj_idx: Option<u128>,
    module_input_from_slot: i8,
    module_temp_output_obj_idx: Option<u128>,
    module_output_to_slot: i8,
) -> Vec<DspbptkBuildingData> {
    if belts.is_empty() {
        return Vec::new();
    }

    let next_info = belts
        .iter()
        .skip(1)
        .map(|b| (b.uuid, 1))
        .chain(std::iter::once((
            module_temp_output_obj_idx,
            module_output_to_slot,
        )));

    let last_info = std::iter::once((module_temp_input_obj_idx, module_input_from_slot)).chain(
        std::iter::repeat((
            DspbptkBuildingData::default().temp_input_obj_idx,
            DspbptkBuildingData::default().input_from_slot,
        )),
    );

    last_info
        .zip(next_info)
        .zip(belts)
        .map(
            |(
                ((temp_input_obj_idx, input_from_slot), (temp_output_obj_idx, output_to_slot)),
                belt,
            )| {
                DspbptkBuildingData {
                    temp_output_obj_idx,
                    temp_input_obj_idx,
                    output_to_slot,
                    input_from_slot,
                    ..belt.clone()
                }
            },
        )
        .collect::<Vec<_>>()
}
