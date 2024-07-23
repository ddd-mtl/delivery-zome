import {css, html} from "lit";
import {property, state, customElement} from "lit/decorators.js";
import { DnaElement, AgentId, EntryId } from "@ddd-qc/lit-happ";
import {SecretDvm, SecretDvmPerspective} from "../viewModels/secret.dvm";
import {SecretPerspective} from "../viewModels/secret.zvm";
import {DeliveryPerspective} from "@ddd-qc/delivery";


/**
 * @element
 */
@customElement("secret-page")
export class SecretPage extends DnaElement<SecretDvmPerspective, SecretDvm> {

  constructor() {
    super(SecretDvm.DEFAULT_BASE_ROLE_NAME)
  }

  /** -- Fields -- */
  @state() private _initialized = false;
  @state() private _sender?: AgentId;
  @state() private _selectedSecretEh?: EntryId;
  @state() private _senderSecrets: EntryId[] = [];


  @property({ type: Boolean, attribute: 'debug' })
  debugMode: boolean = false;


  @property({type: Object, attribute: false, hasChanged: (_v, _old) => true})
  secretPerspective!: SecretPerspective;

  @property({type: Object, attribute: false, hasChanged: (_v, _old) => true})
  deliveryPerspective!: DeliveryPerspective;

  /** -- Methods -- */

  /** */
  protected async dvmUpdated(newDvm: SecretDvm, oldDvm?: SecretDvm): Promise<void> {
    console.log("<secret-page>.dvmUpdated()");
    if (oldDvm) {
      console.log("\t Unsubscribed to secretZvm's roleName = ", oldDvm.secretZvm.cell.name)
      oldDvm.secretZvm.unsubscribe(this);
      oldDvm.deliveryZvm.unsubscribe(this);
    }
    newDvm.secretZvm.subscribe(this, 'secretPerspective');
    newDvm.deliveryZvm.subscribe(this, 'deliveryPerspective');
    console.log("\t Subscribed secretZvm's roleName = ", newDvm.secretZvm.cell.name)
    newDvm.probeAll();
    this._sender = undefined;
    this._initialized = true;
  }


  /** */
  async refresh(_e?: any) {
    //console.log("tasker-page.refresh() called")
    await this._dvm.probeAll();
    await this._dvm.probePublicMessages();
  }


  /** */
  async onPublishMessage(e: any) {
    const textInput = this.shadowRoot!.getElementById("messageInput") as HTMLInputElement;
    if (textInput.value.length == 0) {
      alert("message string is empty");
      return;
    }
    let res = await this._dvm.publishMessage(textInput.value);
    console.log("onPublishMessage() res:", res);
    /** */
    textInput.value = "";
  }

  /** */
  async onSendSecret(e: any) {
    const textInput = this.shadowRoot!.getElementById("secretInput") as HTMLInputElement;
    const agentSelect = this.shadowRoot!.getElementById("recipientSelector") as HTMLSelectElement;
    const canSplitChk = this.shadowRoot!.getElementById("splitChk") as HTMLInputElement;
    if (textInput.value.length == 0) {
      alert("secret string is empty");
      return;
    }
    let _res = await this._dvm.secretZvm.sendSecretToOne(textInput.value, new AgentId(agentSelect.value), canSplitChk.checked);
    textInput.value = "";
  }


  /** */
  async onSenderSelected(e: any) {
    console.log("onSenderSelected() CALLED", e)
    const selector = this.shadowRoot!.getElementById("senderSelector") as HTMLSelectElement;
    if (!selector || !selector.value) {
      console.warn("No list selector value", selector);
      return;
    }
    console.log("onSenderSelected() value", selector.value);
    this._sender = new AgentId(selector.value);
    this._senderSecrets = await this._dvm.secretZvm.getSecretsFrom(this._sender);
  }


  /** */
  async onSecretSelected(e: any) {
    console.log("onSecretSelected() CALLED", e)
    const selector = this.shadowRoot!.getElementById("secretSelector") as HTMLSelectElement;
    if (!selector || !selector.value) {
      console.warn("No list selector value", selector);
      return;
    }
    console.log("onSenderSelected() value", selector.value)
    this._selectedSecretEh = new EntryId(selector.value);
  }



