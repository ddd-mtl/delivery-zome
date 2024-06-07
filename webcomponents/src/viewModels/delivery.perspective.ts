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
import {Dictionary} from "@ddd-qc/lit-happ";
import {ActionHashB64, AgentPubKeyB64, encodeHashToBase64, decodeHashFromBase64, EntryHashB64, Timestamp} from "@holochain/client";


/** */
export interface PublicParcelRecordMat {
    prEh: EntryHashB64,
    parcelEh: EntryHashB64,
    description: ParcelDescription,
    creationTs?: Timestamp,
    author?: AgentPubKeyB64,
    deleteInfo?: [Timestamp, AgentPubKeyB64],
}


export type DeliveryPerspective = DeliveryPerspectiveCore & DeliveryPerspectiveLive;

/** */
export interface DeliveryPerspectiveLive {
    probeDhtCount: number,
    /** -- PROBLEMS -- */
    orphanPublicChunks: EntryHashB64[],
    orphanPrivateChunks: EntryHashB64[],
    //incompleteManifests: EntryHashB64[],
}


/** */
export interface DeliveryPerspectiveCore {
    /** -- -- */
    inbox: ActionHashB64[],

    /** pp_eh -> (pr_eh, ParcelDescription, ...)  */
    publicParcels: Dictionary<PublicParcelRecordMat>,
    /** pr_eh -> pp_eh */
    parcelReferences: Dictionary<EntryHashB64>

    /** Parcels */
    /** manifest_eh -> (ParcelManifest, timestamp) */
    privateManifests: Dictionary<[ParcelManifest, Timestamp]>,
    /** manifest_eh -> ParcelManifest */
    localPublicManifests: Dictionary<[ParcelManifest, Timestamp]>,
    /** data_hash -> [manifest_eh, isPrivate] */
    localManifestByData: Dictionary<[EntryHashB64, boolean]>,
    // /** data_hash -> number of chunks on chain */
    // chunkCounts: Dictionary<number>,

    /** -- OUTBOUND -- */
    /** distrib_ah -> [Distribution, Timestamp, DistributionState, AgentPubKey -> DeliveryState] */
    distributions: Dictionary<[Distribution, Timestamp, DistributionState, Dictionary<DeliveryState>]>,
    /** distrib_ah -> (recipientKey -> NoticeAck) */
    noticeAcks: Dictionary<Dictionary<[NoticeAck, Timestamp]>>,
    /** distrib_ah -> (recipientKey -> ReplyAck) */
    replyAcks: Dictionary<Dictionary<[ReplyAck, Timestamp]>>,
    /** distrib_ah -> (recipientKey -> ReceptionAck) */
    receptionAcks: Dictionary<Dictionary<[ReceptionAck, Timestamp]>>,

    /** -- INBOUND -- */
    /** notice_eh -> Timestamp, Notice, State, Missing chunks */
    notices: Dictionary<[DeliveryNotice, Timestamp, NoticeState, Set<EntryHashB64>]>,
    /** parcel_eh -> notice_eh */
    noticeByParcel: Dictionary<EntryHashB64>,
    /** notice_eh -> NoticeReply */
    replies: Dictionary<NoticeReply>,
    /** notice_eh -> ReceptionProof */
    receptions: Dictionary<[ReceptionProof, Timestamp]>,
}


/** */
export function createDeliveryPerspective(): DeliveryPerspective {
    return {
        inbox: [],
        publicParcels: {},
        parcelReferences: {},
        privateManifests: {},
        localPublicManifests: {},
        localManifestByData: {},
        //chunkCounts: {},
        /* Problems */
        orphanPublicChunks: [],
        orphanPrivateChunks: [],
        //incompleteManifests: [],
        /** Inbound */
        distributions: {},
        noticeAcks: {},
        replyAcks: {},
        receptionAcks: {},
        /** Outbound */
        notices: {},
        noticeByParcel: {},
        replies: {},
        receptions: {},
        /** meta */
        probeDhtCount: 0,
    };
}


export interface ParcelManifestMat {
    description: ParcelDescription
    data_hash: string
    chunks: EntryHashB64[]
}


export function materializeParcelManifest(pm: ParcelManifest): ParcelManifestMat {
    const chunks = pm.chunks.map((eh) => encodeHashToBase64(eh));
    return {
        description: pm.description,
        data_hash: pm.data_hash,
        chunks,
    }
}


export function dematerializeParcelManifest(pm: ParcelManifestMat): ParcelManifest {
    const chunks = pm.chunks.map((eh) => decodeHashFromBase64(eh));
    return {
        description: pm.description,
        data_hash: pm.data_hash,
        chunks,
    }
}



export function materializePublicParcelRecord(ppr: PublicParcelRecord): PublicParcelRecordMat {
    return {
        prEh: encodeHashToBase64(ppr.pr_eh),
        parcelEh: encodeHashToBase64(ppr.pp_eh),
        description: ppr.description,
        creationTs: ppr.creation_ts,
        author: encodeHashToBase64(ppr.author),
        deleteInfo: ppr.deleteInfo? [ppr.deleteInfo[0], encodeHashToBase64(ppr.deleteInfo[1])] : undefined,
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
