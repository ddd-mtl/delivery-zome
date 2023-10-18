use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;


/// Ask for every missing chunk
#[hdk_extern]
pub fn request_missing_chunks(manifest_eh: EntryHash) -> ExternResult<()> {
   debug!("START - {:?}", manifest_eh);
   let manifest = get_typed_from_eh::<ParcelManifest>(manifest_eh.clone())?;
   /// Find chunks
   let missing_chunks = check_manifest_integrity(manifest_eh.clone(), manifest)?;
   if missing_chunks.is_empty() {
      return Ok(());
   }
   debug!("missing_chunks: {}", missing_chunks.len());
   /// Private: Find notice

   let notices = query_DeliveryNotice(DeliveryNoticeQueryField::Parcel(manifest_eh.clone()))?;
   if notices.is_empty() {
      debug!("No Notice found for post-committed ParcelManifest");
      /// Normal if agent is original creator of ParcelManifest
      return Ok(())
   }
   let notice = notices[0].0.clone();
   // if !notice.summary.distribution_strategy.can_dm() {
   //    return error("Not allowed to receive this Parcel via DM");
   // }
   let notice_eh = hash_entry(notice.clone())?;
   debug!("notice_eh: {}", notice_eh);
   ///
   for chunk_eh in missing_chunks.clone() {
      let dm = DeliveryProtocol::ChunkRequest(chunk_eh.clone());
      send_dm_signal(notice.sender.clone(), dm)?;
   }
   debug!("END");
   /// Done
   Ok(())
}
