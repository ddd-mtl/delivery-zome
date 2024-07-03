import {
  AgentId,
  delay,
  DnaViewModel, EntryId, EntryIdMap,
  EntryPulse,
  LinkPulse,
  materializeEntryPulse, materializeLinkPulse,
  StateChangeType,
  TipProtocol,
  TipProtocolVariantEntry, TipProtocolVariantLink,
  ZomeSignal,
  ZomeSignalProtocol,
  ZomeSignalProtocolType,
  ZvmDef
} from "@ddd-qc/lit-happ";
import {
  DeliveryEntryType,
  DeliveryNotice,
  DeliveryZvm,
  ParcelKindType,
  ParcelManifest,
  ParcelReference, PublicParcelRecordMat,
} from "@ddd-qc/delivery";
import {SecretZvm} from "./secret.zvm"
import {AgentDirectoryZvm} from "@ddd-qc/agent-directory"
import {AppSignalCb} from "@holochain/client";
import {AppSignal} from "@holochain/client/lib/api/app/types";
import {decode} from "@msgpack/msgpack";
import {DeliveryLinkType} from "@ddd-qc/delivery/dist/bindings/delivery.integrity";


/** */
export interface SecretDvmPerspective {
  publicMessages:  EntryIdMap<string>
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

  private _perspective: SecretDvmPerspective = {publicMessages: new EntryIdMap()};


  /** Update the perspective accordingly */
  mySignalHandler(appSignal: AppSignal): void {
    console.log("secretDvm received signal", appSignal);
     this.agentDirectoryZvm.zomeProxy.getRegisteredAgents().then((agents) => {
       this._livePeers = agents.map(a => new AgentId(a));
    })
    if (appSignal.zome_name !== DeliveryZvm.DEFAULT_ZOME_NAME) {
      return;
    }
    const deliverySignal = appSignal.payload as ZomeSignal;
    if (!("pulses" in deliverySignal)) {
      return;
    }
    for (const pulse of deliverySignal.pulses) {
      /*await*/ this.handleDeliverySignal(pulse, new AgentId(deliverySignal.from));
    }
  }


  /** */
  async handleDeliverySignal(pulse: ZomeSignalProtocol, from: AgentId): Promise<void> {
    /** Handle Tip first: change tip to Entry/Link pulse */
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
          console.log("Changed Tip to entryPulse:", entryPulse);
        } break;
        case "Link": {
          const linkPulse = (tip as TipProtocolVariantLink).Link;
          pulse = {Link: linkPulse}
          console.log("Changed Tip to linkPulse:", linkPulse);
        } break;
        case "App":
          break;
      }
    }
    /** */
    if (ZomeSignalProtocolType.Entry in pulse) {
      const entryPulse = materializeEntryPulse(pulse.Entry as EntryPulse, Object.values(DeliveryEntryType));
      /** Automatically accept parcel from secret zome */
      switch (entryPulse.entryType) {
        case DeliveryEntryType.DeliveryNotice: {
          const notice = decode(entryPulse.bytes) as DeliveryNotice;
          console.log("ADDING DeliveryNotice:", notice, entryPulse);
          if (ParcelKindType.AppEntry in notice.summary.parcel_reference.description.kind_info) {
            if (entryPulse.isNew && from != this.cell.agentId && "secret_integrity" === notice.summary.parcel_reference.description.zome_origin) {
              this.deliveryZvm.acceptDelivery(entryPulse.eh);
            }
          } else {
            /// split_secret is a Manifest reference
            // if ("secret_integrity" === deliverySignal.NewNotice[1].summary.parcel_reference.Manifest.from_zome) {
            //  this.deliveryZvm.acceptDelivery(noticeEh);
            // }
          }
        }
        break;
        case DeliveryEntryType.PublicParcel: {
          const pr = decode(entryPulse.bytes) as ParcelReference;
          const parcelEh = new EntryId(pr.parcel_eh);
          if (entryPulse.state == StateChangeType.Delete) {
            //const auth = encodeHashToBase64(deliverySignal.DeletedPublicParcel[3]);
            this._perspective.publicMessages.delete(parcelEh);
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
      const linkPulse = materializeLinkPulse(pulse.Link as LinkPulse, Object.values(DeliveryLinkType));
      /** */
      switch (linkPulse.link_type) {
        case DeliveryLinkType.PublicParcels: {
          const parcelEh = this.deliveryZvm.perspective.parcelReferences.get(linkPulse.target);
          console.log("secretDvm handle link signal: PublicParcels", parcelEh, linkPulse.state);
          if (linkPulse.state == StateChangeType.Delete) {
            if (parcelEh) {
              this._perspective.publicMessages.delete(parcelEh);
            }
          }
        }
        break;
      }
    }
  }


  /** */
  async handlePublicParcelPublished(parcelEh: EntryId, from: AgentId) {
    console.log("SecretDvm.handlePublicParcelPublished()", parcelEh, from);
    let msg = undefined;
    do  {
      try {
        await delay(1000);
        msg = await this.deliveryZvm.fetchParcelData(parcelEh);
        this._perspective.publicMessages.set(parcelEh, msg);
        this.notifySubscribers();
      } catch(e) {}
    } while(!msg);
  }


  /** */
  async probePublicMessages(): Promise<void> {
    this._perspective.publicMessages.clear();
    await this.deliveryZvm.probeDht();
    const pds: [EntryId, PublicParcelRecordMat][] = Array.from(this.deliveryZvm.perspective.publicParcels.entries());
    console.log("probePublicMessages() PublicParcels count", pds.length);
    for (const [parcelEh, _tuple] of pds) {
      const str = await this.deliveryZvm.fetchParcelData(parcelEh);
      this._perspective.publicMessages.set(parcelEh, str);
    }
    this.notifySubscribers();
  }


  /** */
  async publishMessage(message: string): Promise<[EntryId, ParcelManifest]> {
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
   return [new EntryId(eh), manifest];
  }
}
