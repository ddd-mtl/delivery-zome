import {css, html} from "lit";
import {property, state, customElement} from "lit/decorators.js";
import { ZomeElement } from "@ddd-qc/lit-happ";
import {DeliveryPerspective, DeliveryZvm} from "../viewModels/delivery.zvm";
import {decodeHashFromBase64, encodeHashToBase64, EntryHashB64} from "@holochain/client";


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
        const myDistribsLi = Object.entries(this.perspective.myDistributions).map(
            ([distribEh, fullState]) => {
                //console.log("MembraneLi", MembraneLi)
              const deliveryLi = Object.entries(fullState[1]).map(
                  ([agent, deliveryState]) => {
                      return html `
                      <li style="margin-top:10px;" title=${agent}>
                          ${JSON.stringify(deliveryState)}<b> Recipient: </b> ${agent.slice(-5)}
                          <button type="button" @click=${() => {this._zvm.getDeliveryState(distribEh, agent)}}>refresh</button>
                      </li>`
                  }
              );

              /** */
              return html `
              <li style="margin-top:10px;" title=${distribEh}>
                  <b>${this.distrib2str(distribEh)}</b>: ${JSON.stringify(fullState[0])}
                  <button type="button" @click=${() => {this._zvm.zomeProxy.getDistributionState(decodeHashFromBase64(distribEh))}}>refresh</button>
                  <ul>
                      ${deliveryLi}
                  </ul>
              </li>`
            }
        )

        const distribsLi = Object.entries(this.perspective.distributions).map(
            ([eh, _pair]) => {
                return html `
          <li style="margin-top:10px;" title=${eh}>
              ${this.distrib2str(eh)}
          </li>`
            }
        )

        const receivedNoticesLi = Object.entries(this.perspective.noticeAcks).map(
            ([eh, received]) => {
                return html `
          <li style="margin-top:10px;" title=${eh}>
              ${this.distrib2str(encodeHashToBase64(received.distribution_eh))}: ${encodeHashToBase64(received.recipient).slice(-5)}
          </li>`
            }
        )

        const receivedRepliesLi = Object.entries(this.perspective.replyAcks).map(
            ([eh, received]) => {
                return html `
          <li style="margin-top:10px;" title=${eh}>
              ${this.distrib2str(encodeHashToBase64(received.distribution_eh))}: ${encodeHashToBase64(received.recipient).slice(-5)} ${received.has_accepted? "accepted" : "declined"}
          </li>`
            }
        )


        const receiptsLi = Object.entries(this.perspective.receipts).map(
            ([eh, receipt]) => {
                return html `
          <li style="margin-top:10px;" title=${eh}>
              ${this.distrib2str(encodeHashToBase64(receipt.distribution_eh))}: ${encodeHashToBase64(receipt.recipient).slice(-5)}
          </li>`
            }
        )


        /* Li */
        //console.log("signals", this.perspective.myDistributions);
        const newNoticesLi =
        Object.entries(this.perspective.newDeliveryNotices).map(
            ([noticeEh, notice]) => {
                const distribEh = encodeHashToBase64(notice.distribution_eh);
                //console.log("MembraneLi", MembraneLi)
                return html `
              <li style="margin-top:10px;" title=${noticeEh}>
                  ${this.notice2str(noticeEh)}
                  <button type="button" @click=${() => {this._zvm.zomeProxy.getNoticeState(decodeHashFromBase64(noticeEh))}}>refresh</button>
                  <button type="button" @click=${() => {this._zvm.acceptDelivery(noticeEh)}}>Accept</button>
                  <button type="button" @click=${() => {this._zvm.declineDelivery(noticeEh)}}>Decline</button>

              </li>`
            }
        )

        const noticesLi = Object.entries(this.perspective.notices).map(
            ([eh, _pair]) => {
                return html `
          <li style="margin-top:10px;" title=${eh}>
              ${this.notice2str(eh)}
          </li>`
            }
        )


        const repliesLi = Object.entries(this.perspective.replies).map(
            ([eh, reply]) => {
                return html `
          <li style="margin-top:10px;" title=${eh}>
              ${this.notice2str(encodeHashToBase64(reply.notice_eh))}: ${reply.has_accepted? "accepted" : "declined"}
          </li>`
            }
        )

        const receivedLi = Object.entries(this.perspective.parcelAcks).map(
            ([eh, received]) => {
                return html `
          <li style="margin-top:10px;" title=${eh}>
              ${this.notice2str(encodeHashToBase64(received.notice_eh))}: ${encodeHashToBase64(received.parcel_eh).slice(-5)}
          </li>`
            }
        )

        /** render all */
        return html`
        <div>
            <h1 style="text-decoration:underline;">Delivery Dashboard</h1>

            <h2>INBOUND</h2>

            <h3>Delivery Notices</h3>
            <ul>${noticesLi}</ul>

            <h3>Replies</h3>
            <ul>${repliesLi}</ul>

            <h3>Received Parcels</h3>
            <ul>${receivedLi}</ul>
            
            <hr style="border: dotted 1px grey"/>
              
            <h3>New Delivery Notices</h3>
            <ul>${newNoticesLi}</ul>
            <h3>Unanswered notices</h3>

            <hr />
            
            <h2>OUTBOUND</h2>

            <h3>Distributions</h3>
            <ul>${distribsLi}</ul>

            <h3>Received Notices</h3>
            <ul>${receivedNoticesLi}</ul>

            <h3>Received Replies</h3>
            <ul>${receivedRepliesLi}</ul>

            <h3>Receipts</h3>
            <ul>${receiptsLi}</ul>
            
            <hr style="border: dotted 1px grey"/>

            <h3>Distributions</h3>
            <ul>${myDistribsLi}</ul>
            
              
        </div>
        `;
    }

    /** -- Utils -- */

    /** */
    distrib2str(distribEh: EntryHashB64): string {
        const pair = this.perspective.distributions[distribEh];
        if (!pair) {
            return "unknown";
        }
        const date = new Date(pair[0] / 1000); // Holochain timestamp is in micro-seconds, Date wants milliseconds
        const date_str = date.toLocaleString('en-US', {hour12: false});
        const agent_str = encodeHashToBase64(pair[1].recipients[0]).slice(-5);
        return "[" + date_str + "] to " + agent_str + " (" + pair[1].recipients.length + ")";
    }


    /** */
    notice2str(noticeEh: EntryHashB64): string {
        const pair = this.perspective.notices[noticeEh];
        if (!pair) {
            return "unknown";
        }
        const date = new Date(pair[0] / 1000); // Holochain timestamp is in micro-seconds, Date wants milliseconds
        const date_str = date.toLocaleString('en-US', {hour12: false});
        const agent_str = encodeHashToBase64(pair[1].sender).slice(-5);
        return "[" + date_str + "] from " + agent_str;
    }
}