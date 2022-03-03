use hdk::prelude::*;

use crate::{
   constants::*,
   functions::*,
};


#[hdk_extern]
fn init_caps(_: ()) -> ExternResult<()> {
   let mut functions: GrantedFunctions = BTreeSet::new();
   functions.insert((zome_info()?.name, REMOTE_ENDPOINT.into()));
   //functions.insert((zome_info()?.name, "get_enc_key".into()));
   create_cap_grant(
      CapGrantEntry {
         tag: "".into(),
         access: ().into(), // empty access converts to unrestricted
         functions,
      }
   )?;
   Ok(())
}


/// Zome Callback
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
   debug!("*** init() callback START");
   /// Set Global Anchors
   Path::from(DIRECTORY_PATH).ensure()?;
   /// Setup initial capabilities
   init_caps(())?;
   /// Create public encryption key and broadcast it
   create_enc_key()?;
   /// Done
   debug!("*** init() callback DONE");
   Ok(InitCallbackResult::Pass)
}
