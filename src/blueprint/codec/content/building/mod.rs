mod version_0;
mod version_neg100;
mod version_neg101;

use nom::{IResult, Parser, branch::alt};

use version_0::{deserialization_version_0, serialization_version_0};
use version_neg100::{deserialization_version_neg100, serialization_version_neg100};
use version_neg101::{deserialization_version_neg101, serialization_version_neg101};

use crate::blueprint::data::content::building::{Building, Version};

/// # Errors
/// 可能的原因：
/// * 建筑数据已经损坏，或者编码不受支持
pub fn deserialization(bin: &[u8]) -> IResult<&[u8], Building> {
    let (unknown, data) = alt((
        deserialization_version_neg101,
        deserialization_version_neg100,
        deserialization_version_0,
    ))
    .parse(bin)?;
    Ok((unknown, data))
}

pub fn serialization(bin: &mut Vec<u8>, data: &Building, version: &Version) {
    match version {
        Version::ZERO => serialization_version_0(bin, data),
        Version::Neg100 => serialization_version_neg100(bin, data),
        Version::Neg101 => serialization_version_neg101(bin, data),
    }
}
