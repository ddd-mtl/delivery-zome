import {Dictionary, ZomeViewModel} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";
import {
    ActionHash,
    ActionHashB64,
    AgentPubKey,
    AgentPubKeyB64, AppSignalCb,
    decodeHashFromBase64,
    encodeHashToBase64, EntryHash, EntryHashB64, Timestamp
} from "@holochain/client";
import {
    DeliveryNotice,
    DeliveryReceipt,
    DeliveryReply,
    DeliveryState,
    Distribution,
    DistributionState,
    DistributionStateType,
    NoticeReceived,
    NoticeState,
    NoticeStateType,
    ParcelReceived,
    ReplyReceived,
    SignalKind,
    SignalProtocol,
    SignalProtocolType, SignalProtocolVariantDistributionCreated,
} from "../bindings/delivery.types";
import {AppSignal} from "@holochain/client/lib/api/app/types";


/** [DistributionState, AgentPubKey -> DeliveryState] */
export type FullDistributionState = [DistributionState, Dictionary<DeliveryState>];

/** */
export interface DeliveryPerspective {
    /** -- Encrytion -- */
    myPubEncKey: Uint8Array,
    /** AgentPubKey -> PubEncKey */
    encKeys: Dictionary<Uint8Array>,

    /** -- -- */
    inbox: ActionHashB64[],

    /** -- OUTBOUND -- */
    allDistributions: Dictionary<[Timestamp, Distribution]>,
    /** distrib_eh -> NoticeReceived */
    allReceivedNotices: Dictionary<NoticeReceived>,
    /** distrib_eh -> ReplyReceived */
    allReceivedReplies: Dictionary<ReplyReceived>,
    /** distrib_eh -> DeliveryReceipt */
    allReceipts: Dictionary<DeliveryReceipt>,

    /** -- INBOUND -- */
    allNotices: Dictionary<[Timestamp, DeliveryNotice]>,
    /** notice_eh -> DeliveryReply */
    allReplies: Dictionary<DeliveryReply>,
    /** notice_eh -> ParcelReceived */
    allReceivedParcels: Dictionary<ParcelReceived>,

    /** -- EXTRA LOGIC -- */

    newDeliveryNotices: Dictionary<DeliveryNotice>,

    /** DistributionEh -> [DistributionState, AgentPubKey -> DeliveryState] */
    myDistributions: Dictionary<FullDistributionState>,

    //incomingDistributions: Dictionary<DistributionState>,

    /** AgentPubKey -> notice_eh */
    unrepliedInbounds: Record<AgentPubKeyB64, EntryHashB64>,
    /** distrib_eh -> [Timestamp , AgentPubKey -> DeliveryState] */
    unrepliedOutbounds: Record<EntryHashB64, [Timestamp, Record<AgentPubKeyB64, DeliveryState>]>,

}


/**
 *
 */
export class DeliveryZvm extends ZomeViewModel {

    static readonly ZOME_PROXY = DeliveryProxy;

    get zomeProxy(): DeliveryProxy {
        return this._zomeProxy as DeliveryProxy;
    }

    /** -- ViewModel -- */

    private _perspective: DeliveryPerspective = {
        myPubEncKey: new Uint8Array(),
        encKeys: {},
        inbox: [],
        /** Inbound */
        allDistributions: {},
        allReceivedNotices: {},
        allReceivedReplies: {},
        allReceipts: {},
        /** Outbound */
        allNotices: {},
        allReplies: {},
        allReceivedParcels: {},
        /** Extra logic */
        newDeliveryNotices: {},
        myDistributions: {},
        unrepliedInbounds: {},
        unrepliedOutbounds: {}
    };


    /* */
    get perspective(): DeliveryPerspective {
        return this._perspective;
    }

    /* */
    protected hasChanged(): boolean {
        // TODO
        return true;
    }


    /** -- Signals -- */

    signalHandler?: AppSignalCb = this.mySignalHandler;


