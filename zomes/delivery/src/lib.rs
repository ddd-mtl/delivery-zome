#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use] extern crate shrinkwraprs;

#[macro_use]
extern crate zome_proc_macro;

#[macro_use] extern crate enum_ordinalize;

//////////////////////////////////////////////////////////////////////////////////////////

//#[cfg(not(target_arch = "wasm32"))]
//pub mod api_error;

mod utils;
mod constants;
mod link_kind;
mod entry_kind;
mod path_kind;

mod dm;
mod dm_protocol;

mod receive;
mod utils_parcel;
mod send_item;
mod parcel;

//pub mod signal_protocol;

pub mod callbacks;
pub mod entries;
pub mod functions;



//////////////////////////////////////////////////////////////////////////////////////////

pub use utils::*;
pub use constants::*;
pub use link_kind::*;
pub use entry_kind::*;
pub use path_kind::*;

pub use dm::*;
pub use dm_protocol::*;

pub use receive::*;
pub use utils_parcel::*;
pub use send_item::*;
pub use parcel::*;
//pub use signal_protocol::*;

