/* This file is generated by zits. Do not edit manually */

import {REMOTE_ENDPOINT, DIRECT_SEND_TIMEOUT_MS, DIRECT_SEND_CHUNK_TIMEOUT_MS, CHUNK_MAX_SIZE, PARCEL_MAX_SIZE, NAME_MIN_LENGTH, DIRECTORY_PATH, DELIVERY_ZOME_NAME, DELIVERY_INTERGRITY_ZOME_NAME, COMMIT_PARCEL_CALLBACK_NAME, DeliveryState, DistributionState, NoticeState, ParcelReference, DistributionStrategy, ItemKind, DeliveryNoticeQueryField, ParcelReceivedQueryField, NoticeReceivedQueryField, LinkTypes, DeliveryEntry, DeliveryProtocol, SignalKind, SignalProtocol, DeliverySummary, DeliveryNotice, DeliveryReceipt, DeliveryReply, Distribution, NoticeReceived, ParcelChunk, ParcelManifest, ParcelReceived, PendingItem, ReplyReceived, DistributeParcelInput, RespondToNoticeInput, FetchChunkInput, GetNoticeOutput, CommitPendingItemInput, GetDeliveryStateInput, PubEncKey, CommitParcelInput, DirectMessage, } from './delivery.types';
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
import {Link} from './deps.types';

import {ZomeProxy} from '@ddd-qc/lit-happ';
import {deliveryFunctionNames} from './delivery.fn';

/**
 *
 */
export class DeliveryProxy extends ZomeProxy {
  static readonly DEFAULT_ZOME_NAME = "delivery"
  static readonly FN_NAMES = deliveryFunctionNames
 

  async commitParcel(input: CommitParcelInput): Promise<ActionHash> {
    return this.call('commit_parcel', input);
  }



  async checkManifest(chunkEh: EntryHash): Promise<EntryHash | null> {
    return this.call('check_manifest', chunkEh);
  }

  async commitChunks(chunks: [ParcelChunk, Link][]): Promise<void> {
    return this.call('commit_chunks', chunks);
  }

  async commitParcelChunk(data: string): Promise<EntryHash> {
    return this.call('commit_parcel_chunk', data);
  }

  async commitParcelManifest(input: ParcelManifest): Promise<EntryHash> {
    return this.call('commit_parcel_manifest', input);
  }

  async commitPendingItem(input: CommitPendingItemInput): Promise<ActionHash> {
    return this.call('commit_pending_item', input);
  }

  async distributeParcel(input: DistributeParcelInput): Promise<EntryHash> {
    return this.call('distribute_parcel', input);
  }

  async fetchChunk(input: FetchChunkInput): Promise<[ParcelChunk, Link | null] | null> {
    return this.call('fetch_chunk', input);
  }

  async fetchParcel(noticeEh: EntryHash): Promise<EntryHash | null> {
    return this.call('fetch_parcel', noticeEh);
  }

  async commitParcelReceived(input: ParcelReceived): Promise<EntryHash> {
    return this.call('commit_ParcelReceived', input);
  }

  async getDeliveryState(input: GetDeliveryStateInput): Promise<DeliveryState> {
    return this.call('get_delivery_state', input);
  }

  async getDistributionState(distributionEh: EntryHash): Promise<DistributionState> {
    return this.call('get_distribution_state', distributionEh);
  }

  async getNotice(distributionEh: EntryHash): Promise<GetNoticeOutput | null> {
    return this.call('get_notice', distributionEh);
  }

  async getParcelNotice(parcelEh: EntryHash): Promise<DeliveryNotice | null> {
    return this.call('get_parcel_notice', parcelEh);
  }

  async getNoticeState(noticeEh: EntryHash): Promise<NoticeState> {
    return this.call('get_notice_state', noticeEh);
  }

  async getEncKey(from: AgentPubKey): Promise<Uint8Array> {
    return this.call('get_enc_key', from);
  }

  async getMyEncKey(): Promise<Uint8Array> {
    return this.call('get_my_enc_key', null);
  }

  async testEncryption(to: AgentPubKey): Promise<void> {
    return this.call('test_encryption', to);
  }

  async pullInbox(): Promise<ActionHash[]> {
    return this.call('pull_inbox', null);
  }

  async queryDistribution(): Promise<[EntryHash, Distribution][]> {
    return this.call('query_Distribution', null);
  }

  async queryDeliveryNotice(queryField: DeliveryNoticeQueryField): Promise<DeliveryNotice[]> {
    return this.call('query_DeliveryNotice', queryField);
  }

  async queryNoticeReceived(field: NoticeReceivedQueryField): Promise<NoticeReceived[]> {
    return this.call('query_NoticeReceived', field);
  }

  async queryParcelReceived(field: ParcelReceivedQueryField): Promise<ParcelReceived | null> {
    return this.call('query_ParcelReceived', field);
  }

  async respondToNotice(input: RespondToNoticeInput): Promise<EntryHash> {
    return this.call('respond_to_notice', input);
  }

  async receiveDeliveryDm(dm: DirectMessage): Promise<DeliveryProtocol> {
    return this.call('receive_delivery_dm', dm);
  }
}
