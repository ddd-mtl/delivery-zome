import {Dictionary, ZomeViewModel} from "@ddd-qc/lit-happ";
import {DeliveryProxy} from "../bindings/delivery.proxy";
import {
    ActionHash,
    ActionHashB64,
    AgentPubKey,
    AgentPubKeyB64, AppSignalCb,
    decodeHashFromBase64,
    encodeHashToBase64, EntryHash, EntryHashB64
} from "@holochain/client";
import {
    DeliveryNotice, DeliveryState,
    Distribution,
    DistributionState,
    SignalKind, SignalProtocol, SignalProtocolType,
    SignalProtocolVariantReceivedNotice
} from "../bindings/delivery.types";
import {AppSignal} from "@holochain/client/lib/api/app/types";


/** [DistributionState, AgentPubKey -> DeliveryState] */
export type FullDistributionState = [DistributionState, Dictionary<DeliveryState>];

/** */
export interface DeliveryPerspective {
    /** -- Encrytion -- */
    myPubEncKey: Uint8Array,
    /** AgentPubKey -> PubEncKey */
    encKeys: Dictionary<Uint8Array>,

    /** -- -- */
    inbox: ActionHashB64[],

    newDeliveryNotices: Dictionary<DeliveryNotice>,

    /** -- Distributions -- */

    /** DistributionEh -> [DistributionState, AgentPubKey -> DeliveryState] */
    myDistributions: Dictionary<FullDistributionState>,

    //incomingDistributions: Dictionary<DistributionState>,
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
        newDeliveryNotices: {},
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
        const deliverySignal = signal.payload as SignalProtocol;
        if (SignalProtocolType.ReceivedNotice in deliverySignal) {
            console.log("ADDING DeliveryNotice", deliverySignal.ReceivedNotice);
            const noticeEh = encodeHashToBase64(deliverySignal.ReceivedNotice[0]);
            this._perspective.newDeliveryNotices[noticeEh] = deliverySignal.ReceivedNotice[1];
        }
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
    async queryDistributions(): Promise<Dictionary<FullDistributionState>> {
        //console.log("queryDistributions()", this._perspective.myDistributions);
        const distribs = await this.zomeProxy.queryDistribution();
        let promises = [];
        for (const [eh, _distrib] of distribs) {
            const p = this.zomeProxy.getDistributionState(eh);
            promises.push(p);
        }
        const distribPromisesResult = await Promise.allSettled(promises);
        let myDistributions: Dictionary<FullDistributionState> = {};
        let i = 0;
        for (const [eh, distrib] of distribs) {
            if (distribPromisesResult[i].status == "fulfilled") {
                const distribState = (distribPromisesResult[i] as PromiseFulfilledResult<DistributionState>).value;
                const deliveryStates = await this.queryDistribution(eh, distrib);
                myDistributions[encodeHashToBase64(eh)] = [distribState, deliveryStates];
            }
            else {
                console.warn("getDistributionState() failed:", (distribPromisesResult[i] as PromiseRejectedResult).reason);
            }
            i += 1;
        }
        console.log("queryDistributions() result", myDistributions);
        this._perspective.myDistributions = myDistributions;
        this.notifySubscribers();
        return myDistributions;
    }


    /** */
    async queryDistribution(eh: EntryHash, distrib: Distribution): Promise<Dictionary<DeliveryState>> {
        let deliveryPromises = [];
        for (const recipient of distrib.recipients) {
            const p = this.zomeProxy.getDeliveryState({distribution_eh: eh, recipient});
            deliveryPromises.push(p);
        }
        const deliveryPromisesResult = await Promise.allSettled(deliveryPromises);
        let deliveryStates: Dictionary<DeliveryState> = {};
        let j = 0;
        for (const recipient of distrib.recipients) {
            if (deliveryPromisesResult[j].status == "fulfilled") {
                deliveryStates[encodeHashToBase64(recipient)] = (deliveryPromisesResult[j] as PromiseFulfilledResult<DeliveryState>).value;
            } else {
                console.warn("getDeliveryState() failed:", (deliveryPromisesResult[j] as PromiseRejectedResult).reason);
            }
            j += 1;
        }
        return deliveryStates;
    }


    /** -- API Sugar -- */

    /** */
    async acceptDelivery(noticeEh: EntryHashB64) {
        this.zomeProxy.respondToNotice({notice_eh: decodeHashFromBase64(noticeEh), has_accepted: true});
    }

    /** */
    async declineDelivery(noticeEh: EntryHashB64) {
        this.zomeProxy.respondToNotice({notice_eh: decodeHashFromBase64(noticeEh), has_accepted: false});
    }

    /** */
    async getDeliveryState(distribEh: EntryHashB64, recipient: AgentPubKeyB64) {
        this.zomeProxy.getDeliveryState({distribution_eh: decodeHashFromBase64(distribEh), recipient: decodeHashFromBase64(recipient)});
    }
}