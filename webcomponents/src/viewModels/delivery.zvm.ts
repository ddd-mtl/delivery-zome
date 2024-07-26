import {
    delay,
    ActionId,
    EntryId,
    AgentId,
    enc64,
    AgentIdMap,
    EntryIdMap,
    ActionIdMap,
    ZomeViewModelWithSignals,
    StateChangeType,
    EntryPulseMat,

    LinkPulseMat, holoIdReviver, assertIsDefined,
} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";
import {EntryHashB64, Timestamp} from "@holochain/client";
import {
    DeliveryEntryType,
    DeliveryNotice,
    DeliveryState,
    Distribution,
    DistributionState,
    NoticeAck, NoticeReply,
    NoticeState,
    ParcelChunk,
    ParcelManifest,
    ParcelReference, ReceptionAck, ReceptionProof, ReplyAck,
} from "../bindings/delivery.types";
import {
    DeliveryPerspective,
    DeliveryPerspectiveMutable, DeliverySnapshot,
    materializeParcelManifest,
    ParcelManifestMat,
} from "./delivery.perspective";
import {decode} from "@msgpack/msgpack";
import {DeliveryLinkType} from "../bindings/delivery.integrity";


/**
 *
 */
export class DeliveryZvm extends ZomeViewModelWithSignals {

    static readonly ZOME_PROXY = DeliveryProxy;
    get zomeProxy(): DeliveryProxy {
        return this._zomeProxy as DeliveryProxy;
    }


    /** -- ViewModel -- */

    private _perspective: DeliveryPerspectiveMutable = new DeliveryPerspectiveMutable();


    /* */
    get perspective(): DeliveryPerspective {
        return this._perspective.readonly;
    }

    /* */
    protected hasChanged(): boolean {
        // TODO
        return true;
    }


    /** -- Signals -- */


    /** */
    async handleLinkPulse(pulse: LinkPulseMat, _from: AgentId): Promise<void> {
        switch(pulse.link_type) {
            case DeliveryLinkType.PublicParcels: {
                if (pulse.state == StateChangeType.Delete) {
                    const ppEh = EntryId.from(pulse.target);
                    const parcelEh = this._perspective.parcelReferences.get(ppEh);
                    if (!parcelEh) {
                        console.warn("Unknown deleted PublicParcel", parcelEh);
                        return;
                    }
                    const pprm = this._perspective.publicParcels.get(parcelEh);
                    if (!pprm) {
                        console.warn("Unknown deleted Parcel", parcelEh);
                        return;
                    }
                    const current = this._perspective.publicParcels.get(parcelEh);
                    current.deleteInfo = [pulse.timestamp, pulse.author];
                    this._perspective.publicParcels.set(parcelEh, current);
                }
            }
            break;
            case DeliveryLinkType.Inbox:
            case DeliveryLinkType.Members:
            case DeliveryLinkType.Pendings:
            break;
        }
    }


