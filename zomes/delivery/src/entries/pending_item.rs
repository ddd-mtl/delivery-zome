use hdk::prelude::*;

use super::Mail;
use crate::entries::*;
use crate::{
   utils::*,
   LinkKind,
   parcel::*,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PendingKind {
   Description,
   DeliveryNotification,
   ReceptionConfirmation,
   Parcel,
   // Chunk,
}


/// Entry representing a mail on the DHT waiting to be received by recipient.
/// The recipient is the agentId where the entry is linked from.
/// The mail is encrypted with the recipient's public encryption key.
#[hdk_entry(id = "PendingItem")]
#[derive(Clone, PartialEq)]
pub struct PendingItem {
    pub kind: PendingKind,
    pub encrypted_data: XSalsa20Poly1305EncryptedData,
    pub author_signature: Signature,
    pub distribution_eh: EntryHash,
}

impl PendingItem {

   /// Create PendingItem
   /// This will encrypt the content with my encryption key and the recipient's public encryption key
   /// called from post_commit()
   fn create<T: Sized>(kind: PendingKind, content: T, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<Self>
      where
         T: serde::Serialize
   {
      /// Get my key
      let me = agent_info()?.agent_latest_pubkey;
      debug!("get_enc_key() for sender {:?}", me);
      let maybe_sender_key = call_remote(
         me.clone(),
         zome_info()?.name,
         "get_enc_key".to_string().into(),
         None,
         me.clone(),
      )?;
      debug!("get_enc_key() for sender result: {:?}", maybe_sender_key);
      let sender_key = match maybe_sender_key {
         ZomeCallResponse::Ok(output) => output.decode()?,
         _ => return error("Self call to get_enc_key(sender) failed")
      };
      debug!("PendingItem: sender_key = {:?}", sender_key);
      /// Get recipient's key
      debug!("get_enc_key() for recipient {:?}", recipient);
      let maybe_recipient_key = call_remote(
         me.clone(),
         zome_info()?.name,
         "get_enc_key".to_string().into(),
         None,
         recipient.clone(),
      )?;
      debug!("get_enc_key() for recipient result: {:?}", maybe_recipient_key);
      let recipient_key = match maybe_recipient_key {
         ZomeCallResponse::Ok(output) => output.decode()?,
         _ => return error("Self call to get_enc_key(recipient) failed")
      };
      debug!("PendingItem: recipient_key = {:?}", recipient_key);
      /// Sign content
      let author_signature = sign(me, &content)
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
         encrypted_data,
         distribution_eh,
         author_signature,
      };
      Ok(item)
   }


   /// called from post_commit()
   pub fn from_description(description: ParcelDescription, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<Self> {
      Self::create::<ParcelDescription>(PendingKind::Description, description, distribution_eh, recipient)
   }

   /// called from post_commit()
   pub fn from_notification(notification: DeliveryNotification, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<Self> {
      Self::create::<DeliveryNotification>(PendingKind::Description, notification, distribution_eh, recipient)
   }

   /// called from post_commit()
   pub fn from_reception(reception: ReceptionConfirmation, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<Self> {
      Self::create::<ReceptionConfirmation>(PendingKind::Description, reception, distribution_eh, recipient)
   }

   ///
   pub fn from_parcel(parcel: Parcel, distribution_eh: EntryHash, recipient: AgentPubKey) -> ExternResult<Self> {
      Self::create::<Parcel>(PendingKind::Parcel, parcel, distribution_eh, recipient)
   }

   /// Attempt to decrypt PendingItem with provided keys
   pub fn attempt_decrypt<T: Sized>(&self, sender: X25519PubKey, recipient: X25519PubKey) -> Option<T> {
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
      /// Deserialize
      let item: T = bincode::deserialize(decrypted.as_ref())
         .expect("Deserialization should work");
      /// Done
      Some(item)
   }


   pub fn try_into<T>(self, from: AgentPubKey) -> ExternResult<Option<T>> {
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
struct CommitPendingItemInput {
   item: PendingItem,
   recipient: AgentPubKey,
}


#[hdk_extern]
fn commit_pending_item(input: CommitPendingItemInput) -> ExternResult<HeaderHash> {
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
   let tag = LinkKind::MailInbox.concat_hash(&me);
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
