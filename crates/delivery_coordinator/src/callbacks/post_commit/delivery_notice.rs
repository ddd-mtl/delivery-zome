use hdk::prelude::*;

use zome_delivery_types::*;
use crate::*;


///
pub fn post_commit_DeliveryNotice(entry: Entry, eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_DeliveryNotice() {:?}", eh);
    let notice = DeliveryNotice::try_from(entry)?;
    /// Emit Signal
    let res = emit_signal(&SignalProtocol::ReceivedNotice(notice));
    if let Err(err) = res.clone() {
        error!("Emit signal failed: {}", err);
    } else {
        debug!("Emit signal successful!");
    }
    /// Done
    Ok(())
}
