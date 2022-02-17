use hdk::prelude::*;

use crate::get_typed_from_eh;

/// Entry representing an authored mail. It is private.
#[hdk_entry(id = "distribution", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct Distribution {
    pub recipients: Vec<AgentPubKey>,
    pub manifest: Manifest,
    pub sender_manifest_signature: Signature,
    //pub can_use_dht: bool,
    //pub can_share_between_recipients: bool, // Make recipient list "public" to recipients
}


fn get_app_entry_size(eh: EntryHash) -> ExternResult<u64> {
    let maybe_element = get(payload[0], GetOptions::content())?;
    let element = match maybe_element {
        Some(el) => el,
        None => return error("No element found at given payload address"),
    };

    let size: u64 = element
       .entry()
       .to_app_option()?
       .ok_or(WasmError::Guest(String::from("No AppEntry found at given payload address")))?
       .into_sb()
       .len();

    // let size = match element.entry() {
    //     Present(entry) => {
    //         match entry {
    //             App(app_bytes) => app_bytes.into_sb().len(),
    //             _ => return error("No AppEntry found at given payload address"),
    //         },
    //         _ => return error("No Entry found at given payload address"),
    //     }
    // }
    Ok(size)
}


///
impl Distribution {

    ///
    pub fn create(
        recipient_list: Vec<AgentPubKey>,
        parcel_type: String,
        payload: Vec<EntryHash>,
    ) -> ExternResult<Self> {
        if recipients.is_empty() || payload.is_empty() {
            return error("Missing a recipient or payload");
        }
        /// Remove duplicate recipients
        let mut recipients = recipient_list.clone();
        let set: HashSet<_> = recipients.drain(..).collect(); // dedup
        recipients.extend(set.into_iter());

        /// No Chunks
        let mut total_parcel_size = 0;
        if payload.len() == 1 {
            total_parcel_size = get_app_entry_size(payload[0])?;
        } else {
            /// Get Chunks
            for chunk_eh in payload.iter() {
                total_parcel_size += get_app_entry_size(chunk_eh)?;
            }
        }
        /// Create Manifest
        let manifest = Manifest {
            parcel_type,
            total_parcel_size,
            payload,
        };
        /// Sign
        let sender_manifest_signature = sign(agent_info()?.agent_latest_pubkey, manifest.clone())?;
        /// Done
        let distribution = Distribution {
            recipients,
            manifest,
            sender_manifest_signature
        };
        Ok(distribution)
    }
}
