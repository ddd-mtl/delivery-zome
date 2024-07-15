import {
    DeliveryNotice,
    DeliveryState,
    Distribution,
    DistributionState,
    NoticeAck,
    NoticeReply,
    NoticeState,
    ParcelDescription,
    ParcelManifest,
    PublicParcelRecord,
    ReceptionAck,
    ReceptionProof,
    ReplyAck,
} from "../bindings/delivery.types";
import {Dictionary, ActionId, EntryId, AgentId, EntryIdMap, ActionIdMap, AgentIdMap, enc64} from "@ddd-qc/lit-happ";
import {EntryHashB64, Timestamp} from "@holochain/client";


/** */
export interface PublicParcelRecordMat {
    prEh: EntryId,
    parcelEh: EntryId,
    description: ParcelDescription,
    creationTs?: Timestamp,
    author?: AgentId,
    deleteInfo?: [Timestamp, AgentId],
}

/** */
export interface DeliveryPerspectiveSnapshot {
    manifests: [ParcelManifestMat, Timestamp][],
    // FIXME
}

// /** */
// export interface DeliveryPerspectiveLive {
//     probeDhtCount: number,
//     /** -- PROBLEMS -- */
//     orphanPublicChunks: EntryId[],
//     orphanPrivateChunks: EntryId[],
//     //incompleteManifests: EntryId[],
// }


/** */
export interface DeliveryPerspectiveCore {
    /** -- -- */
    inbox: ActionId[],

    /** parcel_eh -> (pp_eh, ParcelDescription, ...)  */
    publicParcels: EntryIdMap<PublicParcelRecordMat>,
    /** pp_eh -> parcel_eh */
    parcelReferences: EntryIdMap<EntryId>

    /** Parcels */
    /** manifest_eh -> (ParcelManifest, timestamp) */
    privateManifests: EntryIdMap<[ParcelManifest, Timestamp]>,
    /** manifest_eh -> ParcelManifest */
    localPublicManifests: EntryIdMap<[ParcelManifest, Timestamp]>,
    /** data_hash -> [manifest_eh, isPrivate] */
    localManifestByData: Dictionary<[EntryId, boolean]>,
    // /** data_hash -> number of chunks on chain */
    // chunkCounts: Dictionary<number>,

    /** -- OUTBOUND -- */
    /** distrib_ah -> [Distribution, Timestamp, DistributionState, AgentPubKey -> DeliveryState] */
    distributions: ActionIdMap<[Distribution, Timestamp, DistributionState, AgentIdMap<DeliveryState>]>,
    /** distrib_ah -> (recipientKey -> NoticeAck) */
    noticeAcks: ActionIdMap<AgentIdMap<[NoticeAck, Timestamp]>>,
    /** distrib_ah -> (recipientKey -> ReplyAck) */
    replyAcks: ActionIdMap<AgentIdMap<[ReplyAck, Timestamp]>>,
    /** distrib_ah -> (recipientKey -> ReceptionAck) */
    receptionAcks: ActionIdMap<AgentIdMap<[ReceptionAck, Timestamp]>>,

    /** -- INBOUND -- */
    /** notice_eh -> Timestamp, Notice, State, Missing chunks */
    notices: EntryIdMap<[DeliveryNotice, Timestamp, NoticeState, Set<EntryHashB64>]>,
    /** parcel_eh -> notice_eh */
    noticeByParcel: EntryIdMap<EntryId>,
    /** notice_eh -> NoticeReply */
    replies: EntryIdMap<NoticeReply>,
    /** notice_eh -> ReceptionProof */
    receptions: EntryIdMap<[ReceptionProof, Timestamp]>,
}


/** */
export class DeliveryPerspective implements DeliveryPerspectiveCore {
    inbox: ActionId[] = [];
    publicParcels = new EntryIdMap<PublicParcelRecordMat>();
    parcelReferences = new EntryIdMap<EntryId>();
    privateManifests = new EntryIdMap<[ParcelManifest, Timestamp]>();
    localPublicManifests = new EntryIdMap<[ParcelManifest, Timestamp]>();
    localManifestByData: Dictionary<[EntryId, boolean]> = {};
    //chunkCounts: {},

