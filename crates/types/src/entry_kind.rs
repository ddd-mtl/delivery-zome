use hdk::prelude::*;

use std::str::FromStr;

use std::convert::AsRef;
use strum_macros::AsRefStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
//use strum::EnumProperty;

use crate::entries::*;
use crate::entries::parcel_chunk::ParcelChunk;
use crate::entries::pub_enc_key::*;
use crate::utils::*;

/// Listing all Entry kinds for this DNA
/// !! Visibility prop value must match hdk_entry visibility !!
#[derive(AsRefStr, EnumIter, Ordinalize, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EntryKind {
   Path,
   PubEncKey,
   DeliveryNotice,
   DeliveryReceipt,
   DeliveryReply,
   Distribution,
   NoticeReceived,
   ParcelReceived,
   ReplyReceived,
   PendingItem,
   ParcelManifest,
   ParcelChunk,
}

impl FromStr for EntryKind {
   type Err = ();
   fn from_str(input: &str) -> Result<EntryKind, Self::Err> {
      for entry_kind in EntryKind::iter() {
         //let entry_kind = EntryKind::from_ordinal(ordinal);
         if input == entry_kind.as_ref() {
            return Ok(entry_kind);
         }
      }
      Err(())
   }
}




