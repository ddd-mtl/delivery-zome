#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


use std::fmt::Debug;
use hdk::prelude::*;


pub use zome_delivery_common::*;

use zome_delivery_types::*;

/// Helper function for calling the delivery-zome via inter-zome call
pub fn call_delivery_zome<I>(fn_name: &str, payload: I) -> ExternResult<ZomeCallResponse>
   where
      I: Serialize + Debug,
{
   return call(
      CallTargetCell::Local,
      ZomeName::from(DELIVERY_ZOME_NAME),
      fn_name.to_string().into(),
      None,
      payload,
   );
}
