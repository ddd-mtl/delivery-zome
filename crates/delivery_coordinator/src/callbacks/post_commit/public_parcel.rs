use hdk::prelude::*;
use zome_utils::call_self;
use zome_delivery_types::*;
use crate::SignalProtocol;


///
pub fn post_commit_PublicParcel(entry: Entry, eh: &EntryHash) -> ExternResult<()> {
   debug!("post_commit_PublicParcel() {:?}", eh);
   let parcel_description = ParcelDescription::try_from(entry)?;
   /// Create Anchor
   let _response = call_self("link_public_parcel", eh)?;
   //let _ah: ActionHash = decode_response(response)?;
   /// Emit Signal
   let res = emit_signal(&SignalProtocol::NewPublicParcel((eh.to_owned(), parcel_description)));
   if let Err(err) = res {
      error!("Emit signal failed: {}", err);
   }
   /// Done
   Ok(())
}