use hdk::prelude::*;
use zome_delivery_types::*;
use crate::SignalProtocol;


///
pub fn post_commit_PublicManifest(entry: Entry, manifest_eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_PublicManifest() {:?}", manifest_eh);
   let manifest = ParcelManifest::try_from(entry)?;
   /// Emit signal
   let res = emit_signal(&SignalProtocol::NewManifest((manifest_eh.to_owned(), manifest.clone())));
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(())
}