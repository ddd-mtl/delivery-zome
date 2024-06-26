import {Dictionary, DnaViewModel, ZvmDef} from "@ddd-qc/lit-happ";
import {
  DeliveryZvm,
  ParcelKindType,
  ParcelManifest, StateChangeType, ZomeSignal, ZomeSignalProtocol, ZomeSignalProtocolType,
} from "@ddd-qc/delivery";
import {SecretZvm} from "./secret.zvm"
import {AgentDirectoryZvm} from "@ddd-qc/agent-directory"
import {AgentPubKey, AgentPubKeyB64, AppSignalCb, encodeHashToBase64, EntryHashB64, ZomeName} from "@holochain/client";
import {AppSignal} from "@holochain/client/lib/api/app/types";


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



  /** */
  mySignalHandler(signal: AppSignal): void {
    console.log("secretDvm received signal", signal);
    if (!("signal" in (signal.payload as Object))) {
      return;
    }
    const sig = signal.payload as ZomeSignal;
    for (const pulse of sig.pulses) {
      /*await*/ this.handleDeliverySignal(pulse, encodeHashToBase64(sig.from));
    }
  }


  /** */
  async handleDeliverySignal(deliverySignal: ZomeSignalProtocol, from: AgentPubKeyB64): Promise<void> {
    // if (ZomeSignalProtocolType.Entry in deliverySignal) {
    //   const [entryInfo, entryKind] = deliverySignal.Entry;
    //   const hash = encodeHashToBase64(entryInfo.hash);
    //   const author = encodeHashToBase64(entryInfo.author);
    //   /** Automatically accept parcel from secret zome */
    //   if (ZomeSignalProtocolType.DeliveryNotice in entryKind) {
    //     const notice = entryKind.DeliveryNotice;
    //     console.log("ADDING DeliveryNotice. parcel_description:", notice.summary.parcel_reference.description);
    //     if (ParcelKindType.AppEntry in notice.summary.parcel_reference.description.kind_info) {
    //       if ("secret_integrity" === notice.summary.parcel_reference.description.zome_origin) {
    //         this.deliveryZvm.acceptDelivery(hash);
    //       }
    //     } else {
    //      /// split_secret is a Manifest reference
    //      // if ("secret_integrity" === deliverySignal.NewNotice[1].summary.parcel_reference.Manifest.from_zome) {
    //      //  this.deliveryZvm.acceptDelivery(noticeEh);
    //      // }
    //     }
    //   }
    //   if (ZomeSignalProtocolType.PublicParcel in entryKind) {
    //     console.log("signal PublicParcel", entryKind.PublicParcel);
    //     const parcelEh = encodeHashToBase64(entryKind.PublicParcel.parcel_eh);
    //     if (entryInfo.state == StateChangeType.Delete) {
    //       //const auth = encodeHashToBase64(deliverySignal.DeletedPublicParcel[3]);
    //       delete this._perspective.publicMessages[parcelEh];
    //       this.notifySubscribers();
    //     } else {
    //       this.handlePublicParcelPublished(parcelEh, this.cell.agentPubKey);
    //     }
    //   }
    // }
    // /** */
    // if (ZomeSignalProtocolType.Tip in deliverySignal) {
    //   console.log("signal Gossip", deliverySignal.Gossip);
    //   const gossip = deliverySignal.Gossip;
    //   if (DeliveryGossipProtocolType.PublicParcelPublished in gossip) {
    //     console.log("Gossip signal PublicParcelPublished", gossip.PublicParcelPublished);
    //     const parcelEh = encodeHashToBase64(gossip.PublicParcelPublished[2].parcel_eh);
    //     this.handlePublicParcelPublished(parcelEh, from);
    //   }
    //   if (DeliveryGossipProtocolType.PublicParcelUnpublished in gossip) {
    //     console.log("Gossip signal PublicParcelUnpublished", gossip.PublicParcelUnpublished);
    //     const parcelEh = encodeHashToBase64(gossip.PublicParcelUnpublished[2].parcel_eh);
    //     delete this._perspective.publicMessages[parcelEh];
    //     this.notifySubscribers();
    //   }
    // }
  }


  /** */
  handlePublicParcelPublished(parcelEh: EntryHashB64, from: AgentPubKeyB64) {
    if (from != this.cell.agentPubKey) {
      this.probeAll();
    } else {
      this.deliveryZvm.getParcelData(parcelEh).then((msg: string) => {
        this._perspective.publicMessages[parcelEh] = msg;
        this.notifySubscribers();
      })
    }
  }


  /** */
  async probePublicMessages(): Promise<Dictionary<string>> {
    let publicMessages: Dictionary<string> = {};
    await this.deliveryZvm.probeDht();
    const pds = Object.entries(this.deliveryZvm.perspective.publicParcels);
    console.log("probePublicMessages() PublicParcels count", Object.entries(pds).length);
    for (const [parcelEh, tuple] of pds) {
      publicMessages[parcelEh] = await this.deliveryZvm.getParcelData(parcelEh);
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
