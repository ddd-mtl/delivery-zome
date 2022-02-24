use hdk::prelude::*;


/// Entry representing a file chunk.
#[hdk_entry(id = "ParcelChunk", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelChunk {
    pub data: String,
}

