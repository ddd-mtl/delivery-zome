use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;


///
pub fn unpack_item<T>(pending_item: PendingItem, from: AgentPubKey) -> ExternResult<Option<T>>
   where
      T: for<'de> serde::Deserialize<'de> + Clone + serde::Serialize + std::fmt::Debug
{
   debug!("unpack_item() {:?} from {}", pending_item.kind, snip(&from));
   /// Decrypt
   let maybe_decrypted = ed_25519_x_salsa20_poly1305_decrypt(
         agent_info()?.agent_latest_pubkey,
         from.clone(),
         pending_item.encrypted_data,
      );
   /// Deserialize
   let item: T = bincode::deserialize(maybe_decrypted.unwrap().as_ref())
         .expect("Deserialization should work");
   /// Check signature
   check_signature(from, pending_item.author_signature, item.clone())?;
   /// Done
   Ok(Some(item))
}


///
pub fn unpack_entry(pending_item: PendingItem, from: AgentPubKey) -> ExternResult<Option<Entry>> {
   debug!("unpack_entry() {:?} from {}", pending_item.kind, snip(&from));
   assert!(pending_item.kind == ItemKind::AppEntryBytes);
   /// - Decrypt
   let maybe_decrypted = ed_25519_x_salsa20_poly1305_decrypt(
            agent_info()?.agent_latest_pubkey,
            from.clone(),
            pending_item.encrypted_data,
         );
   /// Convert
   let bytes: UnsafeBytes = maybe_decrypted.unwrap().as_ref().to_vec().into();
   //debug!("unpack_entry() bytes: {:?}", bytes);
   let item_sb: SerializedBytes = SerializedBytes::try_from(bytes)?;
   //debug!("unpack_entry() item_sb: {:?}", item_sb);
   let maybe_entry = Entry::app(item_sb.clone());
   if let Err(e) = maybe_entry {
      return error(&format!("Failed converting packed AppEntryBytes into Entry: {:?}", e));
   }
   let entry = maybe_entry.unwrap();
   trace!("unpack_entry() entry valid");
   /// Check signature
   check_signature(from, pending_item.author_signature, item_sb)?;
   /// Done
   Ok(Some(entry))
}

///
fn check_signature<T>(from: AgentPubKey, signature: Signature, data: T) -> ExternResult<()>
   where
      T: serde::Serialize + std::fmt::Debug
{
   let maybe_verified = verify_signature(from, signature, data);
   match maybe_verified {
      Err(err) => {
         let response_str = "Verifying PendingItem failed";
         trace!("{}: {}", response_str, err);
         return error(response_str);
      }
      Ok(false) => {
         let response_str = "Failed verifying PendingItem signature";
         trace!("{}", response_str);
         return error(response_str);
      }
      Ok(true) => trace!("Valid PendingItem signature"),
   }
   Ok(())
}
