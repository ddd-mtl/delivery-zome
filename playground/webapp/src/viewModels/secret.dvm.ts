import {delay, Dictionary, DnaViewModel, ZvmDef} from "@ddd-qc/lit-happ";
import {
  DeliveryEntryType,
  DeliveryNotice,
  DeliveryZvm,
  EntryPulse, LinkTypes,
  ParcelKindType,
  ParcelManifest,
  ParcelReference,
  StateChangeType,
  TipProtocol,
  TipProtocolVariantEntry,
  ZomeSignal,
  ZomeSignalProtocol,
  ZomeSignalProtocolType,
} from "@ddd-qc/delivery";
import {SecretZvm} from "./secret.zvm"
import {AgentDirectoryZvm} from "@ddd-qc/agent-directory"
import {AgentPubKeyB64, AppSignalCb, encodeHashToBase64, EntryHashB64} from "@holochain/client";
import {AppSignal} from "@holochain/client/lib/api/app/types";
import {getVariantByIndex} from "@ddd-qc/delivery/dist/utils";
import {decode} from "@msgpack/msgpack";


/** */
export interface SecretDvmPerspective {
  publicMessages:  Dictionary<string>
}


/**
 * TODO: Make a "passthrough" DVM generator in dna-client based on ZVM_DEFS
 */
export class SecretDvm extends DnaViewModel {

  /** -- DnaViewModel Interface -- */

  static readonly DEFAULT_BASE_ROLE_NAME = "rSecret";
  static readonly ZVM_DEFS: ZvmDef[] = [
   SecretZvm,
   [DeliveryZvm, "zDelivery"],
   [AgentDirectoryZvm, "zAgentDirectory"],
  ];

  readonly signalHandler?: AppSignalCb = this.mySignalHandler;


  /** QoL Helpers */
  get secretZvm(): SecretZvm {return this.getZomeViewModel(SecretZvm.DEFAULT_ZOME_NAME) as SecretZvm}
  get deliveryZvm(): DeliveryZvm {return this.getZomeViewModel("zDelivery") as DeliveryZvm}
  get agentDirectoryZvm(): AgentDirectoryZvm {return this.getZomeViewModel("zAgentDirectory") as AgentDirectoryZvm}


  /** -- ViewModel Interface -- */

  protected hasChanged(): boolean {return true}

  get perspective(): SecretDvmPerspective {return this._perspective}

  private _perspective: SecretDvmPerspective = {publicMessages: {}};


  /** Update the perspective accordingly */
  mySignalHandler(appSignal: AppSignal): void {
    console.log("secretDvm received signal", appSignal);
     this.agentDirectoryZvm.zomeProxy.getRegisteredAgents().then((agents) => {
       this._livePeers = agents.map(a => encodeHashToBase64(a));
    })
    if (appSignal.zome_name !== DeliveryZvm.DEFAULT_ZOME_NAME) {
      return;
    }
    const deliverySignal = appSignal.payload as ZomeSignal;
    if (!("pulses" in deliverySignal)) {
      return;
    }
    for (const pulse of deliverySignal.pulses) {
      /*await*/ this.handleDeliverySignal(pulse, encodeHashToBase64(deliverySignal.from));
    }
  }


