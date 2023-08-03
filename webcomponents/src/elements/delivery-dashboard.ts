import {css, html} from "lit";
import {property, state, customElement} from "lit/decorators.js";
import { ZomeElement } from "@ddd-qc/lit-happ";
import {DeliveryPerspective, DeliveryZvm} from "../viewModels/delivery.zvm";
import {decodeHashFromBase64, encodeHashToBase64} from "@holochain/client";


/**
 *
 */
@customElement("delivery-dashboard")
export class DeliveryDashboard extends ZomeElement<DeliveryPerspective, DeliveryZvm>  {
    /** */
    constructor() {
        super(DeliveryZvm.DEFAULT_ZOME_NAME)
    }

    /** -- Fields -- */
    @state() private _initialized = false;

    /** After first render only */
    async firstUpdated() {
        //await this.refresh();
        this._initialized = true;
    }


    /** */
    render() {
        console.log("<delivery-dashboard> render()", this._initialized);
        if (!this._initialized) {
            return html`<span>Loading...</span>`;
        }

        /* Li */
        console.log("myDistributions", this.perspective.myDistributions);
        const distribsLi = Object.entries(this.perspective.myDistributions).map(
            ([distribEh, fullState]) => {
                //console.log("MembraneLi", MembraneLi)
              const deliveryLi = Object.entries(fullState[1]).map(
                  ([agent, deliveryState]) => {
                      return html `
                      <li style="margin-top:10px;" title=${agent}>
                          ${JSON.stringify(deliveryState)} <b>; Recipient:</b>${agent}
                          <button type="button" @click=${() => {this._zvm.getDeliveryState(distribEh, agent)}}>refresh</button>
                      </li>`
                  }
              );

              /** */
              return html `
              <li style="margin-top:10px;" title=${distribEh}>
                  <b>${distribEh}</b>: ${JSON.stringify(fullState[0])}
                  <button type="button" @click=${() => {this._zvm.zomeProxy.getDistributionState(decodeHashFromBase64(distribEh))}}>refresh</button>
                  <ul>
                      ${deliveryLi}
                  </ul>
              </li>`
            }
        )


        /* Li */
        //console.log("signals", this.perspective.myDistributions);
        const newNoticesLi =
        Object.entries(this.perspective.newDeliveryNotices).map(
            ([noticeEh, notice]) => {
                const distribEh = encodeHashToBase64(notice.distributionEh);
                //console.log("MembraneLi", MembraneLi)
                return html `
              <li style="margin-top:10px;" title=${noticeEh}>
                  <b>${distribEh}</b> from ${encodeHashToBase64(notice.sender)}
                  <button type="button" @click=${() => {this._zvm.zomeProxy.getNoticeState(decodeHashFromBase64(noticeEh))}}>refresh</button>
                  <button type="button" @click=${() => {this._zvm.acceptDelivery(noticeEh)}}>Accept</button>
                  <button type="button" @click=${() => {this._zvm.declineDelivery(noticeEh)}}>Decline</button>

              </li>`
            }
        )

        const unansweredNoticesLi = html``;
        //     Object.entries(this.perspective.incomingDistributions).map(
        //         ([distribEh, state]) => {
        //             if (state !== ) {
        //                 continue;
        //             }
        //             //console.log("MembraneLi", MembraneLi)
        //             return html `
        //       <li style="margin-top:10px;" title=${distribEh}>
        //           <b>${distribEh}</b> from ${encodeHashToBase64(notice.sender)}
        //           <button type="button" @click=${() => {this._zvm.zomeProxy.getNoticeState(decodeHashFromBase64(noticeEh))}}>refresh</button>
        //           <button type="button" @click=${() => {this._zvm.acceptDelivery(noticeEh)}}>Accept</button>
        //           <button type="button" @click=${() => {this._zvm.declineDelivery(noticeEh)}}>Decline</button>
        //
        //       </li>`
        //         }
        //     )


        /** render all */
        return html`
          <div>
            <h1>Delivery Dashboard</h1>
            <h2>Outgoing Deliveries</h2>
            <ul>${distribsLi}</ul>
            <hr />
            <h2>Incoming Deliveries</h2>
            <h3>New Delivery Notices</h3>
            <ul>${newNoticesLi}</ul>
            <h3>Unanswered notices</h3>
            <ul>${unansweredNoticesLi}</ul>
          </div>
        `;
    }
}