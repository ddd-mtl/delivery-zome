use std::future::Pending;
use hdk::prelude::*;
use crate::{
   utils::*,
   entries::*, utils_parcel::*, LinkKind, utils::*,
   parcel::*, dm::*, DeliveryProtocol,
   EntryKind::DeliveryNotice,
   functions::*,
};


/// Zone Function
/// Return EntryHash of DeliveryReply
#[hdk_extern]
pub fn respond_to_notice(notice_eh: EntryHash, has_accepted: bool) -> ExternResult<Option<EntryHash>> {
   /// Get DeliveryNotification
   let notice: DeliveryNotice = get_typed_from_eh(notice_eh.clone())?;
   /// Create DeliveryReply
   let reply = DeliveryReply {
      notice_eh: notice.distribution_eh,
      has_accepted,
   };
   let eh = hash_entry(reply.clone())?;
   /// Commit DeliveryReply
   let _hh = create_entry(reply)?;
   /// Done
   Ok(Some(eh))
}

