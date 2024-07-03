import {
    delay,
    Dictionary,
    ZomeSignal,
    prettyDate,
    SignalLog,
    SignalType,
    ZomeViewModel,
    ActionId,
    EntryId,
    AgentId,
    enc64,
    dec64,
    intoLinkableId,
    ZomeSignalProtocol,
    ZomeSignalProtocolType,
    ZomeSignalProtocolVariantEntry,
    ZomeSignalProtocolVariantLink,
    AgentIdMap,
    EntryIdMap,
    ActionIdMap,
} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";
import {Timestamp} from "@holochain/client";
import {
    DeliveryEntryType,
    DeliveryNotice,
    DeliveryState,
    Distribution,
    DistributionState,
    EntryPulse,
    LinkPulse, LinkTypes, NoticeAck, NoticeReply,
    NoticeState,
    ParcelChunk,
    ParcelManifest,
    ParcelReference, ReceptionAck, ReceptionProof, ReplyAck,
    StateChangeType,
    TipProtocol,
} from "../bindings/delivery.types";
import {
    createDeliveryPerspective,
    DeliveryPerspective,
    materializeParcelManifest,
    ParcelManifestMat,
} from "./delivery.perspective";
import {getVariantByIndex, prettyState} from "../utils";
import {decode} from "@msgpack/msgpack";


/**
 *
 */
export class DeliveryZvm extends ZomeViewModelWithSignals {

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


    /** */
    async handleLinkPulse(pulse: LinkPulse, from: AgentId): Promise<void> {
        const link = pulse.link;
        const linkAh = new ActionId(link.create_link_hash);
        const author = new AgentId(link.author);
        const base = intoLinkableId((link as any).base);
        const target = intoLinkableId(link.target);
        const state = Object.keys(pulse.state)[0];
        const isNew = (pulse.state as any)[state];
        /** */
        switch(getVariantByIndex(LinkTypes, link.link_type)) {
            case LinkTypes.PublicParcels: {
                if (state == StateChangeType.Delete) {
                    const parcelEh = this._perspective.parcelReferences.get(target);
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
                    current.deleteInfo = [link.timestamp, author];
                    this._perspective.publicParcels.set(parcelEh, current);
                    if (isNew && from.b64 == this.cell.agentId.b64) {
                        let tip: TipProtocol = {Link: pulse};
                        await this.broadcastTip(tip);
                    }
                }
            }
            break;
            case LinkTypes.Inbox:
            case LinkTypes.Members:
            case LinkTypes.Pendings:
            break;
        }
    }


