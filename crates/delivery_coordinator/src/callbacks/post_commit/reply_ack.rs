use hdk::prelude::*;
use zome_delivery_types::*;
//use zome_utils::*;
use crate::*;


///
pub fn post_commit_ReplyAck(sah: &SignedActionHashed, entry: Entry, eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_ReplyAck() {:?}", eh);
   let reply_ack = ReplyAck::try_from(entry)?;
   /// Check signature
   // FIXME
   //let valid = verify_signature(reply.recipient, reply.recipient_signature, )?;
   /// Emit Signal
   let res = emit_signal(&SignalProtocol::NewReplyAck((eh.to_owned(), sah.hashed.content.timestamp(), reply_ack.clone())));
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Send it immediately if it has been accepted
   if reply_ack.has_accepted {
      //FIXME: let _ = send_parcel(reply)?;
   } else {
      info!("Delivery {} refused by {}", reply_ack.distribution_ah, reply_ack.recipient);
   }
   /// Done
   Ok(())
}


// /// Send Parcel
// fn send_parcel(reply: ReplyAck) -> ExternResult<()> {
//    /// Get Distribution
//    let distribution: Distribution = get_typed_from_eh(reply.distribution_eh.clone())?;
//    /// - Send Chunks if Manifest
//    if let ParcelReference::Manifest(mref) = distribution.delivery_summary.parcel_reference.clone() {
//       /// Get manifest
//       let manifest: ParcelManifest = get_typed_from_eh(mref.manifest_eh.clone())?;
//       /// pack each chunk
//       for chunk_eh in manifest.chunks {
//          /// Get chunk
//          let chunk: ParcelChunk = get_typed_from_eh(chunk_eh.clone())?;
//          /// Create PendingItem
//          let pending_item = pack_chunk(
//             chunk,
//             reply.distribution_eh.clone(),
//             reply.recipient.clone(),
//          )?;
//          /// Send it to recipient
//          debug!("Delivery accepted ; sending chunk {}", chunk_eh);
//          let _ = send_item(
//             reply.recipient.clone(),
//             pending_item,
//             distribution.delivery_summary.distribution_strategy.clone(),
//          )?;
//       }
//    }
//
//    /// - Send AppEntry Parcel Entry or ParcelManifest
//    debug!("Delivery accepted ; sending item {:?}", distribution.delivery_summary.parcel_reference);
//    /// Get Entry
//    let record = get_local_from_eh(distribution.delivery_summary.parcel_reference.entry_address().clone())?;
//    let entry = record.entry().clone().into_option().unwrap();
//    /// Create PendingItem
//    let pending_item = pack_entry(
//       entry,
//       reply.distribution_eh.clone(),
//       reply.recipient.clone(),
//    )?;
//    /// Send it to recipient
//    let success = send_item(
//       reply.recipient,
//       pending_item,
//       distribution.delivery_summary.distribution_strategy,
//    )?;
//    debug!("Delivery accepted ; sending result: {:?}", success);
//    /// Done
//    Ok(())
// }

