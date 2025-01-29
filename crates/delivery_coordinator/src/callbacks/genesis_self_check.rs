use hdk::prelude::*;


#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("delivery_coordinatoor.genesis_self_check() START");
   /// Check Properties
   let res = validate_properties()?;
   let InitCallbackResult::Pass = res
      else { return Ok(res); };

   Ok(ValidateCallbackResult::Valid)
}
