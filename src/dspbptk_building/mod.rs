use nalgebra::Vector3;

use crate::{
    blueprint::data::content::building::Building,
    dspbptk_building::uuid::{index_try_from_uuid, uuid_try_from_index},
    error::DspbptkError,
};

pub mod uuid;

#[derive(Debug, Clone)]
pub struct DspbptkBuildingData {
    pub uuid: Option<u128>,
    pub area_index: i8,
    pub local_offset: Vector3<f64>,
    pub yaw: f64,
    pub tilt: f64,
    pub pitch: f64,
    pub local_offset_2: Vector3<f64>,
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

impl DspbptkBuildingData {
    ///  将`DspbptkBuildingData`转换为`BuildingData`
    ///
    /// # Errors
    /// 可能的原因：
    /// * uuid无法转换为index。一般是uuid的数字太大超过了i32的范围，而这又往往是忘记了调用`fix_dspbptk_buildings_index`引起的
    /// * `parameters.len()`太长。如果报这个错说明参数列表真的太长了。原版蓝图不可能出现这个报错。
    #[expect(clippy::cast_possible_truncation)]
    pub fn as_building_data<'a>(self) -> Result<Building, DspbptkError<'a>> {
        Ok(Building {
            index: index_try_from_uuid(self.uuid)?,
            area_index: self.area_index,
            local_offset_x: self.local_offset.x as f32,
            local_offset_y: self.local_offset.y as f32,
            local_offset_z: self.local_offset.z as f32,
            yaw: self.yaw as f32,
            tilt: self.tilt as f32,
            pitch: self.pitch as f32,
            local_offset_x2: self.local_offset_2.x as f32,
            local_offset_y2: self.local_offset_2.y as f32,
            local_offset_z2: self.local_offset_2.z as f32,
            yaw2: self.yaw2 as f32,
            tilt2: self.tilt2 as f32,
            pitch2: self.pitch2 as f32,
            item_id: self.item_id,
            model_index: self.model_index,
            temp_output_obj_idx: index_try_from_uuid(self.temp_output_obj_idx)?,
            temp_input_obj_idx: index_try_from_uuid(self.temp_input_obj_idx)?,
            output_to_slot: self.output_to_slot,
            input_from_slot: self.input_from_slot,
            output_from_slot: self.output_from_slot,
            input_to_slot: self.input_to_slot,
            output_offset: self.output_offset,
            input_offset: self.input_offset,
            recipe_id: self.recipe_id,
            filter_id: self.filter_id,
            parameters_length: u16::try_from(self.parameters.len())
                .map_err(DspbptkError::UnexpectParametersLength)?,
            parameters: self.parameters,
        })
    }
}

impl Default for DspbptkBuildingData {
    fn default() -> Self {
        Self {
            uuid: None,
            area_index: 0,
            local_offset: Vector3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            tilt: 0.0,
            pitch: 0.0,
            local_offset_2: Vector3::new(0.0, 0.0, 0.0),
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

impl Building {
    /// 将当前`Building`对象转换为`DspbptkBuildingData`结构体
    ///
    /// # Errors
    /// 可能的原因：
    /// * index无法转换为uuid，一般是出现了负数index。原版蓝图不可能出现这个报错。
    pub fn as_dspbptk_building_data<'a>(self) -> Result<DspbptkBuildingData, DspbptkError<'a>> {
        Ok(DspbptkBuildingData {
            // 转换主索引为UUID，可能返回NonStandardIndex错误
            uuid: uuid_try_from_index(self.index)?,
            area_index: self.area_index,
            // 转换局部偏移量为f64类型数组
            local_offset: Vector3::new(
                f64::from(self.local_offset_x),
                f64::from(self.local_offset_y),
                f64::from(self.local_offset_z),
            ),
            // 转换方向角为f64类型
            yaw: f64::from(self.yaw),
            tilt: f64::from(self.tilt),
            pitch: f64::from(self.pitch),
            // 转换第二组局部偏移量为f64类型数组
            local_offset_2: Vector3::new(
                f64::from(self.local_offset_x2),
                f64::from(self.local_offset_y2),
                f64::from(self.local_offset_z2),
            ),
            // 转换第二组方向角为f64类型
            yaw2: f64::from(self.yaw2),
            tilt2: f64::from(self.tilt2),
            pitch2: f64::from(self.pitch2),
            item_id: self.item_id,
            model_index: self.model_index,
            // 转换输出/输入对象索引为UUID，可能返回NonStandardIndex错误
            temp_output_obj_idx: uuid_try_from_index(self.temp_output_obj_idx)?,
            temp_input_obj_idx: uuid_try_from_index(self.temp_input_obj_idx)?,
            output_to_slot: self.output_to_slot,
            input_from_slot: self.input_from_slot,
            output_from_slot: self.output_from_slot,
            input_to_slot: self.input_to_slot,
            output_offset: self.output_offset,
            input_offset: self.input_offset,
            recipe_id: self.recipe_id,
            filter_id: self.filter_id,
            parameters: self.parameters,
        })
    }
}

#[must_use]
pub fn fix_dspbptk_buildings_index(
    buildings: Vec<DspbptkBuildingData>,
) -> Vec<DspbptkBuildingData> {
    use std::collections::HashMap;

    let uuid_lut = buildings
        .iter()
        .enumerate()
        .map(|(uuid, building)| (building.uuid, Some(uuid as u128)))
        .collect::<HashMap<_, _>>();

    buildings
        .into_iter()
        .map(|building| DspbptkBuildingData {
            uuid: *uuid_lut.get(&building.uuid).unwrap_or(&None),
            temp_output_obj_idx: uuid_lut
                .get(&building.temp_output_obj_idx)
                .copied()
                .unwrap_or(None),
            temp_input_obj_idx: uuid_lut
                .get(&building.temp_input_obj_idx)
                .copied()
                .unwrap_or(None),
            ..building
        })
        .collect()
}
