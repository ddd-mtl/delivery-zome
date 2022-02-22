use hdk::prelude::*;

use std::convert::TryFrom;

use crate::{
   utils::*,
   entries::*,
   LinkKind,
   //parcel::*,
};
use crate::entries::DeliveryNotice;


/// List of structs that PendingItem can embed
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PendingKind {
   DeliveryNotice,
   DeliveryReply,
   ParcelReceived,
   Entry,
   // ParcelManifest
   ParcelChunk,
}


/// A Public Entry representing an encrypted private Entry on the DHT
/// waiting to be received by some recipient.
/// The recipient is the agentId where the entry is linked from.
/// The Entry is encrypted with the recipient's public encryption key.
#[hdk_entry(id = "PendingItem")]
#[derive(Clone, PartialEq)]
pub struct PendingItem {
    pub kind: PendingKind,
    //pub app_type: &'static str,
    pub author_signature: Signature, // Signature of the Entry's author
    pub encrypted_data: XSalsa20Poly1305EncryptedData,
    pub distribution_eh: EntryHash,
}

impl PendingItem {
   /// Create PendingItem
   /// This will encrypt the content with my encryption key and the recipient's public encryption key
   /// called from post_commit()
   fn create<T>(
      kind: PendingKind,
      content: T,
      distribution_eh: EntryHash,
      recipient: AgentPubKey,
   ) -> ExternResult<Self>
      where
         T: serde::Serialize + Clone + Sized + std::fmt::Debug
   {
      /// Get my key
      let me = agent_info()?.agent_latest_pubkey;
      debug!("get_enc_key() for sender {:?}", me);
      let maybe_sender_key = call_self("get_enc_key", me.clone())?;
      debug!("get_enc_key() for sender result: {:?}", maybe_sender_key);
      let sender_key = match maybe_sender_key {
         ZomeCallResponse::Ok(output) => output.decode()?,
         _ => return error("Self call to get_enc_key(sender) failed")
      };
      debug!("PendingItem: sender_key = {:?}", sender_key);
      /// Get recipient's key
      debug!("get_enc_key() for recipient {:?}", recipient);
      let maybe_recipient_key = call_self(
         "get_enc_key",
         recipient.clone(),
      )?;
      debug!("get_enc_key() for recipient result: {:?}", maybe_recipient_key);
      let recipient_key = match maybe_recipient_key {
         ZomeCallResponse::Ok(output) => output.decode()?,
         _ => return error("Self call to get_enc_key(recipient) failed")
      };
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
   pub fn from_notice(notice: DeliveryNotice, recipient: AgentPubKey) -> ExternResult<Self> {
      Self::create::<DeliveryNotice>(PendingKind::DeliveryNotice, notice.clone(), notice.distribution_eh.clone(), recipient)
   }
   /// called from post_commit()
   pub fn from_reply(reply: DeliveryReply, recipient: AgentPubKey) -> ExternResult<Self> {
      Self::create::<DeliveryReply>(PendingKind::DeliveryReply, reply.clone(), reply.notice_eh.clone(), recipient)
   }
   /// called from post_commit()
   pub fn from_reception(reception: ParcelReceived, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<Self> {
      Self::create::<ParcelReceived>(PendingKind::ParcelReceived, reception, distribution_eh, recipient)
   }
   ///
   pub fn from_parcel(parcel_entry: Entry, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<Self> {
      Self::create::<Entry>(PendingKind::Entry, parcel_entry, distribution_eh, recipient)
   }

   /// Attempt to decrypt PendingItem with provided keys
   pub fn attempt_decrypt<T>(&self, sender: X25519PubKey, recipient: X25519PubKey) -> Option<T>
      where
         T: for<'de> serde::Deserialize<'de>
   {
      trace!("attempt_decrypt of: {:?}", self.encrypted_data.clone());
      trace!("with:\n -    sender = {:?}\n - recipient = {:?}", sender.clone(), recipient.clone());
      /// Decrypt
      let maybe_decrypted = x_25519_x_salsa20_poly1305_decrypt(sender, recipient, self.encrypted_data.clone())
         .expect("Decryption should work");
      trace!("attempt_decrypt maybe_decrypted = {:?}", maybe_decrypted);
      let decrypted = match maybe_decrypted {
         Some(data) => data,
         None => return None,
      };
      let decrypted_ref/*: &'a [u8]*/ = decrypted.as_ref();
      /// Deserialize
      let item: T = bincode::deserialize(decrypted_ref)
         .expect("Deserialization should work");
      /// Done
      Some(item)
   }


   pub fn into_item<T>(self, from: AgentPubKey) -> ExternResult<Option<T>>
      where
         T: for<'de> serde::Deserialize<'de> + Clone + serde::Serialize + std::fmt::Debug //+ Sized
   {
      /// Get my key
      let me = agent_info()?.agent_latest_pubkey;
      let recipient_key = get_enc_key(me.clone())?;
      debug!("try_into() recipient_key: {:?}", recipient_key);
      /// Get sender's key
      let sender_key = get_enc_key(from.clone())?;
      debug!("try_into() sender_key: {:?}", sender_key);
      /// Decrypt
      let maybe_thing: Option<T> = self.attempt_decrypt(sender_key, recipient_key);
      //debug!("try_into() maybe_thing: {:?}", maybe_thing);
      /// Into DeliveryNotification
      if maybe_thing.is_none() {
         return Ok(None);
      }
      let thing = maybe_thing.unwrap();
      /// Check signature
      let maybe_verified = verify_signature(from, self.author_signature, thing.clone());
      match maybe_verified {
         Err(err) => {
            let response_str = "Verifying PendingItem failed";
            debug!("{}: {}", response_str, err);
            return error(response_str);
         }
         Ok(false) => {
            let response_str = "Failed verifying PendingItem signature";
            debug!("{}", response_str);
            return error(response_str);
         }
         Ok(true) => debug!("Valid PendingItem signature"),
      }
      /// Done
      Ok(Some(thing))
   }

}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitPendingItemInput {
   pub item: PendingItem,
   pub recipient: AgentPubKey,
}


#[hdk_extern]
fn commit_PendingItem(input: CommitPendingItemInput) -> ExternResult<HeaderHash> {
   debug!("commit_pending_item() START **********");
   let me = agent_info()?.agent_latest_pubkey;
   /// Commit Pending Item
   let pending_item_eh = hash_entry(&input.item)?;
   let maybe_pending_item_hh = create_entry(&input.item);
   if let Err(err) = maybe_pending_item_hh.clone() {
      debug!("PendingMail create_entry() failed = {:?}", err);
      return Err(maybe_pending_item_hh.err().unwrap());
   };
   let pending_mail_hh = maybe_pending_item_hh.unwrap();
   trace!("pending_mail_hh = {:?}", pending_mail_hh);
   /// Commit Pendings Link
   let tag = LinkKind::Pendings.concat_hash(&input.recipient);
   trace!("pendings tag = {:?}", tag);
   let maybe_link1_hh = create_link(input.item.distribution_eh.clone(), pending_item_eh.clone(), tag);
   if let Err(err) = maybe_link1_hh.clone() {
      trace!("link1 failed = {:?}", err);
      return Err(maybe_link1_hh.err().unwrap());
   };
   let link1_hh = maybe_link1_hh.unwrap();
   trace!("link1_hh = {}", link1_hh);
   /// Commit MailInbox Link
   let tag = LinkKind::Inbox.concat_hash(&me);
   let maybe_link2_hh = create_link(EntryHash::from(input.recipient.clone()), pending_item_eh, tag);
   if let Err(err) = maybe_link2_hh.clone() {
      trace!("link2 failed = {:?}", err);
      return Err(maybe_link2_hh.err().unwrap());
   };
   let link2_hh = maybe_link2_hh.unwrap();
   trace!("link2_hh = {}", link2_hh);
   /// Done
   return Ok(pending_mail_hh)
}
