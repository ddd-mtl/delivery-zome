import {delay, Dictionary, LitHappSignal, prettyDate, SignalLog, SignalType, ZomeViewModel} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";
import {
    ActionHashB64,
    AgentPubKeyB64, AppSignalCb,
    decodeHashFromBase64,
    encodeHashToBase64, EntryHashB64, Timestamp
} from "@holochain/client";
import {
    DeliveryEntryKind, DeliveryEntryKindType,
    DeliveryEntryType,
    DeliveryGossipProtocolType,
    DeliveryNotice, DeliverySignal, DeliverySignalProtocol, DeliverySignalProtocolType,
    DeliveryState,
    Distribution,
    DistributionState, EntryInfo, EntryStateChange,
    NoticeState,
    ParcelManifest, ParcelReference,
} from "../bindings/delivery.types";
import {AppSignal} from "@holochain/client/lib/api/app/types";
import {
    createDeliveryPerspective,
    DeliveryPerspective,
    materializeParcelManifest,
    ParcelManifestMat,
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
    mySignalHandler(appSignal: AppSignal): void {
        const sig = appSignal.payload as DeliverySignal;
        console.log("DELIVERY received signal", sig);
        if (!("signal" in sig)) {
            return;
        }
        for (const signal of sig.signal) {
            /*await*/ this.handleDeliverySignal(signal, encodeHashToBase64(sig.from));
        }
        this.notifySubscribers();
    }


    /** */
    async handleDeliverySignal(deliverySignal: DeliverySignalProtocol, from: AgentPubKeyB64): Promise<void> {
        if (DeliverySignalProtocolType.Entry in deliverySignal) {
            const [entryInfo, entryKind] = deliverySignal.Entry;
            const hash = encodeHashToBase64(entryInfo.hash);
            const author = encodeHashToBase64(entryInfo.author);
            if (DeliveryEntryKindType.ParcelManifest in entryKind) {
                console.log("signal ParcelManifest", entryKind.ParcelManifest);
                const manifest = entryKind.ParcelManifest;
                if (entryInfo.state != EntryStateChange.Deleted) {
                    this.storeManifest(hash, entryInfo.ts, manifest);
                }
            }
            if (DeliveryEntryKindType.ParcelChunk in entryKind) {
                console.log("signal NewLocalChunk", entryKind.ParcelChunk);
                const chunk = entryKind.ParcelChunk;
                /** Update notice state if Chunk is not from us */
                const manifestPair = this._perspective.localManifestByData[chunk.data_hash];
                if (manifestPair) {
                    const manifestEh = manifestPair[0];
                    const noticeEh = this._perspective.noticeByParcel[manifestEh];
                    if (noticeEh) {
                        this._perspective.notices[noticeEh][3].delete(hash);
                        if (this._perspective.notices[noticeEh][3].size == 0) {
                            this.zomeProxy.completeManifest(decodeHashFromBase64(manifestEh));
                        } else {
                            // Ask for next chunk?
                        }
                    }
                }
            }
            if (DeliveryEntryKindType.Distribution in entryKind) {
                console.log("signal Distribution", entryKind.Distribution);
                const distribAh = hash;
                const distribution = entryKind.Distribution;
                this._perspective.distributions[distribAh] = [distribution, entryInfo.ts, DistributionState.Unsent, {}];
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh);
                this._perspective.distributions[distribAh] = [distribution, entryInfo.ts, fullState, deliveryStates];
                ;
            }
            if (DeliveryEntryKindType.DeliveryNotice in entryKind) {
                console.log("signal DeliveryNotice", entryKind.DeliveryNotice);
                const notice = entryKind.DeliveryNotice;
                this._perspective.notices[hash] = [notice, entryInfo.ts, NoticeState.Unreplied, new Set()];
                this._perspective.noticeByParcel[encodeHashToBase64(notice.summary.parcel_reference.parcel_eh)] = hash;

                const [state, pct] = await this.getNoticeState(hash);
                this._perspective.notices[hash] = [notice, entryInfo.ts, state, pct];
                this._perspective.noticeByParcel[encodeHashToBase64(notice.summary.parcel_reference.parcel_eh)] = hash;

            }
            if (DeliveryEntryKindType.NoticeAck in entryKind) {
                console.log("signal NewNoticeAck", entryKind.NoticeAck);
                const noticeAck = entryKind.NoticeAck;
                const distribAh = encodeHashToBase64(noticeAck.distribution_ah);
                const recipient = encodeHashToBase64(noticeAck.recipient);
                if (!this._perspective.noticeAcks[distribAh]) {
                    this._perspective.noticeAcks[distribAh] = {};
                }
                this._perspective.noticeAcks[distribAh][recipient] = [noticeAck, entryInfo.ts];
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh);
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;

            }
            if (DeliveryEntryKindType.NoticeReply in entryKind) {
                console.log("signal NoticeReply", entryKind.NoticeReply);
                const reply = entryKind.NoticeReply;
                const noticeEh = encodeHashToBase64((reply.notice_eh));
                this._perspective.replies[noticeEh] = reply;
                this._perspective.notices[noticeEh][2] = NoticeState.Refused;
                if (reply.has_accepted) {
                    this._perspective.notices[noticeEh][2] = NoticeState.Accepted;
                }
            }
            if (DeliveryEntryKindType.ReplyAck in entryKind) {
                console.log("signal ReplyAck", entryKind.ReplyAck);
                const replyAck = entryKind.ReplyAck;
                const distribAh = encodeHashToBase64(replyAck.distribution_ah);
                const recipient = encodeHashToBase64(replyAck.recipient);
                if (!this._perspective.replyAcks[distribAh]) {
                    this._perspective.replyAcks[distribAh] = {};
                }
                this._perspective.replyAcks[distribAh][recipient] = [replyAck, entryInfo.ts];
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh);
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
            }
            if (DeliveryEntryKindType.ReceptionProof in entryKind) {
                console.log("signal ReceptionProof", entryKind.ReceptionProof);
                const receptionProof = entryKind.ReceptionProof;
                const noticeEh = encodeHashToBase64(receptionProof.notice_eh);
                this._perspective.receptions[noticeEh] = [receptionProof, entryInfo.ts];
                this._perspective.notices[noticeEh][2] = NoticeState.Received;
            }
            if (DeliveryEntryKindType.ReceptionAck in entryKind) {
                console.log("signal NewReceptionAck", entryKind.ReceptionAck);
                const receptionAck = entryKind.ReceptionAck;
                const distribAh = encodeHashToBase64(receptionAck.distribution_ah);
                const recipient = encodeHashToBase64(receptionAck.recipient);
                if (!this._perspective.receptionAcks[distribAh]) {
                    this._perspective.receptionAcks[distribAh] = {};
                }
                this._perspective.receptionAcks[distribAh][recipient] = [receptionAck, entryInfo.ts];
                const [fullState, deliveryStates] = await this.getDistributionState(distribAh)
                this._perspective.distributions[distribAh][2] = fullState;
                this._perspective.distributions[distribAh][3] = deliveryStates;
            }
            // if (DeliveryEntryKindType.PendingItem in entryKind) {
            //     console.log("signal PendingItem", entryKind.PendingItem);
            // }
            if (DeliveryEntryKindType.PublicParcel in entryKind) {
                console.log("signal PublicParcel", entryKind.PublicParcel);
                const pr = entryKind.PublicParcel
                const parcelEh = encodeHashToBase64(pr.parcel_eh);
                this._perspective.publicParcels[parcelEh] = {
                    prEh: hash,
                    parcelEh,
                    description: pr.description,
                    creationTs: entryInfo.ts,
                    author,
                };
                this._perspective.parcelReferences[hash] = parcelEh;
                if (entryInfo.state == EntryStateChange.Deleted) {
                    const created = this._perspective.publicParcels[parcelEh];
                    if (created) {
                        this._perspective.publicParcels[parcelEh].deleteInfo = [entryInfo.ts, author];
                    } else {
                        console.warn("Unknown deleted PublicParcel", parcelEh);
                    }
                }
            }
        }
        if (DeliverySignalProtocolType.Gossip in deliverySignal) {
            console.log("signal Gossip", deliverySignal.Gossip);
            const gossip = deliverySignal.Gossip;
            if (DeliveryGossipProtocolType.PublicParcelPublished in gossip) {
                console.log("Gossip signal PublicParcelPublished", gossip.PublicParcelPublished);
                const pr = gossip.PublicParcelPublished[2];
                const ts = gossip.PublicParcelPublished[1];
                const prEh = encodeHashToBase64(gossip.PublicParcelPublished[0]);
                const parcelEh = encodeHashToBase64(pr.parcel_eh);
                this._perspective.publicParcels[parcelEh] = {prEh, parcelEh: parcelEh, description: pr.description, creationTs: ts, author: from};
                this._perspective.parcelReferences[prEh] = parcelEh;
            }
            if (DeliveryGossipProtocolType.PublicParcelUnpublished in gossip) {
                console.log("Gossip signal PublicParcelUnpublished", gossip.PublicParcelUnpublished);
                const pr = gossip.PublicParcelUnpublished[2];
                const del_ts = gossip.PublicParcelUnpublished[1];
                //const pr_eh = gossip.PublicParcelUnpublished[0];
                const parcelEh = encodeHashToBase64(pr.parcel_eh);
                const created = this._perspective.publicParcels[parcelEh];
                if (created) {
                    this._perspective.publicParcels[parcelEh].deleteInfo = [del_ts, from];
                } else {
                    console.warn("Unknown unpublished PublicParcel", parcelEh);
                }
            }
        }
    }


    /** */
    dumpSignalLogs(signalLogs: SignalLog[]) {
        console.warn(`App signals from zome "${this.zomeName}"`);
        let appSignals: any[] = [];
        signalLogs
          .filter((log) => log.type == SignalType.LitHapp)
          .map((log) => {
              const signal = log.payload as LitHappSignal;
              const delSignals = signal.signal as DeliverySignalProtocol[];
              const from = encodeHashToBase64(signal.from) == this.cell.agentPubKey? "self" : encodeHashToBase64(signal.from);
              for (const signal of delSignals) {
                  if (DeliverySignalProtocolType.Gossip in signal) {
                      const subType = Object.keys(signal.Gossip)[0];
                      const pr = ((signal.Gossip as any)[subType] as any)[2] as ParcelReference;
                      appSignals.push({timestamp: prettyDate(new Date(log.ts)), from, type: DeliverySignalProtocolType.Gossip, subType, hash: encodeHashToBase64(pr.parcel_eh)});
                  }
                  if (DeliverySignalProtocolType.Entry in signal) {
                      const [entryInfo, deliveryEntryKind] = signal.Entry;
                      const entryType = Object.keys(deliveryEntryKind)[0];
                      appSignals.push({timestamp: prettyDate(new Date(log.ts)), from, type: DeliverySignalProtocolType.Entry, subType: entryType, payload: entryInfo.state, hash: encodeHashToBase64(entryInfo.hash)});
                  }
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
