#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(dead_code)]


mod pub_enc_key;
mod validate_app_entry;

pub use pub_enc_key::*;


use hdi::prelude::*;

use zome_delivery_types::*;

/// List of all Link kinds handled by this Zome
#[hdk_link_types]
#[derive(Serialize, Deserialize)]
pub enum LinkTypes {
   EncKey,
   Members,
   Inbox,
   Pendings,
}


#[hdk_entry_defs]
#[unit_enum(DeliveryEntryTypes)]
pub enum DeliveryEntry {
   #[entry_def(required_validations = 2, visibility = "public")]
   PubEncKey(PubEncKey),
   #[entry_def(required_validations = 3, visibility = "private")]
   DeliveryNotice(DeliveryNotice),
   #[entry_def(required_validations = 3, visibility = "private")]
   ReceptionAck(ReceptionAck),
   #[entry_def(required_validations = 3, visibility = "private")]
   NoticeReply(NoticeReply),
   #[entry_def(required_validations = 1, visibility = "private")]
   Distribution(Distribution),
   #[entry_def(required_validations = 1, visibility = "private")]
   ParcelChunk(ParcelChunk),
   #[entry_def(required_validations = 1, visibility = "private")]
   ParcelManifest(ParcelManifest), // WARN. DONT MOVE THIS
   #[entry_def(required_validations = 1, visibility = "private")]
   ReceptionProof(ReceptionProof),
   #[entry_def(required_validations = 1, visibility = "private")]
   NoticeAck(NoticeAck),
   #[entry_def(required_validations = 1, visibility = "public")]
   PendingItem(PendingItem),
   #[entry_def(required_validations = 1, visibility = "private")]
   ReplyAck(ReplyAck),
}


///
pub fn entry_index_to_variant(entry_index: EntryDefIndex) -> ExternResult<DeliveryEntryTypes> {
   let mut i = 0;
   for variant in DeliveryEntryTypes::iter() {
      if i == entry_index.0 {
         return Ok(variant);
      }
      i += 1;
   }
   return Err(wasm_error!(format!("Unknown EntryDefIndex: {}", entry_index.0)));
}
