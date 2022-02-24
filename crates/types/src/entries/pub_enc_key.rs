use hdk::prelude::*;

/// Entry representing the Public Encryption Key of an Agent
#[hdk_entry(id = "PubEncKey", visibility = "public")]
#[derive(Clone, PartialEq)]
pub struct PubEncKey {
   pub value: X25519PubKey,
}

impl PubEncKey {
   pub fn new() -> Self {
      let value = create_x25519_keypair()
         .expect("Create Keypair should work");
      Self {
         value,
      }
   }
}
