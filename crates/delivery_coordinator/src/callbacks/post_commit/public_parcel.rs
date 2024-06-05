use hdk::prelude::*;
use zome_utils::{call_self, decode_response};
use zome_delivery_types::*;
use crate::emit_self_signal;


///
pub fn post_commit_PublicParcel(sah: &SignedActionHashed, entry: Entry, eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_PublicParcel() {:?}", eh);
   let parcel_reference = ParcelReference::try_from(entry)?;
   /// Create Anchor
   let response = call_self("link_public_parcel", eh)?;
   let _ah = decode_response::<ActionHash>(response)?;
   /// Emit Signal
   let res = emit_self_signal(DeliverySignalProtocol::PublicParcelPublished((eh.to_owned(), sah.hashed.content.timestamp(), parcel_reference)));
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(())
}
