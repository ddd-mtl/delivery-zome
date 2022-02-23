use hdk::prelude::*;
use crate::link_kind::*;
use delivery_zome_api::{entries::*, entry_kind::*, parcel::*, utils::*};

use crate::send_dm::*;
use crate::dm_protocol::*;
use crate::functions::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FetchChunkInput {
   chunk_eh: EntryHash,
   notice_eh: EntryHash,
}

/// Zone Function
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
pub fn fetch_chunk(input: FetchChunkInput) -> ExternResult<Option<EntryHash>> {
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(input.notice_eh.clone())?;
   /// Look for Chunk
   let maybe_chunk = pull_chunk(input.chunk_eh, notice)?;
   if maybe_chunk.is_none() {
      return Ok(None);
   };
   /// Commit Chunk
   let parcel_chunk = maybe_chunk.unwrap();
   let chunk_eh = hash_entry(parcel_chunk.clone())?;
   let _chunk_hh = create_entry(parcel_chunk.clone())?;
   /// Done
   Ok(Some(chunk_eh))
}

/// Try to retrieve the chunk entry
pub fn pull_chunk(chunk_eh: EntryHash, notice: DeliveryNotice) -> ExternResult<Option<ParcelChunk>> {
   /// Check Inbox first:
   /// Get all Items in inbox and see if its there
   let me = agent_info()?.agent_latest_pubkey;
   let my_agent_eh = EntryHash::from(me.clone());
   let pending_items = get_links_and_load_type::<PendingItem>(
      my_agent_eh.clone(),
      LinkKind::Inbox.as_tag_opt(),
      false,
   )?;
   /// Check each Inbox link
   for pending_item in &pending_items {
      match pending_item.kind {
         ItemKind::ParcelChunk => {
            if pending_item.distribution_eh != notice.distribution_eh {
               continue;
            }
            /// We have the chunk we just need to deserialize it
            let item: Entry = unpack_item(pending_item.clone(), notice.sender.clone())?
               .expect("PendingItem should hold an Entry");
            let chunk = get_typed_from_entry(item)?;
            return Ok(Some(chunk));
         }
         _ => continue,
      }
   }
   /// Not found in Inbox
   /// Try via DM second
   let dm =  DeliveryProtocol::ChunkRequest(chunk_eh.clone());
   let response = send_dm(notice.sender, dm)?;
   if let DeliveryProtocol::ChunkResponse(chunk) = response {
      return Ok(Some(chunk));
   }
   /// TODO: Ask Recipient peers?
   /// Not found
   Ok(None)
}
