use hdk::prelude::*;


/// Entry representing a file chunk.
#[hdk_entry(id = "ParcelChunk", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct ParcelChunk {
    pub data: String,
}

// impl ParcelChunk {
//     ///
//     pub fn validate(&self, _maybe_package: Option<ValidationPackage>)
//         -> ExternResult<ValidateCallbackResult>
//     {
//         /// Check size
//         if self.data.len() > CHUNK_MAX_SIZE {
//             return Ok(ValidateCallbackResult::Invalid(
//                 format!("A chunk can't be bigger than {} KiB", CHUNK_MAX_SIZE / 1024)));
//         }
//         /// Done
//         Ok(ValidateCallbackResult::Valid)
//     }
//
//
//     ///
//     pub fn post_commit(chunk_eh: &EntryHash, _chunk: Self) -> ExternResult<()> {
//         /// Create ParcelReceived if we fetched all chunks
//         let response = call_self("check_manifest", chunk_eh)?;
//         debug!("check_manifest() response: {:?}", response);
//         assert!(matches!(response, ZomeCallResponse::Ok { .. }));
//         Ok(())
//     }
// }