  /** */
  render() {
    console.log("<secret-page> render()", this._initialized, this._sender, this.secretPerspective, this._dvm.perspective, this._dvm.deliveryZvm.perspective);
    if (!this._initialized) {
      return html`<span>Loading...</span>`;
    }

    //const secrets = this._dvm.secretZvm.secrets;
    let agents: AgentId[] = this._dvm.agentDirectoryZvm.perspective.agents;

    const AgentOptions = agents.map(
      (agentId) => {
        //console.log("" + index + ". " + agentIdB64)
        return html `<option value=${agentId.b64}>${agentId.short}</option>`
      }
    )

    const secretOptions = Object.values(this._senderSecrets).map(
      (secretEh) => {
        //console.log("" + index + ". " + agentIdB64)
        return html `<option value=${secretEh.b64}>${secretEh.short}</option>`
      }
    )


    // const ppLi = Object.values(this.deliveryPerspective.publicParcels).map(
    //   (pprm) => {
    //     //return html`<li value="${pd.reference.eh}">${encodeHashToBase64(pd.reference.eh)} (${pd.size} octets)</li>`
    //     //console.log("" + index + ". " + agentIdB64)
    //     if (pprm.deleteInfo) {
    //       return html``;
    //     }
    //     const prEh = decodeHashFromBase64(pprm.prEh);
    //     return html`<li>${msg} <button style="margin-left:20px" @click=${async (e:any) => {
    //       const _res = await this._dvm.deliveryZvm.zomeProxy.unpublishPublicParcel(prEh);
    //     }}>Remove</button></li>`
    //   }
    // )

    const ppLi = Array.from(this._dvm.perspective.publicMessages.entries()).map(
      ([parcelEh, msg]) => {
        //console.log("" + index + ". " + agentIdB64)
        const pprm = this.deliveryPerspective.publicParcels.get(parcelEh);
        if (pprm.deleteInfo) {
          return html``;
        }
        return html`<li>${msg} <button style="margin-left:20px" @click=${async (e:any) => {
          const _res = await this._dvm.deliveryZvm.zomeProxy.unpublishPublicParcel(pprm.prEh.hash);
        }}>Remove</button></li>`
      }
    )

    const remLi = Array.from(this.deliveryPerspective.publicParcels.entries()).map(
      ([_parcelEh, pprm]) => {
        //console.log("remLi:", pprm.deleteInfo);
        if (!pprm.deleteInfo) {
          return html``;
        }
        //const prEh = decodeHashFromBase64(this._dvm.deliveryZvm.perspective.publicParcels[parcelEh][0]);
        const msg = `[${pprm.deleteInfo[0]}] ${pprm.description.name} | by ${pprm.deleteInfo[1].b64}`
        return html`<li>${msg}</li>`
      }
    )

    /** render all */
    return html`
      <div>
        <h1>Playground: secret 
            <button type="button" @click=${() => {this._dvm.dumpCallLogs(); this._dvm.dumpSignalLogs("zDelivery");}}>dump</button>
            <button type="button" @click=${this.refresh}>Refresh</button>
        </h1>
        <div>
          <label>Publish message:</label>
          <input type="text" id="messageInput" name="message">
          <input type="button" value="publish" @click=${this.onPublishMessage}>
        </div>
        <div style="margin-top:15px;">
          <label>Send secret:</label>
          <input type="text" id="secretInput" name="content">
          to: <select id="recipientSelector">
            ${AgentOptions}
          </select>
          <input type="button" value="send" @click=${this.onSendSecret}>
            split:<input type="checkbox" id="splitChk"">
        </div>              
        <h2>
          Secrets received:
          From: <select id="senderSelector" @click=${this.onSenderSelected}>
            ${AgentOptions}
          </select>
          <select id="secretSelector" @click=${this.onSecretSelected}>
            ${secretOptions}
          </select>
          <div style="margin-top:15px;">
            ${this._selectedSecretEh? this.secretPerspective.secrets.get(this._selectedSecretEh) : "n/a"}
          </div>
        </h2>
        <hr/>
        <h2>Public messages</h2>
        <ul>${ppLi}</ul>
        <h2>Removed Public messages</h2>
        <ul>${remLi}</ul>
      </div>
    `;
  }

}
