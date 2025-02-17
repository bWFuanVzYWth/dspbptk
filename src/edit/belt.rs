use nalgebra::Quaternion;
use uuid::Uuid;

use crate::{
    blueprint::content::building::DspbptkBuildingData,
    edit::{
        compute_3d_rotation_vector, direction_to_local_offset, local_offset_to_direction,
        unit_conversion::arc_from_grid,
    },
    item::Item,
};

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
                temp_input_obj_idx: temp_input_obj_idx,
                input_from_slot: input_from_slot,
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

/// 沿测地线连接两个传送带节点
pub fn create_belts_path(
    from: DspbptkBuildingData, // FIXME 并不能正确处理初始传送带已经有数据的情况
    to: &DspbptkBuildingData,
) -> Vec<DspbptkBuildingData> {
    const BELT_GRID: f64 = 1.83;
    const BELT_ARC: f64 = arc_from_grid(BELT_GRID);

    let from_z = from.local_offset[2];
    let to_z = to.local_offset[2];

    let from_direction = local_offset_to_direction(from.local_offset).normalize();
    let to_direction = local_offset_to_direction(to.local_offset).normalize();

    let cos_theta = from_direction.dot(&to_direction);

    // 如果两传送带在球面上相对，此时存在无数条测地线，应拒绝生成并让用户检查
    assert!(cos_theta > -1.0);

    let axis = from_direction.cross(&to_direction).normalize();

    // TODO 用四元数插值(Slerp)优化性能
    // 用四元数球面插值
    let arc_between = cos_theta.acos();
    let belts_count = (arc_between / BELT_ARC).ceil() as i64; // 加上了from，没算to
    let belts = (0..=belts_count)
        .map(|i| {
            if i == 0 {
                from.clone()
            } else if i == belts_count {
                to.clone()
            } else {
                let k = (i as f64) / (belts_count as f64);
                let half_arc_lerp = arc_between * 0.5 * k;
                let q = Quaternion::new(
                    half_arc_lerp.cos(),
                    axis.x * half_arc_lerp.sin(),
                    axis.y * half_arc_lerp.sin(),
                    axis.z * half_arc_lerp.sin(),
                );
                let inv_q = q.conjugate();
                let belt_direction = compute_3d_rotation_vector(&from_direction, (q, inv_q));
                let belt_z = from_z * (1.0 - k) + to_z * k;
                DspbptkBuildingData {
                    uuid: Some(Uuid::new_v4().to_u128_le()),
                    item_id: Item::极速传送带 as i16,
                    model_index: Item::极速传送带.model()[0],
                    local_offset: direction_to_local_offset(&belt_direction, belt_z),
                    ..Default::default()
                }
            }
        })
        .collect::<Vec<_>>();

    // TODO 根据手性自动判断输入槽位
    connect_belts(belts, None, 0, None, 0)
}
