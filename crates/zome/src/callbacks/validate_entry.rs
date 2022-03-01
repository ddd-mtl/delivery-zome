use hdk::prelude::*;
use hdk::prelude::element::ElementEntry;
//use zome_delivery_types::*;
//use hdk::prelude::countersigning::CounterSigningSessionData;

use zome_utils::*;
//use crate::entries::pub_enc_key::*;
use crate::entry_kind::*;


/// Zome Callback
#[hdk_extern]
fn validate(input: ValidateData) -> ExternResult<ValidateCallbackResult> {
    trace!("*** `validate()` callback called!: {:?}", input);
    std::panic::set_hook(Box::new(my_panic_hook));
    /// Get entry
    let entry = input.element.clone().into_inner().1;
    let entry = match entry {
        ElementEntry::Present(e) => e,
        _ => return Ok(ValidateCallbackResult::Valid), // WARN - Why not invalid?
    };
    /// Determine where to dispatch according to base
    let result = match entry {
        Entry::CounterSign(_data, _bytes) => Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into())), //validate_counter_sign_entry(data, bytes, maybe_package),
        Entry::Agent(agent_hash) => validate_agent_entry(agent_hash, input.validation_package),
        Entry::CapClaim(claim) => validate_claim_entry(claim, input.validation_package),
        Entry::CapGrant(grant) => validate_grant_entry(grant, input.validation_package),
        Entry::App(entry_bytes) => validate_app_entry(entry_bytes, input),
    };
    /// Done
    trace!("*** validate() result = {:?}", result);
    result
}

///
#[allow(unreachable_patterns)]
fn validate_app_entry(entry_bytes: AppEntryBytes, input: ValidateData) -> ExternResult<ValidateCallbackResult> {
    trace!("*** validate_app_entry() callback called!");
    let entry_type = input.element.header().entry_type().unwrap();
    trace!("validate App entry type: {:?}", entry_type);
    let entry_index = if let EntryType::App(app_entry_type) = entry_type {
        app_entry_type.id()
    } else {
        debug!("validation failure: Non App types should have already been filtered out");
        unreachable!("In validate_app_entry")
    };
    let delivery_entry = deserialize_into_zome_entry(&entry_index, entry_bytes)?;
    let validation = delivery_entry.validate(input.validation_package);
    validation
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