mod distribution;
mod notice_reply;
mod private_manifest;
mod reception_proof;
mod reply_ack;
mod delivery_notice;
mod public_parcel;

pub use delivery_notice::*;
pub use distribution::*;
pub use notice_reply::*;
pub use private_manifest::*;
pub use reception_proof::*;
pub use reply_ack::*;
pub use public_parcel::*;


//----------------------------------------------------------------------------------------

use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::*;
use zome_delivery_types::*;
use crate::*;


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
            debug!("CreateLink: {:?} ({}, {:?})", link_type, create_link.zome_index, create_link.link_type);
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
            /// Emit System Signal
            let variant_name = format!("{:?}", entry_index_to_variant(app_entry_def.entry_index).unwrap());
            let _ = emit_system_signal(SystemSignalProtocol::PostCommitStart { entry_type: variant_name.clone() });
            /// handle post_commit_create()
            let result = post_commit_create_app_entry(&sah, &create, &app_entry_def);
            /// Emit System Signal
            let _ = emit_system_signal(SystemSignalProtocol::PostCommitEnd { entry_type: variant_name, succeeded: result.is_ok() });
            ///
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
fn post_commit_create_app_entry(sah: &SignedActionHashed, create: &Create, app_entry_def: &AppEntryDef) -> ExternResult<()> {
   debug!(">> post_commit_create_app_entry() called for a {:?}", app_entry_def);
   let entry = must_get_entry(create.entry_hash.clone())?.content;
   //debug!("post_commit_create_app_entry() entry found");
   /// Emit Entry Signal
   let variant = entry_index_to_variant(app_entry_def.entry_index)?;
   let kind = match variant {
      /// Send/Receive/Ack Notice
      DeliveryEntryTypes::Distribution => {
         let kind = post_commit_create_Distribution(sah, create, entry)?;
         let signal = entry_signal_ah(EntryStateChange::Created, create, kind, sah.action_address().clone());
         return emit_self_signal(signal);
      },
      DeliveryEntryTypes::DeliveryNotice => post_commit_create_DeliveryNotice(sah, create, entry)?,
      DeliveryEntryTypes::NoticeAck => DeliveryEntryKind::NoticeAck(NoticeAck::try_from(entry)?),
      /// Send/Receive Reply
      DeliveryEntryTypes::NoticeReply => post_commit_create_NoticeReply(sah, create, entry)?,
      DeliveryEntryTypes::ReplyAck => post_commit_create_ReplyAck(sah, create, entry)?,
      /// Send/Receive Parcel
      DeliveryEntryTypes::PrivateChunk => DeliveryEntryKind::ParcelChunk(ParcelChunk::try_from(entry)?),
      DeliveryEntryTypes::PrivateManifest => post_commit_create_PrivateManifest(sah, create, entry)?,
      /// Send/Receive ReceptionProof
      DeliveryEntryTypes::ReceptionProof => post_commit_create_ReceptionProof(sah, create, entry)?,
      DeliveryEntryTypes::ReceptionAck => DeliveryEntryKind::ReceptionAck(ReceptionAck::try_from(entry)?),
      /// Public entries
      DeliveryEntryTypes::PendingItem => DeliveryEntryKind::PendingItem(PendingItem::try_from(entry)?),
      DeliveryEntryTypes::PublicChunk => DeliveryEntryKind::ParcelChunk(ParcelChunk::try_from(entry)?),
      DeliveryEntryTypes::PublicManifest => DeliveryEntryKind::ParcelManifest(ParcelManifest::try_from(entry)?),
      DeliveryEntryTypes::PublicParcel => post_commit_create_PublicParcel(sah, create, entry)?,
   };
   /// Emit Signal
   emit_entry_signal(EntryStateChange::Created, create, kind)?;
   Ok(())
}
