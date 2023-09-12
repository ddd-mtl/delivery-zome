import {delay, Dictionary, ZomeViewModel} from "@ddd-qc/lit-happ";
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
import {createDeliveryPerspective, createFds, DeliveryPerspective, FullDistributionState} from "./delivery.perspective";



/**
 *
 */
export class DeliveryZvm extends ZomeViewModel {

    static readonly ZOME_PROXY = DeliveryProxy;

    get zomeProxy(): DeliveryProxy {
        return this._zomeProxy as DeliveryProxy;
    }

    /** -- ViewModel -- */

    private _perspective: DeliveryPerspective = createDeliveryPerspective();


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


    /** Determine full state of Distribution and add it to the perspective */
    async onDistributionCreated(signal: SignalProtocolVariantDistributionCreated): Promise<void> {
        const distribEh = encodeHashToBase64(signal.DistributionCreated[0]);
        const ts = signal.DistributionCreated[1];
        const distribution = signal.DistributionCreated[2];

        let fullState: FullDistributionState = await this.getDistributionState(distribEh);
        let deliveryStates: Dictionary<DeliveryState> = {};
        let i = 0;
        for(const recipient of distribution.recipients) {
            deliveryStates[encodeHashToBase64(recipient)] = fullState.delivery_states[i];
            i += 1;
        }
        this._perspective.distributions[distribEh] = [distribution, ts, fullState.distribution_state, deliveryStates];
        //this._perspective.unrepliedOutbounds[distribEh] = [ts, deliveries];

        this.notifySubscribers();
    }


