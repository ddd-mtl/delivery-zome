use hdk::prelude::*;

/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedActionList: Vec<SignedActionHashed>) {
   debug!("SECRET post_commit() called for {} actions", signedActionList.len());
   /// Process each Action
   for signedAction in signedActionList {
      debug!("SECRET - {:?}", signedAction.action().entry_type());
   }
}
