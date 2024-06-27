use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;



/// Check if an entry is present in source-chain
pub fn has_entry(eh: EntryHash) -> ExternResult<bool> {
    let mut set: HashSet<EntryHash> = HashSet::new();
    set.insert(eh);
    let query_args = ChainQueryFilter::default()
       .include_entries(false)
       .entry_hashes(set);
    let records = query(query_args)?;
    Ok(!records.is_empty())
}


/// Find manifest with that chunk_eh
pub fn find_notice_with_parcel(parcel_eh: EntryHash) -> ExternResult<Vec<DeliveryNotice>> {
    /// Get all Create DeliveryNotice Elements with query
    let query_args = ChainQueryFilter::default()
       .include_entries(true)
       .action_type(ActionType::Create)
       .entry_type(DeliveryEntryTypes::DeliveryNotice.try_into().unwrap());
    let notices = query(query_args)?;
    let mut res = Vec::new();
    for notice_el in notices {
        let notice: DeliveryNotice = get_typed_from_record(notice_el)?;
        let summary_eh = &notice.summary.parcel_reference.parcel_eh;
        if summary_eh == &parcel_eh {
            res.push(notice);
        }
    }
    /// Done
    Ok(res)
}


/// Return percentage of chunks received
/// 100 = all chunks received
pub fn count_chunks_received(manifest_eh: EntryHash) -> ExternResult<usize> {
    /// Get ParcelManifest
    let manifest: ParcelManifest = get_typed_from_eh(manifest_eh)?;
    let len = manifest.chunks.len();
    let chunks_set: HashSet<EntryHash> = HashSet::from_iter(manifest.chunks);
    /// Get all Create ParcelChunk Elements with query
    let query_args = ChainQueryFilter::default()
       .include_entries(false)
       .entry_hashes(chunks_set);
    let chunk_els = query(query_args)?;
    /// Check if all found
    debug!("has_all_chunks: {} == {} ?", chunk_els.len(), len);
    let pct: f32 = chunk_els.len() as f32 / len as f32;
    let iPct: usize = (pct * 100_f32).ceil() as usize;
    debug!("pct == {} ?", pct);
    Ok(iPct)
}


///
pub fn probe_all_inbox_items(maybe_kind: Option<ItemKind>) -> ExternResult<Vec<(PendingItem, Link)>> {
    /// Get typed targets
    let my_agent_eh = EntryHash::from(agent_info()?.agent_latest_pubkey);
    let mut pending_pairs = get_typed_from_links::<PendingItem>(link_input(
        my_agent_eh.clone(),
        LinkTypes::Inbox,
        None,
    ))?;
    /// Filter
    if maybe_kind.is_some() {
        let kind = maybe_kind.unwrap();
        pending_pairs.retain(|pair|  pair.0.kind == kind)
    }
    /// Done
    Ok(pending_pairs)
}


/// Return size of an AppEntry
pub fn get_app_entry_size(eh: EntryHash) -> ExternResult<usize> {
    /// Get Element
    let maybe_element = get(eh, GetOptions::network())?;
    let element = match maybe_element {
        Some(el) => el,
        None => return error("No element found at given payload address"),
    };
    /// Get length of SerializedBytes
    let entry = element
       .entry()
       .as_option()
       .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("No AppEntry found at given payload address"))))
       ?
       .to_owned();
    if let Entry::App(app_bytes) = entry {
        let size: usize = app_bytes
           .into_sb()
           .bytes()
           .len();
        /// Done
        return Ok(size);
    }
    error("Entry not of type App()")
}


///
pub fn public_parcels_path() -> TypedPath {
    Path::from("public-parcels").typed(LinkTypes::PublicParcels).unwrap()
}


// ///
// pub fn sign_parcel(parcel: &Parcel) -> ExternResult<Signature> {
//     let me = agent_info()?.agent_latest_pubkey;
//     let signature = sign(me, parcel)?;
//     Ok(signature)
// }