    /** */
    mySignalHandler(signal: AppSignal): void {
        console.log("DELIVERY received signal", signal);
        const deliverySignal = signal.payload as SignalProtocol;
        if (SignalProtocolType.DistributionCreated in deliverySignal) {
            console.log("signal DistributionCreated", deliverySignal.DistributionCreated);
            this.onDistributionCreated(deliverySignal);
        }
        if (SignalProtocolType.ReceivedNotice in deliverySignal) {
            console.log("signal DeliveryNotice", deliverySignal.ReceivedNotice);
            const noticeEh = encodeHashToBase64(deliverySignal.ReceivedNotice[0]);
            const ts = deliverySignal.ReceivedNotice[1];
            const notice = deliverySignal.ReceivedNotice[2];
            const sender = encodeHashToBase64(notice.sender);
            //this._perspective.newDeliveryNotices[noticeEh] = notice;
            this._perspective.notices[noticeEh] = [0, notice];
            if (!this._perspective.unrepliedInbounds[sender]) {
                this._perspective.unrepliedInbounds[sender] = {};
            }
            if (!(noticeEh in this._perspective.unrepliedInbounds[sender])) {
                this._perspective.unrepliedInbounds[sender][noticeEh] = ts;
            }

        }
        if (SignalProtocolType.ReceivedAck in deliverySignal) {
            console.log("signal ReceivedAck", deliverySignal.ReceivedAck);
            const ack = deliverySignal.ReceivedAck;
            const distribEh = encodeHashToBase64(ack.distribution_eh);
            if (!this._perspective.unrepliedOutbounds[distribEh]) {
                delay(200).then(() => {
                    this._perspective.unrepliedOutbounds[distribEh][1][encodeHashToBase64(ack.recipient)] = {NoticeDelivered: null};
                })
            } else {
                this._perspective.unrepliedOutbounds[distribEh][1][encodeHashToBase64(ack.recipient)] = {NoticeDelivered: null};
            }
        }
        if (SignalProtocolType.ReceivedReply in deliverySignal) {
            console.log("signal ReceivedReply", deliverySignal.ReceivedReply);
            const reply = deliverySignal.ReceivedReply;
            const distribEh = encodeHashToBase64(reply.distribution_eh);
            this._perspective.unrepliedOutbounds[distribEh][1][encodeHashToBase64(reply.recipient)] = reply.has_accepted? {ParcelAccepted: null} : {ParcelRefused: null};
            // TODO maybe do a getDeliveryState here as the state could be "PendingParcel"
        }
        if (SignalProtocolType.ReceivedParcel in deliverySignal) {
            console.log("signal ReceivedParcel", deliverySignal.ReceivedParcel);
            const noticeEh = encodeHashToBase64(deliverySignal.ReceivedParcel.notice_eh);
            const notice = this._perspective.notices[noticeEh][1];
            delete this._perspective.pendingInbounds[encodeHashToBase64(notice.sender)][noticeEh];

        }
        if (SignalProtocolType.ReceivedReceipt in deliverySignal) {
            console.log("signal DeliveryReceipt", deliverySignal.ReceivedReceipt);
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
                const [distribState, deliveryStates] = (distribPromisesResult[i] as PromiseFulfilledResult<FullDistributionState>).value;
                //const deliveryStates  = await this.queryDistribution(eh, distrib);
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
        this._perspective.distributions = {};
        pairs = await this.zomeProxy.queryAllDistribution();
        Object.values(pairs).map(([eh, ts, typed]) => this._perspective.distributions[encodeHashToBase64(eh)] = [ts, typed]);
        console.log("queryAll() distribs: " + pairs.length);


        this._perspective.noticeAcks = {};
        pairs = await this.zomeProxy.queryAllNoticeReceived();
        Object.values(pairs).map(([eh, typed]) => this._perspective.noticeAcks[encodeHashToBase64(eh)] = typed);

        this._perspective.replyAcks = {};
        pairs = await this.zomeProxy.queryAllReplyReceived();
        Object.values(pairs).map(([eh, typed]) => this._perspective.replyAcks[encodeHashToBase64(eh)] = typed);

        this._perspective.receipts = {};
        pairs = await this.zomeProxy.queryAllDeliveryReceipt();
        Object.values(pairs).map(([eh, typed]) => this._perspective.receipts[encodeHashToBase64(eh)] = typed);


        this._perspective.notices = {};
        pairs = await this.zomeProxy.queryAllDeliveryNotice();
        Object.values(pairs).map(([eh, ts, typed]) => this._perspective.notices[encodeHashToBase64(eh)] = [ts, typed]);
        console.log("queryAll() notices: " + pairs.length);

        this._perspective.replies = {};
        pairs = await this.zomeProxy.queryAllDeliveryReply();
        Object.values(pairs).map(([eh, typed]) => this._perspective.replies[encodeHashToBase64(eh)] = typed);

        this._perspective.parcelAcks = {};
        pairs = await this.zomeProxy.queryAllParcelReceived();
        Object.values(pairs).map(([eh, typed]) => this._perspective.parcelAcks[encodeHashToBase64(eh)] = typed);

        return null;
    }



    /** */
    async determineUnrepliedInbounds(): Promise<void> {
        this._perspective.unrepliedInbounds = {};
        this._perspective.pendingInbounds = {};
        console.log("determineUnrepliedInbounds() allNotices count", Object.entries(this._perspective.notices).length);
        for (const [noticeEh, [ts, notice]] of Object.entries(this._perspective.notices)) {
            const state = await this.getNoticeState(noticeEh);
            const sender = encodeHashToBase64(notice.sender);
            console.log("determineUnrepliedInbounds() state", state);
            if (NoticeStateType.Unreplied in state) {
                if (!this._perspective.unrepliedInbounds[sender]) {
                    this._perspective.unrepliedInbounds[sender] = {};
                }
                this._perspective.unrepliedInbounds[sender][noticeEh] = ts;
            }
            if (NoticeStateType.Accepted in state) {
                if (!this._perspective.pendingInbounds[sender]) {
                    this._perspective.pendingInbounds[sender] = {};
                }
                this._perspective.pendingInbounds[sender][noticeEh] = ts;
            }
        }
        console.log("determineUnrepliedInbounds() count", Object.values(this._perspective.unrepliedInbounds));
        this.notifySubscribers();
    }


    /** */
    async determineUnrepliedOutbounds(): Promise<void> {
        this._perspective.unrepliedOutbounds = {};
        console.log("determineUnrepliedOutbounds() allDistributions count", Object.entries(this._perspective.distributions).length);
        for (const [eh, [ts, distrib]] of Object.entries(this._perspective.distributions)) {
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
        const [_ts, notice] = this._perspective.notices[noticeEh];
        if (!notice) {
            console.error("Accepting unknown notice");
        }
        const eh = await this.zomeProxy.respondToNotice({notice_eh: decodeHashFromBase64(noticeEh), has_accepted: true});
        const ts = this._perspective.unrepliedInbounds[encodeHashToBase64(notice.sender)][noticeEh];
        const sender = encodeHashToBase64(notice.sender);
        delete this._perspective.unrepliedInbounds[sender][noticeEh];
        if (!this._perspective.pendingInbounds[sender]) {
            this._perspective.pendingInbounds[sender] = {};
        }
        this._perspective.pendingInbounds[sender][noticeEh] = ts;
        this.notifySubscribers();
        return encodeHashToBase64(eh);
    }

    /** */
    async declineDelivery(noticeEh: EntryHashB64): Promise<EntryHashB64> {
        const [_ts, notice] = this._perspective.notices[noticeEh];
        if (!notice) {
            console.error("Declining unknown notice");
        }
        const eh = await this.zomeProxy.respondToNotice({notice_eh: decodeHashFromBase64(noticeEh), has_accepted: false});
        delete this._perspective.unrepliedInbounds[encodeHashToBase64(notice.sender)][noticeEh];
        this.notifySubscribers();
        return encodeHashToBase64(eh);
    }

    /** -- API Sugar -- */

    /** */
    async getDeliveryState(distribEh: EntryHashB64, recipient: AgentPubKeyB64): Promise<DeliveryState> {
        return this.zomeProxy.getDeliveryState({distribution_eh: decodeHashFromBase64(distribEh), recipient: decodeHashFromBase64(recipient)});
    }

    /** */
    async getDistributionState(distribEh: EntryHashB64): Promise<FullDistributionState> {
        return this.zomeProxy.getDistributionState(decodeHashFromBase64(distribEh));
    }

    /** */
    async getNoticeState(noticeEh: EntryHashB64): Promise<NoticeState> {
        return this.zomeProxy.getNoticeState(decodeHashFromBase64(noticeEh));
    }
}