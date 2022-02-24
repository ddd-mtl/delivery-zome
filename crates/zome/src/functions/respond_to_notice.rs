use hdk::prelude::*;
use zome_delivery_types::{entries::*, entry_kind::*, parcel::*, utils::*};
use zome_delivery_types::*;

/// Zone Function
/// Return EntryHash of DeliveryReply
#[hdk_extern]
pub fn respond_to_notice(input: RespondToNoticeInput) -> ExternResult<EntryHash> {
   /// Get DeliveryNotification
   let notice: DeliveryNotice = get_typed_from_eh(input.notice_eh.clone())?;
   /// Create DeliveryReply
   let reply = DeliveryReply {
      notice_eh: notice.distribution_eh,
      has_accepted: input.has_accepted,
   };
   let eh = hash_entry(reply.clone())?;
   /// Commit DeliveryReply
   let _hh = create_entry(reply)?;
   /// Done
   Ok(eh)
}
