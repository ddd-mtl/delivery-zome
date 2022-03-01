#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


mod query;
mod system;


pub use query::*;

use hdk::prelude::*;

use holo_hash::*;
use std::convert::TryFrom;

pub type TypedEntryAndHash<T> = (T, HeaderHash, EntryHash);
pub type OptionTypedEntryAndHash<T> = Option<TypedEntryAndHash<T>>;


pub fn snip(agent: &AgentPubKey) -> String {
   //format!("{:?}", agent)[12..24].to_string()
   //format!("{}", agent)[..12].to_string()
   let b64: AgentPubKeyB64 = AgentPubKeyB64::from(agent.clone());
   format!("{:?}", b64)[24..36].to_string()
}


///
pub fn get_context() -> String {
   let mut msg = String::new();
   let maybe_zome_info = zome_info();
   if let Ok(zome_info) = maybe_zome_info {
      let maybe_call_info = call_info();
      if let Ok(call_info) = maybe_call_info {
         let provenance = snip(&call_info.provenance);
         msg.push_str(&format!("\n\nPanic during zome call '{}::{}()' by {} ",
                               zome_info.name, call_info.function_name, provenance));
      }
   }
   let maybe_agent_info = agent_info();
   if let Ok(agent_info) = maybe_agent_info {
      msg.push_str(&format!("in chain of {}", snip(&agent_info.agent_latest_pubkey)));
   }
   msg
}


pub fn my_panic_hook(info: &std::panic::PanicInfo) {
   let mut msg = info.to_string();
   msg.push_str(&get_context());
   error!("{}\n\n", &msg);
}


pub fn error<T>(reason: &str) -> ExternResult<T> {
   let msg = format!("{} ; Context: {}", reason, get_context());
   Err(WasmError::Guest(msg))
}


pub fn invalid(reason: &str) -> ExternResult<ValidateCallbackResult> {
   Ok(ValidateCallbackResult::Invalid(reason.to_string()))
}


/// Returns number of seconds since UNIX_EPOCH
pub fn now() -> u64 {
   let now = sys_time().expect("sys_time() should always work");
   now.as_seconds_and_nanos().0 as u64
}

