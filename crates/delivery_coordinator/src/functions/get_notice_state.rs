use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;


///
#[hdk_extern]
pub fn get_notice_state(notice_eh: EntryHash) -> ExternResult<(NoticeState, Vec<EntryHash>)> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   //debug!("START");
   /// Make sure EntryHash is correct
   let notice: DeliveryNotice = get_typed_from_eh(notice_eh.clone())?;
   /// look for reply
   let maybe_reply = query_NoticeReply(notice_eh.clone())?;
   if maybe_reply.is_none() {
      return Ok((NoticeState::Unreplied, vec![]));
   }
   if !maybe_reply.unwrap().has_accepted {
      return Ok((NoticeState::Refused, vec![]));
   }
   /// Look for parcel
   //let notice: DeliveryNotice = get_typed_from_eh(notice_eh)?;
   //let has_parcel = has_parcel(notice.summary.parcel_reference)?;
   let maybe_parcel = query_ReceptionProof(ReceptionProofQueryField::Notice(notice_eh.clone()))?;
   /// Done
   if maybe_parcel.is_some() {
      return Ok((NoticeState::Received, vec![]));
   }
   /// If its a manifest, see if we have it and how many chunks
   if let ParcelKind::Manifest(_) = notice.summary.parcel_reference.description.kind_info {
      let maybe_manifest = get_typed_from_eh::<ParcelManifest>(notice.summary.parcel_reference.eh.clone());
      if maybe_manifest.is_err() {
         return Ok((NoticeState::Accepted, vec![]));
      }

      let missing_chunks = determine_missing_chunks(notice.summary.parcel_reference.eh)?;
      return Ok((NoticeState::PartiallyReceived, missing_chunks));
   }
   /// Done
   Ok((NoticeState::Accepted, vec![]))
}
