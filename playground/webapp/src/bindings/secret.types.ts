/* This file is generated by zits. Do not edit manually */

import {
WebsocketConnectionOptions,
/** types.ts */
HoloHash,
AgentPubKey,
DnaHash,
WasmHash,
EntryHash,
ActionHash,
AnyDhtHash,
ExternalHash,
KitsuneAgent,
KitsuneSpace,
HoloHashB64,
AgentPubKeyB64,
DnaHashB64,
WasmHashB64,
EntryHashB64,
ActionHashB64,
AnyDhtHashB64,
InstalledAppId,
Signature,
CellId,
DnaProperties,
RoleName,
InstalledCell,
Timestamp,
Duration,
HoloHashed,
NetworkInfo,
FetchPoolInfo,
/** hdk/action.ts */
SignedActionHashed,
RegisterAgentActivity,
ActionHashed,
ActionType,
Action,
NewEntryAction,
Dna,
AgentValidationPkg,
InitZomesComplete,
CreateLink,
DeleteLink,
OpenChain,
CloseChain,
Update,
Delete,
Create,
/** hdk/capabilities.ts */
CapSecret,
CapClaim,
GrantedFunctionsType,
GrantedFunctions,
ZomeCallCapGrant,
CapAccessType,
CapAccess,
CapGrant,
///** hdk/countersigning.ts */
//CounterSigningSessionData,
//PreflightRequest,
//CounterSigningSessionTimes,
//ActionBase,
//CounterSigningAgents,
//PreflightBytes,
//Role,
//CountersigningAgentState,
/** hdk/dht-ops.ts */
DhtOpType,
DhtOp,
getDhtOpType,
getDhtOpAction,
getDhtOpEntry,
getDhtOpSignature,
/** hdk/entry.ts */
EntryVisibility,
AppEntryDef,
EntryType,
EntryContent,
Entry,
/** hdk/record.ts */
Record as HcRecord,
RecordEntry as HcRecordEntry,
/** hdk/link.ts */
AnyLinkableHash,
ZomeIndex,
LinkType,
LinkTag,
RateWeight,
RateBucketId,
RateUnits,
Link,
/** api/admin/types.ts */
InstalledAppInfoStatus,
DeactivationReason,
DisabledAppReason,
StemCell,
ProvisionedCell,
ClonedCell,
CellType,
CellInfo,
AppInfo,
MembraneProof,
FunctionName,
ZomeName,
ZomeDefinition,
IntegrityZome,
CoordinatorZome,
DnaDefinition,
ResourceBytes,
ResourceMap,
CellProvisioningStrategy,
CellProvisioning,
DnaVersionSpec,
DnaVersionFlexible,
AppRoleDnaManifest,
AppRoleManifest,
AppManifest,
AppBundle,
AppBundleSource,
NetworkSeed,
ZomeLocation,
   } from '@holochain/client';

import {
/** Common */
DhtOpHashB64,
//DnaHashB64, (duplicate)
//AnyDhtHashB64, (duplicate)
DhtOpHash,
/** DnaFile */
DnaFile,
DnaDef,
Zomes,
WasmCode,
/** entry-details */
EntryDetails,
RecordDetails,
Details,
DetailsType,
EntryDhtStatus,
/** Validation */
ValidationStatus,
ValidationReceipt,
   } from '@holochain-open-dev/core-types';

/** User defined external dependencies */
export type EntryDefIndex = number;

export const REMOTE_ENDPOINT = "receive_delivery_dm";

export const DIRECT_SEND_TIMEOUT_MS = 1000;

export const DIRECT_SEND_CHUNK_TIMEOUT_MS = 10000;

/** Listing all Holochain Path used in this DNA */
export const DIRECTORY_PATH = "directory";

/** WARNING Must use these names in the dna definition file */
export const DELIVERY_ZOME_NAME = "zDelivery";

export const DELIVERY_INTERGRITY_ZOME_NAME = "zDeliveryIntegrity";

/** State of a single delivery of an item to a unique recipient */
export enum DeliveryState {
	Unsent = 'Unsent',
	PendingNotice = 'PendingNotice',
	NoticeDelivered = 'NoticeDelivered',
	ParcelRefused = 'ParcelRefused',
	ParcelAccepted = 'ParcelAccepted',
	PendingParcel = 'PendingParcel',
	ParcelDelivered = 'ParcelDelivered',
}

