use hdk::prelude::*;

use crate::{entries::*, utils::*, send_dm, dm_protocol::*, link_kind::*, ParcelReference};

/// Zone Function
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
pub fn fetch_parcel(notice_eh: EntryHash) -> ExternResult<Option<EntryHash>> {
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(notice_eh.clone())?;
   /// Look for Parcel
   let maybe_parcel = pull_parcel(notice)?;
   if maybe_parcel.is_none() {
      return Ok(None);
   };
   /// Commit Parcel
   let parcel_entry = maybe_parcel.unwrap();
   let parcel_eh = hash_entry(parcel_entry.clone())?;
   let _parcel_hh = create_entry(parcel_entry.clone())?;
   /// Create ParcelReceived if not a manifest
   if let ParcelReference::AppEntry(_) = notice.parcel_summary.reference {
      let received = ParcelReceived {
         notice_eh,
         parcel_eh: parcel_eh.clone(),
      };
      let _hh = create_entry(received)?;
   }
   /// Done
   Ok(Some(parcel_eh))
}

/// Try to retrieve the parcel entry
pub fn pull_parcel(notice: DeliveryNotice) -> ExternResult<Option<Entry>> {
   /// Request Parcel
   /// Check Inbox first:
   /// Get all Parcels inbox and see if its there
   let me = agent_info()?.agent_latest_pubkey;
   let my_agent_eh = EntryHash::from(me.clone());
   let pending_items = get_links_and_load_type::<PendingItem>(my_agent_eh.clone(), LinkKind::Inbox.as_tag_opt(), false)?;
   /// Check each Inbox link
   for pending_item in &pending_items {
      match pending_item.kind {
         PendingKind::Entry => {
            if pending_item.distribution_eh != notice.distribution_eh {
               continue;
            }
            /// We have the parcel we just need to deserialize it
            let parcel_entry: Entry = pending_item.into_item(notice.sender.clone())?
               .expect("PendingItem should hold an Entry");
            return Ok(Some(parcel_entry));
         }
         _ => continue,
      }
   }
   /// Not found in Inbox
   /// Try via DM second
   let dm = DeliveryProtocol::ParcelRequest(notice.parcel_summary.reference.entry_address());
   let response = send_dm(notice.sender, dm)?;
   if let DeliveryProtocol::ParcelResponse(entry) = response {
      /// Check entry
      let received_eh = hash_entry(entry.clone())?;
      if received_eh != notice.parcel_summary.reference.entry_address() {
         warn!("The entry the sender sent does not match notice's Parcel EntryHash");
         return Ok(None);
      }
      ///
      return Ok(Some(entry));
   }
   /// TODO: Ask Recipient peers?
   /// Not found
   Ok(None)
}
