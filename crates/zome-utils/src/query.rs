use hdk::prelude::*;
use crate::*;

///
pub fn get_all_typed_local<R: TryFrom<Entry>>(entry_type: EntryType) -> ExternResult<Vec<R>> {
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .header_type(HeaderType::Create)
      .entry_type(entry_type);
   let els = query(query_args)?;
   let mut typeds = Vec::new();
   for el in els {
      let typed: R = get_typed_from_el(el.clone())?;
      typeds.push(typed)
   }
   /// Done
   Ok(typeds)
}