    /** */
    async handleEntryPulse(pulse: EntryPulse, from: AgentId): Promise<void> {
        const entryType = getVariantByIndex(DeliveryEntryType, pulse.def.entry_index);
        const author = new AgentId(pulse.author);
        const ah = new ActionId(pulse.ah);
        const eh = new EntryId(pulse.eh);
        const state = Object.keys(pulse.state)[0];
        const isNew = (pulse.state as any)[state];
        let tip: TipProtocol | undefined = undefined;
        switch(entryType) {
            case "PrivateManifest":
            case "PublicManifest":
                const manifest = decode(pulse.bytes) as ParcelManifest;
                if (state != StateChangeType.Delete) {
                    this.storeManifest(eh, pulse.ts, manifest);
                }
            break;
            case "PrivateChunk":
            case "PublicChunk":
                const chunk = decode(pulse.bytes) as ParcelChunk;
                /** Update notice state if Chunk is not from us */
                const manifestPair = this._perspective.localManifestByData[chunk.data_hash];
                if (manifestPair) {
                    const manifestEh = manifestPair[0];
                    const noticeEh = this._perspective.noticeByParcel.get(manifestEh);
                    if (noticeEh) {
                        this._perspective.notices.get(noticeEh)[3].delete(eh);
                        if (this._perspective.notices.get(noticeEh)[3].size == 0) {
                            this.zomeProxy.completeManifest(manifestEh.hash);
                        } else {
                            // Ask for next chunk?
                        }
                    }
                }
            break;
            case "Distribution": {
                const distribution = decode(pulse.bytes) as Distribution;
                this._perspective.distributions.set(ah, [distribution, pulse.ts, DistributionState.Unsent, {}]);
                const [fullState, deliveryStates] = await this.getDistributionState(ah);
                this._perspective.distributions.set(ah, [distribution, pulse.ts, fullState, deliveryStates]);
            }
            break;
            case "DeliveryNotice":
                const notice = decode(pulse.bytes) as DeliveryNotice;
                const parcelId = new EntryId(notice.summary.parcel_reference.parcel_eh);
                this._perspective.notices.set(eh, [notice, pulse.ts, NoticeState.Unreplied, new Set()]);
                this._perspective.noticeByParcel.set(parcelId, eh);
                const [noticeState, pct] = await this.getNoticeState(eh);
                this._perspective.notices.set(eh, [notice, pulse.ts, noticeState, pct]);
                this._perspective.noticeByParcel.set(parcelId, eh);
            break;
            case "NoticeAck": {
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
            case "NoticeReply": {
                const reply = decode(pulse.bytes) as NoticeReply;
                const noticeEh = new EntryId((reply.notice_eh));
                this._perspective.replies.set(noticeEh, reply);
                this._perspective.notices.get(noticeEh)[2] = NoticeState.Refused;
                if (reply.has_accepted) {
                    this._perspective.notices.get(noticeEh)[2] = NoticeState.Accepted;
                }
            }
            break;
            case "ReplyAck": {
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
            case "ReceptionProof": {
                const receptionProof = decode(pulse.bytes) as ReceptionProof;
                const noticeEh = new EntryId(receptionProof.notice_eh);
                this._perspective.receptions.set(noticeEh, [receptionProof, pulse.ts]);
                this._perspective.notices.get(noticeEh)[2] = NoticeState.Received;
            }
            break;
            case "ReceptionAck": {
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
            case "PublicParcel": {
                const pr = decode(pulse.bytes) as ParcelReference;
                const parcelEh = new EntryId(pr.parcel_eh);
                this._perspective.parcelReferences.set(eh, parcelEh);
                if (state != StateChangeType.Delete) {
                    this._perspective.publicParcels.set(parcelEh, {
                        prEh: eh,
                        parcelEh,
                        description: pr.description,
                        creationTs: pulse.ts,
                        author,
                    };
                }
                // else {
                //     delete this._perspective.publicParcels[parcelEh];
                // }
                if (isNew && from.b64 == this.cell.agentId.b64) {
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
    dumpSignalLogs(signalLogs: SignalLog[]) {
        this.dumpCastLogs();
        console.warn(`Signals received from zome "${this.zomeName}"`);
        let appSignals: any[] = [];
        signalLogs
          .filter((log) => log.type == SignalType.Zome)
          .map((log) => {
              const signal = log.zomeSignal as ZomeSignal;
              const pulses = signal.pulses as ZomeSignalProtocol[];
              const timestamp = prettyDate(new Date(log.ts));
              const from = enc64(signal.from) == this.cell.agent.b64? "self" : new AgentId(signal.from);
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
    storeManifest(manifestEh: EntryId, ts: Timestamp, manifest: ParcelManifest) {
        const isPrivate = "Private" === manifest.description.visibility;
        this._perspective.localManifestByData[manifest.data_hash] = [manifestEh, isPrivate];
        if (isPrivate) {
            this._perspective.privateManifests.set(manifestEh, [manifest, ts]);
            const maybeNoticeEh = this._perspective.noticeByParcel.get(manifestEh);
            if (maybeNoticeEh) {
                this._perspective.notices.get(maybeNoticeEh)[2] = NoticeState.PartiallyReceived;
                this._perspective.notices.get(maybeNoticeEh)[3] = new Set(manifest.chunks.map((eh) => new EntryId(eh)));
            }
        } else {
            this._perspective.localPublicManifests.set(manifestEh, [manifest, ts]);
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
        this._perspective.orphanPublicChunks = publicOrphans.map((eh) => new EntryId(eh));
        this._perspective.orphanPrivateChunks = privateOrphans.map((eh) => new EntryId(eh));
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


    /** */
    async probeDht(denyNotify?: boolean): Promise<void> {
        //this._perspective.publicParcels = {};
        await this.zomeProxy.pullPublicParcelsDetails();
        const inbox = await this.zomeProxy.processInbox();
        this._perspective.inbox = inbox.map((ah) => new ActionId(ah));
        this._perspective.probeDhtCount += 1;
        if (denyNotify == undefined) this.notifySubscribers();
    }


    /** */
    async fetchPublicManifest(manifestEh: EntryId): Promise<[ParcelManifest, Timestamp, AgentId]> {
        const [manifest, ts, author] = await this.zomeProxy.fetchPublicManifest(manifestEh.hash);
        return [manifest, ts, new AgentId(author)];
    }


    /** Return base64 data string */
    async fetchParcelData(parcelEh: EntryId): Promise<string> {
        // const pd = this._perspective.publicParcels[parcelEh];
        // if (!pd) {
        //     return Promise.reject("Unknown PublicParcel");
        // }
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
    inbounds(): [EntryIdMap<[DeliveryNotice, Timestamp]>, EntryIdMap<[DeliveryNotice, Timestamp, Set<EntryId>]>] {
        //console.log("inbounds() allNotices count", Object.entries(this._perspective.notices).length);
        let unreplieds: EntryIdMap<[DeliveryNotice, Timestamp]> = new EntryIdMap();
        let incompletes: EntryIdMap<[DeliveryNotice, Timestamp, Set<EntryId>]> = new EntryIdMap();
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
    outbounds(): ActionIdMap<[Distribution, Timestamp, Dictionary<DeliveryState>]> {
        //console.log("outbounds() allDistributions count", Object.entries(this._perspective.distributions).length);
        let res: ActionIdMap<[Distribution, Timestamp, Dictionary<DeliveryState>]> = new ActionIdMap();
        for (const [distribAh, [distrib, ts, state, deliveryStates]] of this._perspective.distributions.entries()) {
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
                        if (!res.get(distribAh)) {
                            res.set(distribAh, [distrib, ts, {}]);
                        }
                        res.get(distribAh)[2][recipient] = state;
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
        const replyEh = await this.zomeProxy.respondToNotice({notice_eh: noticeEh.hash, has_accepted: true});
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
    async getNoticeState(noticeEh: EntryId): Promise<[NoticeState, Set<EntryId>]> {
        const [state, missing_chunks] = await this.zomeProxy.getNoticeState(noticeEh.hash);
        const missingChunks = missing_chunks.map((chunk_eh) => new EntryId(chunk_eh));
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
    exportPerspective(/*originalsZvm: AuthorshipZvm*/): string {
        const manifests: [ParcelManifestMat, Timestamp][] = Array.from(this._perspective.localPublicManifests.values()).map(([manifest, ts]) => [materializeParcelManifest(manifest), ts])
        return JSON.stringify(manifests, null, 2);
    }

}
