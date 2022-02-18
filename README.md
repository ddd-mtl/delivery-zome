# snapmail-rsm

Holochain Zome for P2P custom data delivery service between agents.

Some design documentation is available in the `/spec` folder.

CI and NIX configs are not set up for the moment.


## Design Goal

Enable private sharing of custom data between P2P agents.
Features:
 - Sign and encrypt parcels
 - Delivery Confirmation
 - Multiple distribution strategies:
   - Send via DM
   - Send via DHT
   - Define private Drop-Off points (cap-grants?)
   - recipient 2 recipient sharing
   - public
 - Spliting / Chunking large parcels for delivery
 - Failsafe and recovery (with post_commit())
 - Acknowledgement system
 - A default chunk entry is provided for arbitrary data

Constraints:
A sender can send any Entry to a list of recipients
The Entry must be first committed to the sender's source chain.
The Entry must implement a Parcel trait?
The recipient has the option to refuse to retrieve Parcel.
Parcel types can have the option to not be refusable (ex: acks, mails)


Commit arbitrary entries to your source chain
`send_entry(any_eh, [to])` to send a custom entry to a list of recipient. The entry will be analyzed
`send_parcel(parce_eh)` to

`un/set_drop_off_agent()` Announce publicly which are my allowed drop-off points
`query_drop_off()` Ask drop-off agent if it has a parcel for me.
`take_from_drop_off()` Request drop-off agent to hand over parcel.

## Building

To rebuild the DNA for holochain:
1. [Install rustup](https://rustup.rs/) and the `wasm32` target with: ``rustup target add wasm32-unknown-unknown``
1. Install [holochain and hc](https://github.com/holochain/holochain)
1. Run ``scripts\pack-happ.sh``


## Testing
Steps for running tests:
 1. Install Holochain
 2. Go to ``test`` sub directory.
 3. Run command: `npm test`
 
Test suites can also be enabled/disabled by commenting out the lines in `test\test.ts`


## Running with UI

 1. Download the [snapmail-ui repo](https://github.com/glassbeadsoftware/snapmail-ui) and store it at same folder level as `snapmail-rsm`
 2. CD to its root folder
 2. Make sure bootstrap server and proxy server are up and running.
 3. Launch `alex`, `billy`, or `camille` agents like this:`npm run alex`
 4. Or launch all three with `npm run all`

Browser windows should automatically pop-up for each agent.
