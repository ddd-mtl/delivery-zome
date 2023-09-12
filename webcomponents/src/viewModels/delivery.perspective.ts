import {
    DeliveryNotice,
    DeliveryReceipt, DeliveryReply,
    DeliveryState,
    Distribution,
    DistributionState, FullDistributionState, NoticeAck,
    NoticeReceived, NoticeReply, NoticeState, ParcelManifest, ParcelReceived, ReceptionAck, ReceptionProof, ReplyAck,
    ReplyReceived
} from "../bindings/delivery.types";
import {Dictionary} from "@ddd-qc/lit-happ";
import {ActionHashB64, AgentPubKeyB64, encodeHashToBase64, EntryHashB64, Timestamp} from "@holochain/client";

/** [DistributionState, AgentPubKey -> DeliveryState] */
//export type FullDistributionState = [DistributionState, Dictionary<DeliveryState>];


/** */
export function createFds(distribution: Distribution): FullDistributionState {
    let delivery_states: Dictionary<DeliveryState> = {};
    distribution.recipients.map((recipient) => delivery_states[encodeHashToBase64(recipient)] = {Unsent: null});
    return {distribution_state: {Unsent: null}, delivery_states];
}


/** */
export interface DeliveryPerspective {
    /** -- Encrytion -- */
    myPubEncKey: Uint8Array,
    /** AgentPubKey -> PubEncKey */
    encKeys: Dictionary<Uint8Array>,

    /** -- -- */
    inbox: ActionHashB64[],

    /** Parcels */
    manifests: Dictionary<ParcelManifest>,

    /** -- OUTBOUND -- */
    /** distrib_eh -> [Distribution, Timestamp, DistributionState, AgentPubKey -> DeliveryState] */
    distributions: Dictionary<[Distribution, Timestamp, DistributionState, Dictionary<DeliveryState>]>,
    /** distrib_eh -> NoticeAck */
    noticeAcks: Dictionary<NoticeAck>,
    /** distrib_eh -> ReplyAck */
    replyAcks: Dictionary<ReplyAck>,
    /** distrib_eh -> ReceptionAck */
    receptionAcks: Dictionary<ReceptionAck>,

    /** -- INBOUND -- */
    /** notice_eh -> Timestamp, Notice, State, Download Percentage */
    notices: Dictionary<[Timestamp, DeliveryNotice, NoticeState, number]>,
    /** notice_eh -> NoticeReply */
    replies: Dictionary<NoticeReply>,
    /** notice_eh -> ReceptionProof */
    receptions: Dictionary<ReceptionProof>,

    /** -- EXTRA LOGIC -- */

    //newDeliveryNotices: Dictionary<DeliveryNotice>,

    //incomingDistributions: Dictionary<DistributionState>,

    /** AgentPubKey -> (notice_eh -> distrib_eh) */
    //unrepliedInbounds: Record<AgentPubKeyB64, Record<EntryHashB64, Timestamp>>,
    /** AgentPubKey -> (notice_eh -> distrib_eh) */
    //pendingInbounds: Record<AgentPubKeyB64, Record<EntryHashB64, Timestamp>>,
    /** distrib_eh -> [Timestamp , AgentPubKey -> DeliveryState] */
    //unrepliedOutbounds: Record<EntryHashB64, [Timestamp, Record<AgentPubKeyB64, DeliveryState>]>,

}


export function createDeliveryPerspective(): DeliveryPerspective {
    return {
        myPubEncKey: new Uint8Array(),
            encKeys: {},
        inbox: [],
            /** Inbound */
            distributions: {},
        noticeAcks: {},
        replyAcks: {},
        receptionAcks: {},
        /** Outbound */
        notices: {},
        replies: {},
        receptions: {},
        /** Extra logic */
        newDeliveryNotices: {},
        //myDistributions: {},
        unrepliedInbounds: {},
        pendingInbounds: {},
        unrepliedOutbounds: {}
    };
}