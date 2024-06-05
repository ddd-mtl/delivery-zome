use hdk::prelude::*;
use zome_delivery_types::*;
use crate::emit_self_signal;


///
pub fn post_commit_PublicManifest(sah: &SignedActionHashed, entry: Entry, manifest_eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_PublicManifest() {:?}", manifest_eh);
   let manifest = ParcelManifest::try_from(entry)?;
   /// Emit signal
   let res = emit_self_signal(DeliverySignalProtocol::NewLocalManifest((manifest_eh.to_owned(), sah.hashed.content.timestamp(), manifest.clone())));
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(())
}
