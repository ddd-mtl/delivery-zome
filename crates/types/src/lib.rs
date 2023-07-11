#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


mod entries;
mod inputs;
mod delivery;
mod callbacks;
mod signal_protocol;


pub use entries::*;
pub use inputs::*;
pub use delivery::*;
pub use callbacks::*;
pub use signal_protocol::*;

///----------------------------------------------------------------------------------------
/// API

pub const DELIVERY_ZOME_NAME: &'static str = "delivery";

use hdk::prelude::*;

/// Helper function for calling the delivery-zome via inter-zome call
pub fn call_delivery_zome<T>(fn_name: &str, payload: T) -> ExternResult<ZomeCallResponse>
   where
      T: serde::Serialize + std::fmt::Debug,
{
   call(
      CallTargetCell::Local,
      DELIVERY_ZOME_NAME.into(),
      fn_name.to_string().into(),
      None,
      payload,
   )
}