  /** */
  async handleDeliverySignal(pulse: ZomeSignalProtocol, from: AgentPubKeyB64): Promise<void> {
    /** Handle Tip first: change tip to Entry pulse */
    if (ZomeSignalProtocolType.Tip in pulse) {
      const tip = pulse.Tip as TipProtocol;
      const tipType = Object.keys(tip)[0];
      /* Handle tip according to its type */
      switch (tipType) {
        case "Ping":
        case "Pong":
          break;
        case "Entry": {
          const entryPulse = (tip as TipProtocolVariantEntry).Entry;
          pulse = {Entry: entryPulse}
        } break;
        case "Link":
        case "App":
          break;
      }
    }
    /** */
    if (ZomeSignalProtocolType.Entry in pulse) {
      const entryPulse = pulse.Entry as EntryPulse;
      const entryType = getVariantByIndex(DeliveryEntryType, entryPulse.def.entry_index);
      const author = encodeHashToBase64(entryPulse.author);
      const ah = encodeHashToBase64(entryPulse.ah);
      const eh = encodeHashToBase64(entryPulse.eh);
      const state = Object.keys(entryPulse.state)[0];
      const isNew = (entryPulse.state as any)[state];
      /** Automatically accept parcel from secret zome */
      switch (entryType) {
        case "DeliveryNotice": {
          const notice = decode(entryPulse.bytes) as DeliveryNotice;
          console.log("ADDING DeliveryNotice. parcel_description:", notice.summary.parcel_reference.description);
          if (ParcelKindType.AppEntry in notice.summary.parcel_reference.description.kind_info) {
            if ("secret_integrity" === notice.summary.parcel_reference.description.zome_origin) {
              this.deliveryZvm.acceptDelivery(eh);
            }
          } else {
            /// split_secret is a Manifest reference
            // if ("secret_integrity" === deliverySignal.NewNotice[1].summary.parcel_reference.Manifest.from_zome) {
            //  this.deliveryZvm.acceptDelivery(noticeEh);
            // }
          }
        }
        break;
        case "PublicParcel": {
          const pr = decode(entryPulse.bytes) as ParcelReference;
          const parcelEh = encodeHashToBase64(pr.parcel_eh);
          if (state == StateChangeType.Delete) {
            //const auth = encodeHashToBase64(deliverySignal.DeletedPublicParcel[3]);
            delete this._perspective.publicMessages[parcelEh];
            this.notifySubscribers();
          } else {
            /*await */ this.handlePublicParcelPublished(parcelEh, from);
          }
        }
        break;
      }
    }
    /** */
    if (ZomeSignalProtocolType.Link in pulse) {
      const link = pulse.Link.link;
      const linkAh = encodeHashToBase64(link.create_link_hash);
      const author = encodeHashToBase64(link.author);
      const base = encodeHashToBase64((link as any).base);
      const target = encodeHashToBase64(link.target);
      const state = Object.keys(pulse.Link.state)[0];
      const isNew = (pulse.Link.state as any)[state];
      /** */
      switch (getVariantByIndex(LinkTypes, link.link_type)) {
        case LinkTypes.PublicParcels: {
          const parcelEh = this.deliveryZvm.perspective.parcelReferences[target];
          console.log("secretDvm handle link signal: PublicParcels", parcelEh, state);
          if (state == StateChangeType.Delete) {
            if (parcelEh) {
              delete this._perspective.publicMessages[parcelEh];
            }
          }
        }
        break;
      }
    }
  }


  /** */
  async handlePublicParcelPublished(parcelEh: EntryHashB64, from: AgentPubKeyB64) {
    console.log("SecretDvm.handlePublicParcelPublished()", parcelEh, from);
    let msg = undefined;
    do  {
      try {
        await delay(1000);
        msg = await this.deliveryZvm.fetchParcelData(parcelEh);
        this._perspective.publicMessages[parcelEh] = msg;
        this.notifySubscribers();
      } catch(e) {}
    } while(!msg);
  }


  /** */
  async probePublicMessages(): Promise<Dictionary<string>> {
    let publicMessages: Dictionary<string> = {};
    await this.deliveryZvm.probeDht();
    const pds = Object.entries(this.deliveryZvm.perspective.publicParcels);
    console.log("probePublicMessages() PublicParcels count", Object.entries(pds).length);
    for (const [parcelEh, tuple] of pds) {
      publicMessages[parcelEh] = await this.deliveryZvm.fetchParcelData(parcelEh);
    }
    this._perspective.publicMessages = publicMessages;
    this.notifySubscribers();
    return publicMessages;
  }


  /** */
  async publishMessage(message: string): Promise<[EntryHashB64, ParcelManifest]> {
   const data_hash = message; // should be an actual hash, but we don't care in this example code.
   const chunk_ehs = await this.deliveryZvm.zomeProxy.publishChunks([{data_hash, data: message}]);
   const manifest: ParcelManifest = {
     data_hash,
     chunks: [chunk_ehs[0]],
     description: {
       size: 0,
       zome_origin: "secret_integrity",
       kind_info: {Manifest: "public_secret"},
       name: message[0],
       visibility: "Public" //{Public: null}
     },
   };
   const eh = await this.deliveryZvm.zomeProxy.publishPublicParcel(manifest);
   return [encodeHashToBase64(eh), manifest];
  }
}
