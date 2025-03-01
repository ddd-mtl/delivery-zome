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

export const CHUNK_MAX_SIZE = 200 * 1024;

export const PARCEL_MAX_SIZE = 10 * 1024 * 1024;

export const NAME_MIN_LENGTH = 2;

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

/** Information for commiting Entry */
export interface EntryReference {
  eh: EntryHash
  zome_name: ZomeName
  entry_index: EntryDefIndex
  visibility: EntryVisibility
}

/** Informantion about where the data is from */
export interface ManifestReference {
  manifest_eh: EntryHash
  entry_zome_name: ZomeName
  entry_type_name: string
}

/** Shared data between a Distribution and a DeliveryNotice */
export interface DeliverySummary {
  distribution_strategy: DistributionStrategy
  parcel_size: number
  parcel_reference: ParcelReference
}

/** A Parcel is a generic Entry or a ParcelManifest */
export enum ParcelReferenceType {
	AppEntry = 'AppEntry',
	Manifest = 'Manifest',
}
export type ParcelReferenceVariantAppEntry = {AppEntry: EntryReference}
export type ParcelReferenceVariantManifest = {Manifest: ManifestReference}
export type ParcelReference = 
 | ParcelReferenceVariantAppEntry | ParcelReferenceVariantManifest;

/**  */
export type DistributionStrategy =
  | {NORMAL: null} | {DM_ONLY: null} | {DHT_ONLY: null};
export enum DistributionStrategyType {
	Normal = 'Normal',
	DmOnly = 'DmOnly',
	DhtOnly = 'DhtOnly',
}

/** Entry representing a received Manifest */
export interface DeliveryNotice {
  distribution_eh: EntryHash
  summary: DeliverySummary
  sender: AgentPubKey
  sender_summary_signature: Signature
}

/** Entry for confirming a delivery has been well received or refused by a recipient */
export interface DeliveryReceipt {
  distribution_eh: EntryHash
  recipient: AgentPubKey
  recipient_signature: Signature
}

/** Entry for confirming a delivery has been well received or refused by a recipient */
export interface DeliveryReply {
  notice_eh: EntryHash
  has_accepted: boolean
}

/** Entry representing a request to send a Parcel to one or multiple recipients */
export interface Distribution {
  recipients: AgentPubKey[]
  delivery_summary: DeliverySummary
  summary_signature: Signature
}

/** Entry for confirming a manifest has been well received by a recipient */
export interface NoticeReceived {
  distribution_eh: EntryHash
  recipient: AgentPubKey
  recipient_summary_signature: Signature
}

/** Entry representing a file chunk. */
export interface ParcelChunk {
  data: string
}

/** WARN : Change MANIFEST_ENTRY_NAME const when renaming */
export interface ParcelManifest {
  name: string
  custum_entry_type: string
  size: number
  chunks: EntryHash[]
}

/**
 * Entry for confirming a delivery has been well received or refused by a recipient
 * TODO: This should be a private link instead of an entry
 */
export interface ParcelReceived {
  notice_eh: EntryHash
  parcel_eh: EntryHash
}

/** List of structs that PendingItem can embed */
export type ItemKind =
  | {NoticeReceived: null} | {DeliveryReply: null} | {ParcelReceived: null} | {DeliveryNotice: null} | {AppEntryBytes: null} | {ParcelChunk: null};
export enum ItemKindType {
	NoticeReceived = 'NoticeReceived',
	DeliveryReply = 'DeliveryReply',
	ParcelReceived = 'ParcelReceived',
	DeliveryNotice = 'DeliveryNotice',
	AppEntryBytes = 'AppEntryBytes',
	ParcelChunk = 'ParcelChunk',
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
  distribution_eh: EntryHash
}

/** Entry for confirming a delivery has been well received or refused by a recipient */
export interface ReplyReceived {
  distribution_eh: EntryHash
  recipient: AgentPubKey
  has_accepted: boolean
  recipient_signature: Signature
}

export interface DistributeParcelInput {
  recipients: AgentPubKey[]
  strategy: DistributionStrategy
  parcel_ref: ParcelReference
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
  state: NoticeState
}

export enum DeliveryNoticeQueryFieldType {
	Sender = 'Sender',
	Distribution = 'Distribution',
	Parcel = 'Parcel',
}
export type DeliveryNoticeQueryFieldVariantSender = {Sender: AgentPubKey}
export type DeliveryNoticeQueryFieldVariantDistribution = {Distribution: EntryHash}
export type DeliveryNoticeQueryFieldVariantParcel = {Parcel: EntryHash}
export type DeliveryNoticeQueryField = 
 | DeliveryNoticeQueryFieldVariantSender | DeliveryNoticeQueryFieldVariantDistribution | DeliveryNoticeQueryFieldVariantParcel;

export enum ParcelReceivedQueryFieldType {
	Notice = 'Notice',
	Parcel = 'Parcel',
}
export type ParcelReceivedQueryFieldVariantNotice = {Notice: EntryHash}
export type ParcelReceivedQueryFieldVariantParcel = {Parcel: EntryHash}
export type ParcelReceivedQueryField = 
 | ParcelReceivedQueryFieldVariantNotice | ParcelReceivedQueryFieldVariantParcel;

export enum NoticeReceivedQueryFieldType {
	Recipient = 'Recipient',
	Distribution = 'Distribution',
}
export type NoticeReceivedQueryFieldVariantRecipient = {Recipient: AgentPubKey}
export type NoticeReceivedQueryFieldVariantDistribution = {Distribution: EntryHash}
export type NoticeReceivedQueryField = 
 | NoticeReceivedQueryFieldVariantRecipient | NoticeReceivedQueryFieldVariantDistribution;

export interface CommitPendingItemInput {
  item: PendingItem
  recipient: AgentPubKey
}

export interface GetDeliveryStateInput {
  distribution_eh: EntryHash
  recipient: AgentPubKey
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
