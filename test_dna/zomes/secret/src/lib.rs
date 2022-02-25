#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


use hdk::prelude::*;
use zome_delivery_types::*;

entry_defs![
   Secret::entry_def()
];

//----------------------------------------------------------------------------------------
// Helpers
//----------------------------------------------------------------------------------------


pub fn error<T>(reason: &str) -> ExternResult<T> {
   //Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
   Err(WasmError::Guest(String::from(reason)))
}


///
pub fn decode_response<T>(response: ZomeCallResponse) -> ExternResult<T>
   where
      T: for<'de> serde::Deserialize<'de> + std::fmt::Debug
{
   return match response {
      ZomeCallResponse::Ok(output) => Ok(output.decode()?),
      ZomeCallResponse::Unauthorized(_, _, _, _) => error("Unauthorized call"),
      ZomeCallResponse::NetworkError(e) => error(&format!("NetworkError: {:?}", e)),
      ZomeCallResponse::CountersigningSession(e) => error(&format!("CountersigningSession: {:?}", e)),
   };
}

/// Call get() to obtain EntryHash and AppEntry from an EntryHash
pub fn get_typed_from_eh<T: TryFrom<Entry>>(eh: EntryHash) -> ExternResult<T> {
   match get(eh, GetOptions::content())? {
      Some(element) => Ok(get_typed_from_el(element)?),
      None => error("Entry not found"),
   }
}

/// Obtain AppEntry from Element
pub fn get_typed_from_el<T: TryFrom<Entry>>(element: Element) -> ExternResult<T> {
   match element.entry() {
      element::ElementEntry::Present(entry) => get_typed_from_entry::<T>(entry.clone()),
      _ => error("Could not convert element"),
   }
}

// Obtain AppEntry from Entry
pub fn get_typed_from_entry<T: TryFrom<Entry>>(entry: Entry) -> ExternResult<T> {
   return match T::try_from(entry.clone()) {
      Ok(a) => Ok(a),
      Err(_) => error(&format!("get_typed_from_entry() failed for: {:?}", entry)),
   }
}

fn call_delivery_zome<T>(fn_name: &str, payload: T) -> ExternResult<ZomeCallResponse>
   where
      T: serde::Serialize + std::fmt::Debug,
{
   call(
      CallTargetCell::Local,
      "delivery".into(),
      fn_name.to_string().into(),
      None,
      payload,
   )
}

//----------------------------------------------------------------------------------------

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
   // let set: HashSet<_> = vec![eh].drain(..).collect(); // dedup
   // let query_args = ChainQueryFilter::default()
   //    .include_entries(true)
   //    .entry_hashes(set);
   // let entries = query(query_args)?;
   // if entries.len() != 1 {
   //    return Err(WasmError::Guest(String::from("No Secret found at given EntryHash")));
   // }
   let maybe_secret: ExternResult<Secret> = get_typed_from_eh(eh.clone());
   if let Ok(secret) = maybe_secret {
      return Ok(secret.value);
   }
   /// Not a Secret Entry, so it should be a Manifest
   let manifest: ParcelManifest = get_typed_from_eh(eh)?;
   /// Get all chunks
   let set: HashSet<_> = manifest.chunks.clone().drain(..).collect(); // dedup
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .entry_hashes(set);
   let entries = query(query_args)?;
   if entries.len() != manifest.chunks.len() {
      return Err(WasmError::Guest(String::from("Not all chunks have been found on chain")));
   }
   /// Concat all chunks
   if manifest.custum_entry_type != "split_secret".to_owned() {
      return Err(WasmError::Guest(String::from("Manifest of an unknown entry type")));
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
      strategy: DistributionStrategy::NORMAL,
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
#[hdk_extern]
pub fn get_secrets_from(sender: AgentPubKey) -> ExternResult<Vec<EntryHash>> {
   debug!("get_secrets_from() START");
   let response = call_delivery_zome(
      "query_DeliveryNotice",
      DeliveryNoticeQueryField::Sender(sender),
   )?;
   let notices: Vec<DeliveryNotice> = decode_response(response)?;
   let parcels = notices.iter().map(|x| x.parcel_summary.reference.entry_address()).collect();
   debug!("get_secrets_from() END");
   Ok(parcels)
}


/// Zome Function
#[hdk_extern]
pub fn accept_secret(parcel_eh: EntryHash) -> ExternResult<EntryHash> {
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
      has_accepted: true,
   };
   let response = call_delivery_zome("respond_to_notice", input)?;
   // return respond_to_notice(input)?;
   let eh: EntryHash = decode_response(response)?;
   Ok(eh)
}