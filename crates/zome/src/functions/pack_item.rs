use hdk::prelude::*;

use zome_delivery_types::*;
use zome_utils::*;

fn create_PendingItem<T>(
   kind: ItemKind,
   content: T,
   distribution_eh: EntryHash,
   recipient: AgentPubKey,
) -> ExternResult<PendingItem>
   where
      T: serde::Serialize + Clone + Sized + std::fmt::Debug
{
   debug!("create_pending_item() {:?} for {}", kind, snip(&recipient));
   assert!(kind != ItemKind::AppEntryBytes);
   let me = agent_info()?.agent_latest_pubkey;
   /// Sign content
   let author_signature = sign(me.clone(), content.clone())
      .expect("Should be able to sign with my key");
   /// Serialize
   let data: XSalsa20Poly1305Data = bincode::serialize(&content).unwrap().into();
   /// Encrypt
   let encrypted_data = encrypt_parcel(data, recipient)?;
   /// Done
   let item = PendingItem {
      kind,
      author: me,
      encrypted_data,
      distribution_eh,
      author_signature,
   };
   Ok(item)
}

///
fn create_pending_parcel(
   kind: ItemKind,
   entry_bytes: AppEntryBytes,
   distribution_eh: EntryHash,
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
   let encrypted_data = encrypt_parcel(data, recipient)?;
   /// Done
   let item = PendingItem {
      kind,
      author: me,
      encrypted_data,
      distribution_eh,
      author_signature,
   };
   Ok(item)
}


/// Encrypt some data with my encryption key and the recipient's public encryption key
/// called from post_commit()
fn encrypt_parcel(
   data: XSalsa20Poly1305Data,
   recipient: AgentPubKey,
) -> ExternResult<XSalsa20Poly1305EncryptedData> {
   /// Get my key
   trace!("get_enc_key() for sender {:?}", agent_info()?.agent_latest_pubkey);
   let response = call_self("get_enc_key", agent_info()?.agent_latest_pubkey)?;
   trace!("get_enc_key() for sender result: {:?}", response);
   let sender_key: X25519PubKey = decode_response(response)?;
   trace!("PendingItem: sender_key found");
   /// Get recipient's key
   trace!("get_enc_key() for recipient {:?}", recipient);
   let response = call_self("get_enc_key", recipient.clone())?;
   trace!("get_enc_key() for recipient result: {:?}", response);
   let recipient_key: X25519PubKey = decode_response(response)?;
   trace!("PendingItem: recipient_key found");
   /// Encrypt
   let encrypted_data = x_25519_x_salsa20_poly1305_encrypt(sender_key, recipient_key, data)
      .expect("Encryption should work");
   //trace!("Encrypted: {:?}", encrypted_data.clone());
   Ok(encrypted_data)
}


/// called from post_commit()
pub fn pack_notice(notice: DeliveryNotice, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<DeliveryNotice>(ItemKind::DeliveryNotice, notice.clone(), notice.distribution_eh.clone(), recipient)
}
/// called from post_commit()
pub fn pack_reply(reply: DeliveryReply, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<DeliveryReply>(ItemKind::DeliveryReply, reply.clone(), distribution_eh, recipient)
}
/// called from post_commit()
pub fn pack_reception(reception: ParcelReceived, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<ParcelReceived>(ItemKind::ParcelReceived, reception, distribution_eh, recipient)
}
/// called from post_commit()
pub fn pack_entry(parcel_entry: Entry, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   if let Entry::App(entry_bytes) = parcel_entry {
      return create_pending_parcel(ItemKind::AppEntryBytes, entry_bytes, distribution_eh, recipient);
   }
   return error("Can only pack Entry::App entries");
}
/// called from post_commit()
pub fn pack_chunk(chunk: ParcelChunk, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<ParcelChunk>(ItemKind::ParcelChunk, chunk, distribution_eh, recipient)
}