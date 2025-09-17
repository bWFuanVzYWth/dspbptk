mod v0;
mod v100;
mod v101;

use crate::blueprint::{
    Building,
    Version::{self, Neg100, Neg101, Zero},
};
use nom::{IResult, Parser, branch::alt};

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
        Zero => v0::serialization(bin, data),
        Neg100 => v100::serialization(bin, data),
        Neg101 => v101::serialization(bin, data),
    }
}
