mod distribution;
mod notice_reply;
mod private_chunk;
mod private_manifest;
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
pub use private_chunk::*;
pub use private_manifest::*;
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
use zome_delivery_types::{SystemSignalProtocol};
use crate::{emit_system_signal};


/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedActionList: Vec<SignedActionHashed>) {
   debug!("DELIVERY post_commit() called for {} actions. ({})", signedActionList.len(), zome_info().unwrap().id);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Process each Action
   for sah in signedActionList {
      //debug!(" - {}", sah.action());
      match sah.action() {
         ///
         Action::DeleteLink(delete_link) => {
            let Ok(Some(record)) = get(delete_link.link_add_address.clone(), GetOptions::local())
              else { error!("Failed to get CreateLink action"); continue };
            let Action::CreateLink(create_link) = record.action()
              else { error!("Record should be a CreateLink"); continue };
            let Ok(Some(link_type)) = LinkTypes::from_type(create_link.zome_index, create_link.link_type)
              else { error!("CreateLink should have a LinkType"); continue };
            match link_type {
               LinkTypes::PublicParcels => gossip_public_parcel(create_link, sah.hashed.content.timestamp(), false),
               _ => (),
            }
         },
         ///
         Action::CreateLink(create_link) => {
            let Ok(Some(link_type)) = LinkTypes::from_type(create_link.zome_index, create_link.link_type)
              else { error!("CreateLink should have a LinkType. Could be a Link from a different zome: {} ({})", create_link.link_type.0, create_link.zome_index); continue };
            match link_type {
               LinkTypes::PublicParcels => gossip_public_parcel(create_link, sah.hashed.content.timestamp(), true),
               _ => (),
            }
         },
         ///
         Action::Delete(_delete) => {},
         Action::Update(_update) => {},
         ///
         Action::Create(create) => {
            let EntryType::App(app_entry_def) = &create.entry_type
              else { continue };
            let variant_name = format!("{:?}", entry_index_to_variant(app_entry_def.entry_index).unwrap());
            let _ = emit_system_signal(SystemSignalProtocol::PostCommitStart { entry_type: variant_name.clone() });
            let result = post_commit_create_app_entry(&sah, &create.entry_hash, &app_entry_def);
            let _ = emit_system_signal(SystemSignalProtocol::PostCommitEnd { entry_type: variant_name, succeeded: result.is_ok() });
            if let Err(e) = result {
               error!("<< post_commit() failed: {:?}", e);
            } else {
               debug!("<< post_commit() SUCCEEDED");
            }
         },
         _ => (),
      }
   }
}


///
fn post_commit_create_app_entry(sah: &SignedActionHashed, eh: &EntryHash, app_entry_def: &AppEntryDef) -> ExternResult<()> {
   debug!(">> post_commit_create_app_entry() called for a {:?}", app_entry_def);
   let variant = entry_index_to_variant(app_entry_def.entry_index)?;

   /// Get Entry from local chain
   // let monad: HashSet<EntryHash> = HashSet::from([eh.clone()]);
   // let query_args = ChainQueryFilter::default()
   //    .include_entries(true)
   //    .entry_hashes(monad);
   // let records = query(query_args)?;
   // if records.is_empty() {
   //    return zome_error!("Post committed entry not found on chain");
   // }
   //
   let entry = must_get_entry(eh.clone())?.content;

   debug!("post_commit_create_app_entry() entry found");
   //let entry = records[0].entry().as_option().unwrap().to_owned();
   /// Deserialize it and call its post_commit()
   let Entry::App(ref entry_bytes) = entry
      else {
         return zome_error!("EntryHash has already been filtered as an App type");
   };

   // let entry_kind = EntryKind::from_index(&app_entry_def.id());

   // let delivery_zome_entry = entry_kind.into_zome_entry(entry_bytes.clone())?;


   match variant {
      /// Send/Receive/Ack Notice
      DeliveryEntryTypes::Distribution => post_commit_Distribution(sah, entry, eh),
      DeliveryEntryTypes::DeliveryNotice => post_commit_DeliveryNotice(sah, entry, eh),
      DeliveryEntryTypes::NoticeAck => post_commit_NoticeAck(sah, entry, eh),
      /// Send/Receive Reply
      DeliveryEntryTypes::NoticeReply => post_commit_NoticeReply(sah, entry, eh),
      DeliveryEntryTypes::ReplyAck => post_commit_ReplyAck(sah, entry, eh),
      /// Send/Receive Parcel
      DeliveryEntryTypes::PrivateChunk => post_commit_PrivateChunk(sah, entry, eh),
      DeliveryEntryTypes::PrivateManifest => post_commit_PrivateManifest(sah, entry, eh),
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
