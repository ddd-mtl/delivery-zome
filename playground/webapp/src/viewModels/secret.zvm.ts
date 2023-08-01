import {SecretProxy} from '../bindings/secret.proxy';
import {ZomeViewModel, CellProxy, DnaViewModel} from "@ddd-qc/lit-happ";
import {
  AgentPubKey,
  AgentPubKeyB64,
  decodeHashFromBase64,
  encodeHashToBase64,
  EntryHash,
  EntryHashB64
} from "@holochain/client";


/** */
export interface SecretPerspective {
  /** AgentPubKey -> secret_eh */
  parcelsByAgent: Record<AgentPubKeyB64, EntryHashB64>,
 /** secret_eh -> Value */
  secrets: Record<EntryHashB64, string>,
}


/**
 *
 */
export class SecretZvm extends ZomeViewModel {

  static readonly ZOME_PROXY = SecretProxy;
  get zomeProxy(): SecretProxy {return this._zomeProxy as SecretProxy;}



  /** -- ViewModel -- */

  private _perspective: SecretPerspective = {parcelsByAgent: {}, secrets: {}}

  /* */
  get perspective(): SecretPerspective {return this._perspective}

  /* */
  protected hasChanged(): boolean {
    // TODO
    return true;
  }


  /** -- Methods -- */


  /** */
  async probeAll() {
    console.log("SecretViewModel.probeAll() called");

    this.notifySubscribers()
  }


  /** */
  async sendSecretToOne(text: string, recipient: AgentPubKeyB64): Promise<EntryHashB64> {
    const secret_eh = await this.zomeProxy.createSecret(text);
    const input = {
      secret_eh,
      strategy: {NORMAL: null},
      recipients: [decodeHashFromBase64(recipient)],
    }
    const res = await this.zomeProxy.sendSecret(input);
    return encodeHashToBase64(res);
  }

  // async probeSecrets(): Promise<AgentPubKeyB64[]> {
  //   const secret_eh = await this.zomeProxy.createSecret(text);
  // }
}
