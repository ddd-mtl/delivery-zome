use hdk::prelude::*;

use crate::{
   entry_kind::*,
   utils::*,
};
use crate::entries::*;
//use crate::functions::*;
//use crate::strum::AsStaticRef;

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
      EntryKind::DeliveryNotice => {
         let notice = get_typed_from_eh::<DeliveryNotice>(eh)?;
         DeliveryNotice::post_commit(&eh, notice)?;
      },
      EntryKind::DeliveryReceipt => {
         let _ = get_typed_from_eh::<DeliveryReceipt>(eh)?;
      },
      EntryKind::DeliveryReply => {
         let reply = get_typed_from_eh::<DeliveryReply>(eh)?;
         DeliveryReply::post_commit(&eh, reply)?;
      },
      EntryKind::Distribution => {
         let distribution = get_typed_from_eh::<Distribution>(eh.clone())?;
         Distribution::post_commit(&eh, distribution)?;
      },
      EntryKind::NoticeReceived => {
         let _ = get_typed_from_eh::<NoticeReceived>(eh)?;
      },
      EntryKind::ParcelReceived => {
         let reception = get_typed_from_eh::<ParcelReceived>(eh)?;
         ParcelReceived::post_commit(&eh, reception)?;
      },
      EntryKind::ReplyReceived => {
         let _ = get_typed_from_eh::<ReplyReceived>(eh)?;
      },
      EntryKind::PendingItem => {
         let _ = get_typed_from_eh::<PendingItem>(eh)?;
      },
      EntryKind::ParcelChunk => {
         let chunk = get_typed_from_eh::<ParcelChunk>(eh.clone())?;
         ParcelChunk::post_commit(&eh, chunk)?;
      },
      EntryKind::ParcelManifest => {
         let manifest = get_typed_from_eh::<ParcelManifest>(eh)?;
         ParcelManifest::post_commit(&eh, manifest)?;

      },
      // Add type handling here
      // ..
   }
   // Done
   Ok(())
}

