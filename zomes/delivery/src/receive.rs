use hdk::prelude::*;

use crate::{
    AckMessage,
    DeliveryProtocol,
    file::{FileChunk, FileManifest},
    mail::{
      self,
   },
    ReceptionRequestMessage, ReceivedAck, signal_protocol::*,
    snapmail_now,
    utils::*,
    entries::*,
};

/// Start point for any remote call in this zome
/// WARN: Name of function must match REMOTE_ENDPOINT const value
#[hdk_extern]
pub fn receive_delivery_dm(from: AgentPubKey, dm: DeliveryProtocol) -> DeliveryProtocol {
    debug!("Received from: {}", from);
    match dm {
        DeliveryProtocol::Chunk(chunk) => {
            mail::receive_direct_chunk(from, chunk)
        },
        DeliveryProtocol::FileManifest(manifest) => {
            mail::receive_direct_manifest(from, manifest)
        },
        DeliveryProtocol::ReceptionRequest(msg) => {
            mail::receive_dm_reception_request(from, msg)
        },
        DeliveryProtocol::Ack(ack) => {
            mail::receive_dm_ack(from, ack)
        },
        DeliveryProtocol::RequestChunk(address) => {
            mail::receive_direct_request_chunk(from, address)
        },
        DeliveryProtocol::RequestManifest(address) => {
            mail::receive_direct_request_manifest(from, address)
        },
        DeliveryProtocol::Ping => {
            DeliveryProtocol::Success(String::new())
        },
        _ => {
             DeliveryProtocol::Failure("Unexpected protocol".to_owned())
        },
    }
}

/// Handle a MailMessage.
/// Emits `ReceivedMail` signal.
/// Returns Success or Failure.
pub fn receive_dm_reception_request(from: AgentPubKey, msg: ReceptionRequestMessage) -> DeliveryProtocol {
    /// Check signature
    let maybe_verified = verify_signature(
        from.clone(),
        msg.sender_description_signature.clone(),
        msg.description.clone());
    match maybe_verified {
        Err(err) => {
            let response_str = "Verifying ReceptionRequestMessage failed";
            debug!("{}: {}", response_str, err);
            return DeliveryProtocol::Failure(response_str.to_string());
        }
        Ok(false) => {
            let response_str = "Failed verifying ReceptionRequestMessage signature";
            debug!("{}", response_str);
            return DeliveryProtocol::Failure(response_str.to_string());
        }
        Ok(true) => debug!("Valid ReceptionRequestMessage signature"),
    }
    /// Create DeliveryNotification
    let notification = DeliveryNotification::from_direct(msg.clone(), from.clone());
    /// Commit DeliveryNotification
    let maybe_hh = create_entry(&notification);
    if let Err(err) = maybe_hh {
        let response_str = "Failed committing DeliveryNotification";
        debug!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }

    /// Return Success response
    return DeliveryProtocol::Success("ReceptionRequest received".to_string());
}

