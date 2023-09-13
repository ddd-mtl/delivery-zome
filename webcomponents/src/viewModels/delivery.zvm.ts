import {delay, Dictionary, ZomeViewModel} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";
import {
    ActionHashB64,
    AgentPubKeyB64, AppSignalCb,
    decodeHashFromBase64,
    encodeHashToBase64, EntryHashB64, Timestamp
} from "@holochain/client";
import {
    DeliveryNotice,
    DeliveryState, DeliveryStateType,
    Distribution,
    DistributionState,
    DistributionStateType,
    NoticeState,
    NoticeStateType,
    SignalProtocol,
    SignalProtocolType,
} from "../bindings/delivery.types";
import {AppSignal} from "@holochain/client/lib/api/app/types";
import {createDeliveryPerspective, DeliveryPerspective} from "./delivery.perspective";


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

    /** Update the perspective accordingly */
    mySignalHandler(signal: AppSignal): void {
        console.log("DELIVERY received signal", signal);
        const deliverySignal = signal.payload as SignalProtocol;
        if (SignalProtocolType.NewManifest in deliverySignal) {
            console.log("signal NewManifest", deliverySignal.NewManifest);
            const manifestEh = encodeHashToBase64(deliverySignal.NewManifest[0]);
            const manifest = deliverySignal.NewManifest[1];
            this._perspective.manifests[manifestEh] = manifest;
            this._perspective.manifestByData[manifest.data_hash] = manifestEh;
        }
        if (SignalProtocolType.ReceivedChunk in deliverySignal) {
            console.log("signal ReceivedChunk", deliverySignal.ReceivedChunk);
            const noticeEh = encodeHashToBase64(deliverySignal.ReceivedChunk[0]);
            const completion_pct = deliverySignal.ReceivedChunk[1];
            const noticeTuple = this._perspective.notices[noticeEh];
            if (!noticeTuple) {
                console.error("Notice not found for chunk", noticeEh);
                return;
            }
            this._perspective.notices[noticeEh][3] = completion_pct;
        }
        if (SignalProtocolType.NewDistribution in deliverySignal) {
            console.log("signal NewDistribution", deliverySignal.NewDistribution);
            const distribAh = encodeHashToBase64(deliverySignal.NewDistribution[0]);
            const distribution = deliverySignal.NewDistribution[1];
            const ts = deliverySignal.NewDistribution[2];
            this._perspective.distributions[distribAh] = [distribution, ts, {Unsent: null}, {}];
            this.getDistributionState(distribAh).then(([fullState, deliveryStates]) => {
                this._perspective.distributions[distribAh] = [distribution, ts, fullState, deliveryStates];
                this.notifySubscribers();
            });
        }
        if (SignalProtocolType.NewNotice in deliverySignal) {
            console.log("signal NewNotice", deliverySignal.NewNotice);
            const noticeEh = encodeHashToBase64(deliverySignal.NewNotice[0]);
            const notice = deliverySignal.NewNotice[1];
            const ts = deliverySignal.NewNotice[2];
            this._perspective.notices[noticeEh] = [notice, ts, {Unreplied: null}, 0];
        }
        if (SignalProtocolType.NewNoticeAck in deliverySignal) {
            console.log("signal NewNoticeAck", deliverySignal.NewNoticeAck);
            const noticeAck = deliverySignal.NewNoticeAck[1];
            const distribAh = encodeHashToBase64(noticeAck.distribution_ah);
            this._perspective.noticeAcks[distribAh] = noticeAck;
            this.getDistributionState(distribAh).then(([fullState, deliveryStates]) => {
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
                this.notifySubscribers();
            });
        }
        if (SignalProtocolType.NewReply in deliverySignal) {
            console.log("signal NewReply", deliverySignal.NewReply);
            const reply = deliverySignal.NewReply[1];
            const noticeEh = encodeHashToBase64((reply.notice_eh));
            this._perspective.replies[noticeEh] = reply;
            this._perspective.notices[noticeEh][2] = reply.has_accepted? {Accepted: null} : {Refused: null};
        }
        if (SignalProtocolType.NewReplyAck in deliverySignal) {
            console.log("signal NewReplyAck", deliverySignal.NewReplyAck);
            const replyAck = deliverySignal.NewReplyAck[1];
            const distribAh = encodeHashToBase64(replyAck.distribution_ah);
            this._perspective.replyAcks[distribAh] = replyAck;
            this.getDistributionState(distribAh).then(([fullState, deliveryStates]) => {
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
                this.notifySubscribers();
            });
        }
        if (SignalProtocolType.NewReceptionProof in deliverySignal) {
            console.log("signal NewReceptionProof", deliverySignal.NewReceptionProof);
            const receptionProof = deliverySignal.NewReceptionProof[1];
            const noticeEh = encodeHashToBase64(receptionProof.notice_eh);
            this._perspective.receptions[noticeEh] = receptionProof;
            this._perspective.notices[noticeEh][2] = {Received: null};
        }
        if (SignalProtocolType.NewReceptionAck in deliverySignal) {
            console.log("signal NewReceptionAck", deliverySignal.NewReceptionAck);
            const receptionAck = deliverySignal.NewReceptionAck[1];
            const distribAh = encodeHashToBase64(receptionAck.distribution_ah);
            this._perspective.receptionAcks[distribAh] = receptionAck;
            this.getDistributionState(distribAh).then(([fullState, deliveryStates]) => {
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
                this.notifySubscribers();
            });
        }
        if (SignalProtocolType.NewPendingItem in deliverySignal) {
            console.log("signal NewPendingItem", deliverySignal.NewPendingItem);
        }
        /** Done */
        this.notifySubscribers();
    }


    /** -- probe -- */

    /** */
    async probeAll(): Promise<void> {
        this._perspective.myPubEncKey = await this.zomeProxy.getMyEncKey();
        await this.queryAll();
        await this.probeInbox();
        //await this.queryDistributions();
        //await this.determineUnrepliedInbounds();
        //await this.determineUnrepliedOutbounds();
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


    // /** */
    // async queryDistributions(): Promise<Dictionary<FullDistributionState>> {
    //     //console.log("queryDistributions()", this._perspective.myDistributions);
    //     const distribs = await this.zomeProxy.queryDistribution();
    //     let promises = [];
    //     for (const [eh, _distrib] of distribs) {
    //         const p = this.zomeProxy.getDistributionState(eh);
    //         promises.push(p);
    //     }
    //     const distribPromisesResult = await Promise.allSettled(promises);
    //     let myDistributions: Dictionary<FullDistributionState> = {};
    //     let i = 0;
    //     for (const [eh, distrib] of distribs) {
    //         if (distribPromisesResult[i].status == "fulfilled") {
    //             const fullState = (distribPromisesResult[i] as PromiseFulfilledResult<FullDistributionState>).value;
    //             //const deliveryStates  = await this.queryDistribution(eh, distrib);
    //             myDistributions[encodeHashToBase64(eh)] = fullState;
    //         }
    //         else {
    //             console.warn("getDistributionState() failed:", (distribPromisesResult[i] as PromiseRejectedResult).reason);
    //         }
    //         i += 1;
    //     }
    //     console.log("queryDistributions() result", myDistributions);
    //     this._perspective.myDistributions = myDistributions;
    //     this.notifySubscribers();
    //     return myDistributions;
    // }


    // /** */
    // async queryDistribution(eh: EntryHash, distrib: Distribution): Promise<Dictionary<DeliveryState>> {
    //     let deliveryPromises = [];
    //     for (const recipient of distrib.recipients) {
    //         const p = this.zomeProxy.getDeliveryState({distribution_eh: eh, recipient});
    //         deliveryPromises.push(p);
    //     }
    //     const deliveryPromisesResult = await Promise.allSettled(deliveryPromises);
    //     let deliveryStates: Dictionary<DeliveryState> = {};
    //     let j = 0;
    //     for (const recipient of distrib.recipients) {
    //         if (deliveryPromisesResult[j].status == "fulfilled") {
    //             deliveryStates[encodeHashToBase64(recipient)] = (deliveryPromisesResult[j] as PromiseFulfilledResult<DeliveryState>).value;
    //         } else {
    //             console.warn("getDeliveryState() failed:", (deliveryPromisesResult[j] as PromiseRejectedResult).reason);
    //         }
    //         j += 1;
    //     }
    //     return deliveryStates;
    // }


    /** */
    async queryAll(): Promise<null> {
        let tuples = [];
        this._perspective.distributions = {};
        tuples = await this.zomeProxy.queryAllDistribution();
        Object.values(tuples).map(async([ah, ts, typed]) => {
            const [fullState, deliveryStates] = await this.getDistributionState(encodeHashToBase64(ah), typed);
            this._perspective.distributions[encodeHashToBase64(ah)] = [typed, ts, fullState, deliveryStates];
        });
        console.log("queryAll() distribs: " + tuples.length);

        this._perspective.noticeAcks = {};
        tuples = await this.zomeProxy.queryAllNoticeAck();
        Object.values(tuples).map(([_eh, typed]) => this._perspective.noticeAcks[encodeHashToBase64(typed.distribution_ah)] = typed);

        this._perspective.replyAcks = {};
        tuples = await this.zomeProxy.queryAllReplyAck();
        Object.values(tuples).map(([_eh, typed]) => this._perspective.replyAcks[encodeHashToBase64(typed.distribution_ah)] = typed);

        this._perspective.receptionAcks = {};
        tuples = await this.zomeProxy.queryAllReceptionAck();
        Object.values(tuples).map(([_eh, typed]) => this._perspective.receptionAcks[encodeHashToBase64(typed.distribution_ah)] = typed);


        this._perspective.notices = {};
        tuples = await this.zomeProxy.queryAllDeliveryNotice();
        Object.values(tuples).map(async([eh, ts, notice]) => {
            const noticeEh = encodeHashToBase64(eh);
            const [state, pct] = await this.getNoticeState(noticeEh);
            this._perspective.notices[noticeEh] = [notice, ts, state, pct];
        });
        console.log("queryAll() notices: " + tuples.length);

        this._perspective.replies = {};
        tuples = await this.zomeProxy.queryAllNoticeReply();
        Object.values(tuples).map(([_eh, reply]) => this._perspective.replies[encodeHashToBase64(reply.notice_eh)] = reply);

        this._perspective.receptions = {};
        tuples = await this.zomeProxy.queryAllReceptionProof();
        Object.values(tuples).map(([_eh, recepetion]) => this._perspective.receptions[encodeHashToBase64(recepetion.notice_eh)] = recepetion);


        this._perspective.manifests = {};
        tuples = await this.zomeProxy.queryAllManifest();
        Object.values(tuples).map(([eh, manifest]) => {
            const manifestEh = encodeHashToBase64(eh);
            this._perspective.manifests[manifestEh] = manifest;
            this._perspective.manifestByData[manifest.data_hash] = manifestEh;
        });


        return null;
    }


    /** Return notice_eh -> [notice, Timestamp, Percentage]  */
    inbounds(): Dictionary<[DeliveryNotice, Timestamp, number]> {
        console.log("inbounds() allNotices count", Object.entries(this._perspective.notices).length);
        let res: Dictionary<[DeliveryNotice, Timestamp, number]> = {};
        for (const [noticeEh, [notice, ts, state, pct]] of Object.entries(this._perspective.notices)) {
            const sender = encodeHashToBase64(notice.sender);
            console.log("inbounds() state", state);
            if (NoticeStateType.Unreplied in state) {
                res[noticeEh] = [notice, ts, -1];
            }
            if (NoticeStateType.Accepted in state) {
                res[noticeEh] = [notice, ts, pct];
            }
        }
        console.log("inbounds() count", Object.values(res));
        return res;
    }


    /** Return distrib_ah -> [distrib, Timestamp, recipient -> state] */
    outbounds(): Dictionary<[Distribution, Timestamp, Dictionary<DeliveryState>]> {
        console.log("outbounds() allDistributions count", Object.entries(this._perspective.distributions).length);
        let res: Dictionary<[Distribution, Timestamp, Dictionary<DeliveryState>]> = {};
        for (const [distribAh, [distrib, ts, state, deliveryStates]] of Object.entries(this._perspective.distributions)) {
            console.log("outbounds() distrib state", state);
            if (DistributionStateType.Unsent in state || DistributionStateType.AllNoticesSent in state || DistributionStateType.AllNoticeReceived in state) {
                console.log("outbounds() recipients", distrib.recipients.length);
                for (const [recipient, state] of Object.entries(deliveryStates)) {
                    //console.log("outbounds() state", deliveryStates[agentB64], agentB64);
                    if (!(DeliveryStateType.ParcelDelivered in state)) {
                        if (!res[distribAh]) {
                            res[distribAh] = [distrib, ts, {}];
                        }
                        res[distribAh][2][recipient] = state;
                    }
                }
            }
        }
        console.log("outbounds() count", Object.values(res));
        return res;
    }


    /** -- API Sugar -- */

    /** */
    async acceptDelivery(noticeEh: EntryHashB64): Promise<EntryHashB64> {
        const [_ts, notice] = this._perspective.notices[noticeEh];
        if (!notice) {
            console.error("Accepting unknown notice");
        }
        const replyEh = await this.zomeProxy.respondToNotice({notice_eh: decodeHashFromBase64(noticeEh), has_accepted: true});
        return encodeHashToBase64(replyEh);
    }

    /** */
    async declineDelivery(noticeEh: EntryHashB64): Promise<EntryHashB64> {
        const [_ts, notice] = this._perspective.notices[noticeEh];
        if (!notice) {
            console.error("Declining unknown notice");
        }
        const eh = await this.zomeProxy.respondToNotice({notice_eh: decodeHashFromBase64(noticeEh), has_accepted: false});
        return encodeHashToBase64(eh);
    }


    /** -- API Sugar -- */

    /** */
    async getDeliveryState(distribAh: ActionHashB64, recipient: AgentPubKeyB64): Promise<DeliveryState> {
        return this.zomeProxy.getDeliveryState({distribution_ah: decodeHashFromBase64(distribAh), recipient: decodeHashFromBase64(recipient)});
    }


    /** */
    async getDistributionState(distribAh: ActionHashB64, distribution?: Distribution): Promise<[DistributionState, Dictionary<DeliveryState>]> {
        const fullState = await this.zomeProxy.getDistributionState(decodeHashFromBase64(distribAh));
        let deliveryStates: Dictionary<DeliveryState> = {};
        let i = 0;
        if (!distribution) {
            distribution = this._perspective.distributions[distribAh][0];
            if (!distribution) {
                console.error("Distribution not found");
                return Promise.reject(new Error('Distribution not found'));
            }
        }
        for(const recipient of distribution.recipients) {
            deliveryStates[encodeHashToBase64(recipient)] = fullState.delivery_states[i];
            i += 1;
        }
        return [fullState.distribution_state, deliveryStates];
    }


    /** */
    async getNoticeState(noticeEh: EntryHashB64): Promise<[NoticeState, number]> {
        return this.zomeProxy.getNoticeState(decodeHashFromBase64(noticeEh));
    }
}