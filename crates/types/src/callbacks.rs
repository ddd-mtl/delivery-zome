//! Extra callbacks that will be called by the delivery zome

use hdk::prelude::*;

pub const COMMIT_PARCEL_CALLBACK_NAME: &'static str = "commit_parcel";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitParcelInput {
   pub entry_def_id: EntryDefId,
   pub entry: Entry,
   pub maybe_link_hh: Option<HeaderHash>,
}

/// Zome Function Callback required by delivery-zome
/// Should not be called directly. Only via remote call to self
/// Name function must match COMMIT_PARCEL_CALLBACK_NAME global const
#[hdk_extern]
fn commit_parcel(input: CommitParcelInput) -> ExternResult<HeaderHash> {
   debug!("commit_parcel() entry_def_id = {:?} | {}", input.entry_def_id, zome_info()?.name);
   /// Create CreateInput
   let create_input = CreateInput::new(
      input.entry_def_id,
      input.entry,
      ChainTopOrdering::Relaxed, // Strict //Relaxed
   );
   /// Commit Parcel
   let parcel_hh = create_entry(create_input)?;
   /// Delete Link
   if let Some(link_hh) = input.maybe_link_hh {
      let _hh = delete_link(link_hh)?;
   }
   /// Done
   Ok(parcel_hh)
}
