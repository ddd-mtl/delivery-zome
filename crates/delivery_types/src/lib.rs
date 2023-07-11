#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

mod delivery;
mod constants;

pub use delivery::*;
pub use constants::*;

///----------------------------------------------------------------------------------------
/// API

pub const DELIVERY_ZOME_NAME: &'static str = "zDelivery";

