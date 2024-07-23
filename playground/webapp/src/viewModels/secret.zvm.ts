import {SecretProxy} from '../bindings/secret.proxy';
import {AgentIdMap, EntryId, EntryIdMap, ZomeViewModel, AgentId, ActionId} from "@ddd-qc/lit-happ";
import {DistributionStrategy} from "../bindings/secret.types";
import {HoloHash} from "@holochain/client";


/** */
export interface SecretPerspective {
  /** AgentPubKey -> secret_eh[] */
  secretsByAgent: AgentIdMap<EntryId[]>,
 /** secret_eh -> Value */
  secrets: EntryIdMap<string>,
}


/**
 *
 */
export class SecretZvm extends ZomeViewModel {

  static readonly ZOME_PROXY = SecretProxy;
  get zomeProxy(): SecretProxy {return this._zomeProxy as SecretProxy;}



  /** -- ViewModel -- */

  private _perspective: SecretPerspective = {secretsByAgent: new AgentIdMap(), secrets: new EntryIdMap()}

  /* */
  get perspective(): SecretPerspective {return this._perspective}

  /* */
  protected hasChanged(): boolean {
    // TODO
    return true;
  }


  /** -- Methods -- */


  // /** */
  // async probeAll() {
  //   console.log("SecretViewModel.probeAll() called");
  //   this.notifySubscribers()
  // }


  // /** */
  // async probeSecrets(): Promise<AgentPubKeyB64[]> {
  //   const secret_eh = await this.zomeProxy.createSecret(text);
  // }



  /** */
  async sendSecretToOne(text: string, recipient: AgentId, canSplit: boolean): Promise<ActionId> {
    const secret_eh = canSplit
        ? await this.zomeProxy.createSplitSecret(text)
        : await this.zomeProxy.createSecret(text)
    const input = {
      secret_eh,
      strategy: DistributionStrategy.Normal,
      recipients: [new HoloHash(recipient.hash)],
    }
    const res = await this.zomeProxy.sendSecret(input);
    return new ActionId(res);
  }


  /** */
  async getSecretsFrom(sender: AgentId): Promise<EntryId[]> {
    console.log("getSecretsFrom()", sender);
    const res = (await this.zomeProxy.getSecretsFrom(new HoloHash(sender.hash))).map(hash => new EntryId(hash));
    console.log("getSecretsFrom() res", res);
    for (const secretEh of res) {
      this._perspective.secrets.set(secretEh, await this.zomeProxy.getSecret(new HoloHash(secretEh.hash)));
    }
    this._perspective.secretsByAgent.set(sender, res);
    return res;
  }
}
