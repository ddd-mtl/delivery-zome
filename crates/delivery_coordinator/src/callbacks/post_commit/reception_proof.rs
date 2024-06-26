use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;

///
pub fn post_commit_create_ReceptionProof(_sah: &SignedActionHashed, eh: &EntryHash, entry: Entry) -> ExternResult<()> {
   debug!("post_commit_ReceptionProof() {:?}", eh);
   let reception_proof = ReceptionProof::try_from(entry)?;
   /// Get DeliveryNotice
   let notice: DeliveryNotice = get_typed_from_eh(reception_proof.notice_eh.clone())?;
   /// Create PendingItem
   let pending_item = pack_reception_proof(
      reception_proof.clone(),
      notice.distribution_ah.clone(),
      notice.sender.clone(),
   )?;
   /// Send it to sender
   let _ = send_item(
      notice.sender,
      pending_item,
      notice.summary.distribution_strategy,
   )?;
   /// Done
   Ok(())
}
