use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::functions::*;
use crate::link_kind::LinkKind;


///
#[hdk_extern]
pub fn get_distribution_state(distribution_eh: EntryHash) -> ExternResult<DistributionState> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("get_destribution_state() CALLED");
   let distribution: Distribution = get_typed_from_eh(distribution_eh.clone())?;
   /// Get delivery state for each recipient
   //let mut deliveries: HashMap<AgentPubKey, DeliveryState> = HashMap::new();
   let mut deliveries: Vec<DeliveryState> = Vec::new();
   for recipient in distribution.recipients {
      let state = get_delivery_state(distribution_eh.clone(), &recipient)?;
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


///
//#[hdk_extern]
pub fn get_delivery_state(distribution_eh: EntryHash, recipient: &AgentPubKey) -> ExternResult<DeliveryState> {
   //std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("get_delivery_state() CALLED");
   /// Look for DeliveryReceipt
   let receipts = query_DeliveryReceipt(
      Some(distribution_eh.clone()),
      Some(recipient.clone()),
   )?;
   if !receipts.is_empty() {
      debug!("get_delivery_state() DeliveryReceipt found");
      return Ok(DeliveryState::ParcelDelivered);
   }
   /// Look for ReplyReceived
   let replies = query_ReplyReceived(
      Some(distribution_eh.clone()),
      Some(recipient.clone()),
   )?;
   if !replies.is_empty() {
      debug!("get_delivery_state() ReplyReceived found: {}", replies[0].has_accepted);
      if !replies[0].has_accepted {
         return Ok(DeliveryState::ParcelRefused);
      }
      // Look for PendingParcel
      let maybe_pending = find_PendingItem(distribution_eh, recipient.clone(), ItemKind::AppEntryBytes)?;
      if maybe_pending.is_some() {
         debug!("get_delivery_state() PendingParcel found");
         return Ok(DeliveryState::PendingParcel);
      }
      return Ok(DeliveryState::ParcelAccepted);
   }
   /// Look for NoticeReceived
   let mut receiveds = query_NoticeReceived(NoticeReceivedQueryField::Distribution(distribution_eh.clone()))?;
   receiveds.retain(|received| &received.recipient == recipient);
   if receiveds.is_empty() {
      // Look for PendingNotice
      let maybe_pending = find_PendingItem(distribution_eh, recipient.clone(), ItemKind::DeliveryNotice)?;
      if maybe_pending.is_some() {
         debug!("get_delivery_state() PendingNotice found");
         return Ok(DeliveryState::PendingNotice);
      }
      return Ok(DeliveryState::Unsent);
   }
   debug!("get_delivery_state() NoticeDelivered found");
   Ok(DeliveryState::NoticeDelivered)
}


///
#[hdk_extern]
pub fn get_notice_state(notice_eh: EntryHash) -> ExternResult<NoticeState> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("get_notice_state() CALLED");
   /// look for reply
   let maybe_reply = query_DeliveryReply(notice_eh.clone())?;
   if maybe_reply.is_none() {
      return Ok(NoticeState::Unreplied);
   }
   if !maybe_reply.unwrap().has_accepted {
      return Ok(NoticeState::Refused);
   }
   /// Look for parcel
   //let notice: DeliveryNotice = get_typed_from_eh(notice_eh)?;
   //let has_parcel = has_parcel(notice.summary.parcel_reference)?;
   let maybe_parcel = query_ParcelReceived(ParcelReceivedQueryField::Notice(notice_eh.clone()))?;
   /// Done
   if maybe_parcel.is_some() {
      return Ok(NoticeState::Received)
   }
   Ok(NoticeState::Accepted)
}


///
pub fn find_PendingItem(distribution_eh: EntryHash, recipient: AgentPubKey, kind: ItemKind)
   -> ExternResult<Option<PendingItem>> {
   let mut pairs: Vec<(PendingItem, Link)> = get_typed_from_links(
      distribution_eh,
      Some(LinkKind::Pendings.concat_hash(&recipient)),
   )?;
   pairs.retain(|pair| pair.0.kind == kind);
   /// Search through results
   for pair in pairs {
      return Ok(Some(pair.0.clone()));
   }
   /// Done
   Ok(None)
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
