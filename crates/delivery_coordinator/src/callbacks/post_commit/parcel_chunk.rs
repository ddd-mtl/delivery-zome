use hdk::prelude::*;
//use zome_utils::*;
use zome_delivery_types::*;
use crate::SignalProtocol;


// ///
// pub fn post_commit_ParcelChunk(entry: Entry, chunk_eh: &EntryHash) -> ExternResult<()> {
//     debug!("post_commit_ParcelChunk() {:?}", chunk_eh);
//     let _ = ParcelChunk::try_from(entry)?;
//     /// Check if parcel completely received. Will automatically create ReceptionProof if complete
//     let response = call_self("check_manifest", chunk_eh)?;
//     debug!("check_manifest() response: {:?}", response);
//
//     let maybe_result: ExternResult<Option<Vec<(EntryHash, Result<EntryHash, usize>)>>> = decode_response(response);
//     /// Notify UI of completion status
//     if let Ok(Some(result)) = maybe_result {
//         //debug!("result = {:?}", result);
//         if let Err(pct) = result[0].1 {
//             let notice_ehs = result.into_iter().map(|(eh, _pct)| eh).collect();
//             let res = emit_signal(&SignalProtocol::ReceivedChunk((notice_ehs, pct)));
//             if let Err(err) = res {
//                 error!("Emit signal failed: {}", err);
//             }
//         }
//     }
//     Ok(())
// }



///
pub fn post_commit_ParcelChunk(_sah: &SignedActionHashed, entry: Entry, chunk_eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_ParcelChunk() {:?}", chunk_eh);
    let chunk = ParcelChunk::try_from(entry)?;
    /// Emit signal
    let res = emit_signal(&SignalProtocol::NewChunk((chunk_eh.to_owned(), chunk.clone())));
    if let Err(err) = res {
        error!("Emit signal failed: {}", err);
    }
    Ok(())
}