use uuid::Uuid;

use crate::{blueprint::content::building::INDEX_NULL, error::DspbptkError};

pub fn index_from_uuid<'a>(uuid: Option<u128>) -> Result<i32, DspbptkError<'a>> {
    uuid.map_or(Ok(INDEX_NULL), |num| {
        i32::try_from(num).map_err(DspbptkError::NonStandardUuid)
    })
}

pub fn uuid_from_index<'a>(index: i32) -> Result<Option<u128>, DspbptkError<'a>> {
    if index == INDEX_NULL {
        Ok(None)
    } else {
        Ok(Some(
            u128::try_from(index).map_err(DspbptkError::NonStandardIndex)?,
        ))
    }
}

pub fn new_uuid() -> Option<u128> {
    Some(Uuid::new_v4().to_u128_le())
}
