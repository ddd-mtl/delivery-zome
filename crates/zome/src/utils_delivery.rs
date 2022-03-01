use hdk::prelude::*;
//use zome_utils::*;
use zome_delivery_types::*;


///
pub fn get_delivery_state(_distribution_eh: EntryHash, _recipient: &AgentPubKey) -> ExternResult<DeliveryState> {
   /// FIXME
   Ok(DeliveryState::Unsent)
}


///
pub fn get_destribution_state(_distribution_eh: EntryHash) -> ExternResult<DistributionState> {
   /// FIXME
   Ok(DistributionState::Unsent)
}


///
pub fn get_notice_state(_notice_eh: EntryHash) -> ExternResult<DistributionState> {
   /// FIXME
   Ok(DistributionState::Unsent)
}