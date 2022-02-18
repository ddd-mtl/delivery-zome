use hdk::prelude::*;

use crate::{
   entry_kind::*,
   file::*,
   utils::*,
};
use crate::entries::*;
use crate::functions::*;
use crate::strum::AsStaticRef;

/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedHeaderList: Vec<SignedHeaderHashed>) {
   //debug!("post_commit() called: {:?}", hhList);
   debug!("post_commit() called");
   for signedHeader in signedHeaderList {
      //debug!(" - {:?}", signedHeader.header().entry_type());
      let header = signedHeader.header();

      //let hash = signedHeader.as_hash().get_raw_39();
      //let hash64 = format!("u{}", base64::encode_config(hash, base64::URL_SAFE_NO_PAD));
      // debug!(" - {} ({:?})", hash64, signedHeader.header().entry_type());

      if header.entry_type().is_none() {
         continue;
      }
      let (entry_hash, entry_type) = header.entry_data().unwrap();

      match entry_type {
         EntryType::AgentPubKey => {},
         EntryType::CapClaim => {},
         EntryType::CapGrant => {},
         EntryType::App(app_type) => {
            let res = post_commit_app(entry_hash.clone(), app_type.clone());
            if let Err(e) = res {
               error!("post_commit() error: {:?}", e);
            }
         },
      }
   }
}


fn post_commit_app(eh: EntryHash, app_type: AppEntryType) -> ExternResult<()>{
   let entry_kind = EntryKind::from_index(&app_type.id());
   debug!(" - {} ({})",  entry_kind.as_static(), eh);
   match entry_kind {
      EntryKind::PubEncKey => {},
      EntryKind::Path => {},
      EntryKind::DeliveryConfirmation => {
         let _ = get_typed_from_eh::<DeliveryConfirmation>(eh)?;
      },
      EntryKind::DeliveryNotification => {
         let _ = get_typed_from_eh::<DeliveryNotification>(eh)?;
      },
      EntryKind::ReceptionConfirmation => {
         let _ = get_typed_from_eh::<ReceptionConfirmation>(eh)?;
      },
      EntryKind::ManifestConfirmation => {
         let _ = get_typed_from_eh::<ManifestConfirmation>(eh)?;
      },
      EntryKind::PendingItem => {
         let _ = get_typed_from_eh::<PendingItem>(eh)?;
      },
      EntryKind::Distribution => {
         let distribution = get_typed_from_eh::<Distribution>(eh.clone())?;
         post_commit_distribution(&eh, distribution)?;
      },
      EntryKind::ParcelChunk => {
         let _ = get_typed_from_eh::<ParcelChunk>(eh.clone())?;
      },
      EntryKind::ParcelManifest => {
         let _manifest = get_typed_from_eh::<ParcelManifest>(eh)?;
      },
      // Add type handling here
      // ..
   }
   // Done
   Ok(())
}

