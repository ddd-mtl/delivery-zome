import {ZomeViewModel} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";


/**
 *
 */
export class DeliveryZvm extends ZomeViewModel {

    static readonly ZOME_PROXY = DeliveryProxy;

    get zomeProxy(): DeliveryProxy {
        return this._zomeProxy as DeliveryProxy;
    }

    /** -- ViewModel -- */

    private _perspective: unknown = {};


    /* */
    get perspective(): unknown {
        return {};
    }

    /* */
    protected hasChanged(): boolean {
        // TODO
        return true;
    }


    /** */
    async probeAll(): Promise<void> {

    }


}