use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;

///
#[hdk_extern]
pub fn get_delivery_state(input: GetDeliveryStateInput) -> ExternResult<DeliveryState> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("recipient: {} || distrib: {}", input.recipient, input.distribution_eh);
   /// Look for DeliveryReceipt
   let receipts = query_DeliveryReceipt(
      Some(input.distribution_eh.clone()),
      Some(input.recipient.clone()),
   )?;
   if !receipts.is_empty() {
      debug!("DeliveryReceipt found");
      return Ok(DeliveryState::ParcelDelivered);
   }
   /// Look for ReplyReceived
   let replies = query_ReplyReceived(
      Some(input.distribution_eh.clone()),
      Some(input.recipient.clone()),
   )?;
   if !replies.is_empty() {
      debug!("ReplyReceived found: {}", replies[0].has_accepted);
      if !replies[0].has_accepted {
         return Ok(DeliveryState::ParcelRefused);
      }
      // Look for PendingParcel
      let maybe_pending = find_PendingItem(input.distribution_eh, input.recipient.clone(), ItemKind::AppEntryBytes)?;
      if maybe_pending.is_some() {
         debug!("PendingParcel found");
         return Ok(DeliveryState::PendingParcel);
      }
      return Ok(DeliveryState::ParcelAccepted);
   }
   /// Look for NoticeReceived
   let mut receiveds = query_NoticeReceived(NoticeReceivedQueryField::Distribution(input.distribution_eh.clone()))?;
   debug!("receiveds len1: {}", receiveds.len());
   receiveds.retain(|received| &received.recipient == &input.recipient);
   debug!("receiveds len2: {}", receiveds.len());
   if receiveds.is_empty() {
      // Look for PendingNotice
      let maybe_pending = find_PendingItem(input.distribution_eh, input.recipient.clone(), ItemKind::DeliveryNotice)?;
      if maybe_pending.is_some() {
         debug!("PendingNotice found");
         return Ok(DeliveryState::PendingNotice);
      }
      return Ok(DeliveryState::Unsent);
   }
   debug!("NoticeDelivered found");
   Ok(DeliveryState::NoticeDelivered)
}


///
pub fn find_PendingItem(distribution_eh: EntryHash, recipient: AgentPubKey, kind: ItemKind)
   -> ExternResult<Option<PendingItem>> {
   let mut pairs: Vec<(PendingItem, Link)> = get_typed_from_links(
      distribution_eh,
      LinkTypes::Pendings,
      Some(LinkTag::from(recipient.as_ref().to_vec())),
   )?;
   pairs.retain(|pair| pair.0.kind == kind);
   /// Search through results
   for pair in pairs {
      return Ok(Some(pair.0.clone()));
   }
   /// Done
   Ok(None)
}
