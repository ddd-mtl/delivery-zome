use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_types::*;
use crate::*;

///
pub fn post_commit_create_ReceptionProof(_sah: &SignedActionHashed, create: &Create, entry: Entry) -> ExternResult<DeliveryEntryKind> {
   debug!("post_commit_ReceptionProof() {:?}", create.entry_hash);
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
   Ok(DeliveryEntryKind::ReceptionProof(reception_proof))
}
