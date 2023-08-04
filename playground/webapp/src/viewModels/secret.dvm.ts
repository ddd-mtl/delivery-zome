import { DnaViewModel, ZvmDef } from "@ddd-qc/lit-happ";
import {DeliveryZvm, SignalProtocol, SignalProtocolType} from "@delivery/elements";
import {SecretZvm} from "./secret.zvm"
import {AgentDirectoryZvm} from "@ddd-qc/agent-directory"
import {AppSignalCb, encodeHashToBase64} from "@holochain/client";
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
  if (SignalProtocolType.ReceivedNotice in deliverySignal) {
   console.log("ADDING DeliveryNotice. parcel_reference:", deliverySignal.ReceivedNotice[1].summary.parcel_reference);
   const noticeEh = encodeHashToBase64(deliverySignal.ReceivedNotice[0]);
   if ("AppEntry" in deliverySignal.ReceivedNotice[1].summary.parcel_reference) {
     if ("secret_integrity" === deliverySignal.ReceivedNotice[1].summary.parcel_reference.AppEntry.zome_name) {
      this.deliveryZvm.acceptDelivery(noticeEh);
     }
   } else {
    if ("secret_integrity" === deliverySignal.ReceivedNotice[1].summary.parcel_reference.Manifest.entry_zome_name) {
     this.deliveryZvm.acceptDelivery(noticeEh);
    }
   }
  }
  if (SignalProtocolType.ReceivedReply in deliverySignal) {
   //console.log("ADDING ReplyReceived", deliverySignal.ReceivedReply);
  }
  if (SignalProtocolType.ReceivedParcel in deliverySignal) {
   //console.log("ADDING ParcelReceived", deliverySignal.ReceivedParcel);
  }
  if (SignalProtocolType.ReceivedReceipt in deliverySignal) {
   //console.log("ADDING DeliveryReceipt", deliverySignal.ReceivedReceipt);
  }
 }

}