use crate::blueprint::data::content::building::BuildingData;

impl BuildingData {
    pub fn round_float(&mut self) {
        const ROUND_SCALE_POSITION: f32 = 300.0;
        const ROUND_SCALE_ANGLE: f32 = 20.0;
        let round = |x: f32, scale: f32| (x * scale).round() / scale;
        let round_local_offset = |x: f32| round(x, ROUND_SCALE_POSITION);
        let round_angle = |x: f32| round(x, ROUND_SCALE_ANGLE);

        self.local_offset_x = round_local_offset(self.local_offset_x);
        self.local_offset_y = round_local_offset(self.local_offset_y);
        self.local_offset_z = round_local_offset(self.local_offset_z);
        self.yaw = round_angle(self.yaw);
        self.tilt = round_angle(self.tilt);
        self.pitch = round_angle(self.pitch);
        self.local_offset_x2 = round_local_offset(self.local_offset_x2);
        self.local_offset_y2 = round_local_offset(self.local_offset_y2);
        self.local_offset_z2 = round_local_offset(self.local_offset_z2);
        self.yaw2 = round_angle(self.yaw2);
        self.tilt2 = round_angle(self.tilt2);
        self.pitch2 = round_angle(self.pitch2);
    }
}
