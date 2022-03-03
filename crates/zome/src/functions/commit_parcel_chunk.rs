use hdk::prelude::*;
use zome_delivery_types::*;
use zome_utils::*;

use crate::constants::CHUNK_MAX_SIZE;


/// Zome function
/// Write base64 file as string to source chain
/// Return EntryHash of newly created ParcelChunk
#[hdk_extern]
pub fn commit_parcel_chunk(data: String) -> ExternResult<EntryHash> {
   trace!(" commit_parcel_chunk() {} bytes", data.len());
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Check size
   if data.is_empty() {
      return error("Data string is empty");
   }
   if data.len() > CHUNK_MAX_SIZE {
      return error(&format!("Data string is too high: {} > {}", data.len(), CHUNK_MAX_SIZE));
   }
   /// Commit entry
   let parcel_chunk = ParcelChunk {data};
   let chunk_eh = hash_entry(parcel_chunk.clone())?;
   let _chunk_hh = create_entry(&parcel_chunk)?;
   /// Done
   Ok(chunk_eh)
}
