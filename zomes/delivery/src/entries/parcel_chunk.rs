use hdk::prelude::*;

use crate::{
    CHUNK_MAX_SIZE,
};

/// Entry representing a file chunk.
#[hdk_entry(id = "ParcelChunk", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelChunk {
    pub data: String,
}


///
pub(crate) fn validate_ParcelChunk(chunk: ParcelChunk, _maybe_validation_package: Option<ValidationPackage>)
    -> ExternResult<ValidateCallbackResult>
{
    /// Check size
    if chunk.data.len() > CHUNK_MAX_SIZE {
        return Ok(ValidateCallbackResult::Invalid(
            format!("A chunk can't be bigger than {} KiB", CHUNK_MAX_SIZE / 1024)));
    }
    Ok(ValidateCallbackResult::Valid)
}