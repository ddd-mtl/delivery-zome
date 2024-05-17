use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_integrity::*;
use zome_delivery_types::*;


#[hdk_extern]
#[feature(zits_blocking)]
fn commit_pending_item(input: CommitPendingItemInput) -> ExternResult<ActionHash> {
   debug!("START");
   std::panic::set_hook(Box::new(zome_panic_hook));
   let me = agent_info()?.agent_latest_pubkey;
   /// Commit Pending Item
   let pending_item_eh = hash_entry(&input.item)?;
   let maybe_pending_item_ah = create_entry_relaxed(DeliveryEntry::PendingItem(input.item.clone()));
   if let Err(err) = maybe_pending_item_ah.clone() {
      debug!("PendingItem create_entry_relaxed() failed = {:?}", err);
      return Err(maybe_pending_item_ah.err().unwrap());
   };
   let pending_item_ah = maybe_pending_item_ah.unwrap();
   trace!("pending_item_ah = {:?}", pending_item_ah);
   /// Commit Pendings Link
   if input.item.kind.can_link_to_distribution() {
      let tag = LinkTag::from(input.recipient.as_ref().to_vec());
      trace!("pendings tag = {:?}", tag);
      let maybe_link1_ah = create_link_relaxed(
         input.item.distribution_ah.clone(),
         pending_item_eh.clone(),
         LinkTypes::Pendings,
         tag);
      if let Err(err) = maybe_link1_ah.clone() {
         trace!("link1 failed = {:?}", err);
         return Err(maybe_link1_ah.err().unwrap());
      };
      let link1_ah = maybe_link1_ah.unwrap();
      trace!("link1_ah = {}", link1_ah);
   }
   /// Commit Inbox Link
   let tag = LinkTag::from(me.as_ref().to_vec());
   let maybe_link2_ah = create_link_relaxed(
      EntryHash::from(input.recipient.clone()),
      pending_item_eh,
      LinkTypes::Inbox,
      tag,
   );
   if let Err(err) = maybe_link2_ah.clone() {
      trace!("link2 failed = {:?}", err);
      return Err(maybe_link2_ah.err().unwrap());
   };
   let link2_ah = maybe_link2_ah.unwrap();
   trace!("link2_ah = {}", link2_ah);
   /// Done
   return Ok(pending_item_ah)
}