/** Possible states of an OutMail entry */
export enum DistributionState {
	Unsent = 'Unsent',
	AllNoticesSent = 'AllNoticesSent',
	AllNoticeReceived = 'AllNoticeReceived',
	AllRepliesReceived = 'AllRepliesReceived',
	AllAcceptedParcelsReceived = 'AllAcceptedParcelsReceived',
	Deleted = 'Deleted',
}

/** Possible states of a DeliveryNotice entry */
export enum NoticeState {
	Unreplied = 'Unreplied',
	Accepted = 'Accepted',
	Refused = 'Refused',
	PartiallyReceived = 'PartiallyReceived',
	Received = 'Received',
	Deleted = 'Deleted',
}

/** Shared data between a Distribution and a DeliveryNotice */
export interface DeliverySummary {
  distribution_strategy: DistributionStrategy
  parcel_reference: ParcelReference
}

/**  */
export interface ParcelReference {
  eh: EntryHash
  description: ParcelDescription
}

/**  */
export interface ParcelDescription {
  name: string
  size: number
  zome_origin: ZomeName
  visibility: EntryVisibility
  kind_info: ParcelKind
}

/** A Parcel is a generic Entry or a ParcelManifest */
export enum ParcelKindType {
	AppEntry = 'AppEntry',
	Manifest = 'Manifest',
}
export type ParcelKindVariantAppEntry = {AppEntry: EntryDefIndex}
export type ParcelKindVariantManifest = {Manifest: string}
export type ParcelKind = 
 | ParcelKindVariantAppEntry | ParcelKindVariantManifest;

/**  */
export enum DistributionStrategy {
	Normal = 'Normal',
	DmOnly = 'DmOnly',
	DhtOnly = 'DhtOnly',
}

/** Entry representing a request to send a Parcel to one or multiple recipients */
export interface Distribution {
  recipients: AgentPubKey[]
  delivery_summary: DeliverySummary
  summary_signature: Signature
}

/** Entry representing a received delivery request */
export interface DeliveryNotice {
  distribution_ah: ActionHash
  summary: DeliverySummary
  sender: AgentPubKey
  sender_summary_signature: Signature
}

/** Entry for confirming a request has been well received by a recipient */
export interface NoticeAck {
  distribution_ah: ActionHash
  recipient: AgentPubKey
  recipient_summary_signature: Signature
}

/** Entry for accepting or refusing a delivery */
export interface NoticeReply {
  notice_eh: EntryHash
  has_accepted: boolean
}

/** Entry for confirming a recipient's reply on the sender's side */
export interface ReplyAck {
  distribution_ah: ActionHash
  recipient: AgentPubKey
  has_accepted: boolean
  recipient_signature: Signature
}

/** Entry representing a chunk a data (for a parcel) */
export interface ParcelChunk {
  data_hash: string
  data: string
}

/**
 * Entry for holding arbitrary data for a Parcel.
 * Used as a universel way to send data.
 * WARN: Change MANIFEST_ENTRY_NAME const when renaming
 */
export interface ParcelManifest {
  description: ParcelDescription
  data_hash: string
  chunks: EntryHash[]
}

/**
 * Entry for confirming a delivery has been well received or refused by the recipient.
 * TODO: This should be a private link instead of an entry
 */
export interface ReceptionProof {
  notice_eh: EntryHash
  parcel_eh: EntryHash
}

/** Entry for confirming a delivery has been well received or refused by the recipient. */
export interface ReceptionAck {
  distribution_ah: ActionHash
  recipient: AgentPubKey
  recipient_signature: Signature
}

/**
 * A Public Entry representing an encrypted private Entry on the DHT
 * waiting to be received by some recipient.
 * The Entry is encrypted with the recipient's public encryption key.
 * The recipient is the agentId where the entry is linked from.
 */
export interface PendingItem {
  kind: ItemKind
  author: AgentPubKey
  author_signature: Signature
  encrypted_data: unknown
  distribution_ah: ActionHash
}

/** List of structs that PendingItem can embed */
export enum ItemKind {
	NoticeAck = 'NoticeAck',
	NoticeReply = 'NoticeReply',
	ReceptionProof = 'ReceptionProof',
	DeliveryNotice = 'DeliveryNotice',
	ParcelChunk = 'ParcelChunk',
	AppEntryBytes = 'AppEntryBytes',
}

