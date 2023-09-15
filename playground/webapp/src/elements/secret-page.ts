import {css, html} from "lit";
import {property, state, customElement} from "lit/decorators.js";
import { DnaElement } from "@ddd-qc/lit-happ";
import { SecretDvm } from "../viewModels/secret.dvm";
import {AgentPubKeyB64, encodeHashToBase64, EntryHashB64} from "@holochain/client";
import {SecretPerspective} from "../viewModels/secret.zvm";


/**
 * @element
 */
@customElement("secret-page")
export class SecretPage extends DnaElement<unknown, SecretDvm> {

  constructor() {
    super(SecretDvm.DEFAULT_BASE_ROLE_NAME)
  }

  /** -- Fields -- */
  @state() private _initialized = false;
  @state() private _sender?: AgentPubKeyB64;
  @state() private _selectedSecretEh?: EntryHashB64;
  @state() private _senderSecrets: EntryHashB64[] = [];


  @property({ type: Boolean, attribute: 'debug' })
  debugMode: boolean = false;


  @property({type: Object, attribute: false, hasChanged: (_v, _old) => true})
  secretPerspective!: SecretPerspective;

  /** -- Methods -- */

  /** */
  protected async dvmUpdated(newDvm: SecretDvm, oldDvm?: SecretDvm): Promise<void> {
    console.log("<secret-page>.dvmUpdated()");
    if (oldDvm) {
      console.log("\t Unsubscribed to secretZvm's roleName = ", oldDvm.secretZvm.cell.name)
      oldDvm.secretZvm.unsubscribe(this);
    }
    newDvm.secretZvm.subscribe(this, 'secretPerspective');
    console.log("\t Subscribed secretZvm's roleName = ", newDvm.secretZvm.cell.name)
    newDvm.probeAll();
    this._sender = undefined;
    //this.taskerPerspective = emptyTaskerPerspective;
    this._initialized = true;
  }



  // /** After first render only */
  // async firstUpdated() {
  //   this._initialized = true;
  // }


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
    let res = await this._dvm.secretZvm.sendSecretToOne(textInput.value, agentSelect.value, canSplitChk.checked);
    console.log("onSendSecret() res:", res);
    textInput.value = "";
  }


  // /** */
  // async onCreateTask(e: any) {
  //   //console.log("onCreateTask() CALLED", e)
  //   if (!this._selectedListEh) {
  //     return;
  //   }
  //   /* Assignee */
  //   const assigneeSelect = this.shadowRoot!.getElementById("selectedAgent") as HTMLSelectElement;
  //   const assignee = assigneeSelect.value;
  //   //console.log("Assignee value:", assignee);
  //   /* Title */
  //   const input = this.shadowRoot!.getElementById("itemTitleInput") as HTMLInputElement;
  //   //console.log(input)
  //   let res = this._dvm.taskerZvm.createTaskItem(input.value, assignee, this._selectedListEh!);
  //   //console.log("onCreateList res:", res)
  //   input.value = "";
  // }




  /** */
  async onSenderSelected(e: any) {
    console.log("onSenderSelected() CALLED", e)
    const selector = this.shadowRoot!.getElementById("senderSelector") as HTMLSelectElement;
    if (!selector || !selector.value) {
      console.warn("No list selector value", selector);
      return;
    }
    console.log("onSenderSelected() value", selector.value)
    this._senderSecrets = await this._dvm.secretZvm.getSecretsFrom(selector.value);
    //this._senderSecrets = this._dvm.secretZvm.perspective.secretsByAgent[selector.value];
    this._sender = selector.value;
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
    this._selectedSecretEh = selector.value;
  }



  /** */
  render() {
    console.log("<secret-page.render()> render()", this._initialized, this._sender, this.secretPerspective);
    if (!this._initialized) {
      return html`<span>Loading...</span>`;
    }

    //const secrets = this._dvm.secretZvm.secrets;
    let agents: AgentPubKeyB64[] = this._dvm.AgentDirectoryZvm.perspective.agents;

    const AgentOptions = Object.entries(agents).map(
      ([_index, agentIdB64]) => {
        //console.log("" + index + ". " + agentIdB64)
        return html `<option value="${agentIdB64}">${agentIdB64.slice(-5)}</option>`
      }
    )

    const secretOptions = Object.values(this._senderSecrets).map(
      (secretEh) => {
        //console.log("" + index + ". " + agentIdB64)
        return html `<option value="${secretEh}">${secretEh.slice(-5)}</option>`
      }
    )


    // const ppLi = Object.values(this._dvm.deliveryZvm.perspective.publicParcels).map(
    //   (pd) => {
    //     return html`<li value="${pd.reference.eh}">${encodeHashToBase64(pd.reference.eh)} (${pd.size} octets)</li>`
    //   }
    // )

    const ppLi = Object.values(this._dvm.perspective.publicMessages).map(
      (msg) => {
        //console.log("" + index + ". " + agentIdB64)
        return html`<li>${msg}</li>`
      }
    )


    /** render all */
    return html`
      <div>
        <h1>Playground: secret 
            <button type="button" @click=${() => {this._dvm.dumpLogs();}}>dump</button>
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
            ${this._selectedSecretEh? this.secretPerspective.secrets[this._selectedSecretEh] : "n/a"}
          </div>
        </h2>
        <hr/>
        <h2>Public messages</h2>
        <ul>
            ${ppLi}
        </ul>  
      </div>
    `;
  }

}
