use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use zome_delivery_integrity::*;


/// Return EntryHash of NoticeReply
#[hdk_extern]
pub fn respond_to_notice(input: RespondToNoticeInput) -> ExternResult<EntryHash> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Make sure EntryHash is correct and is a DeliveryNotification
   let _notice: DeliveryNotice = get_typed_from_eh(input.notice_eh.clone(), GetStrategy::Network)?;
   debug!("responding with: {:?}", input.has_accepted);
   /// Create NoticeReply
   let reply = NoticeReply {
      notice_eh: input.notice_eh,
      has_accepted: input.has_accepted,
   };
   let eh = hash_entry(reply.clone())?;
   /// Commit NoticeReply
   debug!("Creating reply...");
   let _hh = create_entry_relaxed(DeliveryEntry::NoticeReply(reply))?;
   /// Done
   Ok(eh)
}
