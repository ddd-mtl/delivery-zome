use hdk::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitParcelInput {
   pub entry_def_id: EntryDefId,
   pub entry: Entry,
}

/// Zome Function Callback required by Delivery-zome
#[hdk_extern]
pub fn commit_parcel(input: CommitParcelInput) -> ExternResult<HeaderHash> {
   debug!("commit_parcel() entry_def_id = {:?} | {}", input.entry_def_id, zome_info()?.name);
   /// Create CreateInput
   //let parcel_eh = hash_entry(input.entry.clone())?;
   let create_input = CreateInput {
      entry_def_id: input.entry_def_id,
      entry: input.entry,
      chain_top_ordering: ChainTopOrdering::Strict,
   };
   /// Commit Parcel
   let parcel_hh = create_entry(create_input)?;
   return Ok(parcel_hh);
}
