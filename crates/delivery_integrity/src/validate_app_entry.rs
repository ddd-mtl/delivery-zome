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
        DeliveryEntryTypes::PrivateChunk | DeliveryEntryTypes::PublicChunk => validate_ParcelChunk(entry),
        DeliveryEntryTypes::PrivateManifest | DeliveryEntryTypes::PublicManifest => validate_ParcelManifest(entry),
        DeliveryEntryTypes::PublicParcel => validate_PublicParcel(entry),
        _ => Ok(ValidateCallbackResult::Valid),
    }
}


///
fn validate_description(pd: ParcelDescription) -> ExternResult<ValidateCallbackResult> {
    let Ok(dna_properties) = get_properties() else {
        debug!("Failed to get dna properties, skipping ParcelDescription validation");
        return Ok(ValidateCallbackResult::Valid)
    };
    /// Must meet name length requirements
    if pd.name.len() < dna_properties.min_parcel_name_length as usize {
        return Ok(ValidateCallbackResult::Invalid(format!("Parcel name is too small: {} < {}", pd.name.len(), dna_properties.min_parcel_name_length)));
    }
    if pd.name.len() > dna_properties.max_parcel_name_length as usize {
        return Ok(ValidateCallbackResult::Invalid(format!("Parcel name is too big: {} > {}", pd.name.len(), dna_properties.max_parcel_name_length)));
    }
    /// Must meet size requirements
    if pd.size > dna_properties.max_parcel_size {
        return Ok(ValidateCallbackResult::Invalid(format!("Parcel is too big: {} > {}", pd.size, dna_properties.max_parcel_size)));
    }
    /// Must have Size
    if pd.size == 0 {
        return Ok(ValidateCallbackResult::Invalid("Parcel size description is 0".to_string()));
    }
    /// Done
    Ok(ValidateCallbackResult::Valid)
}


///
fn validate_PublicParcel(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    let pr = ParcelReference::try_from(entry)?;
    /// FIXME: validate parcel ; make sure Parcel entry has been committed
    return validate_description(pr.description);
}

///
fn validate_Distribution(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    let distribution = Distribution::try_from(entry)?;
    if distribution.recipients.is_empty() {
        return Ok(ValidateCallbackResult::Invalid("Need at least one recipient".to_string()));
    }
    /// FIXME: validate parcel ; make sure Parcel entry has been committed
    return validate_description(distribution.delivery_summary.parcel_reference.description);
}


///
fn validate_ParcelChunk(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    let parcel_chunk = ParcelChunk::try_from(entry)?;
    /// Check data size
    if parcel_chunk.data.is_empty() {
        return Ok(ValidateCallbackResult::Invalid(
            format!("Chunk data empty. Must have at least some content")
        ));
    }

    let Ok(dna_properties) = get_properties() else {
        debug!("Failed to get dna properties, skipping ParcelChunk validation");
        return Ok(ValidateCallbackResult::Valid)
    };

    if parcel_chunk.data.len() > dna_properties.max_chunk_size as usize {
        return Ok(ValidateCallbackResult::Invalid(
            format!("A chunk can't be bigger than {} KiB", dna_properties.max_chunk_size / 1024)
        ));
    }
    /// Done
    Ok(ValidateCallbackResult::Valid)
}


///
fn validate_ParcelManifest(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    let parcel_manifest = ParcelManifest::try_from(entry)?;

    /// Must have valid description
    if let ValidateCallbackResult::Invalid(reason) = validate_description(parcel_manifest.description.clone())? {
        return Ok(ValidateCallbackResult::Invalid(reason));
    }

    /// Must have chunks
    if parcel_manifest.chunks.is_empty() {
        return Ok(ValidateCallbackResult::Invalid("Missing chunks".to_string()));
    }

    let Ok(dna_properties) = get_properties() else {
        debug!("Failed to get dna properties, skipping ParcelManifest validation");
        return Ok(ValidateCallbackResult::Valid)
    };

    let PARCEL_MAX_CHUNKS: usize = (dna_properties.max_parcel_size / dna_properties.max_chunk_size as u64 + 1) as usize;

    /// Must not exceed size limit
    if parcel_manifest.chunks.len() > PARCEL_MAX_CHUNKS {
        return Ok(ValidateCallbackResult::Invalid(format!("Parcel is too big: {} > {} chunks", parcel_manifest.chunks.len(), PARCEL_MAX_CHUNKS)));
    }

    /// FIXME: Check each entry exists and is a ParcelChunk
    /// FIXME: Also check total size
    /// Done
    Ok(ValidateCallbackResult::Valid)
}


///
fn entry_index_to_variant(entry_index: EntryDefIndex) -> ExternResult<DeliveryEntryTypes> {
    let mut i = 0;
    for variant in DeliveryEntryTypes::iter() {
        if i == entry_index.0 {
            return Ok(variant);
        }
        i += 1;
    }
    return Err(wasm_error!(format!("Unknown EntryDefIndex: {}", entry_index.0)));
}
