use hdk::prelude::*;


#[hdk_entry(id = "ParcelManifest", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelManifest {
   pub name: String,
   pub entry_id: String,
   //pub data_hash: String,
   pub size: usize,
   pub chunks: Vec<EntryHash>,
}
