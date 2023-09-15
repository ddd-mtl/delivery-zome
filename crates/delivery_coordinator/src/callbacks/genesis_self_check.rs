use hdk::prelude::*;


#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
   // FIXME: Move properties check here. Cannot do it currently since dna_info has been removed from GenesisSelfCheckData
   Ok(ValidateCallbackResult::Valid)
}