import {Dictionary, DnaViewModel, ZvmDef} from "@ddd-qc/lit-happ";
import {
  DeliveryGossipProtocolType,
  DeliverySignal,
  DeliveryZvm,
  ParcelKindType,
  ParcelManifest,
  DeliverySignalProtocol,
  DeliverySignalProtocolType,
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
    const sig = signal.payload as DeliverySignal;
    for (const signal of sig.signal) {
      /*await*/ this.handleDeliverySignal(signal, encodeHashToBase64(sig.from));
    }
  }

  /** */
  async handleDeliverySignal(deliverySignal: DeliverySignalProtocol, from: AgentPubKeyB64): Promise<void> {
    /** Automatically accept parcel from secret zome */
    if (DeliverySignalProtocolType.NewNotice in deliverySignal) {
      console.log("ADDING DeliveryNotice. parcel_description:", deliverySignal.NewNotice[2].summary.parcel_reference.description);
      const noticeEh = encodeHashToBase64(deliverySignal.NewNotice[0]);
      if (ParcelKindType.AppEntry in deliverySignal.NewNotice[2].summary.parcel_reference.description.kind_info) {
        if ("secret_integrity" === deliverySignal.NewNotice[2].summary.parcel_reference.description.zome_origin) {
          this.deliveryZvm.acceptDelivery(noticeEh);
        }
      } else {
       /// split_secret is a Manifest reference
       // if ("secret_integrity" === deliverySignal.NewNotice[1].summary.parcel_reference.Manifest.from_zome) {
       //  this.deliveryZvm.acceptDelivery(noticeEh);
       // }
      }
    }

    if (DeliverySignalProtocolType.NewReceptionProof in deliverySignal) {
      console.log("ADDING NewReceptionProof. parcel_eh:", encodeHashToBase64(deliverySignal.NewReceptionProof[2].parcel_eh));
    }
    if (DeliverySignalProtocolType.PublicParcelPublished in deliverySignal) {
      console.log("signal NewPublicParcel", deliverySignal.PublicParcelPublished);
      const ppEh = encodeHashToBase64(deliverySignal.PublicParcelPublished[2].eh);
      this.handlePublicParcelPublished(ppEh, from);
    }
    if (DeliverySignalProtocolType.Gossip in deliverySignal) {
      console.log("signal Gossip", deliverySignal.Gossip);
      const gossip = deliverySignal.Gossip;
      if (DeliveryGossipProtocolType.PublicParcelPublished in gossip) {
        console.log("Gossip signal PublicParcelPublished", gossip.PublicParcelPublished);
        const ppEh = encodeHashToBase64(gossip.PublicParcelPublished[2].eh);
        this.handlePublicParcelPublished(ppEh, from);
      }
    }
  }


  /** */
  handlePublicParcelPublished(ppEh: EntryHashB64, from: AgentPubKeyB64) {
    if (from != this.cell.agentPubKey) {
      this.probeAll();
    } else {
      this.deliveryZvm.getParcelData(ppEh).then((msg: string) => {
        this._perspective.publicMessages[ppEh] = msg;
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
    for (const [ppEh, tuple] of pds) {
      publicMessages[ppEh] = await this.deliveryZvm.getParcelData(ppEh);
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
   const eh = await this.deliveryZvm.zomeProxy.publishManifest(manifest);
   return [encodeHashToBase64(eh), manifest];
  }
}
