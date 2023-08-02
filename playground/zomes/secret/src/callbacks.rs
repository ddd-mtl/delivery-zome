use hdk::prelude::*;
use zome_utils::zome_error;
use zome_delivery_api::*;

//use zome_delivery_common::post_commit_Distribution;

// /// Zome Callback
// #[hdk_extern(infallible)]
// fn post_commit(signedActionList: Vec<SignedActionHashed>) {
//    debug!("SECRET post_commit() called for {} actions", signedActionList.len());
//    /// Process each Action
//    for signedAction in signedActionList {
//       debug!("SECRET - {:?}", signedAction.action().entry_type());
//    }
// }



/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedActionList: Vec<SignedActionHashed>) {
   debug!("SECRET post_commit() called for {} actions", signedActionList.len());

   let zome_names = dna_info().unwrap().zome_names;

   //std::panic::set_hook(Box::new(zome_panic_hook));
   /// Process each Action
   for signedAction in signedActionList {
      debug!("SECRET - {:?}", signedAction.action().entry_type());
      let action = signedAction.action();
      if action.entry_type().is_none() {
         continue;
      }
      let (eh, entry_type) = action.entry_data().unwrap();
      match entry_type {
         EntryType::AgentPubKey => {},
         EntryType::CapClaim => {},
         EntryType::CapGrant => {},
         EntryType::App(app_entry_def) => {
            let zome_index: usize = app_entry_def.zome_index.0.into();
            let zome_name: &str = &zome_names[zome_index].0;
            debug!(" >> post_commit() called for a {}", zome_name);
            if zome_name == "zome_delivery_integrity" {
               debug!("its for zome_delivery_integrity {:?}", app_entry_def.entry_index);
               //call_delivery_zome("post_commit", vec![signedAction]);
               // let variant = entry_index_to_variant(app_entry_def.entry_index)?;
               // match variant {
               //    DeliveryEntryTypes::Distribution => post_commit_Distribution(entry, eh),
               //    DeliveryEntryTypes::DeliveryNotice => post_commit_DeliveryNotice(entry, eh),
               //    DeliveryEntryTypes::ParcelChunk => post_commit_ParcelChunk(entry, eh),
               //    DeliveryEntryTypes::ParcelManifest => post_commit_ParcelManifest(entry, eh),
               //    _ => Ok(()),
               // }

               /// Get Entry from local chain
               let monad: HashSet<EntryHash> = HashSet::from([eh.clone()]);
               let query_args = ChainQueryFilter::default()
                  .include_entries(true)
                  .entry_hashes(monad);
               let records = query(query_args).unwrap();
               if records.is_empty() {
                  debug!("Post committed entry not found on chain");
                  continue;
               }
               let entry = records[0].entry().as_option().unwrap().to_owned();
               /// Deserialize it and call its post_commit()
               let Entry::App(ref entry_bytes) = entry
                  else {
                     debug!("EntryHash has already been filtered as an App type");
                     continue;
                  };
               if app_entry_def.entry_index.0 == 4 {
                  post_commit_Distribution(entry, eh);
               } else {
                  if app_entry_def.entry_index.0 == 1 {
                     post_commit_DeliveryNotice(entry, eh);
                  }
               }
            }
            //let result = post_commit_app_entry(eh, app_entry_def);
            //debug!(" << post_commit() result = {:?}", result);
         },
      }
   }
}

