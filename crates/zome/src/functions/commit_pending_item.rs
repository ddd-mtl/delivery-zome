use hdk::prelude::*;
use zome_delivery_types::*;
use crate::link_kind::*;
use zome_utils::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitPendingItemInput {
   pub item: PendingItem,
   pub recipient: AgentPubKey,
}


#[hdk_extern]
fn commit_pending_item(input: CommitPendingItemInput) -> ExternResult<HeaderHash> {
   debug!("commit_pending_item() START **********");
   std::panic::set_hook(Box::new(my_panic_hook));

   let me = agent_info()?.agent_latest_pubkey;
   /// Commit Pending Item
   let pending_item_eh = hash_entry(&input.item)?;
   let maybe_pending_item_hh = create_entry(&input.item);
   if let Err(err) = maybe_pending_item_hh.clone() {
      debug!("PendingItem create_entry() failed = {:?}", err);
      return Err(maybe_pending_item_hh.err().unwrap());
   };
   let pending_item_hh = maybe_pending_item_hh.unwrap();
   trace!("pending_item_hh = {:?}", pending_item_hh);
   /// Commit Pendings Link
   if input.item.kind.can_link_to_distribution() {
      let tag = LinkKind::Pendings.concat_hash(&input.recipient);
      trace!("pendings tag = {:?}", tag);
      let maybe_link1_hh = create_link(
         input.item.distribution_eh.clone(),
         pending_item_eh.clone(),
         tag);
      if let Err(err) = maybe_link1_hh.clone() {
         trace!("link1 failed = {:?}", err);
         return Err(maybe_link1_hh.err().unwrap());
      };
      let link1_hh = maybe_link1_hh.unwrap();
      trace!("link1_hh = {}", link1_hh);
   }
   /// Commit Inbox Link
   let tag = LinkKind::Inbox.concat_hash(&me);
   let maybe_link2_hh = create_link(EntryHash::from(input.recipient.clone()), pending_item_eh, tag);
   if let Err(err) = maybe_link2_hh.clone() {
      trace!("link2 failed = {:?}", err);
      return Err(maybe_link2_hh.err().unwrap());
   };
   let link2_hh = maybe_link2_hh.unwrap();
   trace!("link2_hh = {}", link2_hh);
   /// Done
   return Ok(pending_item_hh)
}
