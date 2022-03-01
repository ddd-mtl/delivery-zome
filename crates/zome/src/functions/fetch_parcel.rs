use hdk::prelude::*;

use zome_delivery_types::*;
use crate::send_dm::*;
use crate::dm_protocol::*;
use crate::link_kind::*;
use crate::functions::*;
use zome_utils::*;

/// Zome Function Callback required by Delivery-zome
#[hdk_extern]
pub(crate) fn commit_ParcelReceived(input: ParcelReceived) -> ExternResult<EntryHash> {
   std::panic::set_hook(Box::new(my_panic_hook));
   let eh = hash_entry(input.clone())?;
   let _hh = create_entry(input)?;
   return Ok(eh);
}

/// Zone Function
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
pub fn fetch_parcel(notice_eh: EntryHash) -> ExternResult<Option<EntryHash>> {
   debug!("fetch_parcel() {:?}", notice_eh);
   std::panic::set_hook(Box::new(my_panic_hook));
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(notice_eh.clone())?;
   /// Look for Parcel
   let maybe_parcel = pull_parcel(notice.clone())?;
   if maybe_parcel.is_none() {
      return Ok(None);
   };
   
   /// Call The Zome owner of the entry to commit it
   let input = CommitParcelInput {
      entry_def_id: notice.parcel_summary.reference.entry_def_id(),
      entry: maybe_parcel.unwrap(),
   };
   let zome_name = notice.parcel_summary.reference.entry_zome_name();
   debug!("fetch_parcel() zome_name = {:?}", zome_name);
   let response = call_remote(
      agent_info()?.agent_latest_pubkey,
      zome_name,
      COMMIT_PARCEL_CALLBACK.into(),
      None,
      input.clone(),
   )?;
   let parcel_hh: EntryHash = decode_response(response)?;
   debug!("fetch_parcel() parcel_hh = {:?}", parcel_hh);
   let parcel_eh = hash_entry(input.entry)?;

   /// Create ParcelReceived if its an AppEntry
   /// (for a Manifest, we have to wait for all chunks to be received)
   if let ParcelReference::AppEntry(..) = notice.parcel_summary.reference {
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
pub fn pull_parcel(notice: DeliveryNotice) -> ExternResult<Option<Entry>> {
   debug!("pull_parcel() {:?}", notice.parcel_summary.reference.entry_address());
   /// Request Parcel
   /// Check Inbox first:
   if notice.parcel_summary.distribution_strategy.can_dht() {
      /// Get all Parcels inbox and see if its there
      let me = agent_info()?.agent_latest_pubkey;
      let my_agent_eh = EntryHash::from(me.clone());
      let pending_items = get_links_and_load_type::<PendingItem>(
         my_agent_eh.clone(),
         LinkKind::Inbox.as_tag_opt(),
         //false,
      )?;
      /// Check each Inbox link
      for pending_item in &pending_items {
         match pending_item.kind {
            ItemKind::AppEntryBytes => {
               if pending_item.distribution_eh != notice.distribution_eh {
                  continue;
               }
               /// We have the parcel we just need to deserialize it
               let parcel_entry: Entry = unpack_entry(pending_item.clone(), notice.sender.clone())?
                  .expect("PendingItem should hold an Entry");
               return Ok(Some(parcel_entry));
            }
            _ => continue,
         }
      }
   }
   /// Not found in Inbox
   /// Try via DM second
   if notice.parcel_summary.distribution_strategy.can_dm() {
      let dm = DeliveryProtocol::ParcelRequest(notice.distribution_eh);
      let response = send_dm(notice.sender, dm)?;
      debug!("pull_parcel() dm response: {}", response);
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
   }
   /// TODO: Ask Recipient peers?
   /// Not found
   Ok(None)
}
