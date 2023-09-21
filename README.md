# Delivery Zome

Holochain Zome Module for sharing custom data asynchronously, publicly or privately.


## Design Goal

Enable sharing of custom data between agents.

Features:
 - Sign and encrypt data
 - Spliting of a large data into smaller chunks.
 - A default Chunk entry is provided for custom data.
 - Multi-step delivery with failure recovery (with post_commit()).
 - Affordance for accepting or refusing a delivery notice.
 - Proof of Delivery.
 - Multiple distribution strategies:
   - Send privately via DM
   - Send privately via DHT (encrypted entries).
   - Publicly on the DHT    
   - (TODO) Define private Drop-Off points (cap-grants?)
   - (TODO) recipient 2 recipient sharing
 - (TODO) auto-reception with Scheduler
 - (TODO) ?Parcel types can have the option to not be refusable (ex: mail)? With parcel trait?
 - (TODO) block-list for refusing any message from a sender

A sender can send any Entry to a list of recipients.
The Entry must be first committed to the sender's source chain.
The recipient has the option to accept or refuse an incoming delivery.

The module is to be used by other zomes (via inter-zome calls) for their own entry types.


#### Flow

0. All agents share their public encryption keys.
1. Sender commits arbitrary entries to its source-chain with its own zome functions or the generic function `commit_parcel_*()` (creates some `ParcelChunk` entries and a `ParcelManifest`).
2. Sender sends the committed entries to a list of recipient by calling `distribute_parcel(parcel_eh, [recipients])` (creates a `Distribution` entry).
3. Recipient will first commit a `DeliveryNotice` to its own source-chain. It can be retrieved with `query_DeliveryNotice()`.
4. Sender is notified of Recipient's `DeliveryNotice` (creates a `NoticeAck` entry).
4. Recipient calls `respond_to_notice(notice_eh, yes/no)` to accept/refuse an incoming parcel (creates a `NoticeReply` entry).
4. Sender is notified of Recipient's `NoticeReply` (creates a `ReplyAck` entry).
5. If accepted, the recipient will ask sender for the data (dm protocol), and will commit it on recipient's source-chain.
   6. Once all entries are committed for this parcel, the recipient commits a `ReceptionProof` entry.
   7. Sender is notified of Recipient's `ReceptionProof` (creates a `ReceptionAck` entry).


## How to use

Zome clients must call `call_delivery_post_commit()` in their zome's `post_commit()` callback.
Zome clients can use `call_delivery_zome()` and `call_remote_delivery_zome()` to call the delivery zome.
The Delivery zome must use the name defined by `DELIVERY_ZOME_NAME` in the client dna.

Use `playground` as an integration example. 

### Zome properties

A client dna must define the following dna properties:
- `maxChunkSize`: Maximum size in octets of the content of a `ParcelChunk` entry.
- `maxParcelSize`: Maximum size in octets allowed for a parcel.
- `maxParcelNameLength`: Maximum length allowed for a parcel name
- `minParcelNameLength`: Minimum length allowed for a parcel name


### TODO

- Test simultaneous sends and receives of parcels
- Test failure recovery
- Investigate double send of entry (should we use `receive_entry()`)

`un/set_drop_off_agent()` Announce publicly which are my allowed drop-off points
`query_drop_off()` Ask drop-off agent if it has a parcel for me.
`take_from_drop_off()` Request drop-off agent to hand over parcel.


## Building

To rebuild the DNA for holochain:
1. [Install rustup](https://rustup.rs/) and the `wasm32` target with: ``rustup target add wasm32-unknown-unknown``
1. Install [holochain and hc](https://github.com/holochain/holochain)
1. Run ``scripts\pack-happ.sh``


## Testing

A testing zome has been implemented, called 'secret'.

(OUTDATED) Test suite with sweeettest:
1. `cargo build`
2. `./target/debug/delivery_sweettest.exe <testname>`