    /** */
    async onDistributionCreated(signal: SignalProtocolVariantDistributionCreated): Promise<void> {
        const distribEh = encodeHashToBase64(signal.DistributionCreated[0]);
        const ts = signal.DistributionCreated[1];
        const distribution = signal.DistributionCreated[2];
        this._perspective.allDistributions[distribEh] = [ts, distribution];
        let deliveries: Record<AgentPubKeyB64, DeliveryState> = {};
        // TODO: optimize with Promise.allSettled(promises);
        for (const recipient of distribution.recipients) {
            //deliveries[encodeHashToBase64(recipient)] = {Unsent: null};
            deliveries[encodeHashToBase64(recipient)] = await this.getDeliveryState(distribEh, encodeHashToBase64(recipient));
        };
        this._perspective.unrepliedOutbounds[distribEh] = [ts, deliveries];
        this.notifySubscribers();
    }


    /** */
    mySignalHandler(signal: AppSignal): void {
        console.log("DELIVERY received signal", signal);
        const deliverySignal = signal.payload as SignalProtocol;
        if (SignalProtocolType.DistributionCreated in deliverySignal) {
            console.log("ADDING DistributionCreated", deliverySignal.DistributionCreated);
            this.onDistributionCreated(deliverySignal);
        }
        if (SignalProtocolType.ReceivedNotice in deliverySignal) {
            console.log("ADDING DeliveryNotice", deliverySignal.ReceivedNotice);
            const noticeEh = encodeHashToBase64(deliverySignal.ReceivedNotice[0]);
            const notice = deliverySignal.ReceivedNotice[1]
            this._perspective.newDeliveryNotices[noticeEh] = notice;
            this._perspective.allNotices[noticeEh] = [0, notice];
            this._perspective.unrepliedInbounds[encodeHashToBase64(notice.sender)] = noticeEh;
        }
        if (SignalProtocolType.ReceivedAck in deliverySignal) {
            console.log("ADDING ReceivedAck", deliverySignal.ReceivedAck);
            const ack = deliverySignal.ReceivedAck;
            const distribEh = encodeHashToBase64(ack.distribution_eh);
            this._perspective.unrepliedOutbounds[distribEh][1][encodeHashToBase64(ack.recipient)] = {NoticeDelivered: null};
        }
        if (SignalProtocolType.ReceivedReply in deliverySignal) {
            console.log("ADDING ReceivedReply", deliverySignal.ReceivedReply);
            const reply = deliverySignal.ReceivedReply;
            const distribEh = encodeHashToBase64(reply.distribution_eh);
            this._perspective.unrepliedOutbounds[distribEh][1][encodeHashToBase64(reply.recipient)] = reply.has_accepted? {ParcelAccepted: null} : {ParcelRefused: null};
            // TODO maybe do a getDeliveryState here as the state could be "PendingParcel"
        }
        if (SignalProtocolType.ReceivedParcel in deliverySignal) {
            console.log("ADDING ReceivedParcel", deliverySignal.ReceivedParcel);
        }
        if (SignalProtocolType.ReceivedReceipt in deliverySignal) {
            console.log("ADDING DeliveryReceipt", deliverySignal.ReceivedReceipt);
            const receipt = deliverySignal.ReceivedReceipt;
            const distribEh = encodeHashToBase64(receipt.distribution_eh);
            this._perspective.unrepliedOutbounds[distribEh][1][encodeHashToBase64(receipt.recipient)] = {ParcelDelivered: null};
        }
        this.notifySubscribers();
    }


    /** -- probe -- */

    /** */
    async probeAll(): Promise<void> {
        this._perspective.myPubEncKey = await this.zomeProxy.getMyEncKey();
        await this.probeInbox();
        await this.queryAll();
        await this.queryDistributions();
        await this.determineUnrepliedInbounds();
        await this.determineUnrepliedOutbounds();
        this.notifySubscribers();
    }


    /** */
    private async probeInbox(): Promise<ActionHashB64[]> {
        const inbox = await this.zomeProxy.pullInbox();
        this._perspective.inbox = inbox.map((ah) => encodeHashToBase64(ah));
        this.notifySubscribers();
        return this._perspective.inbox;
    }


    // /** */
    // async probeDeliveryNotices(): Promise<void> {
    //     const inbox = await this.zomeProxy.pullInbox();
    //     this._perspective.inbox = inbox.map((ah) => encodeHashToBase64(ah));
    //     this.notifySubscribers();
    // }


