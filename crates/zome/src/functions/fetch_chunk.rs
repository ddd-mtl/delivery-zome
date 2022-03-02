use hdk::prelude::*;
//use crate::link_kind::*;
use zome_delivery_types::*;
use crate::send_dm::*;
use crate::dm_protocol::*;
use crate::functions::*;
use zome_utils::*;


/// Zone Function
/// Return EntryHash of ParcelEntry if it has been downloaded
#[hdk_extern]
pub fn fetch_chunk(input: FetchChunkInput) -> ExternResult<bool> {
   std::panic::set_hook(Box::new(my_panic_hook));
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(input.notice_eh.clone())?;
   /// Look for Chunk
   let maybe_chunk = pull_chunk(input.chunk_eh, notice)?;
   if maybe_chunk.is_none() {
      return Ok(false);
   };
   /// Commit Chunk
   let (parcel_chunk, maybe_link) = maybe_chunk.unwrap();
   //let _chunk_eh = hash_entry(parcel_chunk.clone())?;
   let _chunk_hh = create_entry(parcel_chunk.clone())?;
   /// Delete Link
   if let Some(link) = maybe_link {
      let _hh = delete_link(link.create_link_hash)?;
   }
   /// Done
   Ok(true)
}

/// Try to retrieve the chunk entry
pub fn pull_chunk(chunk_eh: EntryHash, notice: DeliveryNotice)
   -> ExternResult<Option<(ParcelChunk, Option<Link>)>>
{
   /// Check Inbox first:
   /// Get all Items in inbox and see if its there
   if notice.parcel_summary.distribution_strategy.can_dht() {
      let pending_chunk_pairs = get_all_inbox_items(Some(ItemKind::ParcelChunk))?;
      /// Check each Inbox link
      for (pending_chunk, link) in &pending_chunk_pairs {
         assert!(pending_chunk.kind == ItemKind::ParcelChunk);
         if pending_chunk.distribution_eh != notice.distribution_eh {
            continue;
         }
         /// We have the chunk we are looking for, we just need to deserialize it
         let chunk = unpack_item(pending_chunk.clone(), notice.sender.clone())?
            .expect("PendingItem should hold a ParcelChunk");
         //let chunk = get_typed_from_entry(item)?;
         return Ok(Some((chunk, Some(link.clone()))));
      }
   }
   /// Not found in Inbox
   /// Try via DM second
   if notice.parcel_summary.distribution_strategy.can_dm() {
      let dm = DeliveryProtocol::ChunkRequest(chunk_eh.clone());
      let response = send_dm(notice.sender, dm)?;
      if let DeliveryProtocol::ChunkResponse(chunk) = response {
         return Ok(Some((chunk, None)));
      }
   }
   /// TODO: Ask Recipient peers?
   /// Not found
   Ok(None)
}
