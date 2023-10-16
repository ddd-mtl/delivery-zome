use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;

use crate::*;

//pub type FetchChunkOutput = Option<(ParcelChunk, Option<Link>)>;


/// Return EntryHash of ParcelEntry if it has been downloaded
#[ignore(zits)]
#[hdk_extern]
fn fetch_chunk(input: FetchChunkInput) -> ExternResult<Option<(ParcelChunk, Option<Link>)>> {
   trace!(" fetch_chunk() {:?}", input);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(input.notice_eh.clone())?;
   /// Look for Chunk
   let maybe_chunk = pull_chunk(input.chunk_eh.clone(), notice)?;
   Ok(maybe_chunk)
}


/// Try to retrieve the chunk entry
fn pull_chunk(chunk_eh: EntryHash, notice: DeliveryNotice) -> ExternResult<Option<(ParcelChunk, Option<Link>)>> {
   /// Check Inbox first:
   /// Get all Items in inbox and see if its there
   if notice.summary.distribution_strategy.can_dht() {
      let pending_chunk_pairs = get_all_inbox_items(Some(ItemKind::ParcelChunk))?;
      /// Check each Inbox link
      for (pending_chunk, link) in &pending_chunk_pairs {
         assert!(pending_chunk.kind == ItemKind::ParcelChunk);
         if pending_chunk.distribution_ah != notice.distribution_ah {
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
   if notice.summary.distribution_strategy.can_dm() {
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
