use hdk::prelude::*;

use zome_delivery_types::*;
use crate::utils::*;


/// Create PendingItem
/// This will encrypt the content with my encryption key and the recipient's public encryption key
/// called from post_commit()
fn create_PendingItem<T>(
   kind: ItemKind,
   content: T,
   distribution_eh: EntryHash,
   recipient: AgentPubKey,
) -> ExternResult<PendingItem>
   where
      T: serde::Serialize + Clone + Sized + std::fmt::Debug
{
   /// Get my key
   let me = agent_info()?.agent_latest_pubkey;
   debug!("get_enc_key() for sender {:?}", me);
   let response = call_self("get_enc_key", me.clone())?;
   debug!("get_enc_key() for sender result: {:?}", response);
   let sender_key: X25519PubKey = decode_response(response)?;
   debug!("PendingItem: sender_key = {:?}", sender_key);
   /// Get recipient's key
   debug!("get_enc_key() for recipient {:?}", recipient);
   let response = call_self("get_enc_key", recipient.clone())?;
   debug!("get_enc_key() for recipient result: {:?}", response);
   let recipient_key: X25519PubKey = decode_response(response)?;
   debug!("PendingItem: recipient_key = {:?}", recipient_key);
   /// Sign content
   let author_signature = sign(me, content.clone())
      .expect("Should be able to sign with my key");
   /// Serialize
   let serialized = bincode::serialize(&content).unwrap();
   let data: XSalsa20Poly1305Data = serialized.into();
   /// Encrypt
   let encrypted_data = x_25519_x_salsa20_poly1305_encrypt(sender_key, recipient_key, data)
      .expect("Encryption should work");
   trace!("Encrypted: {:?}", encrypted_data.clone());
   // let me = agent_info().expect("Should have agent info").agent_latest_pubkey;
   // let signature = sign(me, mail).expect("Should be able to sign with my key");
   trace!("with:\n -    sender = {:?}\n - recipient = {:?}", sender_key.clone(), recipient_key.clone());
   /// Done
   let item = PendingItem {
      kind,
      //app_type: type_name::<T>(),
      encrypted_data,
      distribution_eh,
      author_signature,
   };
   Ok(item)
}


/// called from post_commit()
pub fn pack_notice(notice: DeliveryNotice, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<DeliveryNotice>(ItemKind::DeliveryNotice, notice.clone(), notice.distribution_eh.clone(), recipient)
}
/// called from post_commit()
pub fn pack_reply(reply: DeliveryReply, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<DeliveryReply>(ItemKind::DeliveryReply, reply.clone(), reply.notice_eh.clone(), recipient)
}
/// called from post_commit()
pub fn pack_reception(reception: ParcelReceived, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<ParcelReceived>(ItemKind::ParcelReceived, reception, distribution_eh, recipient)
}
///
pub fn pack_parcel(parcel_entry: Entry, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<PendingItem> {
   create_PendingItem::<Entry>(ItemKind::Entry, parcel_entry, distribution_eh, recipient)
}