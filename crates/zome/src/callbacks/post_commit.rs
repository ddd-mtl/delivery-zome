use hdk::prelude::*;
use zome_utils::*;
use crate::entry_kind::*;

/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedHeaderList: Vec<SignedHeaderHashed>) {
   debug!("post_commit() called for {} entries", signedHeaderList.len());
   std::panic::set_hook(Box::new(my_panic_hook));
   /// Process each header
   for signedHeader in signedHeaderList {
      //debug!(" - {:?}", signedHeader.header().entry_type());
      let header = signedHeader.header();

      //let hash = signedHeader.as_hash().get_raw_39();
      //let hash64 = format!("u{}", base64::encode_config(hash, base64::URL_SAFE_NO_PAD));
      // debug!(" - {} ({:?})", hash64, signedHeader.header().entry_type());

      if header.entry_type().is_none() {
         continue;
      }
      let (eh, entry_type) = header.entry_data().unwrap();

      match entry_type {
         EntryType::AgentPubKey => {},
         EntryType::CapClaim => {},
         EntryType::CapGrant => {},
         EntryType::App(app_type) => {
            let result = post_commit_app_entry(eh, app_type);
            debug!(" << post_commit() result = {:?}", result);
         },
      }
   }
}

///
fn post_commit_app_entry(eh: &EntryHash, app_type: &AppEntryType) -> ExternResult<()> {
   debug!(" >> post_commit() called for a {:?}", app_type);
   /// Get Entry from local chain
   let monad: HashSet<EntryHash> = HashSet::from([eh.clone()]);
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .entry_hashes(monad);
   let elements = query(query_args)?;
   if elements.is_empty() {
      return error("Post committed entry not found on chain");
   }
   let entry = elements[0].entry().as_option().unwrap();
   /// Deserialize it and call its post_commit()
   if let Entry::App(entry_bytes) = entry {
      let delivery_entry = deserialize_into_zome_entry(&app_type.id(), entry_bytes.clone())?;
      let res = delivery_entry.post_commit(eh);
      if let Err(e) = res {
         error!("app post_commit() failed: {:?}", e);
      }
      return Ok(());
   }
   return error("EntryHash has already been filtered as an App type");
}
