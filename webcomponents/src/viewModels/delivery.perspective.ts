import {
    DeliveryNotice,
    DeliveryState,
    Distribution,
    DistributionState, FullDistributionState, NoticeAck,
    NoticeReply, NoticeState, ParcelDescription, ParcelManifest, ReceptionAck, ReceptionProof, ReplyAck,
} from "../bindings/delivery.types";
import {Dictionary} from "@ddd-qc/lit-happ";
import {ActionHashB64, AgentPubKeyB64, encodeHashToBase64, EntryHashB64, Timestamp} from "@holochain/client";

/** [DistributionState, AgentPubKey -> DeliveryState] */
//export type FullDistributionState = [DistributionState, Dictionary<DeliveryState>];


// /** */
// export function createFds(distribution: Distribution): FullDistributionState {
//     let delivery_states: Dictionary<DeliveryState> = {};
//     distribution.recipients.map((recipient) => delivery_states[encodeHashToBase64(recipient)] = {Unsent: null});
//     return {distribution_state: {Unsent: null}, delivery_states];
// }


/** */
export interface DeliveryPerspective {
    /** -- Encrytion -- */
    myPubEncKey: Uint8Array,
    /** AgentPubKey -> PubEncKey */
    encKeys: Dictionary<Uint8Array>,

    /** -- -- */
    inbox: ActionHashB64[],

    /** pp_eh -> ParcelDescription */
    publicParcels: Dictionary<ParcelDescription>,

    /** Parcels */
    /** manifest_eh -> ParcelManifest */
    privateManifests: Dictionary<ParcelManifest>,
    /** manifest_eh -> ParcelManifest */
    localPublicManifests: Dictionary<ParcelManifest>,
    /** data_hash -> manifest_eh */
    localManifestByData: Dictionary<EntryHashB64>,

    /** -- OUTBOUND -- */
    /** distrib_ah -> [Distribution, Timestamp, DistributionState, AgentPubKey -> DeliveryState] */
    distributions: Dictionary<[Distribution, Timestamp, DistributionState, Dictionary<DeliveryState>]>,
    /** distrib_ah -> NoticeAck */
    noticeAcks: Dictionary<NoticeAck>,
    /** distrib_ah -> ReplyAck */
    replyAcks: Dictionary<ReplyAck>,
    /** distrib_ah -> ReceptionAck */
    receptionAcks: Dictionary<ReceptionAck>,

    /** -- INBOUND -- */
    /** notice_eh -> Timestamp, Notice, State, Download Percentage */
    notices: Dictionary<[DeliveryNotice, Timestamp, NoticeState, number]>,
    /** notice_eh -> NoticeReply */
    replies: Dictionary<NoticeReply>,
    /** notice_eh -> ReceptionProof */
    receptions: Dictionary<ReceptionProof>,
}


export function createDeliveryPerspective(): DeliveryPerspective {
    return {
        myPubEncKey: new Uint8Array(),
        encKeys: {},
        inbox: [],
        publicParcels: {},
        privateManifests: {},
        localPublicManifests: {},
        localManifestByData: {},
        /** Inbound */
        distributions: {},
        noticeAcks: {},
        replyAcks: {},
        receptionAcks: {},
        /** Outbound */
        notices: {},
        replies: {},
        receptions: {},
    };
}