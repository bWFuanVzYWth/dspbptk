use nalgebra::Vector3;

use crate::dspbptk_building::{DspbptkBuildingData, uuid::new_uuid};

impl DspbptkBuildingData {
    // TODO 重命名
    #[must_use]
    pub fn clone_offset(&self, offset: Vector3<f64>) -> Self {
        Self {
            uuid: new_uuid(),
            local_offset: self.local_offset + offset,
            ..self.clone()
        }
    }
}
