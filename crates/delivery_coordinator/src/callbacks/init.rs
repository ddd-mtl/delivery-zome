use hdk::prelude::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;

//#[hdk_extern]
fn init_caps(_: ()) -> ExternResult<()> {
   let mut functions = BTreeSet::new();
   functions.insert((zome_info()?.name, REMOTE_ENDPOINT.into()));
   //functions.insert((zome_info()?.name, "get_enc_key".into()));
   create_cap_grant(
      CapGrantEntry {
         tag: "".into(),
         access: ().into(), // empty access converts to unrestricted
         functions: hdk::prelude::GrantedFunctions::Listed(functions),
      }
   )?;
   Ok(())
}


/// Zome Callback
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
   debug!("*** zDelivery.init() callback START");
   /// Set Global Anchors
   Path::from(DIRECTORY_PATH).typed(LinkTypes::Members)?.ensure()?;
   /// Setup initial capabilities
   init_caps(())?;
   /// Create public encryption key and broadcast it
   create_enc_key()?;
   /// Done
   debug!("*** zDelivery.init() callback DONE");
   Ok(InitCallbackResult::Pass)
}
