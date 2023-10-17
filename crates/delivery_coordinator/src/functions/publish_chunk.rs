use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;

/// Batch call to publish_chunk()
/// WARN: Make sure input does not exceed websocket packet max size limit.
#[hdk_extern]
pub fn publish_chunks(chunks: Vec<ParcelChunk>) -> ExternResult<Vec<EntryHash>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("chunks count: {}", chunks.len());
   let mut result = Vec::new();
   for chunk in chunks {
      let eh = publish_chunk(chunk)?;
      result.push(eh);
   }
   Ok(result)
}


/// Write base64 data as string to source chain
/// Return EntryHash of newly created ParcelChunk
#[hdk_extern]
pub fn publish_chunk(chunk: ParcelChunk) -> ExternResult<EntryHash> {
   trace!("chunk size: {} bytes", chunk.data.len());
   /// Commit entry
   //let chunk = ParcelChunk {data};
   let chunk_eh = hash_entry(chunk.clone())?;
   let _chunk_ah = create_entry(DeliveryEntry::PublicChunk(chunk))?;
   /// Done
   Ok(chunk_eh)
}