#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


use hdk::prelude::*;
use zome_delivery_types::*;
use zome_utils::*;

entry_defs![
   Secret::entry_def()
];


/// Entry representing a secret message
#[hdk_entry(id = "Secret", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct Secret {
   pub value: String,
}

/// Zome Function
#[hdk_extern]
pub fn create_secret(value: String) -> ExternResult<EntryHash> {
   let secret = Secret { value };
   let eh = hash_entry(secret.clone())?;
   let _hh = create_entry(secret)?;
   return Ok(eh);
}

// /// Zome Function
// #[hdk_extern]
// pub fn get_secret(eh: EntryHash) -> ExternResult<String> {
//    let set: HashSet<_> = vec![eh].drain(..).collect(); // dedup
//    let query_args = ChainQueryFilter::default()
//       .include_entries(true)
//       .entry_hashes(set);
//    let entries = query(query_args)?;
//    if entries.len() != 1 {
//       return Err(WasmError::Guest(String::from("No Secret found at given EntryHash")));
//    }
//    let secret: Secret = get_typed_from_el(entries[0].clone())?;
//    return Ok(secret.value);
// }

/// Zome Function
#[hdk_extern]
pub fn create_split_secret(value: String) -> ExternResult<EntryHash> {
   let split = value.split_whitespace();
   /// Commit chunks
   let mut chunks = Vec::new();
   for word in split {
      let response = call_delivery_zome("commit_parcel_chunk", word)?;
      let eh: EntryHash = decode_response(response)?;
      chunks.push(eh);
   }
   /// Commit Manifest
   let manifest = ParcelManifest {
      name: "dummy".to_string(),
      custum_entry_type: "split_secret".to_owned(),
      size: value.len(),
      chunks,
   };
   let response = call_delivery_zome("commit_parcel_manifest", manifest)?;
   let eh: EntryHash = decode_response(response)?;
   /// Done
   return Ok(eh);
}


/// Zome Function
#[hdk_extern]
pub fn get_secret(eh: EntryHash) -> ExternResult<String> {
   ///// Check inbox first
   // debug!("get_secret() - pull_inbox");
   // let response = call_delivery_zome("pull_inbox", ())?;
   // let inbox_items: Vec<HeaderHash> = decode_response(response)?;
   // debug!("get_secret() - inbox_items: {}", inbox_items.len());
   /// Try to get Secret
   let maybe_secret: ExternResult<Secret> = get_typed_from_eh(eh.clone());
   if let Ok(secret) = maybe_secret {
      debug!("get_secret() - secret found");
      return Ok(secret.value);
   }
   debug!("get_secret() - Secret Entry not found, could be a ParcelManifest");
   /// Not a Secret Entry, could be a Manifest
   let maybe_manifest: ExternResult<ParcelManifest> = get_typed_from_eh(eh);
   if maybe_manifest.is_err() {
      return error("No entry found at given EntryHash");
   }
   let manifest = maybe_manifest.unwrap();
   /// Get all chunks
   let set: HashSet<_> = manifest.chunks.clone().drain(..).collect(); // dedup
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .entry_hashes(set);
   let entries = query(query_args)?;
   if entries.len() != manifest.chunks.len() {
      return error("Not all chunks have been found on chain");
   }
   /// Concat all chunks
   if manifest.custum_entry_type != "split_secret".to_owned() {
      return error("Manifest of an unknown entry type");
   }
   let mut secret = String::new();
   for el in entries {
      let chunk: ParcelChunk = get_typed_from_el(el)?;
      secret += &chunk.data;
      secret += ".";
   }
   /// Done
   Ok(secret)
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendSecretInput {
   pub secret_eh: EntryHash,
   pub strategy: DistributionStrategy,
   pub recipient: AgentPubKey,
}

/// Zome Function
#[hdk_extern]
pub fn send_secret(input: SendSecretInput) -> ExternResult<EntryHash> {
   debug!("send_secret() START - {:?}", input.secret_eh);
   /// Determine parcel type depending on Entry
   let maybe_secret: ExternResult<Secret> = get_typed_from_eh(input.secret_eh.clone());
   let parcel_ref =  if let Ok(_secret) = maybe_secret {
      ParcelReference::AppEntry(
         zome_info()?.name,
         EntryDefId::App("Secret".into()),
         input.secret_eh,
      )
   } else {
      /// Should be a Manifest
      let _: ParcelManifest = get_typed_from_eh(input.secret_eh.clone())?;
      ParcelReference::Manifest(input.secret_eh)
   };

   let distribution = DistributeParcelInput {
      recipients: vec![input.recipient],
      strategy: input.strategy,
      parcel_ref,
   };
   debug!("send_secret() call distribute_parcel...");
   let response = call_delivery_zome("distribute_parcel", distribution)?;
   // distribute_parcel(distribution)?;
   let eh: EntryHash = decode_response(response)?;
   debug!("send_secret() END");
   Ok(eh)
}


/// Zome Function
/// Return list of parcels' EntryHash
#[hdk_extern]
pub fn get_secrets_from(sender: AgentPubKey) -> ExternResult<Vec<EntryHash>> {
   debug!("get_secrets_from() START - pull_inbox");
   let response = call_delivery_zome("pull_inbox", ())?;
   let inbox_items: Vec<HeaderHash> = decode_response(response)?;
   debug!("get_secrets_from() - inbox_items: {}", inbox_items.len());
   debug!("get_secrets_from() - query_DeliveryNotice");
   let response = call_delivery_zome(
      "query_DeliveryNotice",
      DeliveryNoticeQueryField::Sender(sender),
   )?;
   let notices: Vec<DeliveryNotice> = decode_response(response)?;
   let parcels: Vec<EntryHash> = notices.iter().map(|x| x.summary.parcel_reference.entry_address()).collect();
   debug!("get_secrets_from() END - secret parcels found: {}", parcels.len());
   Ok(parcels)
}


/// Zome Function
#[hdk_extern]
pub fn refuse_secret(parcel_eh: EntryHash) -> ExternResult<EntryHash> {
   return respond_to_secret(parcel_eh, false);

}


/// Zome Function
#[hdk_extern]
pub fn accept_secret(parcel_eh: EntryHash) -> ExternResult<EntryHash> {
   return respond_to_secret(parcel_eh, true);

}


///
pub fn respond_to_secret(parcel_eh: EntryHash, has_accepted: bool) -> ExternResult<EntryHash> {
   let response = call_delivery_zome(
      "query_DeliveryNotice",
      DeliveryNoticeQueryField::Parcel(parcel_eh),
   )?;
   let notices: Vec<DeliveryNotice> = decode_response(response)?;
   if notices.len() != 1 {
      return Err(WasmError::Guest(String::from("No Secret found at given EntryHash")));
   }
   let notice_eh = hash_entry(notices[0].clone())?;
   let input = RespondToNoticeInput {
      notice_eh,
      has_accepted,
   };
   let response = call_delivery_zome("respond_to_notice", input)?;
   // return respond_to_notice(input)?;
   let eh: EntryHash = decode_response(response)?;
   Ok(eh)
}


// /// Zome Function
// #[hdk_extern]
// pub fn pull_inbox(_: ()) -> ExternResult<()> {
//    let response = call_delivery_zome("pull_inbox", ())?;
//    let _: Vec<HeaderHash> = decode_response(response)?;
//    Ok(())
// }