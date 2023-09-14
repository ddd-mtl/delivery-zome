import { DnaViewModel, ZvmDef } from "@ddd-qc/lit-happ";
import {DeliveryZvm, ParcelKindType, ParcelManifest, SignalProtocol, SignalProtocolType} from "@ddd-qc/delivery";
import {SecretZvm} from "./secret.zvm"
import {AgentDirectoryZvm} from "@ddd-qc/agent-directory"
import {AppSignalCb, encodeHashToBase64, EntryHashB64, ZomeName} from "@holochain/client";
import {AppSignal} from "@holochain/client/lib/api/app/types";



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
  get AgentDirectoryZvm(): AgentDirectoryZvm {return this.getZomeViewModel("zAgentDirectory") as AgentDirectoryZvm}


  /** -- ViewModel Interface -- */

  protected hasChanged(): boolean {return true}

  get perspective(): void {return}


  /** */
  mySignalHandler(signal: AppSignal): void {
   console.log("secretDvm received signal", signal);
   const deliverySignal = signal.payload as SignalProtocol;

   /** Automatically accept parcel from secret zome */
   if (SignalProtocolType.NewNotice in deliverySignal) {
    console.log("ADDING DeliveryNotice. parcel_description:", deliverySignal.NewNotice[1].summary.parcel_description);
    const noticeEh = encodeHashToBase64(deliverySignal.NewNotice[0]);
    if (ParcelKindType.AppEntry in deliverySignal.NewNotice[1].summary.parcel_description.reference.kind_info) {
      if ("secret_integrity" === deliverySignal.NewNotice[1].summary.parcel_description.reference.zome_origin) {
       this.deliveryZvm.acceptDelivery(noticeEh);
      }
    } else {
     /// split_secret is a Manifest reference
     // if ("secret_integrity" === deliverySignal.NewNotice[1].summary.parcel_reference.Manifest.from_zome) {
     //  this.deliveryZvm.acceptDelivery(noticeEh);
     // }
    }
   }

   if (SignalProtocolType.NewReceptionProof in deliverySignal) {
    console.log("ADDING NewReceptionProof. parcel_eh:", encodeHashToBase64(deliverySignal.NewReceptionProof[1].parcel_eh));
   }
  }


  async publishMessage(message: string): Promise<EntryHashB64> {
   const chunk_eh = await this.deliveryZvm.zomeProxy.publishChunk(message);

   const eh = await this.deliveryZvm.zomeProxy.publishParcel({
    manifest: {
     data_hash: "fake-hash",
     chunks: [chunk_eh],
    },
    zome_origin: "secret_integrity",
    data_type: "public_secret",
    name: message[0],
   });
   return encodeHashToBase64(eh);
  }
}