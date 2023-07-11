use hdi::prelude::*;
use zome_delivery_types::*;
use crate::*;



/// Call trait ZomeEntry::validate()
pub(crate) fn validate_app_entry(_creation_action: EntryCreationAction, entry_index: EntryDefIndex, entry: Entry)
    -> ExternResult<ValidateCallbackResult>
{
    let variant = entry_index_to_variant(entry_index)?;
    return match variant {
        DeliveryEntryTypes::Distribution => validate_Distribution(entry),
        DeliveryEntryTypes::ParcelChunk => validate_ParcelChunk(entry),
        DeliveryEntryTypes::ParcelManifest => validate_ParcelManifest(entry),
        _ => Ok(ValidateCallbackResult::Valid),
    }
}






///
fn validate_Distribution(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    let distribution = Distribution::try_from(entry)?;
    if distribution.recipients.is_empty() {
        return Ok(ValidateCallbackResult::Invalid("Need at least one recipient".to_string()));
    }
    /// FIXME: validate parcel ; make sure Parcel entry has been committed
    //validate_parcel(input.parcel_description)?;
    Ok(ValidateCallbackResult::Valid)
}



///
fn validate_ParcelChunk(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    let parcel_chunk = ParcelChunk::try_from(entry)?;
    /// Check size
    if parcel_chunk.data.len() > CHUNK_MAX_SIZE {
        return Ok(ValidateCallbackResult::Invalid(
            format!("A chunk can't be bigger than {} KiB", CHUNK_MAX_SIZE / 1024)
        ));
    }
    /// Done
    Ok(ValidateCallbackResult::Valid)
}


///
fn validate_ParcelManifest(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    let parcel_manifest = ParcelManifest::try_from(entry)?;

    /// Must have chunks
    if parcel_manifest.chunks.is_empty() {
        return Ok(ValidateCallbackResult::Invalid("Missing chunks".to_string()));
    }
    /// Must not exceed size limit
    if parcel_manifest.size > PARCEL_MAX_SIZE {
        return Ok(ValidateCallbackResult::Invalid(format!("Parcel is too big: {} > {}", parcel_manifest.size, PARCEL_MAX_SIZE)));
    }
    /// Must meet minimum name length
    if parcel_manifest.name.len() < NAME_MIN_LENGTH {
        return Ok(ValidateCallbackResult::Invalid(format!("Name is too small: {} > {}", parcel_manifest.name, NAME_MIN_LENGTH)));
    }
    /// FIXME: Check each entry exists and is a ParcelChunk
    /// FIXME: Also check total size
    /// Done
    Ok(ValidateCallbackResult::Valid)
}