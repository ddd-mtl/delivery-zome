use hdk::prelude::*;

use crate::entries::*;
use crate::utils::*;
use crate::entry_kind::*;
use crate::parcel::*;
use crate::functions::*;

/// Attempt to decrypt PendingItem with provided keys
fn attempt_decrypt<T>(packed_item: &PendingItem, sender: X25519PubKey, recipient: X25519PubKey) -> Option<T>
   where
      T: for<'de> serde::Deserialize<'de>
{
   trace!("attempt_decrypt of: {:?}", packed_item.encrypted_data.clone());
   trace!("with:\n -    sender = {:?}\n - recipient = {:?}", sender.clone(), recipient.clone());
   /// Decrypt
   let maybe_decrypted = x_25519_x_salsa20_poly1305_decrypt(
      sender,
      recipient,
      packed_item.encrypted_data.clone(),
   ).expect("Decryption should work");
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


pub fn unpack_item<T>(item: PendingItem, from: AgentPubKey) -> ExternResult<Option<T>>
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
   let maybe_thing: Option<T> = attempt_decrypt(&item,sender_key, recipient_key);
   //debug!("try_into() maybe_thing: {:?}", maybe_thing);
   /// Into DeliveryNotification
   if maybe_thing.is_none() {
      return Ok(None);
   }
   let thing = maybe_thing.unwrap();
   /// Check signature
   let maybe_verified = verify_signature(from, item.author_signature, thing.clone());
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