use hdi::prelude::*;
use zome_delivery_types::{get_properties, DELIVERY_INTERGRITY_ZOME_NAME};

#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
   debug!("{} genesis_self_check() CALLED", DELIVERY_INTERGRITY_ZOME_NAME);
   let _info = dna_info()?;
   let Ok(properties) = get_properties() else {
      return Ok(ValidateCallbackResult::Invalid("No properties".into()));
   };
   //
   return properties.validate();
}
