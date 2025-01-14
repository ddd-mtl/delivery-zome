mod distribution;
mod delivery_reply;
mod parcel_chunk;
mod parcel_manifest;
mod parcel_received;
mod reply_received;
mod delivery_notice;
mod notice_received;
mod delivery_receipt;


pub use delivery_notice::*;
pub use distribution::*;
pub use delivery_reply::*;
pub use parcel_chunk::*;
pub use parcel_manifest::*;
pub use parcel_received::*;
pub use reply_received::*;
pub use notice_received::*;
pub use delivery_receipt::*;

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
   for signedAction in signedActionList {
      debug!(" - {:?}", signedAction.action().entry_type());
      let action = signedAction.action();
      if action.entry_type().is_none() {
         continue;
      }
      let (eh, entry_type) = action.entry_data().unwrap();
      match entry_type {
         EntryType::AgentPubKey => {},
         EntryType::CapClaim => {},
         EntryType::CapGrant => {},
         EntryType::App(app_entry_def) => {
            let result = post_commit_app_entry(eh, app_entry_def);
            debug!(" << post_commit() result = {:?}", result);
         },
      }
   }
}


///
fn post_commit_app_entry(eh: &EntryHash, app_entry_def: &AppEntryDef) -> ExternResult<()> {
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
      DeliveryEntryTypes::Distribution => post_commit_Distribution(entry, eh),
      DeliveryEntryTypes::DeliveryNotice => post_commit_DeliveryNotice(entry, eh),
      DeliveryEntryTypes::NoticeReceived => post_commit_NoticeReceived(entry, eh),
      /// Send/Receive Reply
      DeliveryEntryTypes::DeliveryReply => post_commit_DeliveryReply(entry, eh),
      DeliveryEntryTypes::ReplyReceived => post_commit_ReplyReceived(entry, eh),
      /// Send/Receive Parcel
      DeliveryEntryTypes::ParcelChunk => post_commit_ParcelChunk(entry, eh),
      DeliveryEntryTypes::ParcelManifest => post_commit_ParcelManifest(entry, eh),
      /// Send/Receive Receipt
      DeliveryEntryTypes::ParcelReceived => post_commit_ParcelReceived(entry, eh),
      DeliveryEntryTypes::DeliveryReceipt => post_commit_DeliveryReceipt(entry, eh),
      ///
      _ => Ok(()),
   }
}
