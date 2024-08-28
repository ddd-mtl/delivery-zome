import {html} from "lit";
import {state, customElement} from "lit/decorators.js";
import {ActionId, enc64, EntryId, ZomeElement} from "@ddd-qc/lit-happ";
import {DeliveryZvm} from "../viewModels/delivery.zvm";
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
    override async firstUpdated() {
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
    override render() {
        console.log("<delivery-dashboard> render()", this._initialized, this.perspective);
        if (!this._initialized) {
            return html`<span>Loading...</span>`;
        }

        /* Li */
        console.log("distributions", this.perspective.distributions);
        const myDistribsLi = Array.from(this.perspective.distributions.entries()).map(
            ([distribAh, [distribution, _ts, _distribState, statePerAgent]]) => {
                //console.log("MembraneLi", MembraneLi)
              const deliveryLi = Array.from(statePerAgent.entries()).map(
                  ([agentId, deliveryState]) => {
                      return html `
                      <li style="margin-top:10px;" title=${agentId.b64}>
                          ${JSON.stringify(deliveryState)}<b> Recipient: </b> ${agentId.short}
                          <button type="button" @click=${() => {this._zvm.getDeliveryState(distribAh, agentId)}}>refresh</button>
                      </li>`
                  }
              );

              /** */
              return html `
              <li style="margin-top:10px;" title=${distribAh.b64}>
                  <b>${this.distrib2str(distribAh)}</b>: ${JSON.stringify(distribution)}
                  <button type="button" @click=${() => {this._zvm.zomeProxy.getDistributionState(distribAh.hash)}}>refresh</button>
                  <ul>
                      ${deliveryLi}
                  </ul>
              </li>`
            }
        )

        const distribsLi = Array.from(this.perspective.distributions.entries()).map(
            ([distribAh, _pair]) => {
                return html `
          <li style="margin-top:10px;" title=${distribAh.b64}>
              ${this.distrib2str(distribAh)}
          </li>`
            }
        )

        const receivedNoticesLi = Array.from(this.perspective.noticeAcks.entries()).map(
            ([distribAh, receiveds]) => {
                return Array.from(receiveds.entries()).map(
                  ([recipientKey, [_received, _ts]]) => {
                    return html `
                      <li style="margin-top:10px;" title=${distribAh.b64}>
                          ${this.distrib2str(distribAh)}: ${recipientKey.short}
                      </li>`;
                });
            });

        const receivedRepliesLi = Array.from(this.perspective.replyAcks.entries()).map(
            ([distribAh, receiveds]) => {
              return Array.from(receiveds.entries()).map(
                ([recipientKey, [received, _ts]]) => {
                  return html `
                    <li style="margin-top:10px;" title=${distribAh.b64}>
                        ${this.distrib2str(distribAh)}: ${recipientKey.short} ${received.has_accepted? "accepted" : "declined"}
                    </li>`
                });
            });


        const receptionAcksLi = Array.from(this.perspective.receptionAcks.entries()).map(
            ([distribAh, receiveds]) => {
              return Array.from(receiveds.entries()).map(
                ([recipientKey, [_receptionAck, _ts]]) => {
                  return html `
                    <li style="margin-top:10px;" title=${distribAh.b64}>
                        ${this.distrib2str(distribAh)}: ${recipientKey.short}
                    </li>`
          });
        });


        /* Li */
        const [unreplieds, _] = this._zvm.inbounds();
        console.log("unreplieds", unreplieds);
        const newNoticesLi = Array.from(unreplieds.entries()).map(
            ([noticeEh, [_notice, _ts]]) => {
                let content = html`<button type="button" @click=${() => {this._zvm.zomeProxy.getNoticeState(noticeEh.hash)}}>refresh</button>
                    <button type="button" @click=${() => {this._zvm.acceptDelivery(noticeEh)}}>Accept</button>
                    <button type="button" @click=${() => {this._zvm.declineDelivery(noticeEh)}}>Decline</button>`;
                return html `
              <li style="margin-top:10px;" title=${noticeEh.b64}>
                  ${this.notice2str(noticeEh)}
                  ${content}
              </li>`
            }
        )

        const noticesLi = Array.from(this.perspective.notices.entries()).map(
            ([eh, _pair]) => {
                return html`
          <li style="margin-top:10px;" title=${eh.b64}>
              ${this.notice2str(eh)}
          </li>`
            }
        )


        const repliesLi = Array.from(this.perspective.replies.entries()).map(
            ([eh, reply]) => {
                return html `
          <li style="margin-top:10px;" title=${eh.b64}>
              ${this.notice2str(new EntryId(reply.notice_eh))}: ${reply.has_accepted? "accepted" : "declined"}
          </li>`
            }
        )

        const receptionsLi = Array.from(this.perspective.receptions.entries()).map(
            ([eh, [receptionProof, _ts]]) => {
                return html `
          <li style="margin-top:10px;" title=${eh.b64}>
              ${this.notice2str(new EntryId(receptionProof.notice_eh))}: ${enc64(receptionProof.parcel_eh).slice(-5)}
          </li>`
            }
        )

        const manifestsLi = Object.entries(this.perspective.localManifestByData).map(
            ([dataHash, [manifestEh, _isPrivate]]) => {
                if (!this.perspective.privateManifests.get(manifestEh)) {
                  return html`__privateManifest__`; // TODO
                }
                const [manifest, _ts] = this.perspective.privateManifests.get(manifestEh)!;
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
              await this._zvm.fetchAllPublicManifest();
              const json = this._zvm.export();
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
    distrib2str(distribAh: ActionId): string {
        const tuple = this.perspective.distributions.get(distribAh);
        if (!tuple) {
            return "unknown";
        }
        const date = new Date(tuple[1] / 1000); // Holochain timestamp is in micro-seconds, Date wants milliseconds
        const date_str = date.toLocaleString('en-US', {hour12: false});
        const agent_str = enc64(tuple[0].recipients[0]!).slice(-5);
        return "[" + date_str + "] to " + agent_str + " (" + tuple[0].recipients.length + ")";
    }


    /** */
    notice2str(noticeEh: EntryId): string {
        const tuple = this.perspective.notices.get(noticeEh);
        if (!tuple) {
            return "unknown";
        }
        const date = new Date(tuple[1] / 1000); // Holochain timestamp is in micro-seconds, Date wants milliseconds
        const date_str = date.toLocaleString('en-US', {hour12: false});
        const agent_str = enc64(tuple[0].sender).slice(-5);
        return "[" + date_str + "] from " + agent_str;
    }
}
