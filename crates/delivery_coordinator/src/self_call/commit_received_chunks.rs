use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_integrity::*;
use zome_delivery_types::*;

/// Internal Zome function
#[hdk_extern]
fn commit_received_chunks(chunks: Vec<(ParcelChunk, Option<Link>)>) -> ExternResult<()> {
   debug!("commit_received_chunks() len = {}", chunks.len());
   /// Check if chunk not already committed
   let mut chunk_ehs: Vec<EntryHash> = chunks.iter().map(|pair| {
      hash_entry(pair.0.clone()).unwrap()
   } ).collect();
   let set: HashSet<_> = chunk_ehs.drain(..).collect(); // dedup
   let query_args = ChainQueryFilter::default()
      .include_entries(false)
      .entry_hashes(set)
      ;
   let locals = query(query_args)?;
   let local_ehs: Vec<EntryHash> = locals.iter()
                                         .map(|x| x.action().entry_hash().unwrap().to_owned())
                                         .collect();
   for (chunky, maybe_link) in chunks.into_iter() {
      let chunk: ParcelChunk = chunky;
      /// Skip local
      let chunk_eh = hash_entry(chunk.clone())?;
      if local_ehs.contains(&chunk_eh) {
         continue;
      }
      /// Commit Parcel
      let _ = create_entry_relaxed(DeliveryEntry::ParcelChunk(chunk))?;

      /// Delete Link
      if let Some(link) = maybe_link {
         let _ = delete_link(link.create_link_hash)?;
      }
   }
   /// Done
   Ok(())
}