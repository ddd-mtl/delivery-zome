use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;

use crate::send_dm::*;
use crate::dm_protocol::*;
use crate::functions::*;
use crate::utils_parcel::*;


/// Zome Function Callback required by Delivery-zome
#[hdk_extern]
pub(crate) fn commit_ParcelReceived(input: ParcelReceived) -> ExternResult<EntryHash> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   let eh = hash_entry(input.clone())?;
   let _hh = create_entry(input)?;
   return Ok(eh);
}


/// Zone Function
/// Can only be called via remote call to self
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
fn fetch_parcel(notice_eh: EntryHash) -> ExternResult<Option<EntryHash>> {
   debug!("fetch_parcel() {:?}", notice_eh);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(notice_eh.clone())?;
   /// Look for Parcel
   let maybe_parcel = pull_parcel(notice.clone())?;
   if maybe_parcel.is_none() {
      return Ok(None);
   };
   let (parcel, maybe_link) = maybe_parcel.unwrap();
   debug!("fetch_parcel() Parcel found.");
   let parcel_eh = hash_entry(parcel.clone())?;
   debug!("fetch_parcel() parcel_eh = {:?}", parcel_eh);
   /// Commit parcel
   let parcel_hh = call_commit_parcel(
      parcel,
      &notice,
      maybe_link.map(|x|x.create_link_hash),
   )?;
   debug!("fetch_parcel() parcel_hh = {:?}", parcel_hh);
   /// Create ParcelReceived if its an AppEntry
   /// (for a Manifest, we have to wait for all chunks to be received)
   if let ParcelReference::AppEntry(..) = notice.summary.parcel_reference {
      let received = ParcelReceived {
         notice_eh,
         parcel_eh: parcel_eh.clone(),
      };
      let response = call_self("commit_ParcelReceived", received)?;
      let received_eh: EntryHash = decode_response(response)?;
      debug!("fetch_parcel() received_eh = {:?}", received_eh);
      // let _hh = create_entry(received)?;
   }
   /// Done
   Ok(Some(parcel_eh))
}


/// Try to retrieve the parcel entry
pub fn pull_parcel(notice: DeliveryNotice) -> ExternResult<Option<(Entry, Option<Link>)>> {
   debug!("pull_parcel() {:?}", notice.summary.parcel_reference.entry_address());
   /// Request Parcel
   /// Check Inbox first
   if notice.summary.distribution_strategy.can_dht() {
      let pending_parcel_pairs = get_all_inbox_items(Some(ItemKind::AppEntryBytes))?;
      /// Check each Inbox link
      for (pending_parcel, link) in &pending_parcel_pairs {
         assert!(pending_parcel.kind == ItemKind::AppEntryBytes);
         if pending_parcel.distribution_eh != notice.distribution_eh {
            continue;
         }
         /// We have the parcel we just need to deserialize it
         let parcel_entry: Entry = unpack_entry(pending_parcel.clone(), notice.sender.clone())?
            .expect("PendingItem should hold an Entry");
         return Ok(Some((parcel_entry, Some(link.clone()))));
      }
   }
   /// Not found in Inbox
   /// Try via DM second
   if notice.summary.distribution_strategy.can_dm() {
      let dm = DeliveryProtocol::ParcelRequest(notice.distribution_eh);
      let response = send_dm(notice.sender, dm)?;
      debug!("pull_parcel() dm response: {:?}", response);
      if let DeliveryProtocol::ParcelResponse(entry) = response {
         /// Check entry
         let received_eh = hash_entry(entry.clone())?;
         if received_eh != notice.summary.parcel_reference.entry_address() {
            warn!("The entry the sender sent does not match notice's Parcel EntryHash");
            return Ok(None);
         }
         ///
         return Ok(Some((entry, None)));
      }
   }
   /// TODO: Ask Recipient peers?
   /// Not found
   Ok(None)
}
