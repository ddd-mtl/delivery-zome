use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use zome_delivery_integrity::*;


/// Zone Function
/// Return EntryHash of DeliveryReply
#[hdk_extern]
pub fn respond_to_notice(input: RespondToNoticeInput) -> ExternResult<EntryHash> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("Notice reponse: {:?}", input.has_accepted);
   /// Make sure its a DeliveryNotification
   let _: DeliveryNotice = get_typed_from_eh(input.notice_eh.clone())?;
   /// Create DeliveryReply
   let reply = DeliveryReply {
      notice_eh: input.notice_eh,
      has_accepted: input.has_accepted,
   };
   let eh = hash_entry(reply.clone())?;
   /// Commit DeliveryReply
   debug!("Creating reply...");
   let _hh = create_entry_relaxed(DeliveryEntry::DeliveryReply(reply))?;
   /// Done
   Ok(eh)
}