/// Remote call to self
pub fn call_self<I>(fn_name: &str, payload: I) -> ExternResult<ZomeCallResponse>
   where
      I: serde::Serialize + std::fmt::Debug
{
   call_remote(
      agent_info()?.agent_latest_pubkey,
      zome_info()?.name,
      fn_name.to_string().into(),
      None,
      payload,
   )
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


/// Get EntryType out of an Entry & EntryHash
pub fn determine_entry_type(eh: EntryHash, entry: &Entry) -> ExternResult<EntryType> {
   Ok(match entry {
      Entry::Agent(_agent_hash) => EntryType::AgentPubKey,
      Entry::CapClaim(_claim) => EntryType::CapClaim,
      Entry::CapGrant(_grant) => EntryType::CapGrant,
      Entry::App(_entry_bytes) => get_entry_type_from_eh(eh)?,
      Entry::CounterSign(_data, _bytes) => unreachable!("CounterSign"),
   })
}

/// Get Element at address using query()
pub fn get_entry_type_from_eh(eh: EntryHash) -> ExternResult<EntryType> {
   let maybe_element = get(eh, GetOptions::latest())?;
   if maybe_element.is_none() {
      return error("no element found for entry_hash");
   }
   let element = maybe_element.unwrap();
   let entry_type = element.header().entry_type().unwrap().clone();
   Ok(entry_type)
}

/// Get Element at address using query()
pub fn get_local_from_hh(hh: HeaderHash) -> ExternResult<Element> {
   let query_args = ChainQueryFilter::default()
      .include_entries(true);
   let maybe_vec = query(query_args);
   if let Err(err) = maybe_vec {
      return error(&format!("{:?}",err));
   }
   let vec = maybe_vec.unwrap();
   for element in vec {
      if element.header_address() == &hh {
         return Ok(element.clone());
      }
   }
   return error("Element not found at given HeaderHash");
}

/// Get Element at address using query()
pub fn get_local_from_eh(eh: EntryHash) -> ExternResult<Element> {
   let mut set = HashSet::with_capacity(1);
   set.insert(eh);
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .entry_hashes(set);
   let vec = query(query_args)?;
   if vec.len() != 1 {
      return error("Element not found at given EntryHash");
   }
   Ok(vec[0].clone())
}


/// Get EntryHash for Element
pub fn get_eh(element: &Element) -> ExternResult<EntryHash> {
   let maybe_eh = element.header().entry_hash();
   if let None = maybe_eh {
      warn!("get_eh(): entry_hash not found");
      return error("get_eh(): entry_hash not found");
   }
   Ok(maybe_eh.unwrap().clone())
}

/// Call get() to obtain EntryHash from a HeaderHash
pub fn hh_to_eh(hh: HeaderHash) -> ExternResult<EntryHash> {
   trace!("hh_to_eh(): START - get...");
   let maybe_element = get(hh, GetOptions::content())?;
   trace!("hh_to_eh(): START - get DONE");
   if let None = maybe_element {
      warn!("hh_to_eh(): Element not found");
      return error("hh_to_eh(): Element not found");
   }
   return get_eh(&maybe_element.unwrap());
}


/// Call get() to obtain EntryHash and AppEntry from a HeaderHash
pub fn get_typed_from_hh<T: TryFrom<Entry>>(hash: HeaderHash)
   -> ExternResult<(EntryHash, T)>
{
   match get(hash.clone(), GetOptions::content())? {
      Some(element) => {
         let eh = element.header().entry_hash().expect("Converting HeaderHash which does not have an Entry");
         Ok((eh.clone(), get_typed_from_el(element)?))
      },
      None => error("get_typed_from_hh(): Entry not found"),
   }
}


/// Call get() to obtain EntryHash and AppEntry from an EntryHash
pub fn get_typed_from_eh<T: TryFrom<Entry>>(eh: EntryHash) -> ExternResult<T> {
   match get(eh, GetOptions::content())? {
      Some(element) => Ok(get_typed_from_el(element)?),
      None => error("get_typed_from_eh(): Entry not found"),
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

/// Obtain latest AppEntry at EntryHash and get its author
/// Conditions: Must be a single author entry type
pub fn get_typed_and_author<T: TryFrom<Entry>>(eh: &EntryHash)
   -> ExternResult<(AgentPubKey, T)>
{
   let maybe_maybe_element = get(eh.clone(), GetOptions::latest());
   if let Err(err) = maybe_maybe_element {
      warn!("Failed getting element: {}", err);
      return Err(err);
   }
   let maybe_element = maybe_maybe_element.unwrap();
   if maybe_element.is_none() {
      return error("no element found at address");
   }
   let element = maybe_element.unwrap();
   //assert!(entry_item.headers.len() > 0);
   //assert!(entry_item.headers[0].provenances().len() > 0);
   let author = element.header().author();
   let app_entry = get_typed_from_el::<T>(element.clone())?;
   Ok((author.clone(), app_entry))
}


// #[derive(Serialize, Deserialize, SerializedBytes)]
// struct StringLinkTag(String);
// pub fn link_tag(tag: &str) -> LinkTag {
//     let sb: SerializedBytes = StringLinkTag(tag.into())
//        .try_into()
//        .expect("StringLinkTag should convert to SerializedBytes");
//     LinkTag(sb.bytes().clone())
// }

/// From Connor @acorn ///

pub fn get_header_hash(shh: element::SignedHeaderHashed) -> HeaderHash {
   shh.header_hashed().as_hash().to_owned()
}

///
pub fn get_latest_typed_from_eh<T: TryFrom<SerializedBytes, Error = SerializedBytesError>>(
   entry_hash: EntryHash,
) -> ExternResult<OptionTypedEntryAndHash<T>> {
   /// First, make sure we DO have the latest header_hash address
   let maybe_latest_header_hash = match get_details(entry_hash.clone(), GetOptions::latest())? {
      Some(Details::Entry(details)) => match details.entry_dht_status {
         metadata::EntryDhtStatus::Live => match details.updates.len() {
            // pass out the header associated with this entry
            0 => Some(get_header_hash(details.headers.first().unwrap().to_owned())),
            _ => {
               let mut sortlist = details.updates.to_vec();
               // unix timestamp should work for sorting
               sortlist.sort_by_key(|update| update.header().timestamp().as_micros());
               // sorts in ascending order, so take the last element
               let last = sortlist.last().unwrap().to_owned();
               Some(get_header_hash(last))
            }
         },
         metadata::EntryDhtStatus::Dead => None,
         _ => None,
      },
      _ => None,
   };
   let latest_header_hash = match maybe_latest_header_hash {
      None => return Ok(None),
      Some(hh) => hh,
   };
   /// Second, go and get that element, and return its entry and header_address
   let maybe_latest_element = get(latest_header_hash, GetOptions::latest())?;
   let element = match maybe_latest_element {
      None => return Ok(None),
      Some(el) => el,
   };
   let maybe_typed_entry = element.entry().to_app_option::<T>()?;
   let entry = match maybe_typed_entry {
      None => return Ok(None),
      Some(e) => e,
   };
   let hh = match element.header() {
      /// we DO want to return the header for the original instead of the updated
      Header::Update(update) => update.original_header_address.clone(),
      Header::Create(_) => element.header_address().clone(),
      _ => unreachable!("Can't have returned a header for a nonexistent entry"),
   };
   let eh =  element.header().entry_hash().unwrap().to_owned();
   /// Done
   Ok(Some((entry, hh, eh)))
}


//----------------------------------------------------------------------------------------
// Copied from hc-utils
//----------------------------------------------------------------------------------------

/// optimized get details by links
pub fn my_get_details(links: Vec<Link>, option: GetOptions) -> ExternResult<Vec<Option<Details>>> {
   let msg_results_input: Vec<GetInput> = links
      .into_iter()
      .map(|link| GetInput::new(link.target.into(), option.clone()))
      .collect();
   let res = HDK.with(|hdk| hdk.borrow().get_details(msg_results_input))?;
   Ok(res)
}

///
pub fn get_links_and_load_type<R: TryFrom<Entry>>(
   base: EntryHash,
   tag: Option<LinkTag>,
   //include_latest_updated_entry: bool,
) -> ExternResult<Vec<R>> {
   let links = get_links(base.into(), tag)?;
   debug!("get_links_and_load_type() links found: {}", links.len());
   let all_results_elements = my_get_details(links, GetOptions::default())?;
   let res = all_results_elements
      .iter()
      .flat_map(|details| match details {
         Some(Details::Entry(EntryDetails { entry, .. })) => {
            match R::try_from(entry.clone()) {
               Ok(r) => Ok(r),
               Err(_) => error(
                  "Could not convert get_links result to requested type",
               ),
            }
         }
         _ => error("get_links did not return an app entry"),
      })
      .collect();
   Ok(res)
}