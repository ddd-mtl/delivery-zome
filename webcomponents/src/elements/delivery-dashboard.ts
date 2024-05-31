import {css, html} from "lit";
import {property, state, customElement} from "lit/decorators.js";
import { ZomeElement } from "@ddd-qc/lit-happ";
import {DeliveryZvm} from "../viewModels/delivery.zvm";
import {ActionHashB64, decodeHashFromBase64, encodeHashToBase64, EntryHashB64} from "@holochain/client";
import {DeliveryPerspective} from "../viewModels/delivery.perspective";


/**
 *
 */
@customElement("delivery-dashboard")
export class DeliveryDashboard extends ZomeElement<DeliveryPerspective, DeliveryZvm>  {

    constructor() {
        super(DeliveryZvm.DEFAULT_ZOME_NAME)
    }

    @state() private _initialized = false;


    /** After first render only */
    async firstUpdated() {
        //await this.refresh();
        this._initialized = true;
    }


    /** */
    downloadTextFile(filename: string, content: string): void {
      const blob = new Blob([content], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = filename;
      link.click();
      URL.revokeObjectURL(url);
    }


    /** */
    render() {
        console.log("<delivery-dashboard> render()", this._initialized, this.perspective);
        if (!this._initialized) {
            return html`<span>Loading...</span>`;
        }

        /* Li */
        console.log("distributions", this.perspective.distributions);
        const myDistribsLi = Object.entries(this.perspective.distributions).map(
            ([distribAh, fullState]) => {
                //console.log("MembraneLi", MembraneLi)
              const deliveryLi = Object.entries(fullState[1]).map(
                  ([agent, deliveryState]) => {
                      return html `
                      <li style="margin-top:10px;" title=${agent}>
                          ${JSON.stringify(deliveryState)}<b> Recipient: </b> ${agent.slice(-5)}
                          <button type="button" @click=${() => {this._zvm.getDeliveryState(distribAh, agent)}}>refresh</button>
                      </li>`
                  }
              );

              /** */
              return html `
              <li style="margin-top:10px;" title=${distribAh}>
                  <b>${this.distrib2str(distribAh)}</b>: ${JSON.stringify(fullState[0])}
                  <button type="button" @click=${() => {this._zvm.zomeProxy.getDistributionState(decodeHashFromBase64(distribAh))}}>refresh</button>
                  <ul>
                      ${deliveryLi}
                  </ul>
              </li>`
            }
        )

        const distribsLi = Object.entries(this.perspective.distributions).map(
            ([distribAh, _pair]) => {
                return html `
          <li style="margin-top:10px;" title=${distribAh}>
              ${this.distrib2str(distribAh)}
          </li>`
            }
        )

        const receivedNoticesLi = Object.entries(this.perspective.noticeAcks).map(
            ([distribAh, receiveds]) => {
                return Object.entries(receiveds).map(
                  ([recipientKey, [_received, _ts]]) => {
                    return html `
                      <li style="margin-top:10px;" title=${distribAh}>
                          ${this.distrib2str(distribAh)}: ${recipientKey.slice(-5)}
                      </li>`;
                });
            });

        const receivedRepliesLi = Object.entries(this.perspective.replyAcks).map(
            ([distribAh, receiveds]) => {
              return Object.entries(receiveds).map(
                ([recipientKey, [received, _ts]]) => {
                  return html `
                    <li style="margin-top:10px;" title=${distribAh}>
                        ${this.distrib2str(distribAh)}: ${recipientKey.slice(-5)} ${received.has_accepted? "accepted" : "declined"}
                    </li>`
                });
            });


        const receptionAcksLi = Object.entries(this.perspective.receptionAcks).map(
            ([distribAh, receiveds]) => {
              return Object.entries(receiveds).map(
                ([recipientKey, [_receptionAck, _ts]]) => {
                  return html `
                    <li style="margin-top:10px;" title=${distribAh}>
                        ${this.distrib2str(distribAh)}: ${recipientKey.slice(-5)}
                    </li>`
          });
        });


        /* Li */
        const [unreplieds, _] = this._zvm.inbounds();
        console.log("unreplieds", unreplieds);
        const newNoticesLi = Object.entries(unreplieds).map(
            ([noticeEh, [_notice, _ts]]) => {
                let content = html`<button type="button" @click=${() => {this._zvm.zomeProxy.getNoticeState(decodeHashFromBase64(noticeEh))}}>refresh</button>
                    <button type="button" @click=${() => {this._zvm.acceptDelivery(noticeEh)}}>Accept</button>
                    <button type="button" @click=${() => {this._zvm.declineDelivery(noticeEh)}}>Decline</button>`;
                return html `
              <li style="margin-top:10px;" title=${noticeEh}>
                  ${this.notice2str(noticeEh)}
                  ${content}
              </li>`
            }
        )

        const noticesLi = Object.entries(this.perspective.notices).map(
            ([eh, _pair]) => {
                return html`
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

        const receptionsLi = Object.entries(this.perspective.receptions).map(
            ([eh, [receptionProof, _ts]]) => {
                return html `
          <li style="margin-top:10px;" title=${eh}>
              ${this.notice2str(encodeHashToBase64(receptionProof.notice_eh))}: ${encodeHashToBase64(receptionProof.parcel_eh).slice(-5)}
          </li>`
            }
        )

        const manifestsLi = Object.entries(this.perspective.localManifestByData).map(
            ([dataHash, [manifestEh, _isPrivate]]) => {
                if (!this.perspective.privateManifests[manifestEh]) {
                  return html`__privateManifest__`; // TODO
                }
                const [manifest, _ts] = this.perspective.privateManifests[manifestEh];
                return html `
          <li style="margin-top:10px;" title=${dataHash}>
              ${dataHash} | ${manifest.chunks.length} chunks
          </li>`
            }
        )

        /** render all */
        return html`
        <div>
            <h1 style="text-decoration:underline;">Delivery Dashboard</h1>
            <button @click=${async (_e: any) => {
              await this._zvm.getAllPublicManifest();
              const json = this._zvm.exportPerspective();
              this.downloadTextFile("dump.json", json);
            }}>Export</button>
            <h2>Local Parcels</h2>
            <ul>${manifestsLi}</ul>
            
            <hr />
            <h2>INBOUND</h2>

            <h3>Delivery Notices</h3>
            <ul>${noticesLi}</ul>

            <h3>Replies</h3>
            <ul>${repliesLi}</ul>

            <h3>Receptions</h3>
            <ul>${receptionsLi}</ul>
            
            <hr style="border: dotted 1px grey"/>
            
            <h3>Unreplied</h3>
            <ul>${newNoticesLi}</ul>
            <hr />
            
            <h2>OUTBOUND</h2>

            <h3>Distributions</h3>
            <ul>${distribsLi}</ul>

            <h3>NoticeAcks</h3>
            <ul>${receivedNoticesLi}</ul>

            <h3>ReplyAcks</h3>
            <ul>${receivedRepliesLi}</ul>

            <h3>ReceptionAcks</h3>
            <ul>${receptionAcksLi}</ul>
            
            <hr style="border: dotted 1px grey"/>

            <h3>Distributions</h3>
            <ul>${myDistribsLi}</ul>
            
              
        </div>
        `;
    }

    /** -- Utils -- */

    /** */
    distrib2str(distribAh: ActionHashB64): string {
        const tuple = this.perspective.distributions[distribAh];
        if (!tuple) {
            return "unknown";
        }
        const date = new Date(tuple[1] / 1000); // Holochain timestamp is in micro-seconds, Date wants milliseconds
        const date_str = date.toLocaleString('en-US', {hour12: false});
        const agent_str = encodeHashToBase64(tuple[0].recipients[0]).slice(-5);
        return "[" + date_str + "] to " + agent_str + " (" + tuple[0].recipients.length + ")";
    }


    /** */
    notice2str(noticeEh: EntryHashB64): string {
        const tuple = this.perspective.notices[noticeEh];
        if (!tuple) {
            return "unknown";
        }
        const date = new Date(tuple[1] / 1000); // Holochain timestamp is in micro-seconds, Date wants milliseconds
        const date_str = date.toLocaleString('en-US', {hour12: false});
        const agent_str = encodeHashToBase64(tuple[0].sender).slice(-5);
        return "[" + date_str + "] from " + agent_str;
    }
}
