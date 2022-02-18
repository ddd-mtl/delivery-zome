use hdk::prelude::*;
use crate::constants::CHUNK_MAX_SIZE;
use crate::EntryKind::ParcelChunk;


/// Zome function
/// Write base64 file as string to source chain
/// Return EntryHash of newly created ParcelChunk
#[hdk_extern]
#[snapmail_api]
pub fn commit_parcel_chunk(data: String) -> ExternResult<EntryHash> {
   trace!(" commit_parcel_chunk() {} bytes", data.len());
   /// Check size
   if data.is_empty() {
      return error("Data string is empty");
   }
   if data.len() > CHUNK_MAX_SIZE {
      return error(&format!("Data string is too high: {} > {}", data.len(), CHUNK_MAX_SIZE));
   }
   /// Commit entry
   let parcel_chunk = ParcelChunk {data};
   let chunk_eh = hash_entry(parcel_chunk)?;
   let _chunk_hh = create_entry(&parcel_chunk)?;
   /// Done
   Ok(chunk_eh)
}
