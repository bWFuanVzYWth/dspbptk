mod building_0;
mod building_neg100;
mod building_neg101;

use nom::{IResult, Parser, branch::alt};

use building_0::{deserialization_version_0, serialization_version_0};
use building_neg100::{deserialization_version_neg100, serialization_version_neg100};
use building_neg101::{deserialization_version_neg101, serialization_version_neg101};

use crate::blueprint::data::content::building::{BuildingData, BuildingDataVersion};

/// # Errors
/// 可能的原因：
/// * 建筑数据已经损坏，或者编码不受支持
pub fn deserialization(bin: &[u8]) -> IResult<&[u8], BuildingData> {
    let (unknown, data) = alt((
        deserialization_version_neg101,
        deserialization_version_neg100,
        deserialization_version_0,
    ))
    .parse(bin)?;
    Ok((unknown, data))
}

pub fn serialization(bin: &mut Vec<u8>, data: &BuildingData, version: &BuildingDataVersion) {
    match version {
        BuildingDataVersion::ZERO => serialization_version_0(bin, data),
        BuildingDataVersion::NEG100 => serialization_version_neg100(bin, data),
        BuildingDataVersion::NEG101 => serialization_version_neg101(bin, data),
    }
}
