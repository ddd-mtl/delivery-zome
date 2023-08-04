use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;


///
pub fn get_all_inbox_items(maybe_kind: Option<ItemKind>) -> ExternResult<Vec<(PendingItem, Link)>> {
    /// Get typed targets
    let my_agent_eh = EntryHash::from(agent_info()?.agent_latest_pubkey);
    let mut pending_pairs = get_typed_from_links::<PendingItem>(
        my_agent_eh.clone(),
        LinkTypes::Inbox,
        None,
    )?;
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
    let maybe_element = get(eh, GetOptions::content())?;
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


// ///
// pub fn sign_parcel(parcel: &Parcel) -> ExternResult<Signature> {
//     let me = agent_info()?.agent_latest_pubkey;
//     let signature = sign(me, parcel)?;
//     Ok(signature)
// }
