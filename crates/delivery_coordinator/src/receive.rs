use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_integrity::*;
use zome_delivery_types::*;

use crate::*;



/// Starting point for any remote call in this zome (i.e. sending a dm)
/// Name of this function must match REMOTE_ENDPOINT value
#[hdk_extern]
pub fn receive_delivery_dm(dm: DirectMessage) -> ExternResult<DeliveryProtocol> {
    debug!("Received DM from: {} ; msg: {}", snip(&dm.from), dm.msg);
    let reply = match dm.msg {
        DeliveryProtocol::ChunkRequest(chunk_eh) => {
            receive_dm_chunk_request(dm.from, chunk_eh)
        },
        DeliveryProtocol::ParcelRequest(distribution_eh) => {
            receive_dm_parcel_request(dm.from, distribution_eh)
        },
        DeliveryProtocol::Item(pending_item) => {
            match pending_item.kind {
                /// Sent by recipient
                ItemKind::DeliveryReply => {
                    let _ = receive_reply(dm.from, pending_item.clone())?;
                    let signature = sign(agent_info()?.agent_latest_pubkey, pending_item.clone())?;
                    DeliveryProtocol::Success(signature)
                },
                ItemKind::ParcelReceived => {
                    let _ = receive_reception(dm.from, pending_item.clone())?;
                    let signature = sign(agent_info()?.agent_latest_pubkey, pending_item.clone())?;
                    DeliveryProtocol::Success(signature)
                },
                ItemKind::NoticeReceived => {
                    let _ = receive_ack(dm.from, pending_item.clone())?;
                    let signature = sign(agent_info()?.agent_latest_pubkey, pending_item.clone())?;
                    DeliveryProtocol::Success(signature)
                },
                /// Sent by sender
                ItemKind::DeliveryNotice => {
                    let notice = receive_notice(dm.from, pending_item.clone())?;
                    let signature = sign(agent_info()?.agent_latest_pubkey, notice.summary)?;
                    DeliveryProtocol::Success(signature)
                },
                ItemKind::AppEntryBytes => {
                    let _ = receive_entry(dm.from, pending_item.clone())?;
                    let signature = sign(agent_info()?.agent_latest_pubkey, pending_item.clone())?;
                    DeliveryProtocol::Success(signature)
                },
                ItemKind::ParcelChunk => {
                    let _ = receive_chunk(dm.from, pending_item.clone())?;
                    let signature = sign(agent_info()?.agent_latest_pubkey, pending_item.clone())?;
                    DeliveryProtocol::Success(signature)
                },
                //_ => panic!("ItemKind '{:?}' should not be received via DM", item.kind),
            }
        },
        DeliveryProtocol::Ping => DeliveryProtocol::Pong,
        _ => {
             DeliveryProtocol::Failure("Unexpected protocol".to_owned())
        },
    };
    Ok(reply)
}


///
fn receive_entry(from: AgentPubKey, item: PendingItem) -> ExternResult<()> {
    let maybe_entry: Option<Entry> = unpack_entry(item.clone(), from.clone())?;
    if maybe_entry.is_none() {
        return zome_error!("Failed deserializing Entry");
    }
    let parcel = maybe_entry.unwrap();

    /// Make sure we accepted to receive this Entry
    // FIXME

    /// Get notice
    let parcel_eh = hash_entry(parcel.clone())?;
    let maybe_notice = find_notice(parcel_eh)?;
    if maybe_notice.is_none() {
        return zome_error!("Failed finding DeliveryNotice for received parcel");
    }
    /// Commit Entry
    let _hh = call_commit_parcel(parcel, &maybe_notice.unwrap(), None)?;
    /// Done
    Ok(())
}


///
pub fn receive_chunk(from: AgentPubKey, item: PendingItem) -> ExternResult<()> {
    let maybe_chunk: Option<ParcelChunk> = unpack_item(item.clone(), from.clone())?;
    if maybe_chunk.is_none() {
        return zome_error!("Failed deserializing ParcelChunk");
    }
    /// Make sure we accepted to receive this chunk
    // FIXME
    /// Commit entry
    let _maybe_hh = create_entry_relaxed(DeliveryEntry::ParcelChunk(maybe_chunk.unwrap()))?;
    /// Done
    Ok(())
}


/// Returns ChunkResponse or Failure
pub fn receive_dm_chunk_request(_from: AgentPubKey, chunk_eh: EntryHash) -> DeliveryProtocol {
    /// Get Distribution Entry
    let maybe_chunk: ExternResult<ParcelChunk> = get_typed_from_eh(chunk_eh);
    if let Err(err) = maybe_chunk {
        return failure_err("ParcelChunk not found", err);
    }
    return DeliveryProtocol::ChunkResponse(maybe_chunk.unwrap().to_owned());
}


