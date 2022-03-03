use hdk::prelude::*;

use zome_utils::*;
use zome_delivery_types::*;

/// Call The Zome owner of the entry to commit it
pub fn call_commit_parcel(entry: Entry, notice: &DeliveryNotice, maybe_link_hh: Option<HeaderHash>)
    -> ExternResult<HeaderHash>
{
    let input = CommitParcelInput {
        entry_def_id: notice.parcel_summary.parcel_reference.entry_def_id(),
        entry,
        maybe_link_hh,
    };
    debug!("call_commit_parcel() zome_name = {:?}", notice.parcel_summary.parcel_reference.entry_zome_name());
    let response = call_remote(
        agent_info()?.agent_latest_pubkey,
        notice.parcel_summary.parcel_reference.entry_zome_name(),
        COMMIT_PARCEL_CALLBACK_NAME.into(),
        None,
        input.clone(),
    )?;
    let hh = decode_response(response)?;
    Ok(hh)
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
       .ok_or(WasmError::Guest(String::from("No AppEntry found at given payload address")))
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


// ///
// pub(crate) fn get_confirmations(package_eh: EntryHash) -> ExternResult<Vec<DeliveryConfirmation>> {
//     /// Get all InAck entries
//     let query_args = ChainQueryFilter::default()
//        .include_entries(true)
//        .header_type(HeaderType::Create)
//        .entry_type(EntryKind::DeliveryConfirmation.as_type());
//     let elements = query(query_args)?;
//     let mut confirmations = Vec::new();
//     //debug!("get_confirmations() elements.len(): {}", elements.len());
//     if elements.len() == 0 {
//         return Ok(Vec::new());
//     }
//     /// Filter for this package
//     for el in elements {
//         let confirmation = get_typed_from_el::<DeliveryConfirmation>(el)?;
//         if confirmation.package_eh == package_eh {
//             confirmations.push(confirmation)
//         }
//     }
//     /// Done
//     Ok(confirmations)
// }

//
// /// If no confirmation, and there is a pending/s link but no inbox link, create a DeliveryConfirmation
// /// Return true if a DeliveryConfirmation has been created
// pub(crate) fn try_confirming_pending_mail_has_been_received(package_eh: EntryHash, recipient: &AgentPubKey) -> ExternResult<bool> {
//     debug!("try_confirming_pending_mail_has_been_received() - START");
//     /// Check confirmations
//     let confirmations = get_confirmations(package_eh.clone())?;
//     if !confirmations.is_empty() {
//         return Ok(false);
//     }
//     let mut pending_found = false;
//     /// If a pending link and and inbox link match, still waiting for confirmation
//     let pendings_links = get_links(package_eh.clone(), Some(LinkKind::Pendings.as_tag()))?;
//     let inbox_links = get_links(recipient.to_owned().into(), LinkKind::MailInbox.as_tag_opt())?;
//     let inbox_targets: Vec<EntryHash> = inbox_links.iter().map(|x|x.target.clone()).collect();
//     for pendings_link in pendings_links.iter() {
//         let res = LinkKind::Pendings.unconcat_hash(&pendings_link.tag);
//         if let Ok(agent) = res {
//             // inbox link found ; check if tag is recipient
//             if &agent == recipient {
//                 pending_found = true;
//                 if inbox_targets.contains(&pendings_link.target) {
//                     return Ok(false);
//                 }
//             }
//         }
//     }
//     /// Create confirmation if conditions are met
//     if pending_found {
//         debug!("try_confirming_pending_mail_has_been_received() - CREATING CONFIRMATION");
//         let confirmation = DeliveryConfirmation::new(package_eh.clone(), recipient.clone());
//         let _ = create_entry(confirmation)?;
//         return Ok(true);
//     }
//     /// Done
//     Ok(false)
// }



// /// If no confirmation, and there is a pending/s link but no inbox link, create a DeliveryConfirmation
// /// Return true if a DeliveryConfirmation has been created
// pub(crate) fn try_confirming_pending_ack_has_been_received(package_eh: EntryHash, recipient: &AgentPubKey) -> ExternResult<bool> {
//     debug!("try_confirming_pending_ack_has_been_received() - START");
//     /// Check confirmations
//     let confirmations = get_confirmations(package_eh.clone())?;
//     if !confirmations.is_empty() {
//         return Ok(false);
//     }
//     /// If a pending link and and inbox link match, still waiting for confirmation
//     let pending_links = get_links(package_eh.clone(), Some(LinkKind::Pending.as_tag()))?;
//     for pending_link in pending_links.iter() {
//         if pending_link.tag != LinkKind::Pending.as_tag() {
//             continue;
//         }
//         /// Check for inbox link: If no link, it means it has been deleted by recipient
//         let links = get_links(recipient.to_owned().into(), LinkKind::AckInbox.as_tag_opt())?;
//         for link in links.iter() {
//             let res = LinkKind::AckInbox.unconcat_hash(&link.tag);
//             if let Ok(agent) = res {
//                 // inbox link found ; check if tag is recipient
//                 if &agent == recipient {
//                     return Ok(false);
//                 }
//             }
//         }
//         /// Create confirmation since Pending found but not inbox link
//         debug!("try_confirming_pending_ack_has_been_received() - CREATING CONFIRMATION");
//         let confirmation = DeliveryConfirmation::new(package_eh.clone(), recipient.clone());
//         let _ = create_entry(confirmation)?;
//         return Ok(true);
//     }
//     Ok(false)
// }
