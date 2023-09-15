/* This file is generated by zits. Do not edit manually */

import {
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
export type DeliveryState =
  | {Unsent: null} | {PendingNotice: null} | {NoticeDelivered: null} | {ParcelRefused: null} | {ParcelAccepted: null} | {PendingParcel: null} | {ParcelDelivered: null};
export enum DeliveryStateType {
	Unsent = 'Unsent',
	PendingNotice = 'PendingNotice',
	NoticeDelivered = 'NoticeDelivered',
	ParcelRefused = 'ParcelRefused',
	ParcelAccepted = 'ParcelAccepted',
	PendingParcel = 'PendingParcel',
	ParcelDelivered = 'ParcelDelivered',
}

/** Possible states of an OutMail entry */
export type DistributionState =
  | {Unsent: null} | {AllNoticesSent: null} | {AllNoticeReceived: null} | {AllRepliesReceived: null} | {AllAcceptedParcelsReceived: null} | {Deleted: null};
export enum DistributionStateType {
	Unsent = 'Unsent',
	AllNoticesSent = 'AllNoticesSent',
	AllNoticeReceived = 'AllNoticeReceived',
	AllRepliesReceived = 'AllRepliesReceived',
	AllAcceptedParcelsReceived = 'AllAcceptedParcelsReceived',
	Deleted = 'Deleted',
}

/** Possible states of a DeliveryNotice entry */
export type NoticeState =
  | {Unreplied: null} | {Accepted: null} | {Refused: null} | {Received: null} | {Deleted: null};
export enum NoticeStateType {
	Unreplied = 'Unreplied',
	Accepted = 'Accepted',
	Refused = 'Refused',
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
export type DistributionStrategy =
  | {NORMAL: null} | {DM_ONLY: null} | {DHT_ONLY: null};
export enum DistributionStrategyType {
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
export type ItemKind =
  | {NoticeAck: null} | {NoticeReply: null} | {ReceptionProof: null} | {DeliveryNotice: null} | {ParcelChunk: null} | {AppEntryBytes: null};
export enum ItemKindType {
	NoticeAck = 'NoticeAck',
	NoticeReply = 'NoticeReply',
	ReceptionProof = 'ReceptionProof',
	DeliveryNotice = 'DeliveryNotice',
	ParcelChunk = 'ParcelChunk',
	AppEntryBytes = 'AppEntryBytes',
}

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
  state: [NoticeState, number]
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

/** Dna properties */
export interface DeliveryProperties {
  maxChunkSize: number
  maxParcelSize: number
  maxParcelNameLength: number
  minParcelNameLength: number
}

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