    /** */
    async handleEntryPulse(pulse: EntryPulseMat, _from: AgentId): Promise<void> {
        switch(pulse.entryType) {
            case DeliveryEntryType.PrivateManifest:
            case DeliveryEntryType.PublicManifest:
                const manifest = decode(pulse.bytes) as ParcelManifest;
                if (pulse.state != StateChangeType.Delete) {
                    this._perspective.storeManifest(pulse.eh, pulse.ts, pulse.author, manifest);
                }
            break;
            case DeliveryEntryType.PrivateChunk:
            case DeliveryEntryType.PublicChunk:
                const chunk = decode(pulse.bytes) as ParcelChunk;
                //console.log("Received Chunk", chunk, pulse.visibility, pulse.eh);
                /** Update notice state if Chunk is not from us */
                const manifestPair = this._perspective.localManifestByData[chunk.data_hash];
                if (manifestPair) {
                    const manifestEh = manifestPair[0];
                    const noticeEh = this._perspective.noticeByParcel.get(manifestEh);
                    if (noticeEh) {
                        const noticeTuple = this._perspective.notices.get(noticeEh);
                        noticeTuple[3].delete(pulse.eh.b64);
                        this._perspective.notices.set(noticeEh, noticeTuple);
                        if (noticeTuple[3].size == 0) {
                            this.zomeProxy.completeManifest(manifestEh.hash);
                        } else {
                            // Ask for next chunk?
                        }
                    }
                }
            break;
            case DeliveryEntryType.Distribution: {
                const distribution = decode(pulse.bytes) as Distribution;
                this._perspective.distributions.set(pulse.ah, [distribution, pulse.ts, DistributionState.Unsent, new AgentIdMap<DeliveryState>()]);
                const [fullState, deliveryStates] = await this.getDistributionState(pulse.ah);
                this._perspective.distributions.set(pulse.ah, [distribution, pulse.ts, fullState, deliveryStates]);
            }
            break;
            case DeliveryEntryType.DeliveryNotice:
                const notice = decode(pulse.bytes) as DeliveryNotice;
                const parcelId = new EntryId(notice.summary.parcel_reference.parcel_eh);
                //console.log("Received DeliveryNotice", this._perspective, parcelId, notice);
                this._perspective.notices.set(pulse.eh, [notice, pulse.ts, NoticeState.Unreplied, new Set()]);
                this._perspective.noticeByParcel.set(parcelId, pulse.eh);
                const [noticeState, chunks] = await this.getNoticeState(pulse.eh);
                this._perspective.notices.set(pulse.eh, [notice, pulse.ts, noticeState, chunks]);
                this._perspective.noticeByParcel.set(parcelId, pulse.eh);
            break;
            case DeliveryEntryType.NoticeAck: {
                const noticeAck = decode(pulse.bytes) as NoticeAck;
                const distribAh = new ActionId(noticeAck.distribution_ah);
                const recipient = new AgentId(noticeAck.recipient);
                if (!this._perspective.noticeAcks.get(distribAh)) {
                    this._perspective.noticeAcks.set(distribAh, new AgentIdMap());
                }
                this._perspective.noticeAcks.get(distribAh).set(recipient, [noticeAck, pulse.ts]);
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh);
                this._perspective.distributions.get(distribAh)[2] = fullState;
                this._perspective.distributions.get(distribAh)[3] = deliveryStates;
            }
            break;
            case DeliveryEntryType.NoticeReply: {
                const reply = decode(pulse.bytes) as NoticeReply;
                const noticeEh = new EntryId((reply.notice_eh));
                //console.log("Received NoticeReply", this._perspective, noticeEh, reply);
                this._perspective.replies.set(noticeEh, reply);
                if (this._perspective.notices.get(noticeEh)) {
                    this._perspective.notices.get(noticeEh)[2] = NoticeState.Refused;
                    if (reply.has_accepted) {
                        this._perspective.notices.get(noticeEh)[2] = NoticeState.Accepted;
                    }
                }
            }
            break;
            case DeliveryEntryType.ReplyAck: {
                const replyAck = decode(pulse.bytes) as ReplyAck;
                const distribAh = new ActionId(replyAck.distribution_ah);
                const recipient = new AgentId(replyAck.recipient);
                if (!this._perspective.replyAcks.get(distribAh)) {
                    this._perspective.replyAcks.set(distribAh, new AgentIdMap());
                }
                this._perspective.replyAcks.get(distribAh).set(recipient, [replyAck, pulse.ts]);
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh);
                this._perspective.distributions.get(distribAh)[2] = fullState;
                this._perspective.distributions.get(distribAh)[3] = deliveryStates;
            }
            break;
            case DeliveryEntryType.ReceptionProof: {
                const receptionProof = decode(pulse.bytes) as ReceptionProof;
                const noticeEh = new EntryId(receptionProof.notice_eh);
                //console.log("Received ReceptionProof", noticeEh, receptionProof);
                this._perspective.receptions.set(noticeEh, [receptionProof, pulse.ts]);
                if (this._perspective.notices.get(noticeEh)) {
                    this._perspective.notices.get(noticeEh)[2] = NoticeState.Received;
                }
            }
            break;
            case DeliveryEntryType.ReceptionAck: {
                const receptionAck = decode(pulse.bytes) as ReceptionAck;
                const distribAh = new ActionId(receptionAck.distribution_ah);
                const recipient = new AgentId(receptionAck.recipient);
                if (!this._perspective.receptionAcks.get(distribAh)) {
                    this._perspective.receptionAcks.set(distribAh, new AgentIdMap());
                }
                this._perspective.receptionAcks.get(distribAh).set(recipient, [receptionAck, pulse.ts]);
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh)
                this._perspective.distributions.get(distribAh)[2] = fullState;
                this._perspective.distributions.get(distribAh)[3] = deliveryStates;
            }
            break;
            case DeliveryEntryType.PublicParcel: {
                const pr = decode(pulse.bytes) as ParcelReference;
                const parcelEh = new EntryId(pr.parcel_eh);
                this._perspective.parcelReferences.set(pulse.eh, parcelEh);
                if (pulse.state != StateChangeType.Delete) {
                    this._perspective.publicParcels.set(parcelEh, {
                        prEh: pulse.eh,
                        parcelEh,
                        description: pr.description,
                        creationTs: pulse.ts,
                        author: pulse.author,
                    });
                }
                // else {
                //     delete this._perspective.publicParcels[parcelEh];
                // }
            }
            break;
        }
    }


    /** -- Init -- */

    /** */
    async initializePerspectiveOffline(): Promise<void> {
        await this.zomeProxy.queryAll();
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
        const orphanPublicChunks = publicOrphans.map((eh) => new EntryId(eh));
        const orphanPrivateChunks = privateOrphans.map((eh) => new EntryId(eh));
        this._perspective.storeOrphans(orphanPublicChunks, orphanPrivateChunks);
    }


    /** */
    async requestMissingChunks(noticeEh: EntryId): Promise<void> {
        const notice = this._perspective.notices.get(noticeEh);
        if (!notice) {
            console.warn("Requesting unknown notice");
            return;
        }
        const missingChunks = await this.zomeProxy.determineMissingChunks(notice[0].summary.parcel_reference.parcel_eh);
        const notice_eh = noticeEh.hash;
        for (const chunk_eh of missingChunks) {
            this.zomeProxy.pullChunk({notice_eh, chunk_eh});
        }
    }


    /** -- probe -- */

    /** */
    async probeAllInner(): Promise<void> {
        console.log("DeliveryZvm.probeAllInner()");
        //console.trace();
        await this.zomeProxy.queryAll();
        await this.scanProblems();
        await this.probeDht(true);
        /** */
        this.notifySubscribers();
    }


    private _probeDhtCount = 0;
    get probeDhtCount() {return this._probeDhtCount}

    /** */
    async probeDht(denyNotify?: boolean): Promise<void> {
        //this._perspective.publicParcels = {};
        await this.zomeProxy.pullPublicParcelsDetails();
        const inbox = await this.zomeProxy.processInbox();
        this._perspective.inbox = inbox.map((ah) => new ActionId(ah));
        this._probeDhtCount += 1;
        if (denyNotify == undefined) this.notifySubscribers();
    }


    /** */
    async fetchPublicManifest(manifestEh: EntryId): Promise<[ParcelManifest, Timestamp, AgentId]> {
        assertIsDefined(manifestEh);
        const maybeLocal = this._perspective.localPublicManifests.get(manifestEh);
        if (maybeLocal) {
            return maybeLocal;
        }
        const [manifest, ts, author] = await this.zomeProxy.fetchPublicManifest(manifestEh.hash);
        return [manifest, ts, new AgentId(author)];
    }


    /** Return base64 data string */
    async fetchParcelData(parcelEh: EntryId): Promise<string> {
        assertIsDefined(parcelEh);
        const [manifest, _ts, _author] = await this.fetchPublicManifest(parcelEh);
        let dataB64 = "";
        for (const chunk_eh of manifest.chunks) {
            let chunk = await this.zomeProxy.fetchChunk(chunk_eh);
            dataB64 += chunk.data;
        }
        return dataB64;
    }


    /**
     * Return
     *  - unreplieds: notice_eh -> [notice, Timestamp]
     *  - incompletes: notice_eh -> [notice, Timestamp, MissingChunks]
     */
    inbounds(): [EntryIdMap<[DeliveryNotice, Timestamp]>, EntryIdMap<[DeliveryNotice, Timestamp, Set<EntryHashB64>]>] {
        //console.log("inbounds() allNotices count", Object.entries(this._perspective.notices).length);
        let unreplieds: EntryIdMap<[DeliveryNotice, Timestamp]> = new EntryIdMap();
        let incompletes: EntryIdMap<[DeliveryNotice, Timestamp, Set<EntryHashB64>]> = new EntryIdMap();
        for (const [noticeEh, [notice, ts, state, missingChunks]] of this._perspective.notices.entries()) {
            //const sender = encodeHashToBase64(notice.sender);
            //console.log("inbounds() state", state);
            if (NoticeState.Unreplied == state) {
                unreplieds.set(noticeEh, [notice, ts]);
            }
            if (NoticeState.Accepted == state) {
                incompletes.set(noticeEh, [notice, ts, missingChunks]);
            }
            if (NoticeState.PartiallyReceived == state) {
                incompletes.set(noticeEh, [notice, ts, missingChunks]);
            }
        }
        //console.log("inbounds() count", Object.values(res));
        return [unreplieds, incompletes];
    }


    /** Return distrib_ah -> [distrib, Timestamp, recipient -> state] */
    outbounds(): ActionIdMap<[Distribution, Timestamp, AgentIdMap<DeliveryState>]> {
        //console.log("outbounds() allDistributions count", Object.entries(this._perspective.distributions).length);
        let res: ActionIdMap<[Distribution, Timestamp, AgentIdMap<DeliveryState>]> = new ActionIdMap();
        for (const [distribAh, [distrib, ts, state, deliveryStates]] of this._perspective.distributions.entries()) {
            //console.log("outbounds() distrib state", state);
            if (DistributionState.Unsent == state
              || DistributionState.AllNoticesSent == state
              || DistributionState.AllNoticeReceived == state
              || DistributionState.AllRepliesReceived == state
            ) {
                //console.log("outbounds() recipients", distrib.recipients.length);
                for (const [recipient, state] of Array.from(deliveryStates.entries())) {
                    //console.log("outbounds() state", deliveryStates[agentB64], agentB64);
                    if (!(DeliveryState.ParcelDelivered == state)) {
                        if (!res.get(distribAh)) {
                            res.set(distribAh, [distrib, ts, new AgentIdMap()]);
                        }
                        res.get(distribAh)[2].set(recipient, state);
                    }
                }
            }
        }
        //console.log("outbounds() count", Object.values(res));
        return res;
    }


    /** -- API -- */

    /** */
    async acceptDelivery(noticeEh: EntryId): Promise<EntryId> {
        const [_ts, notice] = this._perspective.notices.get(noticeEh);
        if (!notice) {
            console.error("Accepting unknown notice");
        }
        const replyEh = await this.zomeProxy.respondToNotice({notice_eh:  noticeEh.hash, has_accepted: true});
        return new EntryId(replyEh);
    }

    /** */
    async declineDelivery(noticeEh: EntryId): Promise<EntryId> {
        const [_ts, notice] = this._perspective.notices.get(noticeEh);
        if (!notice) {
            console.error("Declining unknown notice");
        }
        const eh = await this.zomeProxy.respondToNotice({notice_eh: noticeEh.hash, has_accepted: false});
        return new EntryId(eh);
    }


    /** -- API -- */

    /** */
    async getDeliveryState(distribAh: ActionId, recipient: AgentId): Promise<DeliveryState> {
        return this.zomeProxy.getDeliveryState({distribution_ah: distribAh.hash, recipient: recipient.hash});
    }


    /** */
    async getDistributionState(distribAh: ActionId, distribution?: Distribution): Promise<[DistributionState, AgentIdMap<DeliveryState>]> {
        const fullState = await this.zomeProxy.getDistributionState(distribAh.hash);
        let deliveryStates: AgentIdMap<DeliveryState> = new AgentIdMap();
        let i = 0;
        if (!distribution) {
            distribution = this._perspective.distributions.get(distribAh)[0];
            if (!distribution) {
                console.error("Distribution not found");
                return Promise.reject(new Error('Distribution not found'));
            }
        }
        for(const recipient of distribution.recipients) {
            deliveryStates.set(new AgentId(recipient), fullState.delivery_states[i]);
            i += 1;
        }
        return [fullState.distribution_state, deliveryStates];
    }


    /** */
    async getNoticeState(noticeEh: EntryId): Promise<[NoticeState, Set<EntryHashB64>]> {
        const [state, missing_chunks] = await this.zomeProxy.getNoticeState(noticeEh.hash);
        const missingChunks = missing_chunks.map((chunk_eh) => enc64(chunk_eh));
        return [state, new Set(missingChunks)];
    }


    /** */
    async fetchAllPublicManifest(): Promise<[ParcelManifestMat, Timestamp, AgentId][]> {
        const manifests: [ParcelManifestMat, Timestamp, AgentId][] = [];
        for (const [parcelEh, pprm] of this._perspective.publicParcels.entries()) {
            if (pprm.deleteInfo) {
                continue;
            }
            const [manifest, _ts2, _author] = await this.fetchPublicManifest(parcelEh);
            manifests.push([materializeParcelManifest(manifest), pprm.creationTs!, pprm.author!]);
        }
        return manifests;
    }


    /** Dump perspective as JSON  (caller should call getAllPublicManifest() first) */
    export(/*originalsZvm: AuthorshipZvm*/): string {
        const snapshot = this._perspective.makeSnapshot();
        return JSON.stringify(snapshot, null, 2);
    }

    /** */
    import(json: string, _canPublish: boolean) {
        const snapshot = JSON.parse(json, holoIdReviver) as DeliverySnapshot;
        // if (canPublish) {
        // }
        this._perspective.restore(snapshot)
    }

}