/** Protocol for sending data between agents */
export enum DeliveryGossipProtocolType {
	PublicParcelPublished = 'PublicParcelPublished',
	PublicParcelRemoved = 'PublicParcelRemoved',
	Ping = 'Ping',
	Pong = 'Pong',
}
export type DeliveryGossipProtocolVariantPublicParcelPublished = {PublicParcelPublished: [EntryHash, Timestamp, ParcelReference]}
export type DeliveryGossipProtocolVariantPublicParcelRemoved = {PublicParcelRemoved: [EntryHash, Timestamp, ParcelReference]}
export type DeliveryGossipProtocolVariantPing = {Ping: null}
export type DeliveryGossipProtocolVariantPong = {Pong: null}
export type DeliveryGossipProtocol = 
 | DeliveryGossipProtocolVariantPublicParcelPublished | DeliveryGossipProtocolVariantPublicParcelRemoved | DeliveryGossipProtocolVariantPing | DeliveryGossipProtocolVariantPong;

export interface DistributeParcelInput {
  recipients: AgentPubKey[]
  strategy: DistributionStrategy
  parcel_reference: ParcelReference
}

export interface RespondToNoticeInput {
  notice_eh: EntryHash
  has_accepted: boolean
}

export interface FetchChunkInput {
  chunk_eh: EntryHash
  notice_eh: EntryHash
}

export interface GetNoticeOutput {
  notice: DeliveryNotice
  state: [NoticeState, EntryHash[]]
}

export enum DeliveryNoticeQueryFieldType {
	Sender = 'Sender',
	Distribution = 'Distribution',
	Parcel = 'Parcel',
}
export type DeliveryNoticeQueryFieldVariantSender = {Sender: AgentPubKey}
export type DeliveryNoticeQueryFieldVariantDistribution = {Distribution: ActionHash}
export type DeliveryNoticeQueryFieldVariantParcel = {Parcel: EntryHash}
export type DeliveryNoticeQueryField = 
 | DeliveryNoticeQueryFieldVariantSender | DeliveryNoticeQueryFieldVariantDistribution | DeliveryNoticeQueryFieldVariantParcel;

export enum ReceptionProofQueryFieldType {
	Notice = 'Notice',
	Parcel = 'Parcel',
}
export type ReceptionProofQueryFieldVariantNotice = {Notice: EntryHash}
export type ReceptionProofQueryFieldVariantParcel = {Parcel: EntryHash}
export type ReceptionProofQueryField = 
 | ReceptionProofQueryFieldVariantNotice | ReceptionProofQueryFieldVariantParcel;

export enum NoticeAckQueryFieldType {
	Recipient = 'Recipient',
	Distribution = 'Distribution',
}
export type NoticeAckQueryFieldVariantRecipient = {Recipient: AgentPubKey}
export type NoticeAckQueryFieldVariantDistribution = {Distribution: ActionHash}
export type NoticeAckQueryField = 
 | NoticeAckQueryFieldVariantRecipient | NoticeAckQueryFieldVariantDistribution;

export interface CommitPendingItemInput {
  item: PendingItem
  recipient: AgentPubKey
}

export interface GetDeliveryStateInput {
  distribution_ah: ActionHash
  recipient: AgentPubKey
}

export interface BroadcastInput {
  peers: AgentPubKey[]
  pr: ParcelReference
  timestamp: Timestamp
  removed: boolean
}

export interface PublicParcelRecord {
  pr_eh: EntryHash
  pp_eh: EntryHash
  description: ParcelDescription
  creation_ts: Timestamp
  author: AgentPubKey
  deleteInfo?: [Timestamp, AgentPubKey]
}

/** Dna properties */
export interface DeliveryProperties {
  maxChunkSize: number
  maxParcelSize: number
  maxParcelNameLength: number
  minParcelNameLength: number
}

export interface SystemSignal {
  System: SystemSignalProtocol
}

export interface DeliverySignal {
  from: AgentPubKey
  signal: SignalProtocol
}

