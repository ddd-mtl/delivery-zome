use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;


///
#[hdk_extern]
pub fn get_distribution_state(distribution_eh: EntryHash) -> ExternResult<DistributionState> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("distribution_eh: {}", distribution_eh.clone());
   let distribution: Distribution = get_typed_from_eh(distribution_eh.clone())?;
   /// Get delivery state for each recipient
   //let mut deliveries: HashMap<AgentPubKey, DeliveryState> = HashMap::new();
   let mut deliveries: Vec<DeliveryState> = Vec::new();
   for recipient in distribution.recipients {
      let state = get_delivery_state(GetDeliveryStateInput{ distribution_eh: distribution_eh.clone(), recipient})?;
      deliveries.push( state);
   }
   /// - Determine distribution state
   /// Return 'Unsent' if at least one delivery is unsent
   if deliveries.contains(&DeliveryState::Unsent) {
      return Ok(DistributionState::Unsent);
   }
   /// Return 'AllNoticesSent' if at least one Notice is Pending
   if deliveries.contains(&DeliveryState::PendingNotice) {
      return Ok(DistributionState::AllNoticesSent);
   }
   /// Return 'AllNoticeReceived' if at least one reply is missing
   if deliveries.contains(&DeliveryState::NoticeDelivered) {
      return Ok(DistributionState::AllNoticeReceived);
   }
   /// Return 'AllRepliesReceived' if at least one ParcelDelivered is missing
   if deliveries.contains(&DeliveryState::ParcelAccepted) {
      return Ok(DistributionState::AllRepliesReceived);
   }
   /// Return 'AllRepliesReceived' if at least one ParcelDelivered is missing
   if deliveries.contains(&DeliveryState::PendingParcel) {
      return Ok(DistributionState::AllRepliesReceived);
   }
   /// All accepted should have been received
   Ok(DistributionState::AllAcceptedParcelsReceived)
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
