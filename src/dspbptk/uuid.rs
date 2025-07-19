use uuid::Uuid;

use crate::{blueprint::data::content::building::INDEX_NULL, error::DspbptkError};

/// # Errors
/// 可能的原因：
/// * 尝试把uuid转换到index时，超出定义域
pub fn index_try_from_uuid(uuid: Option<u128>) -> Result<i32, DspbptkError> {
    uuid.map_or(Ok(INDEX_NULL), |num| {
        i32::try_from(num).map_err(DspbptkError::TryFromUuidError)
    })
}

/// # Errors
/// 可能的原因：
/// * 尝试把index转换到uuid时，超出定义域
pub fn uuid_try_from_index(index: i32) -> Result<Option<u128>, DspbptkError> {
    if index == INDEX_NULL {
        Ok(None)
    } else {
        Ok(Some(
            u128::try_from(index).map_err(DspbptkError::TryFromIndexError)?,
        ))
    }
}

#[must_use]
pub fn new_uuid() -> u128 {
    Uuid::new_v4().to_u128_le()
}

#[must_use]
pub fn some_new_uuid() -> Option<u128> {
    Some(Uuid::new_v4().to_u128_le())
}
