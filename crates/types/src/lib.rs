#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


mod entries;
mod parcel;
mod inputs;
mod delivery;
mod commit_parcel;


pub use entries::*;
pub use inputs::*;
pub use parcel::*;
pub use delivery::*;
pub use commit_parcel::*;


pub const DELIVERY_ZOME_NAME: &'static str = "delivery";

pub const COMMIT_PARCEL_CALLBACK: &'static str = "commit_parcel";

use hdk::prelude::*;
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
