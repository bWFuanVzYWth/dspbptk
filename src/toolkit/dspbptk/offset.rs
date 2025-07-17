use nalgebra::Vector3;

use crate::dspbptk_building::{DspbptkBuildingData, uuid::new_uuid};

impl DspbptkBuildingData {
    #[must_use]
    pub fn clone_offset(&self, offset: &Vector3<f64>, index_offset: u128) -> Self {
        Self {
            uuid: self.uuid.map(|uuid| uuid.wrapping_add(index_offset)),
            local_offset: self.local_offset + offset,
            local_offset_2: self.local_offset_2 + offset,
            temp_output_obj_idx: self
                .temp_output_obj_idx
                .map(|uuid| uuid.wrapping_add(index_offset)),
            temp_input_obj_idx: self
                .temp_input_obj_idx
                .map(|uuid| uuid.wrapping_add(index_offset)),
            ..self.clone()
        }
    }
}

#[must_use]
pub fn offset(
    module: &[DspbptkBuildingData],
    basis_vector: &Vector3<f64>,
) -> Vec<DspbptkBuildingData> {
    let index_offset = new_uuid();
    module
        .iter()
        .map(move |building| building.clone_offset(basis_vector, index_offset))
        .collect()
}

/// 生成线性阵列的建筑模块实例
///
/// # 参数
/// * `module` - 基础建筑模块的数据数组，每个元素代表一个模块
/// * `basis_vector` - 线性排列的基向量，决定排列方向和单步长度
/// * `count` - 需要生成的实例数量
///
/// # 返回值
/// 包含所有偏移后建筑模块的向量，每个模块按线性模式排列。
#[must_use]
pub fn linear_pattern(
    module: &[DspbptkBuildingData],
    basis_vector: &Vector3<f64>,
    count: u32,
) -> Vec<DspbptkBuildingData> {
    (0..count)
        .flat_map(|i| {
            let offset = f64::from(i) * basis_vector;
            let index_offset = new_uuid();
            module
                .iter()
                .map(move |building| building.clone_offset(&offset, index_offset))
        })
        .collect()
}
