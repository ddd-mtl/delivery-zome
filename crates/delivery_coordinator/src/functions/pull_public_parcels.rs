use hdk::prelude::*;
use zome_utils::*;
use zome_signals::*;
use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;


// /// Get All public Parcels
// #[hdk_extern]
// pub fn pull_public_parcels(_:()) -> ExternResult<Vec<(EntryHash, ParcelReference, Timestamp, AgentPubKey)>> {
//    std::panic::set_hook(Box::new(zome_panic_hook));
//    let anchor = public_parcels_path().path_entry_hash()?;
//    let pps: Vec<(EntryHash, ParcelReference, Timestamp, AgentPubKey)> = get_typed_from_links::<ParcelReference>(link_input(anchor, LinkTypes::PublicParcels, None))
//       .map_err(|err| wasm_error!(WasmErrorInner::Guest(err.to_string())))?
//       .into_iter()
//       .map(|(pp, link)| (EntryHash::try_from(link.target).unwrap(), pp, link.timestamp, link.author)).collect();
//    debug!(" pps count: {}", pps.len());
//    /// Done
//    Ok(pps)
// }


#[hdk_extern]
pub fn pull_public_parcels_details(_:()) -> ExternResult<()> {
  std::panic::set_hook(Box::new(zome_panic_hook));
  let anchor_eh = public_parcels_path().path_entry_hash()?;
  debug!("pull_public_parcels_details() {}", anchor_eh);
  let links = get_link_details(anchor_eh, LinkTypes::PublicParcels, None, GetOptions::network())?;
  //debug!(" pull_public_parcels_details() get_link_details = {}", links.clone().into_inner().len());
  debug!("   links count: {}", links.clone().into_inner().len());
  let mut pulses: Vec<ZomeSignalProtocol> = Vec::new();
  for (create_sah, maybe_deletes) in links.into_inner() {
    let Action::CreateLink(create_link) = create_sah.hashed.content
      else { panic!("get_link_details() should return a CreateLink Action") };
    let pr_eh = EntryHash::try_from(create_link.target_address.clone()).unwrap();
    let Ok(Some(Details::Entry(details))) = get_details(pr_eh.clone(), GetOptions::network())
      else { continue };
    let Ok(_pr) = ParcelReference::try_from(details.entry.clone())
      else { warn!("CreateLink to an entry which is not a ParcelReference"); continue };
    assert!(!details.actions.is_empty());
    let create_sah = details.actions[0].clone();
    let create_record = Record::new(create_sah.clone(), Some(details.entry.clone()));
    //let pr_eh = create_sah.action().entry_address();
    //let pr_eh = AnyDhtHash::try_from(hash_entry(pr.clone())?).unwrap();
    let entry_pulse = EntryPulse::try_from_new_record(create_record, ValidatedBy::Network, false)?;
    //let first = (EntryPulse {hash: pr_eh.clone(), author: create_link.author, ts: create_link.timestamp, state: EntryStateChange::Created}, kind.clone());
    pulses.push(ZomeSignalProtocol::Entry(entry_pulse));
    if maybe_deletes.len() > 0 {
      let Action::DeleteLink(delete) = maybe_deletes[0].clone().hashed.content
        else { panic!("get_link_details() should return a DeleteLink Action") };
      //let second = (EntryPulse {hash: pr_eh.clone(), author: delete.author, ts: delete.timestamp, state: EntryStateChange::Deleted}, kind);
      let entry_pulse = EntryPulse::try_from_delete_record(create_sah.hashed, details.entry, ValidatedBy::Network, false)?;
      pulses.push(ZomeSignalProtocol::Entry(entry_pulse));
      let link_pulse = LinkPulse { link: link_from_delete(&delete, &create_link), state: StateChange::Delete(false), validation: ValidatedBy::Network };
      pulses.push(ZomeSignalProtocol::Link(link_pulse));
    }
  }
  debug!(" pulses count: {}", pulses.len());
  /// Return as signal
  emit_zome_signal(pulses)?;
  /// Done
  Ok(())
}


#[hdk_extern]
pub fn fetch_parcel_ref(pr_eh : EntryHash) -> ExternResult<Option<ParcelReference>> {
  let wtf = get_details(pr_eh, GetOptions::network())?;
  let Some(Details::Entry(details)) = wtf
    else {return Ok(None)};
  let typed = ParcelReference::try_from(details.entry)?;
  Ok(Some(typed))
}
