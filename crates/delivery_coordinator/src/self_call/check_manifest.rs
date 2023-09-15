use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;

use zome_delivery_integrity::*;
use crate::*;


/// Self called Zome Function (Careful when renaming this function)
/// Check if all chunks have been committed for this parcel.
/// Return EntryHash of Notice if Manifest is received from remote agent.
/// Return EntryHash of ReceptionProof if it has been completly downloaded.
/// Otherwise return download completion percentage.
#[ignore(zits)]
#[hdk_extern]
pub fn check_manifest(chunk_eh: EntryHash) -> ExternResult<Option<Vec<(EntryHash, Result<EntryHash, usize>)>>> {
   debug!("START {}", chunk_eh);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Find manifest with that chunk_eh
   let maybe_manifest = find_ParcelManifest(chunk_eh)?;
   if maybe_manifest.is_none() {
      debug!("ABORT - Manifest not found");
      return Ok(None);
      //return error("Manifest not found for chunk");
   }
   /// Find notice with that manifest
   let manifest_eh = hash_entry(maybe_manifest.unwrap())?;
   let notices = find_notice_with_parcel(manifest_eh.clone())?;
   if notices.is_empty() {
      debug!("ABORT - Notice not found for manifest {}", manifest_eh);
      /// Excepted if agent is original creator of ParcelManifest
      return Ok(None);
   }
   /// Matching notice(s) found. Check if we have all chunks
   let received_pct = count_chunks_received(manifest_eh.clone())?;
   debug!("received_pct = {}", received_pct);
   if received_pct != 100 {
      debug!("ABORT - Missing chunks");
      let vec = notices.into_iter().map(|notice| {
         let notice_eh = hash_entry(notice).unwrap();
         (notice_eh, Err(received_pct))
      }).collect();
      return Ok(Some(vec));
   }
   /// All chunks found.
   /// Create ReceptionProof for each Notice
   let mut res = Vec::new();
   for notice in notices {
      let notice_eh = hash_entry(notice)?;
      /// Must not already have a ReceptionProof
      let maybe_reception_proof = query_ReceptionProof(ReceptionProofQueryField::Notice(notice_eh.clone()))?;
      if let Some((_reception_eh, reception_proof)) = maybe_reception_proof {
         debug!("ReceptionProof found for notice: {:?}", notice_eh);
         let reception_proof_eh = hash_entry(reception_proof.clone())?;
         res.push((notice_eh, Ok(reception_proof_eh)));
         //return Ok(Some((notice_eh, Ok(reception.parcel_eh))));
         continue;
      }
      let reception_proof = ReceptionProof {
         notice_eh: notice_eh.clone(),
         parcel_eh: manifest_eh.clone(),
      };
      let reception_proof_eh = hash_entry(reception_proof.clone())?;
      let _ah = create_entry_relaxed(DeliveryEntry::ReceptionProof(reception_proof.clone()))?;
      res.push((notice_eh, Ok(reception_proof_eh)));
   }
   /// Done
   Ok(Some(res))
}


///Find manifest with that chunk_eh
pub fn find_ParcelManifest(chunk_eh: EntryHash) -> ExternResult<Option<ParcelManifest>> {
   /// Get all Create ParcelManifest Elements with query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .action_type(ActionType::Create)
      .entry_type(DeliveryEntryTypes::ParcelManifest.try_into().unwrap());
   let manifests = query(query_args)?;
   for manifest_record in manifests {
      let manifest: ParcelManifest = get_typed_from_record(manifest_record)?;
      if manifest.chunks.contains(&chunk_eh) {
         return Ok(Some(manifest))
      }
   }
   /// Done
   Ok(None)
}

