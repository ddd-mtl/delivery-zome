import {Dictionary, ZomeViewModel} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";
import {
    ActionHash,
    ActionHashB64,
    AgentPubKey,
    AgentPubKeyB64,
    decodeHashFromBase64,
    encodeHashToBase64
} from "@holochain/client";


export interface DeliveryPerspective {
    myPubEncKey: Uint8Array,
    /** AgentPubKey -> PubEncKey */
    encKeys: Dictionary<Uint8Array>,
    inbox: ActionHashB64[],
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

    private _perspective: DeliveryPerspective = {myPubEncKey: new Uint8Array(), inbox: [], encKeys: {}};


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
        this._perspective.myPubEncKey = await this.zomeProxy.getMyEncKey();
        await this.probeInbox();
        this.notifySubscribers();
    }

    /** */
    async probeInbox(): Promise<void> {
        const inbox = await this.zomeProxy.pullInbox();
        this._perspective.inbox = inbox.map((ah) => encodeHashToBase64(ah));
        this.notifySubscribers();
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
}