mod building_0;
mod building_neg100;
mod building_neg101;

use nom::{IResult, branch::alt};

use building_0::deserialization_version_0;
use building_neg100::deserialization_version_neg100;
use building_neg101::{deserialization_version_neg101, serialization_version_neg101};

use crate::error::DspbptkError::{
    self, NonStandardIndex, NonStandardUuid, UnexpectParametersLength,
};

pub const INDEX_NULL: i32 = -1;

#[derive(Debug, Clone)]
pub struct BuildingData {
    pub index: i32,
    pub area_index: i8,
    pub local_offset_x: f32,
    pub local_offset_y: f32,
    pub local_offset_z: f32,
    pub yaw: f32,
    pub tilt: f32,
    pub pitch: f32,
    pub local_offset_x2: f32,
    pub local_offset_y2: f32,
    pub local_offset_z2: f32,
    pub yaw2: f32,
    pub tilt2: f32,
    pub pitch2: f32,
    pub item_id: i16,
    pub model_index: i16,
    pub temp_output_obj_idx: i32,
    pub temp_input_obj_idx: i32,
    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,
    pub output_offset: i8,
    pub input_offset: i8,
    pub recipe_id: i16,
    pub filter_id: i16,
    pub parameters_length: u16,
    pub parameters: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct DspbptkBuildingData {
    pub uuid: Option<u128>,
    pub area_index: i8,
    pub local_offset: [f64; 3],
    pub yaw: f64,
    pub tilt: f64,
    pub pitch: f64,
    pub local_offset_2: [f64; 3],
    pub yaw2: f64,
    pub tilt2: f64,
    pub pitch2: f64,
    pub item_id: i16,
    pub model_index: i16,
    pub temp_output_obj_idx: Option<u128>,
    pub temp_input_obj_idx: Option<u128>,
    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,
    pub output_offset: i8,
    pub input_offset: i8,
    pub recipe_id: i16,
    pub filter_id: i16,
    pub parameters: Vec<i32>,
}

fn uuid_from_index<'a>(index: i32) -> Result<Option<u128>, DspbptkError<'a>> {
    if index == INDEX_NULL {
        Ok(None)
    } else {
        Ok(Some(u128::try_from(index).map_err(NonStandardIndex)?))
    }
}

fn index_from_uuid<'a>(uuid: Option<u128>) -> Result<i32, DspbptkError<'a>> {
    uuid.map_or(Ok(INDEX_NULL), |num| {
        i32::try_from(num).map_err(NonStandardUuid)
    })
}

impl BuildingData {
    /// 将当前`Building`对象转换为`DspbptkBuildingData`结构体
    ///
    /// # Errors
    /// 失败时返回包含`DspbptkError`错误的Err结果
    pub fn to_dspbptk_building_data(&self) -> Result<DspbptkBuildingData, DspbptkError> {
        Ok(DspbptkBuildingData {
            // 转换主索引为UUID，可能返回错误（如索引越界）
            uuid: uuid_from_index(self.index)?,
            area_index: self.area_index,
            // 转换局部偏移量为f64类型数组
            local_offset: [
                f64::from(self.local_offset_x),
                f64::from(self.local_offset_y),
                f64::from(self.local_offset_z),
            ],
            // 转换方向角为f64类型
            yaw: f64::from(self.yaw),
            tilt: f64::from(self.tilt),
            pitch: f64::from(self.pitch),
            // 转换第二组局部偏移量为f64类型数组
            local_offset_2: [
                f64::from(self.local_offset_x2),
                f64::from(self.local_offset_y2),
                f64::from(self.local_offset_z2),
            ],
            // 转换第二组方向角为f64类型
            yaw2: f64::from(self.yaw2),
            tilt2: f64::from(self.tilt2),
            pitch2: f64::from(self.pitch2),
            item_id: self.item_id,
            model_index: self.model_index,
            // 转换临时输出/输入对象索引为UUID，可能返回错误
            temp_output_obj_idx: uuid_from_index(self.temp_output_obj_idx)?,
            temp_input_obj_idx: uuid_from_index(self.temp_input_obj_idx)?,
            // 槽位连接信息直接复制
            output_to_slot: self.output_to_slot,
            input_from_slot: self.input_from_slot,
            output_from_slot: self.output_from_slot,
            input_to_slot: self.input_to_slot,
            output_offset: self.output_offset,
            input_offset: self.input_offset,
            recipe_id: self.recipe_id,
            filter_id: self.filter_id,
            // 克隆参数集合以保证数据独立性
            parameters: self.parameters.clone(),
        })
    }
}

impl DspbptkBuildingData {
    #[allow(clippy::cast_possible_truncation)]
    ///  将`DspbptkBuildingData`转换为`BuildingData`
    ///
    /// # Errors
    /// 失败时返回包含`DspbptkError`错误的Err结果
    pub fn to_building_data(&self) -> Result<BuildingData, DspbptkError> {
        Ok(BuildingData {
            index: index_from_uuid(self.uuid)?,
            area_index: self.area_index,
            local_offset_x: self.local_offset[0] as f32,
            local_offset_y: self.local_offset[1] as f32,
            local_offset_z: self.local_offset[2] as f32,
            yaw: self.yaw as f32,
            tilt: self.tilt as f32,
            pitch: self.pitch as f32,
            local_offset_x2: self.local_offset_2[0] as f32,
            local_offset_y2: self.local_offset_2[1] as f32,
            local_offset_z2: self.local_offset_2[2] as f32,
            yaw2: self.yaw2 as f32,
            tilt2: self.tilt2 as f32,
            pitch2: self.pitch2 as f32,
            item_id: self.item_id,
            model_index: self.model_index,
            temp_output_obj_idx: index_from_uuid(self.temp_output_obj_idx)?,
            temp_input_obj_idx: index_from_uuid(self.temp_input_obj_idx)?,
            output_to_slot: self.output_to_slot,
            input_from_slot: self.input_from_slot,
            output_from_slot: self.output_from_slot,
            input_to_slot: self.input_to_slot,
            output_offset: self.output_offset,
            input_offset: self.input_offset,
            recipe_id: self.recipe_id,
            filter_id: self.filter_id,
            parameters_length: u16::try_from(self.parameters.len())
                .map_err(UnexpectParametersLength)?,
            parameters: self.parameters.clone(),
        })
    }
}

impl Default for BuildingData {
    fn default() -> Self {
        Self {
            index: INDEX_NULL,
            area_index: 0,
            local_offset_x: 0.0,
            local_offset_y: 0.0,
            local_offset_z: 0.0,
            yaw: 0.0,
            tilt: 0.0,
            pitch: 0.0,
            local_offset_x2: 0.0,
            local_offset_y2: 0.0,
            local_offset_z2: 0.0,
            yaw2: 0.0,
            tilt2: 0.0,
            pitch2: 0.0,
            item_id: 0,
            model_index: 0,
            temp_output_obj_idx: INDEX_NULL,
            temp_input_obj_idx: INDEX_NULL,
            output_to_slot: 0,
            input_from_slot: 0,
            output_from_slot: 0,
            input_to_slot: 0,
            output_offset: 0,
            input_offset: 0,
            recipe_id: 0,
            filter_id: 0,
            parameters_length: 0,
            parameters: Vec::new(),
        }
    }
}

impl Default for DspbptkBuildingData {
    fn default() -> Self {
        Self {
            uuid: None,
            area_index: 0,
            local_offset: [0.0, 0.0, 0.0],
            yaw: 0.0,
            tilt: 0.0,
            pitch: 0.0,
            local_offset_2: [0.0, 0.0, 0.0],
            yaw2: 0.0,
            tilt2: 0.0,
            pitch2: 0.0,
            item_id: 0,
            model_index: 0,
            temp_output_obj_idx: None,
            temp_input_obj_idx: None,
            output_to_slot: 0,
            input_from_slot: 0,
            output_from_slot: 0,
            input_to_slot: 0,
            output_offset: 0,
            input_offset: 0,
            recipe_id: 0,
            filter_id: 0,
            parameters: Vec::new(),
        }
    }
}

/// 依次尝试匹配不同版本的格式
///
/// # Errors
/// 失败时返回`nom::IResult`
pub fn deserialization(bin: &[u8]) -> IResult<&[u8], BuildingData> {
    let (unknown, data) = alt((
        deserialization_version_neg101,
        deserialization_version_neg100,
        deserialization_version_0,
    ))(bin)?;
    Ok((unknown, data))
}

pub fn serialization(bin: &mut Vec<u8>, data: &BuildingData) {
    serialization_version_neg101(bin, data);
}

#[cfg(test)]
mod test {
    // TODO test 检查每一种不同建筑
}
