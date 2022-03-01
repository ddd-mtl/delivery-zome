use hdk::prelude::*;
use zome_delivery_types::*;
use crate::zome_entry_trait::*;
use zome_utils::*;
use crate::send_item::*;
use crate::functions::*;

impl ZomeEntry for ReplyReceived {
   ///
   fn post_commit(&self, reply_eh: &EntryHash) -> ExternResult<()> {
      debug!("post_commit_ReplyReceived() {:?}", reply_eh);
      /// Get ReplyReceived
      let reply: ReplyReceived = get_typed_from_eh(reply_eh.clone())?;
      /// Check signature
      // FIXME
      /// Bail if delivery refused
      if !reply.has_accepted {
         info!("Delivery {} refused by {}", reply.distribution_eh, reply.recipient);
         return Ok(());
      }
      /// - Send Parcel
      /// Get Distribution
      let distribution: Distribution = get_typed_from_eh(reply.distribution_eh.clone())?;
      /// - Send Chunks if Manifest
      if let ParcelReference::Manifest(manifest_eh) = distribution.parcel_summary.reference.clone() {
         /// Get manifest
         let manifest: ParcelManifest = get_typed_from_eh(manifest_eh.clone())?;
         /// pack each chunk
         for chunk_eh in manifest.chunks {
            /// Get chunk
            let chunk: ParcelChunk = get_typed_from_eh(chunk_eh.clone())?;
            /// Create PendingItem
            let pending_item = pack_chunk(
               chunk,
               reply.distribution_eh.clone(),
               reply.recipient.clone(),
            )?;
            /// Send it to recipient
            let _ = send_item(
               reply.recipient.clone(),
               pending_item,
               distribution.parcel_summary.distribution_strategy.clone(),
            )?;
         }
      }
      /// - Send AppEntry Parcel
      /// Get Entry
      let el: Element = get_local_from_eh(distribution.parcel_summary.reference.entry_address().clone())?;
      let entry: Entry = el.entry().clone().into_option().unwrap();
      /// Create PendingItem
      let pending_item = pack_parcel(
         entry,
         reply.distribution_eh.clone(),
         reply.recipient.clone(),
      )?;
      /// Send it to recipient
      let _ = send_item(
         reply.recipient,
         pending_item,
         distribution.parcel_summary.distribution_strategy,
      )?;
      /// Done
      Ok(())
   }
}
