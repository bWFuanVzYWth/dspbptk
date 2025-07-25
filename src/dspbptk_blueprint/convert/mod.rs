use crate::{
    blueprint,
    dspbptk_blueprint::{
        self,
        uuid::{index_try_from_uuid, uuid_try_from_index},
    },
    error::DspbptkError,
};
use nalgebra::Vector3;

impl TryInto<blueprint::Building> for dspbptk_blueprint::Building {
    type Error = DspbptkError;

    ///  将`DspbptkBuildingData`转换为`BuildingData`
    ///
    /// # Errors
    /// 可能的原因：
    /// * uuid无法转换为index。一般是uuid的数字太大超过了i32的范围，而这又往往是忘记了调用`fix_dspbptk_buildings_index`引起的
    /// * `parameters.len()`太长。如果报这个错说明参数列表真的太长了。原版蓝图不可能出现这个报错。
    #[expect(clippy::cast_possible_truncation)]
    fn try_into(self) -> Result<blueprint::Building, Self::Error> {
        Ok(blueprint::Building {
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

impl TryFrom<blueprint::Building> for dspbptk_blueprint::Building {
    type Error = DspbptkError;

    /// 将当前`Building`对象转换为`DspbptkBuildingData`结构体
    ///
    /// # Errors
    /// 可能的原因：
    /// * index无法转换为uuid，一般是出现了负数index。原版蓝图不可能出现这个报错。
    fn try_from(b: blueprint::Building) -> Result<Self, Self::Error> {
        Ok(Self {
            // 转换主索引为UUID，可能返回NonStandardIndex错误
            uuid: uuid_try_from_index(b.index)?,
            area_index: b.area_index,
            // 转换局部偏移量为f64类型数组
            local_offset: Vector3::new(
                f64::from(b.local_offset_x),
                f64::from(b.local_offset_y),
                f64::from(b.local_offset_z),
            ),
            // 转换方向角为f64类型
            yaw: f64::from(b.yaw),
            tilt: f64::from(b.tilt),
            pitch: f64::from(b.pitch),
            // 转换第二组局部偏移量为f64类型数组
            local_offset_2: Vector3::new(
                f64::from(b.local_offset_x2),
                f64::from(b.local_offset_y2),
                f64::from(b.local_offset_z2),
            ),
            // 转换第二组方向角为f64类型
            yaw2: f64::from(b.yaw2),
            tilt2: f64::from(b.tilt2),
            pitch2: f64::from(b.pitch2),
            item_id: b.item_id,
            model_index: b.model_index,
            // 转换输出/输入对象索引为UUID，可能返回NonStandardIndex错误
            temp_output_obj_idx: uuid_try_from_index(b.temp_output_obj_idx)?,
            temp_input_obj_idx: uuid_try_from_index(b.temp_input_obj_idx)?,
            output_to_slot: b.output_to_slot,
            input_from_slot: b.input_from_slot,
            output_from_slot: b.output_from_slot,
            input_to_slot: b.input_to_slot,
            output_offset: b.output_offset,
            input_offset: b.input_offset,
            recipe_id: b.recipe_id,
            filter_id: b.filter_id,
            parameters: b.parameters,
        })
    }
}
