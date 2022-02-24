use hdk::prelude::*;
use hdk::prelude::element::ElementEntry;
use zome_delivery_types::entry_kind::*;
//use hdk::prelude::countersigning::CounterSigningSessionData;

//use crate::entries::*;
//use crate::entries::pub_enc_key::*;


/// Zome Callback
#[hdk_extern]
fn validate(input: ValidateData) -> ExternResult<ValidateCallbackResult> {
    trace!("*** `validate()` callback called!: {:?}", input);
    /// Get entry
    let maybe_package = input.validation_package;
    let element = input.element;
    let entry = element.clone().into_inner().1;
    let entry = match entry {
        ElementEntry::Present(e) => e,
        _ => return Ok(ValidateCallbackResult::Valid), // WARN - Why not invalid?
    };
    /// Determine where to dispatch according to base
    let result = match entry {
        Entry::CounterSign(_data, _bytes) => Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into())), //validate_counter_sign_entry(data, bytes, maybe_package),
        Entry::Agent(agent_hash) => validate_agent_entry(agent_hash, maybe_package),
        Entry::CapClaim(claim) => validate_claim_entry(claim, maybe_package),
        Entry::CapGrant(grant) => validate_grant_entry(grant, maybe_package),
        Entry::App(entry_bytes) => {
            let entry_type = element.header().entry_type().unwrap();
            trace!("validate App entry type: {:?}", entry_type);
            let app_type_id = if let EntryType::App(app_entry_type) = entry_type {
                app_entry_type.id()
            } else {
                debug!("validation failure: Non App types should have already been filtered out");
                unreachable!()
            };
            validate_app_entry(app_type_id, entry_bytes, maybe_package)
        },
    };
    /// Done
    trace!("*** validate() result = {:?}", result);
    result
}

///
#[allow(unreachable_patterns)]
fn validate_app_entry(
    entry_type_id: EntryDefIndex,
    _entry_bytes: AppEntryBytes,
    _maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    trace!("*** validate_app_entry() callback called!");
    //let sb = entry_bytes.into_sb();
    //let entry_kind = EntryKind::from_index(&entry_type_id);

    // FIXME
    Ok(ValidateCallbackResult::Valid)
    //
    // match entry_kind {
    //     EntryKind::Distribution => {
    //         let maybe = Distribution::try_from(sb.clone());
    //         if let Err(_err) = maybe {
    //             return error("Failed to deserialize DeliveryNotice");
    //         }
    //         let distribution = maybe.unwrap();
    //         let res = distribution.validate(maybe_validation_package);
    //         res
    //     },
    //     // EntryKind::PubEncKey => {
    //     //     let maybe_key = PubEncKey::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_key {
    //     //         return error("Failed to deserialize PubEncKey");
    //     //     }
    //     //     let key = maybe_key.unwrap();
    //     //     let res = validate_PubEncKey_entry(key, maybe_validation_package);
    //     //     res
    //     // },
    //     // EntryKind::Path => {
    //     //     let maybe_content = PathEntry::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize PathEntry");
    //     //     }
    //     //     // FIXME validation
    //     //     Ok(ValidateCallbackResult::Valid)
    //     // },
    //     // EntryKind::InMail => {
    //     //     let maybe_content = InMail::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize InMail");
    //     //     }
    //     //     // FIXME validation
    //     //     // return validate_inmail_entry(inmail, maybe_validation_package);
    //     //     Ok(ValidateCallbackResult::Valid)
    //     // },
    //     // EntryKind::InAck => {
    //     //     let maybe_content = InAck::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize InAck");
    //     //     }
    //     //     // FIXME validation
    //     //     Ok(ValidateCallbackResult::Valid)
    //     // },
    //     // EntryKind::PendingMail => {
    //     //     let maybe_content = PendingMail::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize PendingMail");
    //     //     }
    //     //     // FIXME validation
    //     //     Ok(ValidateCallbackResult::Valid)
    //     // },
    //     // EntryKind::PendingAck => {
    //     //     let maybe_content = PendingAck::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize PendingAck");
    //     //     }
    //     //     // FIXME
    //     //     Ok(ValidateCallbackResult::Valid)
    //     // },
    //     // EntryKind::DeliveryConfirmation => {
    //     //     let maybe_content = DeliveryConfirmation::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize DeliveryConfirmation");
    //     //     }
    //     //     // FIXME
    //     //     Ok(ValidateCallbackResult::Valid)
    //     // },
    //     // EntryKind::OutMail => {
    //     //     let maybe_content = OutMail::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize OutMail");
    //     //     }
    //     //     // FIXME
    //     //     Ok(ValidateCallbackResult::Valid)
    //     // },
    //     // EntryKind::OutAck => {
    //     //     let maybe_content = OutAck::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize OutAck");
    //     //     }
    //     //     // FIXME
    //     //     Ok(ValidateCallbackResult::Valid)
    //     // },
    //     // EntryKind::FileManifest => {
    //     //     let maybe_content = FileManifest::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize FileManifest");
    //     //     }
    //     //     let manifest = maybe_content.unwrap();
    //     //     let res = validate_file(manifest, maybe_validation_package);
    //     //     res
    //     // },
    //     // EntryKind::FileChunk => {
    //     //     let maybe_content = FileChunk::try_from(sb.clone());
    //     //     if let Err(_err) = maybe_content {
    //     //         return error("Failed to deserialize FileChunk");
    //     //     }
    //     //     let chunk = maybe_content.unwrap();
    //     //     let res = validate_chunk(chunk, maybe_validation_package);
    //     //     res
    //     // },
    //     // /// Add entry validation per type here
    //     // /// ..
    //
    //     /// Unreachable but doesnt compile without it. Yay Rust
    //     _ => Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into())),
    // }
}