/** Protocol for notifying the ViewModel (UI) of system level events */
export type SystemSignalProtocolVariantPostCommitStart = {
  type: "PostCommitStart"
  entry_type: string
}
export type SystemSignalProtocolVariantPostCommitEnd = {
  type: "PostCommitEnd"
  entry_type: string
  succeeded: boolean
}
export type SystemSignalProtocolVariantSelfCallStart = {
  type: "SelfCallStart"
  zome_name: string
  fn_name: string
}
export type SystemSignalProtocolVariantSelfCallEnd = {
  type: "SelfCallEnd"
  zome_name: string
  fn_name: string
  succeeded: boolean
}
export type SystemSignalProtocol =
  | SystemSignalProtocolVariantPostCommitStart
  | SystemSignalProtocolVariantPostCommitEnd
  | SystemSignalProtocolVariantSelfCallStart
  | SystemSignalProtocolVariantSelfCallEnd;

/** Protocol for notifying the ViewModel (UI) */
export enum SignalProtocolType {
	System = 'System',
	Gossip = 'Gossip',
	NewLocalManifest = 'NewLocalManifest',
	NewLocalChunk = 'NewLocalChunk',
	ReceivedChunk = 'ReceivedChunk',
	NewDistribution = 'NewDistribution',
	NewNotice = 'NewNotice',
	NewNoticeAck = 'NewNoticeAck',
	NewReply = 'NewReply',
	NewReplyAck = 'NewReplyAck',
	NewReceptionProof = 'NewReceptionProof',
	NewReceptionAck = 'NewReceptionAck',
	NewPendingItem = 'NewPendingItem',
	PublicParcelPublished = 'PublicParcelPublished',
	PublicParcelRemoved = 'PublicParcelRemoved',
}
export type SignalProtocolVariantSystem = {System: SystemSignalProtocol}
export type SignalProtocolVariantGossip = {Gossip: DeliveryGossipProtocol}
export type SignalProtocolVariantNewLocalManifest = {NewLocalManifest: [EntryHash, Timestamp, ParcelManifest]}
export type SignalProtocolVariantNewLocalChunk = {NewLocalChunk: [EntryHash, ParcelChunk]}
export type SignalProtocolVariantReceivedChunk = {ReceivedChunk: [EntryHash[], number]}
export type SignalProtocolVariantNewDistribution = {NewDistribution: [ActionHash, Timestamp, Distribution]}
export type SignalProtocolVariantNewNotice = {NewNotice: [EntryHash, Timestamp, DeliveryNotice]}
export type SignalProtocolVariantNewNoticeAck = {NewNoticeAck: [EntryHash, Timestamp, NoticeAck]}
export type SignalProtocolVariantNewReply = {NewReply: [EntryHash, Timestamp, NoticeReply]}
export type SignalProtocolVariantNewReplyAck = {NewReplyAck: [EntryHash, Timestamp, ReplyAck]}
export type SignalProtocolVariantNewReceptionProof = {NewReceptionProof: [EntryHash, Timestamp, ReceptionProof]}
export type SignalProtocolVariantNewReceptionAck = {NewReceptionAck: [EntryHash, Timestamp, ReceptionAck]}
export type SignalProtocolVariantNewPendingItem = {NewPendingItem: [EntryHash, PendingItem]}
export type SignalProtocolVariantPublicParcelPublished = {PublicParcelPublished: [EntryHash, Timestamp, ParcelReference]}
export type SignalProtocolVariantPublicParcelRemoved = {PublicParcelRemoved: [EntryHash, Timestamp, ParcelReference]}
export type SignalProtocol = 
 | SignalProtocolVariantSystem | SignalProtocolVariantGossip | SignalProtocolVariantNewLocalManifest | SignalProtocolVariantNewLocalChunk | SignalProtocolVariantReceivedChunk | SignalProtocolVariantNewDistribution | SignalProtocolVariantNewNotice | SignalProtocolVariantNewNoticeAck | SignalProtocolVariantNewReply | SignalProtocolVariantNewReplyAck | SignalProtocolVariantNewReceptionProof | SignalProtocolVariantNewReceptionAck | SignalProtocolVariantNewPendingItem | SignalProtocolVariantPublicParcelPublished | SignalProtocolVariantPublicParcelRemoved;

export interface SendSecretInput {
  secret_eh: EntryHash
  strategy: DistributionStrategy
  recipients: AgentPubKey[]
}

export enum SecretEntryType {
	Secret = 'Secret',
}
export type SecretEntryVariantSecret = {Secret: Secret}
export type SecretEntry = 
 | SecretEntryVariantSecret;

/** Entry representing a secret message */
export interface Secret {
  value: string
}
