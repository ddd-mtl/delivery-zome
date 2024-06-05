use hdk::prelude::*;
use zome_utils::*;

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
  let mut signals: Vec<DeliverySignalProtocol> = Vec::new();
  for (create_sah, maybe_deletes) in links.into_inner() {
    let Action::CreateLink(create) = create_sah.hashed.content
      else { panic!("get_link_details() should return a CreateLink Action") };
    let pr_eh = EntryHash::try_from(create.target_address).unwrap();
    let Ok(Some(Details::Entry(details))) = get_details(pr_eh.clone(), GetOptions::network())
      else { continue };
    let Ok(pr) = ParcelReference::try_from(details.entry)
      else { continue };
    signals.push(DeliverySignalProtocol::NewPublicParcel((pr_eh.clone(), create.timestamp, pr.clone(), create.author)));
    if maybe_deletes.len() > 0 {
      let Action::DeleteLink(delete) = maybe_deletes[0].clone().hashed.content
        else { panic!("get_link_details() should return a DeleteLink Action") };
      signals.push(DeliverySignalProtocol::RemovedPublicParcel((pr_eh, delete.timestamp, pr, delete.author)));
    }
  }
  debug!(" signals count: {}", signals.len());
  /// Return as signals
  emit_self_signals(signals)?;
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