    //incompleteManifests: [],
    /** Outbound */
    distributions = new ActionIdMap<[Distribution, Timestamp, DistributionState, AgentIdMap<DeliveryState>]>();
    noticeAcks = new ActionIdMap<AgentIdMap<[NoticeAck, Timestamp]>>();
    replyAcks = new ActionIdMap<AgentIdMap<[ReplyAck, Timestamp]>>();
    receptionAcks = new ActionIdMap<AgentIdMap<[ReceptionAck, Timestamp]>>();
    /** Inbound */
    notices = new EntryIdMap<[DeliveryNotice, Timestamp, NoticeState, Set<EntryHashB64>]>();
    noticeByParcel = new EntryIdMap<EntryId>();
    replies = new EntryIdMap<NoticeReply>();
    receptions = new EntryIdMap<[ReceptionProof, Timestamp]>();

    /* Live */
    private _orphanPublicChunks: EntryId[] = [];
    private _orphanPrivateChunks: EntryId[] = [];
    //private _probeDhtCount = 0;


    /** -- Methods -- */

    /** */
    storeManifest(manifestEh: EntryId, ts: Timestamp, manifest: ParcelManifest) {
        const isPrivate = "Private" === manifest.description.visibility;
        this.localManifestByData[manifest.data_hash] = [manifestEh, isPrivate];
        if (isPrivate) {
            this.privateManifests.set(manifestEh, [manifest, ts]);
            const maybeNoticeEh = this.noticeByParcel.get(manifestEh);
            if (maybeNoticeEh) {
                this.notices.get(maybeNoticeEh)[2] = NoticeState.PartiallyReceived;
                this.notices.get(maybeNoticeEh)[3] = new Set(manifest.chunks.map((eh) => enc64(eh)));
            }
        } else {
            this.localPublicManifests.set(manifestEh, [manifest, ts]);
        }
    }

    /** */
    storeOrphans(orphanPublicChunks: EntryId[], orphanPrivateChunks: EntryId[]) {
        this._orphanPublicChunks = orphanPublicChunks;
        this._orphanPrivateChunks = orphanPrivateChunks;
    }


    /** -- Memento -- */

    /** TODO: deep copy */
    makeSnapshot(): DeliveryPerspectiveSnapshot {
        // FIXME
        const manifests: [ParcelManifestMat, Timestamp][] = Array.from(this.localPublicManifests.values())
          .map(([manifest, ts]) => [materializeParcelManifest(manifest), ts]);
        /** */
        return {
            manifests,
        }
    }


    /** */
    restore(snapshot: DeliveryPerspectiveSnapshot) {
        /** Clear */
        this.inbox = [];
        this.publicParcels.clear();
        this.parcelReferences.clear();
        this.privateManifests.clear();
        this.localPublicManifests.clear();
        this.localManifestByData = {};
        this.distributions.clear();
        this.noticeAcks.clear();
        this.replyAcks.clear();
        this.receptionAcks.clear();
        this.notices.clear();
        this.noticeByParcel.clear();
        this.replies.clear();
        this.receptions.clear();
        /* */
        this._orphanPublicChunks = [];
        this._orphanPrivateChunks = [];
        /** Load */
        // FIXME
    }

}


export interface ParcelManifestMat {
    description: ParcelDescription,
    data_hash: string,
    chunks: EntryId[],
}


export function materializeParcelManifest(pm: ParcelManifest): ParcelManifestMat {
    const chunks = pm.chunks.map((eh) => new EntryId(eh));
    return {
        description: pm.description,
        data_hash: pm.data_hash,
        chunks,
    }
}


export function dematerializeParcelManifest(pm: ParcelManifestMat): ParcelManifest {
    const chunks = pm.chunks.map((id) => (id.hash));
    return {
        description: pm.description,
        data_hash: pm.data_hash,
        chunks,
    }
}



export function materializePublicParcelRecord(ppr: PublicParcelRecord): PublicParcelRecordMat {
    return {
        prEh: new EntryId(ppr.pr_eh),
        parcelEh: new EntryId(ppr.pp_eh),
        description: ppr.description,
        creationTs: ppr.creation_ts,
        author: new AgentId(ppr.author),
        deleteInfo: ppr.deleteInfo? [ppr.deleteInfo[0], new AgentId(ppr.deleteInfo[1])] : undefined,
    }
}


// export function dematerializePublicParcelRecord(ppr: PublicParcelRecordMat): PublicParcelRecord {
//     return {
//         pr_eh: decodeHashFromBase64(ppr.prEh),
//         pp_eh: decodeHashFromBase64(ppr.parcelEh),
//         description: ppr.description,
//         creation_ts: ppr.creationTs,
//         author: decodeHashFromBase64(ppr.author),
//         deleteInfo: ppr.deleteInfo? [ppr.deleteInfo[0], decodeHashFromBase64(ppr.deleteInfo[1])] : undefined,
//     }
// }
