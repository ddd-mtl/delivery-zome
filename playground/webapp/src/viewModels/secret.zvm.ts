import {SecretProxy} from '../bindings/secret.proxy';
import {ZomeViewModel, CellProxy, DnaViewModel} from "@ddd-qc/lit-happ";


/**
 *
 */
export class SecretZvm extends ZomeViewModel {

  static readonly ZOME_PROXY = SecretProxy;
  get zomeProxy(): SecretProxy {return this._zomeProxy as SecretProxy;}



  /** -- ViewModel -- */

  private _perspective = {}

  /* */
  get perspective(): unknown {return this._perspective}

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

}
