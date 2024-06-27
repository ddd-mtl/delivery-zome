import {delay, Dictionary, LitHappSignal, prettyDate, SignalLog, SignalType, ZomeViewModel} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";
import {
    ActionHashB64,
    AgentPubKeyB64, AppSignalCb,
    decodeHashFromBase64,
    encodeHashToBase64, EntryHashB64, Timestamp
} from "@holochain/client";
import {
    DeliveryEntryType,
    DeliveryNotice,
    DeliveryState,
    Distribution,
    DistributionState,
    EntryPulse,
    LinkPulse, NoticeAck, NoticeReply,
    NoticeState,
    ParcelChunk,
    ParcelManifest,
    ParcelReference, ReceptionAck, ReceptionProof, ReplyAck,
    StateChangeType,
    TipProtocol, TipProtocolVariantApp, TipProtocolVariantEntry, TipProtocolVariantLink,
    ZomeSignal,
    ZomeSignalProtocol,
    ZomeSignalProtocolType, ZomeSignalProtocolVariantEntry, ZomeSignalProtocolVariantLink,
} from "../bindings/delivery.types";
import {AppSignal} from "@holochain/client/lib/api/app/types";
import {
    createDeliveryPerspective,
    DeliveryPerspective,
    materializeParcelManifest,
    ParcelManifestMat,
} from "./delivery.perspective";
import {getVariantByIndex, prettyState} from "../utils";
import {decode, encode} from "@msgpack/msgpack";


/** */
export interface CastLog {
    ts: Timestamp,
    tip: TipProtocol,
    peers: AgentPubKeyB64[],
}


/**
 *
 */
export class DeliveryZvm extends ZomeViewModel {

    static readonly ZOME_PROXY = DeliveryProxy;

    get zomeProxy(): DeliveryProxy {
        return this._zomeProxy as DeliveryProxy;
    }

    private _castLogs: CastLog[] = [];


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
    mySignalHandler(appSignal: AppSignal): void {
        console.log("DeliveryZvm.mySignalHandler()", appSignal, DeliveryZvm.DEFAULT_ZOME_NAME);
        if (appSignal.zome_name !== DeliveryZvm.DEFAULT_ZOME_NAME) {
            return;
        }
        const deliverySignal = appSignal.payload as ZomeSignal;
        console.log("DELIVERY received signal", deliverySignal);
        if (!("pulses" in deliverySignal)) {
            return;
        }
        /*await*/ this.handleSignal(deliverySignal);
    }


    /** */
    async handleSignal(signal: ZomeSignal): Promise<void> {
        const from = encodeHashToBase64(signal.from);
        let all = [];
        for (let pulse of signal.pulses) {
            /** -- Handle Signal according to type -- */
            /** Change tip to Entry or Link signal */
            if (ZomeSignalProtocolType.Tip in pulse) {
                pulse = this.handleTip(pulse.Tip as TipProtocol, from)!;
                if (!pulse) {
                    continue;
                }
            }
            if (ZomeSignalProtocolType.Entry in pulse) {
                all.push(this.handleEntrySignal(pulse.Entry as EntryPulse, from));
                continue;
            }
            // if (ZomeSignalProtocolType.Link in pulse) {
            //     all.push(this.handleLinkSignal(pulse.Link as LinkPulse, from));
            //     continue;
            // }
        }
        await Promise.all(all);
        console.log("deliveryZvm.handleSignal() notifySubscribers");
        this.notifySubscribers();
    }


