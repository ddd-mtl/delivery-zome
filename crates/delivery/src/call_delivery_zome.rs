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
