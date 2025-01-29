use hdk::prelude::*;
use zome_utils::*;
//use zome_delivery_integrity::*;
use zome_delivery_types::*;


///
fn create_PendingItem<T>(
   kind: ItemKind,
   content: T,
   distribution_ah: ActionHash,
   recipient: ActionHash,
) -> ExternResult<PendingItem>
   where
      T: serde::Serialize + Clone + Sized + std::fmt::Debug
{
   //debug!("create_pending_item() {:?} for {}", kind, snip(&recipient));
   assert!(kind != ItemKind::AppEntryBytes);
   let author = agent_info()?.agent_latest_pubkey;
   /// Sign content
   let author_signature = sign(author.clone(), content.clone())
      .expect("Should be able to sign with my key");
   /// Serialize
   let data: XSalsa20Poly1305Data = bincode::serialize(&content).unwrap().into();
   /// Encrypt
   let encrypted_data = ed_25519_x_salsa20_poly1305_encrypt(
      agent_info()?.agent_latest_pubkey, recipient, data)
     .expect("Encryption should work");
   /// Done
   let item = PendingItem {
      kind,
      author,
      encrypted_data,
      distribution_ah,
      author_signature,
   };
   Ok(item)
}


///
fn create_pending_parcel(
   kind: ItemKind,
   entry_bytes: AppEntryBytes,
   distribution_ah: ActionHash,
   recipient: AgentPubKey,
) -> ExternResult<PendingItem>
{
   debug!("create_pending_item() {:?} for {}", kind, snip(&recipient));
   let me = agent_info()?.agent_latest_pubkey;
   /// Sign content
   let author_signature = sign(me.clone(), entry_bytes.clone())
      .expect("Should be able to sign with my key");
   /// Serialize
   let bytes =  entry_bytes.into_sb().bytes().to_owned();
   trace!("create_pending_parcel() bytes: {:?}", bytes);
   let data: XSalsa20Poly1305Data = XSalsa20Poly1305Data::from(bytes);
   /// Encrypt
   let encrypted_data = ed_25519_x_salsa20_poly1305_encrypt(agent_info()?.agent_latest_pubkey, recipient, data)?;
   /// Done
   let item = PendingItem {
      kind,
      author: me,
      encrypted_data,
      distribution_ah,
      author_signature,
   };
   Ok(item)
}


/// called from post_commit()
pub fn pack_notice(notice: DeliveryNotice, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<DeliveryNotice>(ItemKind::DeliveryNotice, notice.clone(), notice.distribution_ah.clone(), recipient)
}
/// called from post_commit()
pub fn pack_notice_ack(ack: NoticeAck, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<NoticeAck>(ItemKind::NoticeAck, ack.clone(), ack.distribution_ah.clone(), recipient)
}
/// called from post_commit()
pub fn pack_reply(reply: NoticeReply, distribution_ah: ActionHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<NoticeReply>(ItemKind::NoticeReply, reply.clone(), distribution_ah, recipient)
}
/// called from post_commit()
pub fn pack_reception_proof(reception: ReceptionProof, distribution_ah: ActionHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<ReceptionProof>(ItemKind::ReceptionProof, reception, distribution_ah, recipient)
}
/// called from post_commit()
pub fn pack_entry(parcel_entry: Entry, distribution_ah: ActionHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   if let Entry::App(entry_bytes) = parcel_entry {
      return create_pending_parcel(ItemKind::AppEntryBytes, entry_bytes, distribution_ah, recipient);
   }
   return error("Can only pack Entry::App entries");
}
/// called from post_commit()
pub fn pack_chunk(chunk: ParcelChunk, distribution_ah: ActionHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<ParcelChunk>(ItemKind::ParcelChunk, chunk, distribution_ah, recipient)
}
