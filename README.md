# Delivery Zome

Holochain Zome Module for sending private data between agents asynchronously.


## Design

Enable private sharing of custom data between P2P agents (parcels).
Features:
 - Sign and encrypt parcels
 - Delivery Receipt
 - Multiple distribution strategies:
   - Send via DM
   - Send via DHT
   - (TODO) Define private Drop-Off points (cap-grants?)
   - (TODO) recipient 2 recipient sharing
   - (TODO) public
 - Spliting / Chunking large parcels into many small ones
 - A default Chunk entry is provided for arbitrary data
 - Robust multi-step and failure recovery (with post_commit())
 - (TODO) auto-reception with Scheduler
 - (TODO) auto Chunk ordering
 - (TODO) Acknowledgement system?


A sender can send any Entry to a list of recipients
The Entry must be first committed to the sender's source chain.
The recipient has the option to accept or refuse an incoming delivery.

The module is to be used by other zomes (via inter-zome calls) for their own entry types.

?Parcel types can have the option to not be refusable (ex: acks, mails)? With parcel trait?

Flow:

1. Commit arbitrary entries to your source chain with your own zome functions or use `commit_parcel_*()`.
2. Send the committed entries to a list of recipient: `distribute_parcel(parcel_eh, [recipients])` 
3. Recipient will first commit a DeliveryNotice to its own source-chain. It can be retrieved with `query_DeliveryNotice()`.
`respond_to_notice(notice_eh, yes/no)` to accept/refuse an incoming parcel
get() the entry

`un/set_drop_off_agent()` Announce publicly which are my allowed drop-off points
`query_drop_off()` Ask drop-off agent if it has a parcel for me.
`take_from_drop_off()` Request drop-off agent to hand over parcel.

### TODO

- Investigate double send of entry (should we use `receive_entry()`)
- Test simultaneous sends and receives of parcels
- Test big file transfer (> 16 MiB)
- Test failure recovery
- Add signaling
- Cleanup code base

## Building

To rebuild the DNA for holochain:
1. [Install rustup](https://rustup.rs/) and the `wasm32` target with: ``rustup target add wasm32-unknown-unknown``
1. Install [holochain and hc](https://github.com/holochain/holochain)
1. Run ``scripts\pack-happ.sh``


## Testing

A testing zome has been implemented, called 'secret'.

Test suite with sweeettest:
1. `cargo build`
2. `./target/debug/delivery_sweettest.exe <testname>`