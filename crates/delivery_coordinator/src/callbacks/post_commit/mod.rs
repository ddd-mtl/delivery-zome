mod distribution;
mod notice_reply;
mod parcel_chunk;
mod parcel_manifest;
mod reception_proof;
mod reply_ack;
mod delivery_notice;
mod notice_ack;
mod reception_ack;
mod pending_item;
mod public_chunk;
mod public_manifest;
mod public_parcel;


pub use delivery_notice::*;
pub use distribution::*;
pub use notice_reply::*;
pub use parcel_chunk::*;
pub use parcel_manifest::*;
pub use reception_proof::*;
pub use reply_ack::*;
pub use notice_ack::*;
pub use reception_ack::*;
pub use pending_item::*;
pub use public_chunk::*;
pub use public_manifest::*;
pub use public_parcel::*;

//----------------------------------------------------------------------------------------

use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::*;


/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedActionList: Vec<SignedActionHashed>) {
   //debug!("DELIVERY post_commit() called for {} actions", signedActionList.len());
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Process each Action
   for sah in signedActionList {
      debug!(" - {:?}", sah.action().entry_type());
      let action = sah.action();
      if action.entry_type().is_none() {
         continue;
      }
      let (eh, entry_type) = action.entry_data().unwrap();
      match entry_type {
         EntryType::AgentPubKey => {},
         EntryType::CapClaim => {},
         EntryType::CapGrant => {},
         EntryType::App(app_entry_def) => {
            let result = post_commit_app_entry(&sah, eh, app_entry_def);
            debug!(" << post_commit() result = {:?}", result);
         },
      }
   }
}


///
fn post_commit_app_entry(sah: &SignedActionHashed, eh: &EntryHash, app_entry_def: &AppEntryDef) -> ExternResult<()> {
   debug!(" >> post_commit() called for a {:?}", app_entry_def);
   /// Get Entry from local chain
   let monad: HashSet<EntryHash> = HashSet::from([eh.clone()]);
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .entry_hashes(monad);
   let records = query(query_args)?;
   if records.is_empty() {
      return zome_error!("Post committed entry not found on chain");
   }
   let entry = records[0].entry().as_option().unwrap().to_owned();
   /// Deserialize it and call its post_commit()
   let Entry::App(ref entry_bytes) = entry
      else {
         return zome_error!("EntryHash has already been filtered as an App type");
   };

   // let entry_kind = EntryKind::from_index(&app_entry_def.id());

   // let delivery_zome_entry = entry_kind.into_zome_entry(entry_bytes.clone())?;
   let variant = entry_index_to_variant(app_entry_def.entry_index)?;
   match variant {
      /// Send/Receive/Ack Notice
      DeliveryEntryTypes::Distribution => post_commit_Distribution(sah, entry, eh),
      DeliveryEntryTypes::DeliveryNotice => post_commit_DeliveryNotice(sah, entry, eh),
      DeliveryEntryTypes::NoticeAck => post_commit_NoticeAck(sah, entry, eh),
      /// Send/Receive Reply
      DeliveryEntryTypes::NoticeReply => post_commit_NoticeReply(sah, entry, eh),
      DeliveryEntryTypes::ReplyAck => post_commit_ReplyAck(sah, entry, eh),
      /// Send/Receive Parcel
      DeliveryEntryTypes::ParcelChunk => post_commit_ParcelChunk(sah, entry, eh),
      DeliveryEntryTypes::ParcelManifest => post_commit_ParcelManifest(sah, entry, eh),
      /// Send/Receive ReceptionProof
      DeliveryEntryTypes::ReceptionProof => post_commit_ReceptionProof(sah, entry, eh),
      DeliveryEntryTypes::ReceptionAck => post_commit_ReceptionAck(sah, entry, eh),
      /// Public entries
      DeliveryEntryTypes::PendingItem => post_commit_PendingItem(sah, entry, eh),
      DeliveryEntryTypes::PublicChunk => post_commit_PublicChunk(sah, entry, eh),
      DeliveryEntryTypes::PublicManifest => post_commit_PublicManifest(sah, entry, eh),
      DeliveryEntryTypes::PublicParcel => post_commit_PublicParcel(sah, entry, eh),
      ///
      _ => Ok(()),
   }
}
