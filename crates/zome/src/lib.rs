#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[macro_use] extern crate enum_ordinalize;


//----------------------------------------------------------------------------------------

//#[cfg(not(target_arch = "wasm32"))]
//pub mod api_error;

mod constants;
mod send_dm;
mod dm_protocol;
mod receive_dm;
mod send_item;

//pub mod signal_protocol;

pub mod callbacks;
pub mod functions;
pub(crate) mod zome_entry_trait;
pub mod link_kind;
pub mod utils_parcel;
pub mod entry_kind;
mod entries;
mod utils_delivery;


//----------------------------------------------------------------------------------------

pub use constants::*;
pub use send_dm::*;
pub use dm_protocol::*;
pub use receive_dm::*;
pub use send_item::*;
pub use utils_delivery::*;

//pub use signal_protocol::*;

//----------------------------------------------------------------------------------------
use hdk::prelude::*;
use zome_delivery_types::*;

entry_defs![
   /// -- PubEncKey
   PubEncKey::entry_def(),
   /// -- Delivery
   DeliveryNotice::entry_def(),
   DeliveryReceipt::entry_def(),
   DeliveryReply::entry_def(),
   Distribution::entry_def(),
   NoticeReceived::entry_def(),
   ParcelReceived::entry_def(),
   ReplyReceived::entry_def(),
   PendingItem::entry_def(),
   ParcelManifest::entry_def(),
   ParcelChunk::entry_def(),
   /// -- Other
   PathEntry::entry_def()
];