    /** */
    async probeEncKey(from: AgentPubKeyB64): Promise<Uint8Array> {
        const key = await this.zomeProxy.getEncKey(decodeHashFromBase64(from));
        const maybe = this._perspective.encKeys[from];
        if (!maybe && encodeHashToBase64(maybe) != encodeHashToBase64(key)) {
            this._perspective.encKeys[from] = key;
            this.notifySubscribers();
        }
        return key;
    }


    /** */
    async queryDistributions(): Promise<Dictionary<FullDistributionState>> {
        //console.log("queryDistributions()", this._perspective.myDistributions);
        const distribs = await this.zomeProxy.queryDistribution();
        let promises = [];
        for (const [eh, _distrib] of distribs) {
            const p = this.zomeProxy.getDistributionState(eh);
            promises.push(p);
        }
        const distribPromisesResult = await Promise.allSettled(promises);
        let myDistributions: Dictionary<FullDistributionState> = {};
        let i = 0;
        for (const [eh, distrib] of distribs) {
            if (distribPromisesResult[i].status == "fulfilled") {
                const distribState = (distribPromisesResult[i] as PromiseFulfilledResult<DistributionState>).value;
                const deliveryStates = await this.queryDistribution(eh, distrib);
                myDistributions[encodeHashToBase64(eh)] = [distribState, deliveryStates];
            }
            else {
                console.warn("getDistributionState() failed:", (distribPromisesResult[i] as PromiseRejectedResult).reason);
            }
            i += 1;
        }
        console.log("queryDistributions() result", myDistributions);
        this._perspective.myDistributions = myDistributions;
        this.notifySubscribers();
        return myDistributions;
    }


    /** */
    async queryDistribution(eh: EntryHash, distrib: Distribution): Promise<Dictionary<DeliveryState>> {
        let deliveryPromises = [];
        for (const recipient of distrib.recipients) {
            const p = this.zomeProxy.getDeliveryState({distribution_eh: eh, recipient});
            deliveryPromises.push(p);
        }
        const deliveryPromisesResult = await Promise.allSettled(deliveryPromises);
        let deliveryStates: Dictionary<DeliveryState> = {};
        let j = 0;
        for (const recipient of distrib.recipients) {
            if (deliveryPromisesResult[j].status == "fulfilled") {
                deliveryStates[encodeHashToBase64(recipient)] = (deliveryPromisesResult[j] as PromiseFulfilledResult<DeliveryState>).value;
            } else {
                console.warn("getDeliveryState() failed:", (deliveryPromisesResult[j] as PromiseRejectedResult).reason);
            }
            j += 1;
        }
        return deliveryStates;
    }


    /** */
    async queryAll(): Promise<null> {
        let pairs = [];
        this._perspective.allDistributions = {};
        pairs = await this.zomeProxy.queryAllDistribution();
        Object.values(pairs).map(([eh, ts, typed]) => this._perspective.allDistributions[encodeHashToBase64(eh)] = [ts, typed]);
        console.log("queryAll() distribs: " + pairs.length);


        this._perspective.allReceivedNotices = {};
        pairs = await this.zomeProxy.queryAllNoticeReceived();
        Object.values(pairs).map(([eh, typed]) => this._perspective.allReceivedNotices[encodeHashToBase64(eh)] = typed);

        this._perspective.allReceivedReplies = {};
        pairs = await this.zomeProxy.queryAllReplyReceived();
        Object.values(pairs).map(([eh, typed]) => this._perspective.allReceivedReplies[encodeHashToBase64(eh)] = typed);

        this._perspective.allReceipts = {};
        pairs = await this.zomeProxy.queryAllDeliveryReceipt();
        Object.values(pairs).map(([eh, typed]) => this._perspective.allReceipts[encodeHashToBase64(eh)] = typed);


        this._perspective.allNotices = {};
        pairs = await this.zomeProxy.queryAllDeliveryNotice();
        Object.values(pairs).map(([eh, ts, typed]) => this._perspective.allNotices[encodeHashToBase64(eh)] = [ts, typed]);
        console.log("queryAll() notices: " + pairs.length);

        this._perspective.allReplies = {};
        pairs = await this.zomeProxy.queryAllDeliveryReply();
        Object.values(pairs).map(([eh, typed]) => this._perspective.allReplies[encodeHashToBase64(eh)] = typed);

        this._perspective.allReceivedParcels = {};
        pairs = await this.zomeProxy.queryAllParcelReceived();
        Object.values(pairs).map(([eh, typed]) => this._perspective.allReceivedParcels[encodeHashToBase64(eh)] = typed);

        return null;
    }



