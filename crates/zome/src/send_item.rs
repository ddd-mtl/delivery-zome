use hdk::prelude::*;

use crate::utils::*;
use zome_delivery_types::*;

use crate::{
   send_dm, DeliveryProtocol,
   functions::CommitPendingItemInput,
};

#[allow(non_camel_case_types)]
pub enum SendSuccessKind {
   OK_SELF,
   OK_DIRECT,
   OK_PENDING,
}

/// called from post_commit()
pub fn send_item(
   recipient: AgentPubKey,
   //distribution_eh: EntryHash,
   pending_item: PendingItem,
   //signed_item: Signature,
) -> ExternResult<SendSuccessKind> {
   debug!("send_item() START - {:?}", recipient);
   /// Try sending directly to other Agent if Online
   // let result = send_item_by_dm(recipient, distribution_eh, pending_item.clone(), signed_item);
   let response_dm = send_dm(recipient.clone(), DeliveryProtocol::Item(pending_item.clone()))?;
   debug!("send_item_by_dm() response_dm = {}", response_dm);
   if let DeliveryProtocol::Success = response_dm {
      return Ok(SendSuccessKind::OK_DIRECT);
   } else {
      debug!("send_item() failed: {}", response_dm);
   }
   debug!("send_item() - Commit PendingItem...");
   /// DM failed, send to DHT instead by creating a PendingItem
   /// Create and commit PendingItem with remote call to self
   let input = CommitPendingItemInput {
      item: pending_item,
      recipient: recipient.clone(),
   };
   debug!("send_item() - calling commit_pending_item()");
   let response = call_self("commit_pending_item", input)?;
   debug!("send_confirmation() - commit_pending_item() response: {:?}", response);
   return match response {
      ZomeCallResponse::Ok(_) => Ok(SendSuccessKind::OK_PENDING),
      _ => error("call_self() to commit_pending_item() failed"),
   };
}