    /** */
    async handleEntrySignal(pulse: EntryPulse, from: AgentPubKeyB64): Promise<void> {
        const entryType = getVariantByIndex(DeliveryEntryType, pulse.def.entry_index);
        const author = encodeHashToBase64(pulse.author);
        const ah = encodeHashToBase64(pulse.ah);
        const eh = encodeHashToBase64(pulse.eh);
        const state = Object.keys(pulse.state)[0];
        const isNew = (pulse.state as any)[state];
        let tip: TipProtocol | undefined = undefined;
        switch(entryType) {
            case "ParcelManifest":
                const manifest = decode(pulse.bytes) as ParcelManifest;
                if (state != StateChangeType.Delete) {
                    this.storeManifest(eh, pulse.ts, manifest);
                }
            break;
            case "ParcelChunk":
                const chunk = decode(pulse.bytes) as ParcelChunk;
                /** Update notice state if Chunk is not from us */
                const manifestPair = this._perspective.localManifestByData[chunk.data_hash];
                if (manifestPair) {
                    const manifestEh = manifestPair[0];
                    const noticeEh = this._perspective.noticeByParcel[manifestEh];
                    if (noticeEh) {
                        this._perspective.notices[noticeEh][3].delete(eh);
                        if (this._perspective.notices[noticeEh][3].size == 0) {
                            this.zomeProxy.completeManifest(decodeHashFromBase64(manifestEh));
                        } else {
                            // Ask for next chunk?
                        }
                    }
                }
            break;
            case "Distribution": {
                const distribution = decode(pulse.bytes) as Distribution;
                this._perspective.distributions[ah] = [distribution, pulse.ts, DistributionState.Unsent, {}];
                const [fullState, deliveryStates] = await this.getDistributionState(ah);
                this._perspective.distributions[ah] = [distribution, pulse.ts, fullState, deliveryStates];
            }
            break;
            case "DeliveryNotice":
                const notice = decode(pulse.bytes) as DeliveryNotice;
                this._perspective.notices[eh] = [notice, pulse.ts, NoticeState.Unreplied, new Set()];
                this._perspective.noticeByParcel[encodeHashToBase64(notice.summary.parcel_reference.parcel_eh)] = eh;
                const [noticeState, pct] = await this.getNoticeState(eh);
                this._perspective.notices[eh] = [notice, pulse.ts, noticeState, pct];
                this._perspective.noticeByParcel[encodeHashToBase64(notice.summary.parcel_reference.parcel_eh)] = eh;
            break;
            case "NoticeAck": {
                const noticeAck = decode(pulse.bytes) as NoticeAck;
                const distribAh = encodeHashToBase64(noticeAck.distribution_ah);
                const recipient = encodeHashToBase64(noticeAck.recipient);
                if (!this._perspective.noticeAcks[distribAh]) {
                    this._perspective.noticeAcks[distribAh] = {};
                }
                this._perspective.noticeAcks[distribAh][recipient] = [noticeAck, pulse.ts];
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh);
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
            }
            break;
            case "NoticeReply": {
                const reply = decode(pulse.bytes) as NoticeReply;
                const noticeEh = encodeHashToBase64((reply.notice_eh));
                this._perspective.replies[noticeEh] = reply;
                this._perspective.notices[noticeEh][2] = NoticeState.Refused;
                if (reply.has_accepted) {
                    this._perspective.notices[noticeEh][2] = NoticeState.Accepted;
                }
            }
            break;
            case "ReplyAck": {
                const replyAck = decode(pulse.bytes) as ReplyAck;
                const distribAh = encodeHashToBase64(replyAck.distribution_ah);
                const recipient = encodeHashToBase64(replyAck.recipient);
                if (!this._perspective.replyAcks[distribAh]) {
                    this._perspective.replyAcks[distribAh] = {};
                }
                this._perspective.replyAcks[distribAh][recipient] = [replyAck, pulse.ts];
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh);
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
            }
            break;
            case "ReceptionProof": {
                const receptionProof = decode(pulse.bytes) as ReceptionProof;
                const noticeEh = encodeHashToBase64(receptionProof.notice_eh);
                this._perspective.receptions[noticeEh] = [receptionProof, pulse.ts];
                this._perspective.notices[noticeEh][2] = NoticeState.Received;
            }
            break;
            case "NewReceptionAck": {
                const receptionAck = decode(pulse.bytes) as ReceptionAck;
                const distribAh = encodeHashToBase64(receptionAck.distribution_ah);
                const recipient = encodeHashToBase64(receptionAck.recipient);
                if (!this._perspective.receptionAcks[distribAh]) {
                    this._perspective.receptionAcks[distribAh] = {};
                }
                this._perspective.receptionAcks[distribAh][recipient] = [receptionAck, pulse.ts];
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh)
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
            }
            break;
            case "PublicParcel": {
                const pr = decode(pulse.bytes) as ParcelReference;
                const parcelEh = encodeHashToBase64(pr.parcel_eh);
                this._perspective.parcelReferences[eh] = parcelEh;
                if (state == StateChangeType.Delete) {
                    const created = this._perspective.publicParcels[parcelEh];
                    if (!created) {
                        console.warn("Unknown deleted PublicParcel", parcelEh);
                        this._perspective.publicParcels[parcelEh] = {
                            prEh: eh,
                            parcelEh,
                            description: pr.description,
                        };
                    }
                    this._perspective.publicParcels[parcelEh].deleteInfo = [pulse.ts, author];
                } else {
                    this._perspective.publicParcels[parcelEh] = {
                        prEh: eh,
                        parcelEh,
                        description: pr.description,
                        creationTs: pulse.ts,
                        author,
                    };
                }
                if (isNew && from != this.cell.agentPubKey) {
                    tip = {Entry: pulse}
                }
            }
            break;
        }
        /** */
        if (tip) {
            await this.broadcastTip(tip);
        }
    }


    /** */
    handleTip(tip: TipProtocol, from: AgentPubKeyB64): ZomeSignalProtocol | undefined {
        const type = Object.keys(tip)[0];
        console.log("handleTip()", type, from, tip);
        /* Handle tip according to its type */
        switch (type) {
            case "Ping":
            case "Pong":
                break;
            case "Entry": return {Entry: (tip as TipProtocolVariantEntry).Entry} as ZomeSignalProtocolVariantEntry; break;
            case "Link": return {Link: (tip as TipProtocolVariantLink).Link} as ZomeSignalProtocolVariantLink; break;
            case "App":
                break;
        }
    }


    /** */
    async broadcastTip(tip: TipProtocol, agents?: Array<AgentPubKeyB64>): Promise<void> {
        agents = agents? agents : this._dvmParent.livePeers;
        /** Skip if no recipients or sending to self only */
        const filtered = agents.filter((key) => key != this.cell.agentPubKey);
        const tipType = Object.keys(tip)[0];
        console.log(`ThreadsZvm.broadcastTip() Sending Tip "${tipType}" to`, filtered, this.cell.agentPubKey);
        //if (!agents || agents.length == 1 && agents[0] === this._cellProxy.cell.agentPubKey) {
        if (!filtered || filtered.length == 0) {
            console.log("ThreadsZvm.broadcastTip() aborted: No recipients")
            return;
        }
        /** Broadcast */
        const peers = agents.map((key) => decodeHashFromBase64(key));
        await this.zomeProxy.castTip({tip, peers});
        /** Log */
        this._castLogs.push({ts: Date.now(), tip, peers: agents});
    }


    /** */
    dumpCastLogs() {
        console.warn(`Tips sent from zome "${this.zomeName}"`);
        let appSignals: any[] = [];
        this._castLogs.map((log) => {
            const type = Object.keys(log.tip)[0];
            const payload = (log.tip as any)[type];
            appSignals.push({timestamp: prettyDate(new Date(log.ts)), type, payload, count: log.peers.length, first: log.peers[0]});
        });
        console.table(appSignals);
    }


    /** */
    dumpSignalLogs(signalLogs: SignalLog[]) {
        this.dumpCastLogs();
        console.warn(`Signals received from zome "${this.zomeName}"`);
        let appSignals: any[] = [];
        signalLogs
          .filter((log) => log.type == SignalType.LitHapp)
          .map((log) => {
              const signal = log.payload as LitHappSignal;
              const pulses = signal.pulses as ZomeSignalProtocol[];
              const timestamp = prettyDate(new Date(log.ts));
              const from = encodeHashToBase64(signal.from) == this.cell.agentPubKey? "self" : encodeHashToBase64(signal.from);
              for (const pulse of pulses) {
                  if (ZomeSignalProtocolType.Tip in pulse) {
                      const tip: TipProtocol = pulse.Tip;
                      const subType = Object.keys(tip)[0];
                      appSignals.push({timestamp, from, type: ZomeSignalProtocolType.Tip, subType, payload: tip});
                  }
                  if (ZomeSignalProtocolType.Entry in pulse) {
                      const entryPulse = pulse.Entry;
                      const entryType = getVariantByIndex(DeliveryEntryType, entryPulse.def.entry_index);
                      const threadsEntry = decode(entryPulse.bytes); //as ThreadsEntry;
                      appSignals.push({timestamp, from, type: ZomeSignalProtocolType.Entry, subType: entryType, state: prettyState(entryPulse.state), payload: threadsEntry, hash: encodeHashToBase64(entryPulse.ah)});
                  }
                  // if (ZomeSignalProtocolType.Link in pulse) {
                  //     const linkPulse = pulse.Link;
                  //     const hash = `${encodeHashToBase64((linkPulse.link as any).base)} -> ${encodeHashToBase64(linkPulse.link.target)}`;
                  //     appSignals.push({timestamp, from, type: ZomeSignalProtocolType.Link, subType: getVariantByIndex(DeliveryLinkType, linkPulse.link.link_type), state: prettyState(linkPulse.state), payload: linkPulse.link.tag, hash});
                  // }
              }
          });
        console.table(appSignals);
    }


    /** -- Store -- */

    /** */
    storeManifest(manifestEh: EntryHashB64, ts: Timestamp, manifest: ParcelManifest) {
        const isPrivate = "Private" === manifest.description.visibility;
        this._perspective.localManifestByData[manifest.data_hash] = [manifestEh, isPrivate];
        if (isPrivate) {
            this._perspective.privateManifests[manifestEh] = [manifest, ts];
            const maybeNoticeEh = this._perspective.noticeByParcel[manifestEh];
            if (maybeNoticeEh) {
                this._perspective.notices[maybeNoticeEh][2] = NoticeState.PartiallyReceived;
                this._perspective.notices[maybeNoticeEh][3] = new Set(manifest.chunks.map((eh) => encodeHashToBase64(eh)));
            }
        } else {
            this._perspective.localPublicManifests[manifestEh] = [manifest, ts];
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
        const missingChunks = await this.zomeProxy.determineMissingChunks(notice[0].summary.parcel_reference.parcel_eh);
        const notice_eh = decodeHashFromBase64(noticeEh);
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


    /** */
    async probeDht(denyNotify?: boolean): Promise<void> {
        //this._perspective.publicParcels = {};
        await this.zomeProxy.pullPublicParcelsDetails();
        const inbox = await this.zomeProxy.pullInbox();
        this._perspective.inbox = inbox.map((ah) => encodeHashToBase64(ah));
        this._perspective.probeDhtCount += 1;
        if (denyNotify == undefined) this.notifySubscribers();
    }


    /** */
    async fetchManifest(manifestEh: EntryHashB64, preventNotify?: boolean): Promise<[ParcelManifest, Timestamp]> {
        const [manifest, ts, author] = await this.zomeProxy.fetchPublicManifest(decodeHashFromBase64(manifestEh));
        this.storeManifest(manifestEh, ts, manifest);
        return [manifest, ts];
    }


    /** Return base64 data string */
    async getParcelData(parcelEh: EntryHashB64): Promise<string> {
        // const pd = this._perspective.publicParcels[parcelEh];
        // if (!pd) {
        //     return Promise.reject("Unknown PublicParcel");
        // }
        const [manifest, _ts] = await this.fetchManifest(parcelEh);
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
    inbounds(): [Dictionary<[DeliveryNotice, Timestamp]>, Dictionary<[DeliveryNotice, Timestamp, Set<EntryHashB64>]>] {
        //console.log("inbounds() allNotices count", Object.entries(this._perspective.notices).length);
        let unreplieds: Dictionary<[DeliveryNotice, Timestamp]> = {};
        let incompletes: Dictionary<[DeliveryNotice, Timestamp, Set<EntryHashB64>]> = {};
        for (const [noticeEh, [notice, ts, state, missingChunks]] of Object.entries(this._perspective.notices)) {
            //const sender = encodeHashToBase64(notice.sender);
            //console.log("inbounds() state", state);
            if (NoticeState.Unreplied == state) {
                unreplieds[noticeEh] = [notice, ts];
            }
            if (NoticeState.Accepted == state) {
                incompletes[noticeEh] = [notice, ts, missingChunks];
            }
            if (NoticeState.PartiallyReceived == state) {
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
            if (DistributionState.Unsent == state
              || DistributionState.AllNoticesSent == state
              || DistributionState.AllNoticeReceived == state
              || DistributionState.AllRepliesReceived == state
            ) {
                //console.log("outbounds() recipients", distrib.recipients.length);
                for (const [recipient, state] of Object.entries(deliveryStates)) {
                    //console.log("outbounds() state", deliveryStates[agentB64], agentB64);
                    if (!(DeliveryState.ParcelDelivered == state)) {
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
    async fetchAllPublicManifest(): Promise<[ParcelManifestMat, Timestamp, AgentPubKeyB64][]> {
        const manifests: [ParcelManifestMat, Timestamp, AgentPubKeyB64][] = [];
        for (const [parcelEh, pprm] of Object.entries(this._perspective.publicParcels)) {
            if (pprm.deleteInfo) {
                continue;
            }
            const [manifest, _ts2] = await this.fetchManifest(parcelEh, true);
            manifests.push([materializeParcelManifest(manifest), pprm.creationTs!, pprm.author!]);
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