///
fn validate_agent_entry(
    _agent_hash: AgentPubKey,
    _maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    trace!("*** validate_agent_entry() called!");
    // FIXME
    Ok(ValidateCallbackResult::Valid)
}

///
fn validate_claim_entry(
    _claim: CapClaim,
    _maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    trace!("*** validate_claim_entry() called!");
    // FIXME validation
    Ok(ValidateCallbackResult::Valid)
    //Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into()))
}

///
fn validate_grant_entry(
    _grant: ZomeCallCapGrant,
    _maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    trace!("*** validate_grant_entry() called!");
    // FIXME validation
    Ok(ValidateCallbackResult::Valid)
    //Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into()))
}

//
// ///
// /// TODO: Check if data_hash not already stored in source chain
// pub(crate) fn validate_file(manifest: FileManifest, _maybe_validation_package: Option<ValidationPackage>)
//     -> ExternResult<ValidateCallbackResult>
// {
//     /// Check size
//     if manifest.orig_filesize > FILE_MAX_SIZE {
//         return Ok(ValidateCallbackResult::Invalid(
//             format!("A file can't be bigger than {} MiB", FILE_MAX_SIZE / (1024 * 1024))));
//     }
//     if manifest.orig_filesize < 1 {
//         return Ok(ValidateCallbackResult::Invalid("A file cannot be empty".into()));
//     }
//     if manifest.chunks.len() < 1 {
//         return Ok(ValidateCallbackResult::Invalid("A file must have at least one chunk".into()));
//     }
//     Ok(ValidateCallbackResult::Valid)
// }


//
//fn validate_counter_sign_entry(
//    _data: Box<CounterSigningSessionData, Global>,
//    _bytes: AppEntryBytes,
//    _maybe_validation_package: Option<ValidationPackage>,
//) -> ExternResult<ValidateCallbackResult>
//{
//    trace!("*** validate_counter_sign_entry() called!");
//    // FIXME validation
//    //Ok(ValidateCallbackResult::Valid)
//    Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into()))
//}