/// Handle a AckMessage.
/// Emits `ReceivedAck` signal.
/// Returns Success or Failure.
pub fn receive_dm_ack(from: AgentPubKey, ack_msg: AckMessage) -> DeliveryProtocol {
    debug!("receive_dm_ack() from: {:?} ; for {:?}", from, ack_msg.outmail_eh);
    /// Check if we have acked outmail
    let maybe_outmail = get_local_from_eh(ack_msg.outmail_eh.clone());
    if let Err(err) = maybe_outmail {
        let response_str = "Failed to find OutMail from Ack";
        warn!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let outmail_hh = maybe_outmail.unwrap().header_address().clone();
    /// Check ack signature
    let maybe_verified = verify_signature(from.clone(), ack_msg.ack_signature.clone(), ack_msg.outmail_eh.clone());
    match maybe_verified {
        Err(err) => {
            let response_str = "Verifying AckMessage failed";
            debug!("{}: {}", response_str, err);
            return DeliveryProtocol::Failure(response_str.to_string());
        }
        Ok(false) => {
            let response_str = "Failed verifying AckMessage signature";
            debug!("{}", response_str);
            return DeliveryProtocol::Failure(response_str.to_string());
        }
        Ok(true) => debug!("Valid AckMessage signature"),
    }
    /// Create InAck
    let outmail_eh = ack_msg.outmail_eh.clone();
    debug!("outmail_eh = {:?}", outmail_eh);
    let res = mail::create_inack(outmail_eh, &from, ack_msg.ack_signature);
    if let Err(err) = res {
        let response_str = "Failed committing InAck";
        error!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Emit Signal
    let signal = SignalProtocol::ReceivedAck(ReceivedAck {
        from: from.clone(),
        for_mail: outmail_hh,
    });
    let res = emit_signal(&signal);
    if let Err(err) = res {
        error!("Emit signal failed: {}", err);
    }
    /// Return Success response
    debug!("receive_direct_ack() success!");
    return DeliveryProtocol::Success("Ack received".to_string());
}


/// Handle a RequestFileManifestMessage
/// TODO: Emits `received_request_manifest` signal.
/// Returns FileManifest, UnknownEntry or Failure.
pub fn receive_direct_request_manifest(from: AgentPubKey, manifest_eh: EntryHash) -> DeliveryProtocol {
    debug!("received request manifest from: {}", from);
    let maybe_maybe_el = get(manifest_eh.clone(), GetOptions::content());
    if let Err(err) = maybe_maybe_el {
        let response_str = "Failed on get_entry()";
        warn!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let maybe_el = maybe_maybe_el.unwrap();
    if let None = maybe_el {
        return DeliveryProtocol::UnknownEntry;
    }
    debug!("Sending manifest: {}", manifest_eh);
    let maybe_manifest = get_typed_from_el::<FileManifest>(maybe_el.unwrap());
    if let Err(_err) = maybe_manifest {
        let response_str = "Requested entry is not a FileManifest";
        error!("{}", response_str);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    // Return Success response
    return DeliveryProtocol::FileManifest(maybe_manifest.unwrap());
}

/// Handle a FileManifestMessage.
/// Emits `received_manifest` signal.
/// Returns Success or Failure.
pub fn receive_direct_manifest(from: AgentPubKey, manifest: FileManifest) -> DeliveryProtocol {
    debug!("received manifest from: {}", from);
    // FIXME: Check if already have file?
    /// Commit FileManifest
    let maybe_hh = create_entry(&manifest);
    if let Err(err) = maybe_hh {
        let response_str = "Failed committing FileManifest";
        warn!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let manifest_hh = maybe_hh.unwrap();
    debug!("received manifest_address: {}", manifest_hh);
    // FIXME: emit signal
    /// Return Success response
    return DeliveryProtocol::Success(manifest_hh.to_string());
}

/// Handle a RequestFileChunkMessage.
/// Emits `received_request_chunk` signal.
/// Returns FileChunk, UnknownEntry or Failure.
pub fn receive_direct_request_chunk(from: AgentPubKey, chunk_eh: EntryHash) -> DeliveryProtocol {
    debug!("received request chunk from: {}", from);
    // FIXME: emit signal
    let maybe_maybe_el = get(chunk_eh.clone(), GetOptions::content());
    if let Err(err) = maybe_maybe_el {
        let response_str = "Failed on get_entry()";
        error!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let maybe_el = maybe_maybe_el.unwrap();
    if let None = maybe_el {
        return DeliveryProtocol::UnknownEntry;
    }
    debug!("Sending chunk: {}", chunk_eh);
    let maybe_chunk = get_typed_from_el::<FileChunk>(maybe_el.unwrap());
    if let Err(_err) = maybe_chunk {
        let response_str = "Requested entry is not a FileChunk";
        error!("{}", response_str);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    /// Return Success response
    return DeliveryProtocol::Chunk(maybe_chunk.unwrap());
}

/// Handle a ChunkMessage.
/// Emits `received_chunk` signal.
/// Returns Success or Failure.
pub fn receive_direct_chunk(_from: AgentPubKey, chunk: FileChunk) -> DeliveryProtocol {
    // FIXME: Check if already have chunk?
    /// Commit FileChunk
    let maybe_address = create_entry(&chunk);
    if let Err(err) = maybe_address {
        let response_str = "Failed committing FileChunk";
        error!("{}: {}", response_str, err);
        return DeliveryProtocol::Failure(response_str.to_string());
    }
    let chunk_address = maybe_address.unwrap();
    debug!("received chunk_address: {}",  chunk_address);
    // FIXME: emit signal
    /// Return Success response
    return DeliveryProtocol::Success(chunk_address.to_string());
}