/// Returns ParcelResponse or Failure
pub fn receive_dm_parcel_request(from: AgentPubKey, distribution_eh: EntryHash) -> DeliveryProtocol {
    /// Get Distribution Entry
    let maybe_distribution: ExternResult<Distribution> = get_typed_from_eh(distribution_eh);
    if let Err(err) = maybe_distribution {
        return failure_err("Distribution not found", err);
    }
    let distribution = maybe_distribution.unwrap();
    /// Make sure DM is from a valid recipient
    if !distribution.recipients.contains(&from) {
        return failure("Request from invalid recipient");
    }
    /// Get Parcel entry hash
    let parcel_eh = distribution.delivery_summary.parcel_reference.entry_address();
    debug!("Looking for Parcel: {:?}", parcel_eh);
    /// Get entry
    let maybe_maybe_element = get(parcel_eh, GetOptions::content());
    if let Err(err) = maybe_maybe_element {
        return failure_err("Failed to get Parcel Element", err);
    }
    let element = match maybe_maybe_element.unwrap() {
        None => return failure("Parcel Element not found"),
        Some(el) => el,
    };
    /// Return Entry
    debug!("Parcel Element found: {:?}", element);
    let maybe_entry = element.entry().as_option();
    if maybe_entry.is_none() {
        return failure("Parcel Entry not found in Parcel Element");
    }
    return DeliveryProtocol::ParcelResponse(maybe_entry.unwrap().to_owned());
}


/// Commit received DeliveryNotice from sender
pub fn receive_notice(from: AgentPubKey, item: PendingItem) -> ExternResult<DeliveryNotice> {
    let maybe_notice: Option<DeliveryNotice> = unpack_item(item, from.clone())?;
    let Some(notice) = maybe_notice
       else { return zome_error!("Failed deserializing DeliveryNotice (2)"); };
    /// Check for duplicate DeliveryNotice
    let maybe_already = find_notice(notice.summary.parcel_reference.entry_address())?;
    if maybe_already.is_some() {
        return zome_error!("Already have this Notice");
    }
    /// Commit DeliveryNotice
    let _hh = create_entry_relaxed(DeliveryEntry::DeliveryNotice(notice.clone()))?;
    /// Done
    Ok(notice)
}

/// Commit received DeliveryNotice from sender
pub fn receive_ack(from: AgentPubKey, item: PendingItem) -> ExternResult<()> {
    let maybe_ack: Option<NoticeReceived> = unpack_item(item, from.clone())?;
    let Some(ack) = maybe_ack
       else { return zome_error!("Failed deserializing NoticeReceived"); };
    // /// Check for duplicate DeliveryNotice
    // let maybe_already = find_notice(ack.summary.parcel_reference.entry_address())?;
    // if maybe_already.is_some() {
    //     return zome_error!("Already have this Notice");
    // }
    /// Commit DeliveryNotice
    let _hh = create_entry_relaxed(DeliveryEntry::NoticeReceived(ack.clone()))?;
    /// Done
    Ok(())
}


/// Create and commit a ReplyReceived from a DeliveryReply
pub fn receive_reply(from: AgentPubKey, pending_item: PendingItem) -> ExternResult<()> {
    let maybe_reply: Option<DeliveryReply> = unpack_item(pending_item.clone(), from.clone())?;
    if maybe_reply.is_none() {
        return error("Failed deserializing DeliveryReply");
    }
    /// Create ReplyReceived
    let receipt = ReplyReceived {
        distribution_eh: pending_item.distribution_eh,
        recipient: from,
        recipient_signature: pending_item.author_signature,
        has_accepted: maybe_reply.unwrap().has_accepted,
        //date: now(),
    };
    /// Commit ReplyReceived
    let _hh = create_entry_relaxed(DeliveryEntry::ReplyReceived(receipt.clone()))?;
    /// Emit Signal
    let res = emit_signal(&SignalProtocol::ReceivedReply(receipt));
    if let Err(err) = res {
        error!("Emit signal failed: {}", err);
    }
    /// Done
    Ok(())
}


/// Create and commit a DeliveryReceipt from a ParcelReceived
pub fn receive_reception(from: AgentPubKey, pending_item: PendingItem) -> ExternResult<()> {
    /// Make sure it unpacks correctly
    let _received: Option<ParcelReceived> = unpack_item(pending_item.clone(),from.clone())?;
    /// Create DeliveryReceipt
    let receipt = DeliveryReceipt {
        distribution_eh: pending_item.distribution_eh,
        recipient: from,
        recipient_signature: pending_item.author_signature,
        //date_of_response: now(),
    };
    /// Commit DeliveryReceipt
    let _hh = create_entry_relaxed(DeliveryEntry::DeliveryReceipt(receipt.clone()))?;
    /// Done
    Ok(())
}
