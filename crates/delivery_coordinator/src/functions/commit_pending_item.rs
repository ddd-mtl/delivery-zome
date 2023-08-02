use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_integrity::*;
use zome_delivery_types::*;
use zome_delivery_common::*;


#[hdk_extern]
fn commit_pending_item(input: CommitPendingItemInput) -> ExternResult<ActionHash> {
   debug!("commit_pending_item() START");
   std::panic::set_hook(Box::new(zome_panic_hook));
   let me = agent_info()?.agent_latest_pubkey;
   /// Commit Pending Item
   let pending_item_eh = hash_entry(&input.item)?;
   let maybe_pending_item_hh = create_entry_relaxed(DeliveryEntry::PendingItem(input.item.clone()));
   if let Err(err) = maybe_pending_item_hh.clone() {
      debug!("PendingItem create_entry_relaxed() failed = {:?}", err);
      return Err(maybe_pending_item_hh.err().unwrap());
   };
   let pending_item_hh = maybe_pending_item_hh.unwrap();
   trace!("pending_item_hh = {:?}", pending_item_hh);
   /// Commit Pendings Link
   if input.item.kind.can_link_to_distribution() {
      let tag = LinkTag::from(input.recipient.as_ref().to_vec());
      trace!("pendings tag = {:?}", tag);
      let maybe_link1_hh = create_link_relaxed(
         input.item.distribution_eh.clone(),
         pending_item_eh.clone(),
         LinkTypes::Pendings,
         tag);
      if let Err(err) = maybe_link1_hh.clone() {
         trace!("link1 failed = {:?}", err);
         return Err(maybe_link1_hh.err().unwrap());
      };
      let link1_hh = maybe_link1_hh.unwrap();
      trace!("link1_hh = {}", link1_hh);
   }
   /// Commit Inbox Link
   let tag = LinkTag::from(me.as_ref().to_vec());
   let maybe_link2_hh = create_link_relaxed(
      EntryHash::from(input.recipient.clone()),
      pending_item_eh,
      LinkTypes::Inbox,
      tag,
   );
   if let Err(err) = maybe_link2_hh.clone() {
      trace!("link2 failed = {:?}", err);
      return Err(maybe_link2_hh.err().unwrap());
   };
   let link2_hh = maybe_link2_hh.unwrap();
   trace!("link2_hh = {}", link2_hh);
   /// Done
   return Ok(pending_item_hh)
}
