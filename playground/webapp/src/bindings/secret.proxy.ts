/* This file is generated by zits. Do not edit manually */

import {DELIVERY_INTERGRITY_ZOME_NAME, DELIVERY_ZOME_NAME, DIRECTORY_PATH, DIRECT_SEND_CHUNK_TIMEOUT_MS, DIRECT_SEND_TIMEOUT_MS, REMOTE_ENDPOINT, DeliveryNoticeQueryField, DeliveryState, DeliveryTipProtocol, DistributionState, DistributionStrategy, ItemKind, NoticeAckQueryField, NoticeState, ParcelKind, ReceptionProofQueryField, SecretEntry, BroadcastInput, CommitPendingItemInput, DeliveryNotice, DeliveryProperties, DeliverySummary, DistributeParcelInput, Distribution, FetchChunkInput, GetDeliveryStateInput, GetNoticeOutput, NoticeAck, NoticeReply, ParcelChunk, ParcelDescription, ParcelManifest, ParcelReference, PendingItem, PublicParcelRecord, ReceptionAck, ReceptionProof, ReplyAck, RespondToNoticeInput, Secret, SendSecretInput, } from './secret.types';
import {
WebsocketConnectionOptions,
/** types.ts */
//HoloHash,
//AgentPubKey,
//DnaHash,
//WasmHash,
//EntryHash,
//ActionHash,
//AnyDhtHash,
//ExternalHash,
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


/// Simple Hashes
type AgentArray = Uint8Array;
type DnaArray = Uint8Array;
type WasmArray = Uint8Array;
type EntryArray = Uint8Array;
type ActionArray = Uint8Array;
type AnyDhtArray = Uint8Array;
type AnyLinkableArray = Uint8Array;

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

import {ZomeProxy} from '@ddd-qc/lit-happ';
import {secretFunctionNames} from './secret.fn';
import {SecretUnitEnum, SecretLinkType} from './secret.integrity';

/**
 *
 */
export class SecretProxy extends ZomeProxy {
  static readonly DEFAULT_ZOME_NAME = "zSecret";
  static readonly FN_NAMES = secretFunctionNames;
  static readonly ENTRY_TYPES = Object.values(SecretUnitEnum);
  static readonly LINK_TYPES = Object.values(SecretLinkType);
 
  async createSecret(value: string): Promise<EntryArray> {
    return this.call('create_secret', value);
  }

  async createSplitSecret(value: string): Promise<EntryArray> {
    return this.call('create_split_secret', value);
  }

  async getSecret(eh: EntryArray): Promise<string> {
    return this.call('get_secret', eh);
  }

  async getSecretsFrom(sender: AgentArray): Promise<EntryArray[]> {
    return this.call('get_secrets_from', sender);
  }

  async refuseSecret(parcelEh: EntryArray): Promise<EntryArray> {
    return this.call('refuse_secret', parcelEh);
  }

  async acceptSecret(parcelEh: EntryArray): Promise<EntryArray> {
    return this.call('accept_secret', parcelEh);
  }

  async sendSecret(input: SendSecretInput): Promise<ActionArray> {
    return this.call('send_secret', input);
  }
}
