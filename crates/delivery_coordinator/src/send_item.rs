use hdk::prelude::*;

use zome_utils::*;
use zome_delivery_types::*;
//use zome_delivery_integrity::*;

use crate::{
   send_dm, DeliveryProtocol,
};


#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Debug)]
pub enum SendSuccessKind {
   OK_SELF,
   OK_DIRECT(Signature),
   OK_PENDING,
}

/// called from post_commit()
pub fn send_item(
   recipient: ActionHash,
   pending_item: PendingItem,
   strategy: DistributionStrategy,
) -> ExternResult<SendSuccessKind> {
   debug!("START - '{:?}' to {}", pending_item.kind, snip(&recipient));
   /// Try with DM
   if strategy.can_dm() {
      debug!("DM - {:?}", strategy);
      /// get owners
      let owners = probe_owners(recipient)?;
      if !owners.is_empty() {
         let first_owner: AgentPubKey = owners[0];
         /// Try sending directly to other Agent if Online
         // let result = send_item_by_dm(recipient, distribution_ah, pending_item.clone(), signed_item);
         let response_dm = send_dm(
            first_owner.clone(),
            DeliveryProtocol::Item(pending_item.clone())
            , )?;
         debug!("response_dm = {}", response_dm);
         if let DeliveryProtocol::Success(signature) = response_dm {
            return Ok(SendSuccessKind::OK_DIRECT(signature));
         } else {
            debug!("failed: {}", response_dm);
         }
      }
   }
   /// Try with DHT by committing public PendingItem
   if strategy.can_dht() {
      debug!("DHT - {:?}", strategy);
      debug!("Commit PendingItem...");
      /// DM failed, send to DHT instead by creating a PendingItem
      /// Create and commit PendingItem with remote call to self
      let input = CommitPendingItemInput {
         item: pending_item,
         recipient: recipient.clone(),
      };
      debug!("calling commit_pending_item()");
      let response = call_self("commit_pending_item", input)?;
      //debug!("send_item() - commit_pending_item() response: {:?}", response);
      return match response {
         ZomeCallResponse::Ok(_) => Ok(SendSuccessKind::OK_PENDING),
         ZomeCallResponse::NetworkError(err) => {
            return error(&format!("call_self() to commit_pending_item() failed: {}", err));
         },
         _ => error("call_self() to commit_pending_item() failed"),
      };
   }
   ///
   return error("Failed to send item");
}
