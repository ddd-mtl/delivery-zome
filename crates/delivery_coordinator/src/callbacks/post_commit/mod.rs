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
use zome_signals::*;
use zome_delivery_integrity::*;


///
#[hdk_extern(infallible)]
fn post_commit(signedActionList: Vec<SignedActionHashed>) {
   debug!("DELIVERY post_commit() called for {} actions. ({})", signedActionList.len(), zome_info().unwrap().id);
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Process each Action
   for sah in signedActionList {
      // debug!(" - {}", sah.action());
      let ah = sah_to_ah(sah.clone());
      match sah.action() {
         ///
         Action::DeleteLink(delete_link) => {
            let Ok(Some(record)) = get(delete_link.link_add_address.clone(), GetOptions::local())
              else { error!("Failed to get CreateLink action"); continue };
            let Action::CreateLink(create_link) = record.action()
              else { error!("Record should be a CreateLink"); continue };
            // let Ok(Some(link_type)) = LinkTypes::from_type(create_link.zome_index, create_link.link_type)
            //   else { error!("CreateLink should have a LinkType"); continue };
            // match link_type {
            //    LinkTypes::PublicParcels => self_gossip_public_parcel(create_link, sah.hashed.content.timestamp(), false),
            //    _ => (),
            // }
            let res = emit_link_delete_signal(delete_link, create_link, true);
               if let Err(e) = res {
               error!("Emitting DeleteLink signal failed: {:?}", e);
            }
         },
         ///
         Action::CreateLink(create_link) => {
            let Ok(Some(link_type)) = LinkTypes::from_type(create_link.zome_index, create_link.link_type)
              else { error!("CreateLink should have a LinkType. Could be a Link from a different zome: {} ({})", create_link.link_type.0, create_link.zome_index); continue };
            debug!("CreateLink: {:?} ({}, {:?})", link_type, create_link.zome_index, create_link.link_type);
            // match link_type {
            //    LinkTypes::PublicParcels => self_gossip_public_parcel(create_link, sah.hashed.content.timestamp(), true),
            //    _ => (),
            // }
            let res = emit_link_create_signal(ah, create_link, true);
            if let Err(e) = res {
               error!("Emitting CreateLink signal failed: {:?}", e);
            }
         },
         /// NewEntryAction
         Action::Update(_) |
         Action::Create(_) => {
            let EntryType::App(app_entry_def) = sah.action().entry_type().unwrap()
              else { continue };
            /// Emit System Signal
            let variant = entry_index_to_variant(app_entry_def.entry_index).unwrap();
            let variant_name = format!("{:?}", variant);
            let _ = emit_system_signal(SystemSignalProtocol::PostCommitNewStart { app_entry_type: variant_name.clone() });
            /// handle post_commit_create()
            let result = post_commit_new_app_entry(&sah, variant);
            /// Emit System Signal
            let _ = emit_system_signal(SystemSignalProtocol::PostCommitNewEnd { app_entry_type: variant_name, succeeded: result.is_ok() });
            ///
            if let Err(e) = result {
               error!("<< post_commit() failed: {:?}", e);
            } else {
               debug!("<< post_commit() SUCCEEDED");
            }
         },
         /// DeleteAction
         Action::Delete(delete) => {
            let Ok(new_sah) = must_get_action(delete.deletes_address.clone())
              else { error!("Deleted action not found."); continue; };
            let Ok(he) = must_get_entry(delete.deletes_entry_address.clone())
              else { error!("Deleted entry not found."); continue; };
            let Some(EntryType::App(app_entry_def)) = new_sah.action().entry_type()
              else { error!("Deleted action should have entry_type."); continue; };
            let variant = entry_index_to_variant(app_entry_def.entry_index).unwrap();
            /// Emit System Signal
            let variant_name = format!("{:?}", variant);
            let _ = emit_system_signal(SystemSignalProtocol::PostCommitDeleteStart { app_entry_type: variant_name.clone() });
            /// handle post_commit_delete()
            let result = post_commit_delete_app_entry(delete.clone(), variant, new_sah, he.content);
            /// Emit System Signal
            let _ = emit_system_signal(SystemSignalProtocol::PostCommitDeleteEnd { app_entry_type: variant_name, succeeded: result.is_ok() });
            ///
            if let Err(e) = result {
               error!("<< post_commit() failed: {:?}", e);
            } else {
               debug!("<< post_commit() SUCCEEDED");
            }
         },
         ///
         _ => (),
      }
   }
}


///
fn post_commit_new_app_entry(sah: &SignedActionHashed, variant: DeliveryEntryTypes) -> ExternResult<()> {
   debug!("post_commit_create_app_entry() called for a {:?}", variant);
   let Some(eh) = sah.action().entry_hash() else {
      return zome_error!("Action has no Entry");
   };
   let entry = must_get_entry(eh.to_owned())?.content;
   let record = Record::new(sah.to_owned(), Some(entry.clone()));
   /// Type specific post_commit
   match variant {
      /// Send/Receive/Ack Notice
      DeliveryEntryTypes::Distribution => post_commit_create_Distribution(sah, eh, entry)?,
      DeliveryEntryTypes::DeliveryNotice => post_commit_create_DeliveryNotice(sah, eh, entry)?,
      /// Send/Receive Reply
      DeliveryEntryTypes::NoticeReply => post_commit_create_NoticeReply(sah, eh, entry)?,
      DeliveryEntryTypes::ReplyAck => post_commit_create_ReplyAck(sah, eh, entry)?,
      /// Send/Receive Parcel
      DeliveryEntryTypes::PrivateManifest => post_commit_create_PrivateManifest(sah, eh, entry)?,
      /// Send/Receive ReceptionProof
      DeliveryEntryTypes::ReceptionProof => post_commit_create_ReceptionProof(sah, eh, entry)?,
      /// Public entries
      DeliveryEntryTypes::PublicParcel => post_commit_create_PublicParcel(sah, eh, entry)?,
      ///
      _ => (),
   };
   /// Emit Entry Signal
   emit_new_entry_signal(record, true)?;
   Ok(())
}


///
fn post_commit_delete_app_entry(delete: Delete, variant: DeliveryEntryTypes, new_sah: SignedActionHashed, entry: Entry) -> ExternResult<()> {
   if let DeliveryEntryTypes::PublicParcel = variant {
      debug!("post_commit_delete_PublicParcel() delete: {}", delete.deletes_entry_address.clone());
      let response = call_self("unlink_public_parcel", delete.deletes_entry_address.clone())?;
      /// FIXME: delete ParcelManifest
      let _ah = decode_response::<ActionHash>(response)?;
   }
   /// Emit Entry Signal
   let _ = emit_delete_entry_signal(new_sah.hashed, entry, true)?;
   ///
   Ok(())
}
