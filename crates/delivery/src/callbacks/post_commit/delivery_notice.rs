use hdk::prelude::*;

use zome_delivery_integrity::*;
use zome_delivery_types::*;

use crate::send_item::*;
use crate::functions::*;
use crate::SignalProtocol;


///
pub fn post_commit_DeliveryNotice(entry: Entry, eh: &EntryHash) -> ExternResult<()> {
    debug!("post_commit_DeliveryNotice() {:?}", eh);
    let notice = DeliveryNotice::try_from(entry)?;
    /// Emit Signal
    let res = emit_signal(&SignalProtocol::ReceivedNotice(notice));
    if let Err(err) = res {
        error!("Emit signal failed: {}", err);
    }
    /// Done
    Ok(())
}