    /** */
    async determineUnrepliedInbounds(): Promise<void> {
        this._perspective.unrepliedInbounds = {};
        console.log("determineUnrepliedInbounds() allNotices count", Object.entries(this._perspective.allNotices).length);
        for (const [eh, [_ts, notice]] of Object.entries(this._perspective.allNotices)) {
            const state = await this.getNoticeState(encodeHashToBase64(notice.distribution_eh));
            console.log("determineUnrepliedInbounds() state", state);
            if (NoticeStateType.Unreplied in state) {
                this._perspective.unrepliedInbounds[encodeHashToBase64(notice.sender)] = eh;
            }
        }
        console.log("determineUnrepliedInbounds() count", Object.values(this._perspective.unrepliedInbounds));
        this.notifySubscribers();
    }


    /** */
    async determineUnrepliedOutbounds(): Promise<void> {
        this._perspective.unrepliedOutbounds = {};
        console.log("determineUnrepliedOutbounds() allDistributions count", Object.entries(this._perspective.allDistributions).length);
        for (const [eh, [ts, distrib]] of Object.entries(this._perspective.allDistributions)) {
            const state = await this.getDistributionState(eh);
            console.log("determineUnrepliedOutbounds() distrib state", state);
            if (DistributionStateType.Unsent in state || DistributionStateType.AllNoticesSent in state || DistributionStateType.AllNoticeReceived in state) {
                console.log("determineUnrepliedOutbounds() recipients", distrib.recipients.length);
                let deliveries: Record<AgentPubKeyB64, DeliveryState> = {};
                for (const recipient of distrib.recipients) {
                    const agentB64 = encodeHashToBase64(recipient);
                    const deliveryState = await this.getDeliveryState(eh, agentB64);
                    console.log("determineUnrepliedOutbounds() state", deliveryState, agentB64);
                    deliveries[agentB64] = deliveryState;
                }
                this._perspective.unrepliedOutbounds[eh] = [ts, deliveries];
            }
        }
        console.log("determineUnrepliedOutbounds() count", Object.values(this._perspective.unrepliedOutbounds));
        this.notifySubscribers();
    }


    /** -- API Sugar -- */

    /** */
    async acceptDelivery(noticeEh: EntryHashB64): Promise<EntryHashB64> {
        const [_ts, notice] = this._perspective.allNotices[noticeEh];
        if (!notice) {
            console.error("Accepting unknown notice");
        }
        const eh = await this.zomeProxy.respondToNotice({notice_eh: decodeHashFromBase64(noticeEh), has_accepted: true});
        delete this._perspective.unrepliedInbounds[encodeHashToBase64(notice.sender)];
        this.notifySubscribers();
        return encodeHashToBase64(eh);
    }

    /** */
    async declineDelivery(noticeEh: EntryHashB64): Promise<EntryHashB64> {
        const [_ts, notice] = this._perspective.allNotices[noticeEh];
        if (!notice) {
            console.error("Declining unknown notice");
        }
        const eh = await this.zomeProxy.respondToNotice({notice_eh: decodeHashFromBase64(noticeEh), has_accepted: false});
        delete this._perspective.unrepliedInbounds[encodeHashToBase64(notice.sender)];
        this.notifySubscribers();
        return encodeHashToBase64(eh);
    }

    /** -- API Sugar -- */

    /** */
    async getDeliveryState(distribEh: EntryHashB64, recipient: AgentPubKeyB64): Promise<DeliveryState> {
        return this.zomeProxy.getDeliveryState({distribution_eh: decodeHashFromBase64(distribEh), recipient: decodeHashFromBase64(recipient)});
    }

    /** */
    async getDistributionState(distribEh: EntryHashB64): Promise<DistributionState> {
        return this.zomeProxy.getDistributionState(decodeHashFromBase64(distribEh));
    }

    /** */
    async getNoticeState(noticeEh: EntryHashB64): Promise<NoticeState> {
        return this.zomeProxy.getNoticeState(decodeHashFromBase64(noticeEh));
    }
}