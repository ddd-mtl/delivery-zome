#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


mod callbacks;
mod send_secret;


//----------------------------------------------------------------------------------------

use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_api::*;
use zome_secret_integrity::*;


///
#[hdk_extern]
pub fn create_secret(value: String) -> ExternResult<EntryHash> {
   let secret = Secret { value };
   let eh = hash_entry(secret.clone())?;
   let _ah = create_entry(SecretEntry::Secret(secret))?;
   return Ok(eh);
}


///
#[hdk_extern]
pub fn create_split_secret(value: String) -> ExternResult<EntryHash> {
   let split = value.split_whitespace();
   let data_hash = value.clone(); // Should be a hash but we dont really care as its just an example playground
   /// Commit chunks
   let mut chunks = Vec::new();
   for word in split {
      chunks.push(ParcelChunk{data_hash: data_hash.clone(), data: word.to_string()});
   }
   let response = call_delivery_zome("commit_private_chunks", chunks)?;
   let chunk_ehs: Vec<EntryHash> = decode_response(response)?;

   let description = ParcelDescription {
     name: "".to_string(),
      size: 0,
      zome_origin: "secret_integrity".into(),
      visibility: EntryVisibility::Private,
      kind_info: ParcelKind::Manifest("split_secret".to_string()),
   };
   /// Commit Manifest
   let manifest = ParcelManifest {
      data_hash,
      description,
      chunks: chunk_ehs,
   };
   let response = call_delivery_zome("commit_private_manifest", manifest)?;
   let eh: EntryHash = decode_response(response)?;
   /// Done
   return Ok(eh);
}


///
#[hdk_extern]
pub fn get_secret(eh: EntryHash) -> ExternResult<String> {
   /// Try to get Secret
   let maybe_secret: ExternResult<Secret> = get_typed_from_eh(eh.clone());
   if let Ok(secret) = maybe_secret {
      debug!("get_secret() - secret found");
      return Ok(secret.value);
   }
   debug!("get_secret() - Secret Entry not found, could be a ParcelManifest");
   /// Not a Secret Entry, could be a Manifest
   let maybe_manifest: ExternResult<ParcelManifest> = get_typed_from_eh(eh);
   let Ok(manifest) = maybe_manifest
      else { return error("No entry found at given EntryHash"); };
   /// Get all chunks
   let set: HashSet<_> = manifest.chunks.clone().drain(..).collect(); // dedup
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .entry_hashes(set);
   let records = query(query_args)?;
   if records.len() != manifest.chunks.len() {
      return error("Not all chunks have been found on chain");
   }
   // /// Concat all chunks
   // if manifest.data_type != "split_secret".to_owned() {
   //    return error("Manifest of an unknown entry type");
   // }
   let mut secret = String::new();
   for record in records {
      let chunk: ParcelChunk = get_typed_from_record(record)?;
      secret += &chunk.data;
      secret += ".";
   }
   /// Done
   Ok(secret)
}


/// Return list of parcels' EntryHash from a particular Agent
#[hdk_extern]
pub fn get_secrets_from(sender: AgentPubKey) -> ExternResult<Vec<EntryHash>> {
   debug!("get_secrets_from() START: {:?}", sender);
   let response = call_delivery_zome("pull_inbox", ())?;
   let inbox_items: Vec<ActionHash> = decode_response(response)?;
   debug!("get_secrets_from() - inbox_items: {}", inbox_items.len());
   debug!("get_secrets_from() - query_DeliveryNotice");
   let response = call_delivery_zome(
      "query_DeliveryNotice",
      DeliveryNoticeQueryField::Sender(sender),
   )?;
   let notices: Vec<(DeliveryNotice, Timestamp)> = decode_response(response)?;

   let mut parcels: Vec<EntryHash> = Vec::new();
   for (notice, _ts) in notices {
      let notice_eh = hash_entry(notice)?;
      let response = call_delivery_zome(
         "query_ReceptionProof",
         ReceptionProofQueryField::Notice(notice_eh),
      )?;
      let maybe_reception: Option<(EntryHash, Timestamp, ReceptionProof)> = decode_response(response)?;
      if let Some((_reception_eh, _ts, reception)) = maybe_reception {
         parcels.push(reception.parcel_eh);
      }
   }
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
      return zome_error!("No Secret found at given EntryHash");
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
//    let _: Vec<ActionHash> = decode_response(response)?;
//    Ok(())
// }
