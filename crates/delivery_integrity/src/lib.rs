#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(dead_code)]


mod validate_app_entry;
mod validate;


use hdi::prelude::*;

use zome_delivery_types::*;


/// List of all Link kinds handled by this Zome
#[hdk_link_types]
#[derive(Serialize, Deserialize)]
pub enum LinkTypes {
   Members,
   Inbox,
   Pendings,
   PublicParcels,
}


#[hdk_entry_types]
#[unit_enum(DeliveryEntryTypes)]
pub enum DeliveryEntry {
   #[entry_type(required_validations = 3, visibility = "private")]
   DeliveryNotice(DeliveryNotice),
   #[entry_type(required_validations = 3, visibility = "private")]
   ReceptionAck(ReceptionAck),
   #[entry_type(required_validations = 3, visibility = "private")]
   NoticeReply(NoticeReply),
   #[entry_type(required_validations = 1, visibility = "private")]
   Distribution(Distribution),
   #[entry_type(required_validations = 1, visibility = "private")]
   PrivateChunk(ParcelChunk),
   #[entry_type(required_validations = 1, visibility = "private")]
   PrivateManifest(ParcelManifest), // WARN. DONT MOVE THIS
   #[entry_type(required_validations = 1, visibility = "private")]
   ReceptionProof(ReceptionProof),
   #[entry_type(required_validations = 1, visibility = "private")]
   NoticeAck(NoticeAck),
   #[entry_type(required_validations = 1, visibility = "private")]
   ReplyAck(ReplyAck),
   #[entry_type(required_validations = 1, visibility = "public")]
   PendingItem(PendingItem),
   #[entry_type(required_validations = 1, visibility = "public")]
   PublicManifest(ParcelManifest),
   #[entry_type(required_validations = 1, visibility = "public")]
   PublicChunk(ParcelChunk),
   #[entry_type(required_validations = 1, visibility = "public")]
   PublicParcel(ParcelReference),
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


// pub fn entry2Kind(entry: Entry, entry_type: DeliveryEntryTypes) -> ExternResult<DeliveryEntryKind> {
//    Ok(match entry_type {
//       DeliveryEntryTypes::Distribution => DeliveryEntryKind::Distribution(Distribution::try_from(entry)?),
//       DeliveryEntryTypes::DeliveryNotice => DeliveryEntryKind::DeliveryNotice(DeliveryNotice::try_from(entry)?),
//       DeliveryEntryTypes::NoticeAck => DeliveryEntryKind::NoticeAck(NoticeAck::try_from(entry)?),
//       DeliveryEntryTypes::NoticeReply => DeliveryEntryKind::NoticeReply(NoticeReply::try_from(entry)?),
//       DeliveryEntryTypes::ReplyAck => DeliveryEntryKind::ReplyAck(ReplyAck::try_from(entry)?),
//       DeliveryEntryTypes::PrivateChunk => DeliveryEntryKind::ParcelChunk(ParcelChunk::try_from(entry)?),
//       DeliveryEntryTypes::PrivateManifest => DeliveryEntryKind::ParcelManifest(ParcelManifest::try_from(entry)?),
//       DeliveryEntryTypes::ReceptionProof => DeliveryEntryKind::ReceptionProof(ReceptionProof::try_from(entry)?),
//       DeliveryEntryTypes::ReceptionAck => DeliveryEntryKind::ReceptionAck(ReceptionAck::try_from(entry)?),
//       DeliveryEntryTypes::PendingItem => DeliveryEntryKind::PendingItem(PendingItem::try_from(entry)?),
//       DeliveryEntryTypes::PublicChunk => DeliveryEntryKind::ParcelChunk(ParcelChunk::try_from(entry)?),
//       DeliveryEntryTypes::PublicManifest => DeliveryEntryKind::ParcelManifest(ParcelManifest::try_from(entry)?),
//       DeliveryEntryTypes::PublicParcel => DeliveryEntryKind::PublicParcel(ParcelReference::try_from(entry)?),
//    })
// }
