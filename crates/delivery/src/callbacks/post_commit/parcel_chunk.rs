use hdk::prelude::*;
use zome_delivery_types::*;
use zome_utils::*;

use crate::zome_entry_trait::*;
use crate::constants::*;


impl ZomeEntry for ParcelChunk {


    ///
    fn post_commit(&self, chunk_eh: &EntryHash) -> ExternResult<()> {
        debug!("post_commit_ParcelChunk() {:?}", chunk_eh);
        /// Create ParcelReceived if we fetched all chunks
        let response = call_self("check_manifest", chunk_eh)?;
        debug!("check_manifest() response: {:?}", response);
        assert!(matches!(response, ZomeCallResponse::Ok { .. }));
        Ok(())
    }
}