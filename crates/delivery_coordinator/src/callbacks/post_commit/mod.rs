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
   let filtered = filter_action_from_this_zome(signedActionList).unwrap();
   let result = delivery_post_commit(filtered.clone());
   if let Err(e) = result {
      error!("<< delivery_post_commit() failed: {:?}", e);
   } else {
      debug!("<< delivery_post_commit() SUCCEEDED");
      emit_post_commit::<DeliveryEntry, LinkTypes>(filtered);
   }
}


//#[hdk_extern]
fn delivery_post_commit(signedActionList: Vec<SignedActionHashed>) -> ExternResult<()> {
   debug!("delivery_post_commit() called for {} actions. ({})", signedActionList.len(), zome_info().unwrap().id);
   std::panic::set_hook(Box::new(zome_panic_hook));
   let zome_names = dna_info().unwrap().zome_names;
   /// Process each Action
   for sah in signedActionList.clone() {
      match sah.action() {
         /// NewEntryAction
         Action::Update(_) |
         Action::Create(_) => {
            let Some(EntryType::App(app_entry_def)) = sah.action().entry_type()
              else { return zome_error!("Create action malformed."); };
            /// Bail if from other zome
            if zome_names[app_entry_def.zome_index.0 as usize].0 != "zDeliveryIntegrity" { // Hack: hardcoded name
               continue;
            }
            ///
            let variant = get_variant::<DeliveryEntry>(app_entry_def.entry_index).unwrap();
            let _ = post_commit_new_app_entry(&sah, variant)?;
         },
         /// DeleteAction
         Action::Delete(delete) => {
            let Ok(new_sah) = must_get_action(delete.deletes_address.clone())
              else { return zome_error!("Deleted action not found."); };
            let Some(EntryType::App(app_entry_def)) = new_sah.action().entry_type()
              else { return zome_error!("Deleted action should have entry_type."); };
            /// Bail if from other zome
            if zome_names[app_entry_def.zome_index.0 as usize].0 != "zDeliveryIntegrity" { // Hack: hardcoded name
               continue;
            }
            let variant = get_variant::<DeliveryEntry>(app_entry_def.entry_index).unwrap();
            let _ = post_commit_delete_app_entry(delete.clone(), variant)?;
         },
         ///
         _ => (),
      }
   }
   Ok(())
}


///
fn filter_action_from_this_zome(signedActionList: Vec<SignedActionHashed>) -> ExternResult<Vec<SignedActionHashed>> {
   let zome_names = dna_info().unwrap().zome_names;
   let mut res = Vec::new();
   /// Process each Action
   for sah in signedActionList.clone() {
      let zome_index = match sah.action() {
         /// NewEntryAction
         Action::Update(_) |
         Action::Create(_) => {
            let Some(EntryType::App(app_entry_def)) = sah.action().entry_type()
              else { return zome_error!("Create action malformed."); };
            app_entry_def.zome_index
         },
         /// DeleteAction
         Action::Delete(delete) => {
            let Ok(new_sah) = must_get_action(delete.deletes_address.clone())
              else { return zome_error!("Deleted action not found."); };
            let Some(EntryType::App(app_entry_def)) = new_sah.action().entry_type()
              else { return zome_error!("Deleted action should have entry_type."); };
            app_entry_def.zome_index
         },
         ///
         Action::CreateLink(create_link) => create_link.zome_index,
         ///
         Action::DeleteLink(delete_link) => {
            let Ok(Some(record)) = get(delete_link.link_add_address.clone(), GetOptions::local())
              else { return zome_error!("Failed to get CreateLink action."); };
            let Action::CreateLink(create_link) = record.action()
              else { return zome_error!("Record should be a CreateLink."); };
            create_link.zome_index
         },
         ///
         _ => continue,
      };
      //warn!("delivery_post_commit() zome_name = {}", zome_names[zome_index.0 as usize]);
      if zome_names[zome_index.0 as usize].0 == "zDeliveryIntegrity" { // Hack: hardcoded name
         res.push(sah);
      }
   }
   ///
   Ok(res)
}



///
fn post_commit_new_app_entry(sah: &SignedActionHashed, variant: DeliveryEntryTypes) -> ExternResult<()> {
   debug!("post_commit_create_app_entry() called for a {:?}", variant);
   let Some(eh) = sah.action().entry_hash() else {
      return zome_error!("Action has no Entry");
   };
   let entry = must_get_entry(eh.to_owned())?.content;
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
   ///
   Ok(())
}


///
fn post_commit_delete_app_entry(delete: Delete, variant: DeliveryEntryTypes) -> ExternResult<()> {
   if let DeliveryEntryTypes::PublicParcel = variant {
      debug!("post_commit_delete_PublicParcel() delete: {}", delete.deletes_entry_address.clone());
      let response = call_self("unlink_public_parcel", delete.deletes_entry_address.clone())?;
      /// FIXME: delete ParcelManifest
      let _ah = decode_response::<ActionHash>(response)?;
   }
   ///
   Ok(())
}
