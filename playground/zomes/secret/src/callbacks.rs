use hdk::prelude::*;
//use zome_utils::*;
use zome_delivery_api::*;


/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedActionList: Vec<SignedActionHashed>) {
   debug!("SECRET post_commit() called for {} actions", signedActionList.len());
   debug!("SECRET post_commit() zome_info: {:?}, dna: {:?}", zome_info().unwrap(), dna_info().unwrap().zome_names);
   let res = call_delivery_post_commit(signedActionList);
   if let Err(e) = res {
      debug!("SECRET call to delivery_post_commit() failed: {:?}", e);
   }
}
