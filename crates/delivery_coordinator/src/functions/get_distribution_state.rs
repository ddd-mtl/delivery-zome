use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use crate::*;



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FullDistributionState {
   distribution_state: DistributionState,
   delivery_states: Vec<DeliveryState>, // In the order of the distribution's recipients
}


///
#[hdk_extern]
pub fn get_distribution_state(distribution_eh: EntryHash) -> ExternResult<FullDistributionState> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("distribution_eh: {}", distribution_eh.clone());
   let distribution: Distribution = get_typed_from_eh(distribution_eh.clone())?;
   /// Get delivery state for each recipient
   //let mut deliveries: HashMap<AgentPubKey, DeliveryState> = HashMap::new();
   let mut delivery_states: Vec<DeliveryState> = Vec::new();
   for recipient in distribution.recipients {
      let state = get_delivery_state(GetDeliveryStateInput{ distribution_eh: distribution_eh.clone(), recipient})?;
      delivery_states.push( state);
   }
   let distribution_state = determine_distribution_state(&delivery_states);
   Ok(FullDistributionState {distribution_state, delivery_states})
}


///
fn determine_distribution_state(delivery_states: &Vec<DeliveryState>) -> DistributionState {
   /// - Determine distribution state
   /// Return 'Unsent' if at least one delivery is unsent
   if delivery_states.contains(&DeliveryState::Unsent) {
      return DistributionState::Unsent;
   }
   /// Return 'AllNoticesSent' if at least one Notice is Pending
   if delivery_states.contains(&DeliveryState::PendingNotice) {
      return DistributionState::AllNoticesSent;
   }
   /// Return 'AllNoticeReceived' if at least one reply is missing
   if delivery_states.contains(&DeliveryState::NoticeDelivered) {
      return DistributionState::AllNoticeReceived;
   }
   /// Return 'AllRepliesReceived' if at least one ParcelDelivered is missing
   if delivery_states.contains(&DeliveryState::ParcelAccepted) {
      return DistributionState::AllRepliesReceived;
   }
   /// Return 'AllRepliesReceived' if at least one ParcelDelivered is missing
   if delivery_states.contains(&DeliveryState::PendingParcel) {
      return DistributionState::AllRepliesReceived;
   }
   /// All accepted should have been received
   DistributionState::AllAcceptedParcelsReceived
}


// /// Return True if parcel is fully committed to our local source chain
// pub fn has_parcel(parcel_ref: ParcelReference) -> ExternResult<bool> {
//    match parcel_ref {
//       ParcelReference::AppEntry(_, _, eh) => {
//          let maybe_entry = get_entry_from_eh(eh);
//          return Ok(maybe_entry.is_ok());
//       },
//       ParcelReference::Manifest(manifest_eh) => {
//          let maybe_manifest: ExternResult<ParcelManifest> = get_typed_from_eh(manifest_eh);
//          if maybe_manifest.is_err() {
//             return Ok(false);
//          }
//          for chunk_eh in maybe_manifest.unwrap().chunks {
//             let maybe_entry = get_entry_from_eh(chunk_eh);
//             if maybe_entry.is_err() {
//                return Ok(false);
//             }
//          }
//       }
//    }
//    Ok(true)
// }
