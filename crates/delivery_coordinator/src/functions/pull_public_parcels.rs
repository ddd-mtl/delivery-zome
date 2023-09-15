use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_integrity::*;
use crate::*;

///
/// Get All public Parcels
/// Return ActionHashs of parcels committed during the pull
#[hdk_extern]
pub fn pull_public_parcels(_:()) -> ExternResult<Vec<(EntryHash, ParcelDescription)>> {
   let anchor = public_parcels_path().path_entry_hash()?;
   let public_parcel_pairs = get_typed_from_links::<ParcelDescription>(anchor, LinkTypes::PublicParcels, None)
      .map_err(|err| wasm_error!(WasmErrorInner::Guest(err.to_string())))?
      .into_iter()
      .map(|(pp, _link)| (hash_entry(pp.clone()).unwrap(), pp)).collect();
   Ok(public_parcel_pairs)
}