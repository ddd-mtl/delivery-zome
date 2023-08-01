import {Dictionary, ZomeViewModel} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";
import {
    ActionHash,
    ActionHashB64,
    AgentPubKey,
    AgentPubKeyB64, AppSignalCb,
    decodeHashFromBase64,
    encodeHashToBase64, EntryHashB64
} from "@holochain/client";
import {Distribution, DistributionState} from "../bindings/delivery.types";
import {AppSignal} from "@holochain/client/lib/api/app/types";


export interface DeliveryPerspective {
    /** -- Encrytion -- */
    myPubEncKey: Uint8Array,
    /** AgentPubKey -> PubEncKey */
    encKeys: Dictionary<Uint8Array>,

    /** -- -- */
    inbox: ActionHashB64[],

    /** -- Distributions -- */
    /** DistributionEh -> state */
    myDistributions: Dictionary<DistributionState>

}

/**
 *
 */
export class DeliveryZvm extends ZomeViewModel {

    static readonly ZOME_PROXY = DeliveryProxy;

    get zomeProxy(): DeliveryProxy {
        return this._zomeProxy as DeliveryProxy;
    }

    /** -- ViewModel -- */

    private _perspective: DeliveryPerspective = {
        myPubEncKey: new Uint8Array(),
        encKeys: {},
        inbox: [],
        myDistributions: {},
    };


    /* */
    get perspective(): DeliveryPerspective {
        return this._perspective;
    }

    /* */
    protected hasChanged(): boolean {
        // TODO
        return true;
    }


    /** -- Signals -- */

    signalHandler?: AppSignalCb = this.mySignalHandler;


    /** */
    mySignalHandler(signal: AppSignal): void {
        console.log("DELIVERY received signal", signal);
    }


    /** -- probe -- */

    /** */
    async probeAll(): Promise<void> {
        this._perspective.myPubEncKey = await this.zomeProxy.getMyEncKey();
        await this.probeInbox();
        await this.queryDistributions();
        this.notifySubscribers();
    }

    /** */
    async probeInbox(): Promise<ActionHashB64[]> {
        const inbox = await this.zomeProxy.pullInbox();
        this._perspective.inbox = inbox.map((ah) => encodeHashToBase64(ah));
        this.notifySubscribers();
        return this._perspective.inbox;
    }


    // /** */
    // async probeDeliveryNotices(): Promise<void> {
    //     const inbox = await this.zomeProxy.pullInbox();
    //     this._perspective.inbox = inbox.map((ah) => encodeHashToBase64(ah));
    //     this.notifySubscribers();
    // }


    /** */
    async probeEncKey(from: AgentPubKeyB64): Promise<Uint8Array> {
        const key = await this.zomeProxy.getEncKey(decodeHashFromBase64(from));
        const maybe = this._perspective.encKeys[from];
        if (!maybe && encodeHashToBase64(maybe) != encodeHashToBase64(key)) {
            this._perspective.encKeys[from] = key;
            this.notifySubscribers();
        }
        return key;
    }


    /** */
    async queryDistributions(): Promise<Dictionary<DistributionState>> {
        //console.log("queryDistributions()", this._perspective.myDistributions);
        const distribs = await this.zomeProxy.queryDistribution();
        let promises = [];
        for (const [eh, _distrib] of distribs) {
            const p = this.zomeProxy.getDistributionState(eh);
            promises.push(p);
        }
        const res = await Promise.allSettled(promises);
        let myDistributions: Dictionary<DistributionState> = {};
        let i = 0;
        for (const [eh, _distrib] of distribs) {
            if (res[i].status == "fulfilled") {
                myDistributions[encodeHashToBase64(eh)] = (res[i] as PromiseFulfilledResult<DistributionState>).value;
            }
            i += 1;
        }
        this._perspective.myDistributions = myDistributions;
        this.notifySubscribers();
        return myDistributions;
    }
}