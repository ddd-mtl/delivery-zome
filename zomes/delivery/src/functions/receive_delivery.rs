use std::future::Pending;
use hdk::prelude::*;
use crate::{
   utils::*,
   entries::*, utils_parcel::*, LinkKind, utils::*,
   parcel::*, dm::*, DeliveryProtocol,
   EntryKind::DeliveryNotice,
};

/// Zone Function
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
pub fn receive_parcel(notification_eh: EntryHash) -> ExternResult<Option<EntryHash>> {
   /// Get DeliveryNotification
   let notice: DeliveryNotice = get_typed_from_eh(notification_eh.clone())?;
   /// Look for parcel
   let maybe_parcel = pull_parcel(notice)?;
   if maybe_parcel.is_none() {
      return Ok(None);
   };
   /// Commit Parcel
   let parcel_entry = maybe_parcel.unwrap();
   let eh = hash_entry(parcel_entry.clone())?;
   let hh = create_entry(parcel_entry.clone())?;
   /// Sign
   let signature = sign(agent_info()?.agent_latest_pubkey, parcel_entry)?;
   /// Create ReceptionConfirmation
   let confirmation = ReceptionConfirmation {
      notification_eh,
      reception_response: ReceptionResponse::Accepted((hh, signature)),
   };
   let _hh = create_entry(confirmation)?;
   /// Done
   Ok(Some(eh))
}


/// Try to retrieve the parcel entry
pub fn pull_parcel(notice: DeliveryNotice) -> ExternResult<Option<Entry>> {
   /// Request Parcel
   /// Check Inbox first:
   /// Get all Parcels inbox and see if its there
   let me = agent_info()?.agent_latest_pubkey;
   let my_agent_eh = EntryHash::from(me.clone());
   let pending_items = get_links_and_load_type::<PendingItem>(my_agent_eh.clone(), LinkKind::Inbox.as_tag_opt())?;
   /// Check each Inbox link
   for pending_item in &pending_items {
      match pending_item.kind {
         PendingKind::Parcel => {
            if pending_item.distribution_eh != notice.sender_distribution_eh {
               continue;
            }
            /// We have the parcel we just need to deserialize it
            let parcel_entry: Entry = pending_item.try_into(notice.sender.clone())?
               .expect("PendingItem should hold an Entry");
            return Ok(Some(parcel_entry));
         }
         _ => continue,
      }
   }
   /// Not found in Inbox
   /// Try via DM second
   let dm =  DirectMessageProtocol::ParcelRequest(notice.description.parcel.entry_address());
   let response = send_dm(notice.sender, dm)?;
   if let DeliveryProtocol::ParcelResponse(entry) = response {
      return Ok(Some(entry));
   }
   /// TODO: Ask Recipient peers?
   /// Not found
   Ok(None)
}
