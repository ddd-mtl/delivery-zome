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
    NoticeStateType, ParcelManifest,
    SignalProtocol,
    SignalProtocolType,
} from "../bindings/delivery.types";
import {AppSignal} from "@holochain/client/lib/api/app/types";
import {
    createDeliveryPerspective,
    DeliveryPerspective,
    materializeParcelManifest, materializePublicParcelRecord,
    ParcelManifestMat, PublicParcelRecordMat
} from "./delivery.perspective";


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
        if (SignalProtocolType.NewLocalManifest in deliverySignal) {
            console.log("signal NewLocalManifest", deliverySignal.NewLocalManifest);
            const manifestEh = encodeHashToBase64(deliverySignal.NewLocalManifest[0]);
            const ts = deliverySignal.NewLocalManifest[1];
            const manifest = deliverySignal.NewLocalManifest[2];
            const isPrivate = "Private" in manifest.description.visibility;
            this._perspective.localManifestByData[manifest.data_hash] = [manifestEh, isPrivate];
            if (isPrivate) {
                this._perspective.privateManifests[manifestEh] = [manifest, ts];
                const maybeNoticeEh = this._perspective.noticeByParcel[manifestEh];
                if (maybeNoticeEh) {
                    this._perspective.notices[maybeNoticeEh][2] = {PartiallyReceived: null};
                    this._perspective.notices[maybeNoticeEh][3] = new Set(manifest.chunks.map((eh) => encodeHashToBase64(eh)));
                }
            } else {
                this._perspective.localPublicManifests[manifestEh] = [manifest, ts];
            }
        }
        if (SignalProtocolType.NewLocalChunk in deliverySignal) {
            console.log("signal NewLocalChunk", deliverySignal.NewLocalChunk);
            const chunkEh = encodeHashToBase64(deliverySignal.NewLocalChunk[0]);
            const chunk = deliverySignal.NewLocalChunk[1];
            /** Update notice state if Chunk is not from us */
            const manifestPair = this._perspective.localManifestByData[chunk.data_hash];
            if (manifestPair) {
                const manifestEh = manifestPair[0];
                const noticeEh = this._perspective.noticeByParcel[manifestEh];
                if (noticeEh) {
                    this._perspective.notices[noticeEh][3].delete(chunkEh);
                    if (this._perspective.notices[noticeEh][3].size == 0) {
                        this.zomeProxy.completeManifest(decodeHashFromBase64(manifestEh));
                    } else {
                        // Ask for next chunk?
                    }
                }
            }
        }
        if (SignalProtocolType.NewDistribution in deliverySignal) {
            console.log("signal NewDistribution", deliverySignal.NewDistribution);
            const distribAh = encodeHashToBase64(deliverySignal.NewDistribution[0]);
            const distribution = deliverySignal.NewDistribution[2];
            const ts = deliverySignal.NewDistribution[1];
            this._perspective.distributions[distribAh] = [distribution, ts, {Unsent: null}, {}];
            this.getDistributionState(distribAh).then(([fullState, deliveryStates]) => {
                this._perspective.distributions[distribAh] = [distribution, ts, fullState, deliveryStates];
                this.notifySubscribers();
            });
        }
        if (SignalProtocolType.NewNotice in deliverySignal) {
            console.log("signal NewNotice", deliverySignal.NewNotice);
            const noticeEh = encodeHashToBase64(deliverySignal.NewNotice[0]);
            const notice = deliverySignal.NewNotice[2];
            const ts = deliverySignal.NewNotice[1];
            this._perspective.notices[noticeEh] = [notice, ts, {Unreplied: null}, new Set()];
            this._perspective.noticeByParcel[encodeHashToBase64(notice.summary.parcel_reference.eh)] = noticeEh;
        }
        if (SignalProtocolType.NewNoticeAck in deliverySignal) {
            console.log("signal NewNoticeAck", deliverySignal.NewNoticeAck);
            const noticeAck = deliverySignal.NewNoticeAck[2];
            const ts = deliverySignal.NewNoticeAck[1];
            const distribAh = encodeHashToBase64(noticeAck.distribution_ah);
            const recipient = encodeHashToBase64(noticeAck.recipient);
            if (!this._perspective.noticeAcks[distribAh]) {
                this._perspective.noticeAcks[distribAh] = {};
            }
            this._perspective.noticeAcks[distribAh][recipient] = [noticeAck, ts];
            this.getDistributionState(distribAh).then(([fullState, deliveryStates]) => {
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
                this.notifySubscribers();
            });
        }
        if (SignalProtocolType.NewReply in deliverySignal) {
            console.log("signal NewReply", deliverySignal.NewReply);
            const reply = deliverySignal.NewReply[2];
            const noticeEh = encodeHashToBase64((reply.notice_eh));
            this._perspective.replies[noticeEh] = reply;
            this._perspective.notices[noticeEh][2] = {Refused: null};
            if (reply.has_accepted) {
                this._perspective.notices[noticeEh][2] = {Accepted: null};
            }
        }
        if (SignalProtocolType.NewReplyAck in deliverySignal) {
            console.log("signal NewReplyAck", deliverySignal.NewReplyAck);
            const replyAck = deliverySignal.NewReplyAck[2];
            const ts = deliverySignal.NewReplyAck[1];
            const distribAh = encodeHashToBase64(replyAck.distribution_ah);
            const recipient = encodeHashToBase64(replyAck.recipient);
            if (!this._perspective.replyAcks[distribAh]) {
                this._perspective.replyAcks[distribAh] = {};
            }
            this._perspective.replyAcks[distribAh][recipient] = [replyAck, ts];
            this.getDistributionState(distribAh).then(([fullState, deliveryStates]) => {
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
                this.notifySubscribers();
            });
        }
        if (SignalProtocolType.NewReceptionProof in deliverySignal) {
            console.log("signal NewReceptionProof", deliverySignal.NewReceptionProof);
            const receptionProof = deliverySignal.NewReceptionProof[2];
            const ts = deliverySignal.NewReceptionProof[1];
            const noticeEh = encodeHashToBase64(receptionProof.notice_eh);
            this._perspective.receptions[noticeEh] = [receptionProof, ts];
            this._perspective.notices[noticeEh][2] = {Received: null};
        }
        if (SignalProtocolType.NewReceptionAck in deliverySignal) {
            console.log("signal NewReceptionAck", deliverySignal.NewReceptionAck);
            const receptionAck = deliverySignal.NewReceptionAck[2];
            const ts = deliverySignal.NewReceptionAck[1];
            const distribAh = encodeHashToBase64(receptionAck.distribution_ah);
            const recipient = encodeHashToBase64(receptionAck.recipient);
            if (!this._perspective.receptionAcks[distribAh]) {
                this._perspective.receptionAcks[distribAh] = {};
            }
            this._perspective.receptionAcks[distribAh][recipient] = [receptionAck, ts];
            this.getDistributionState(distribAh).then(([fullState, deliveryStates]) => {
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
                this.notifySubscribers();
            });
        }
        if (SignalProtocolType.NewPendingItem in deliverySignal) {
            console.log("signal NewPendingItem", deliverySignal.NewPendingItem);
        }
        if (SignalProtocolType.NewPublicParcel in deliverySignal) {
            console.log("signal NewPublicParcel", deliverySignal.NewPublicParcel);
            const author = deliverySignal.NewPublicParcel[3];
            const pr = deliverySignal.NewPublicParcel[2];
            const ts = deliverySignal.NewPublicParcel[1];
            const prEh = encodeHashToBase64(deliverySignal.NewPublicParcel[0]);
            const ppEh = encodeHashToBase64(pr.eh);
            this._perspective.publicParcels[ppEh] = {prEh, ppEh, description: pr.description, creationTs: ts, author: encodeHashToBase64(author)};
            this._perspective.parcelReferences[prEh] = ppEh;
        }
        if (SignalProtocolType.RemovedPublicParcel in deliverySignal) {
            console.log("signal RemovedPublicParcel", deliverySignal.RemovedPublicParcel);
            const author = deliverySignal.RemovedPublicParcel[3];
            const pr = deliverySignal.RemovedPublicParcel[2];
            const del_ts = deliverySignal.RemovedPublicParcel[1];
            //const pr_eh = deliverySignal.RemovedPublicParcel[0];
            const ppEh = encodeHashToBase64(pr.eh);
            const created = this._perspective.publicParcels[ppEh];
            if (created) {
                this._perspective.publicParcels[ppEh].deleteInfo = [del_ts, encodeHashToBase64(author)];
            } else {
                console.warn("Unknown Removed PublicParcel", ppEh);
            }
        }
        /** Done */
        this.notifySubscribers();
    }


    /** -- Init -- */


    /** */
    async initializePerspectiveOffline(): Promise<void> {
        await this.queryAll();
        await this.scanProblems();
    }


    /** */
    async initializePerspectiveOnline(): Promise<void> {
        await this.probeDht();
    }


    /** */
    async scanProblems(): Promise<void> {
        // this._perspective.incompleteManifests = (await this.zomeProxy.scanIncompleteManifests())
        //   .map((eh) => encodeHashToBase64(eh));
        const [publicOrphans, privateOrphans] = await this.zomeProxy.scanOrphanChunks();
        this._perspective.orphanPublicChunks = publicOrphans.map((eh) => encodeHashToBase64(eh));
        this._perspective.orphanPrivateChunks = privateOrphans.map((eh) => encodeHashToBase64(eh));
    }


    /** */
    async requestMissingChunks(noticeEh: EntryHashB64): Promise<void> {
        const notice = this._perspective.notices[noticeEh];
        if (!notice) {
            console.warn("Requesting unknown notice");
            return;
        }
        const missingChunks = await this.zomeProxy.determineMissingChunks(notice[0].summary.parcel_reference.eh);
        const notice_eh = decodeHashFromBase64(noticeEh);
        for (const chunk_eh of missingChunks) {
            this.zomeProxy.fetchChunk({notice_eh, chunk_eh});
        }
    }


    /** -- probe -- */

    /** */
    async probeAllInner(): Promise<void> {
        await this.queryAll();
        await this.scanProblems();
        await this.probeDht(true);
        /** */
        this.notifySubscribers();
    }


    /** */
    async probeDht(denyNotify?: boolean): Promise<void> {
        const pds = await this.probePublicParcels(true);
        //const rems = await this.probePublicParcelsRemoved(true);
        await this.probeInbox(true);
        this._perspective.probeDhtCount += 1;
        if (denyNotify == undefined) this.notifySubscribers();
        console.log(`probeDht[${this._perspective.probeDhtCount}] PublicParcels count: ${Object.entries(pds).length}`);
    }


    /** */
    private async probePublicParcels(denyNotify?: boolean): Promise<Dictionary<PublicParcelRecordMat>> {
        const prs = await this.zomeProxy.pullPublicParcelsDetails();
        this._perspective.publicParcels = {};
        prs.map((ppr) => {
            const pprm = materializePublicParcelRecord(ppr);
            this._perspective.publicParcels[pprm.ppEh] = pprm;
            this._perspective.parcelReferences[pprm.prEh] = pprm.ppEh;
        });
        if (denyNotify == undefined) this.notifySubscribers();
        return this._perspective.publicParcels;
    }


    // /** */
    // private async probePublicParcelsRemoved(denyNotify?: boolean): Promise<Dictionary<[EntryHashB64, ParcelDescription, Timestamp, Timestamp, AgentPubKeyB64]>> {
    //     const prs = await this.zomeProxy.pullRemovedPublicParcels();
    //     this._perspective.publicParcelsRemoved = {};
    //     prs.map(async ([pr_eh, create_ts, del_ts, author]) => {
    //         const prEh = encodeHashToBase64(pr_eh);
    //         let pd = undefined;
    //         let ppEh = this._perspective.parcelReferences[prEh];
    //         if (ppEh && this._perspective.publicParcels[ppEh]) {
    //             const tuple = this._perspective.publicParcels[ppEh];
    //             pd = tuple[1];
    //         } else {
    //             const pr = await this.zomeProxy.getParcelRef(pr_eh);
    //             pd = pr!.description;
    //         }
    //         this._perspective.publicParcelsRemoved[ppEh] = [prEh, pd, create_ts, del_ts, encodeHashToBase64(author)];
    //         this._perspective.parcelReferences[prEh] = ppEh;
    //     });
    //     if (denyNotify == undefined) this.notifySubscribers();
    //     return this._perspective.publicParcelsRemoved;
    // }


    /** */
    private async probeInbox(denyNotify?: boolean): Promise<ActionHashB64[]> {
        const inbox = await this.zomeProxy.pullInbox();
        this._perspective.inbox = inbox.map((ah) => encodeHashToBase64(ah));
        if (denyNotify == undefined) this.notifySubscribers();
        return this._perspective.inbox;
    }


    /** */
    async getManifest(manifestEh: EntryHashB64, preventNotify?: boolean): Promise<[ParcelManifest, Timestamp]> {
        const [manifest, ts, author] = await this.zomeProxy.getManifest(decodeHashFromBase64(manifestEh));
        this._perspective.localPublicManifests[manifestEh] = [manifest, ts];
        this._perspective.localManifestByData[manifest.data_hash] = [manifestEh, false];
        if (!preventNotify) {
            this.notifySubscribers();
        }
        return [manifest, ts];
    }


    /** Return base64 data string */
    async getParcelData(parcelEh: EntryHashB64): Promise<string> {
        // const pd = this._perspective.publicParcels[parcelEh];
        // if (!pd) {
        //     return Promise.reject("Unknown PublicParcel");
        // }
        const [manifest, _ts] = await this.getManifest(parcelEh);
        let dataB64 = "";
        for (const chunk_eh of manifest.chunks) {
            let chunk = await this.zomeProxy.getChunk(chunk_eh);
            dataB64 += chunk.data;
        }
        return dataB64;
    }


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
        Object.values(tuples).map(([_eh, ts, typed]) => {
            const distribAh = encodeHashToBase64(typed.distribution_ah);
            const recipient = encodeHashToBase64(typed.recipient);
            if (!this._perspective.noticeAcks[distribAh]) {
                this._perspective.noticeAcks[distribAh] = {};
            }
            this._perspective.noticeAcks[distribAh][recipient] = [typed, ts]
        });

        this._perspective.replyAcks = {};
        tuples = await this.zomeProxy.queryAllReplyAck();
        Object.values(tuples).map(([_eh, ts, typed]) => {
            const distribAh = encodeHashToBase64(typed.distribution_ah);
            const recipient = encodeHashToBase64(typed.recipient);
            if (!this._perspective.replyAcks[distribAh]) {
                this._perspective.replyAcks[distribAh] = {};
            }
            this._perspective.replyAcks[distribAh][recipient] = [typed, ts]
        });

        this._perspective.receptionAcks = {};
        tuples = await this.zomeProxy.queryAllReceptionAck();
        Object.values(tuples).map(([_eh, ts, typed]) => {
            const distribAh = encodeHashToBase64(typed.distribution_ah);
            const recipient = encodeHashToBase64(typed.recipient);
            if (!this._perspective.receptionAcks[distribAh]) {
                this._perspective.receptionAcks[distribAh] = {};
            }
            this._perspective.receptionAcks[distribAh][recipient] = [typed, ts]
        });

        this._perspective.notices = {};
        tuples = await this.zomeProxy.queryAllDeliveryNotice();
        Object.values(tuples).map(async([eh, ts, notice]) => {
            const noticeEh = encodeHashToBase64(eh);
            const [state, pct] = await this.getNoticeState(noticeEh);
            this._perspective.notices[noticeEh] = [notice, ts, state, pct];
            this._perspective.noticeByParcel[encodeHashToBase64(notice.summary.parcel_reference.eh)] = noticeEh;
        });
        console.log("queryAll() notices: " + tuples.length);

        this._perspective.replies = {};
        tuples = await this.zomeProxy.queryAllNoticeReply();
        Object.values(tuples).map(([_eh, ts, reply]) => this._perspective.replies[encodeHashToBase64(reply.notice_eh)] = reply);

        this._perspective.receptions = {};
        tuples = await this.zomeProxy.queryAllReceptionProof();
        Object.values(tuples).map(([_eh, ts, recepetion]) => this._perspective.receptions[encodeHashToBase64(recepetion.notice_eh)] = [recepetion, ts]);


        this._perspective.privateManifests = {};
        tuples = await this.zomeProxy.queryAllPrivateManifests();
        Object.values(tuples).map(([eh, ts, manifest]) => {
            const manifestEh = encodeHashToBase64(eh);
            this._perspective.privateManifests[manifestEh] = [manifest, ts];
            this._perspective.localManifestByData[manifest.data_hash] = [manifestEh, true];
        });

        this._perspective.localPublicManifests = {};
        tuples = await this.zomeProxy.queryAllPublicManifests();
        Object.values(tuples).map(([eh, ts, manifest]) => {
            const manifestEh = encodeHashToBase64(eh);
            this._perspective.localPublicManifests[manifestEh] = [manifest, ts];
            this._perspective.localManifestByData[manifest.data_hash] = [manifestEh, false];
        });

        return null;
    }


    /**
     * Return
     *  - unreplieds: notice_eh -> [notice, Timestamp]
     *  - incompletes: notice_eh -> [notice, Timestamp, MissingChunks]
     */
    inbounds(): [Dictionary<[DeliveryNotice, Timestamp]>, Dictionary<[DeliveryNotice, Timestamp, Set<EntryHashB64>]>] {
        //console.log("inbounds() allNotices count", Object.entries(this._perspective.notices).length);
        let unreplieds: Dictionary<[DeliveryNotice, Timestamp]> = {};
        let incompletes: Dictionary<[DeliveryNotice, Timestamp, Set<EntryHashB64>]> = {};
        for (const [noticeEh, [notice, ts, state, missingChunks]] of Object.entries(this._perspective.notices)) {
            //const sender = encodeHashToBase64(notice.sender);
            //console.log("inbounds() state", state);
            if (NoticeStateType.Unreplied in state) {
                unreplieds[noticeEh] = [notice, ts];
            }
            if (NoticeStateType.Accepted in state) {
                incompletes[noticeEh] = [notice, ts, missingChunks];
            }
            if (NoticeStateType.PartiallyReceived in state) {
                incompletes[noticeEh] = [notice, ts, missingChunks];
            }
        }
        //console.log("inbounds() count", Object.values(res));
        return [unreplieds, incompletes];
    }


    /** Return distrib_ah -> [distrib, Timestamp, recipient -> state] */
    outbounds(): Dictionary<[Distribution, Timestamp, Dictionary<DeliveryState>]> {
        //console.log("outbounds() allDistributions count", Object.entries(this._perspective.distributions).length);
        let res: Dictionary<[Distribution, Timestamp, Dictionary<DeliveryState>]> = {};
        for (const [distribAh, [distrib, ts, state, deliveryStates]] of Object.entries(this._perspective.distributions)) {
            //console.log("outbounds() distrib state", state);
            if (DistributionStateType.Unsent in state
              || DistributionStateType.AllNoticesSent in state
              || DistributionStateType.AllNoticeReceived in state
              || DistributionStateType.AllRepliesReceived in state
            ) {
                //console.log("outbounds() recipients", distrib.recipients.length);
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
        //console.log("outbounds() count", Object.values(res));
        return res;
    }


    /** -- API -- */

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


    /** -- API -- */

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
    async getNoticeState(noticeEh: EntryHashB64): Promise<[NoticeState, Set<EntryHashB64>]> {
        const [state, missing_chunks] = await this.zomeProxy.getNoticeState(decodeHashFromBase64(noticeEh));
        const missingChunks = missing_chunks.map((chunk_eh) => encodeHashToBase64(chunk_eh));
        return [state, new Set(missingChunks)];
    }


    /** */
    async getAllPublicManifest(): Promise<[ParcelManifestMat, Timestamp, AgentPubKeyB64][]> {
        const manifests: [ParcelManifestMat, Timestamp, AgentPubKeyB64][] = [];
        for (const [parcelEh, pprm] of Object.entries(this._perspective.publicParcels)) {
            if (pprm.deleteInfo) {
                continue;
            }
            const [manifest, _ts2] = await this.getManifest(parcelEh, true);
            manifests.push([materializeParcelManifest(manifest), pprm.creationTs, pprm.author]);
        }
        this.notifySubscribers();
        return manifests;
    }


    /** Dump perspective as JSON  (caller should call getAllPublicManifest() first) */
    exportPerspective(/*originalsZvm: AuthorshipZvm*/): string {
        const manifests: [ParcelManifestMat, Timestamp][] = Object.values(this._perspective.localPublicManifests).map(([manifest, ts]) => [materializeParcelManifest(manifest), ts])
        return JSON.stringify(manifests, null, 2);
    }

}
