mod v0;
mod v100;
mod v101;

use nom::{IResult, Parser, branch::alt};

use crate::blueprint::{Building, Version};

/// # Errors
/// 可能的原因：
/// * 建筑数据已经损坏，或者编码不受支持
pub fn deserialization(bin: &[u8]) -> IResult<&[u8], Building> {
    let (unknown, data) = alt((
        v101::deserialization,
        v100::deserialization,
        v0::deserialization,
    ))
    .parse(bin)?;
    Ok((unknown, data))
}

pub fn serialization(bin: &mut Vec<u8>, data: &Building, version: &Version) {
    match version {
        Version::Zero => v0::serialization(bin, data),
        Version::Neg100 => v100::serialization(bin, data),
        Version::Neg101 => v101::serialization(bin, data),
    }
}
