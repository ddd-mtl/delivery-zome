import {css, html} from "lit";
import {property, state, customElement} from "lit/decorators.js";
import { DnaElement } from "@ddd-qc/lit-happ";
import { SecretDvm } from "../viewModels/secret.dvm";
import {AgentPubKeyB64, encodeHashToBase64, EntryHashB64} from "@holochain/client";


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
  @state() private _selectedListEh?: EntryHashB64;

  @property({ type: Boolean, attribute: 'debug' })
  debugMode: boolean = false;


  @property({type: Object, attribute: false, hasChanged: (_v, _old) => true})
  secretPerspective!: unknown;

  /** -- Methods -- */

  protected async dvmUpdated(newDvm: SecretDvm, oldDvm?: SecretDvm): Promise<void> {
    console.log("<secret-page>.dvmUpdated()");
    if (oldDvm) {
      console.log("\t Unsubscribed to secretZvm's roleName = ", oldDvm.secretZvm.cell.name)
      oldDvm.secretZvm.unsubscribe(this);
    }
    newDvm.secretZvm.subscribe(this, 'secretPerspective');
    console.log("\t Subscribed secretZvm's roleName = ", newDvm.secretZvm.cell.name)
    newDvm.probeAll();
    this._selectedListEh = undefined;
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
  }


  /** */
  async onSendSecret(e: any) {
    const textInput = this.shadowRoot!.getElementById("secretInput") as HTMLInputElement;
    const agentSelect = this.shadowRoot!.getElementById("selectedAgent") as HTMLSelectElement;
    let res = await this._dvm.secretZvm.sendSecretToOne(textInput.value, agentSelect.value);
    console.log("onSendSecret() res:", res)
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
  async onListSelect(e: any) {
    console.log("onListSelect() CALLED", e)
    const selector = this.shadowRoot!.getElementById("listSelector") as HTMLSelectElement;
    if (!selector || !selector.value) {
      console.warn("No list selector value", selector);
      return;
    }
    console.log("onListSelect() value", selector.value)
    this._selectedListEh = selector.value;
    this.requestUpdate();
  }


  // /** */
  // async onSubmitCompletion(selectedList: null) {
  //   //console.log("onSubmitCompletion() CALLED", e)
  //   if (!selectedList) {
  //     return;
  //   }
  //   for (const [ehb64, taskItem] of selectedList.items) {
  //     const checkbox = this.shadowRoot!.getElementById(ehb64) as HTMLInputElement;
  //     //console.log("" + checkbox.checked + ". checkbox " + ehb64)
  //     if (checkbox.checked) {
  //       await this._dvm.taskerZvm.completeTask(ehb64)
  //     }
  //   }
  //
  //   this._dvm.taskerZvm.probeAll();
  //   //this.requestUpdate();
  // }



  /** */
  render() {
    console.log("<secret-page.render()> render()", this._initialized, this._selectedListEh);
    if (!this._initialized) {
      return html`<span>Loading...</span>`;
    }

    //const secrets = this._dvm.secretZvm.secrets;
    let agents: AgentPubKeyB64[] = this._dvm.AgentDirectoryZvm.perspective.agents;

    const AgentOptions = Object.entries(agents).map(
        ([index, agentIdB64]) => {
          //console.log("" + index + ". " + agentIdB64)
          return html `<option value="${agentIdB64}">${agentIdB64.substring(0, 12)}</option>`
        }
    )

    return html`
      <div>
        <h1>Playground: secret</h1>
          <label for="listTitleInput">Send secret:</label>
          <input type="text" id="secretInput" name="content">
          <select name="selectedAgent" id="selectedAgent">
            ${AgentOptions}
          </select>        
          <input type="button" value="create" @click=${this.onSendSecret}>
        <h2>
          Received secrets:
          <select name="listSelector" id="listSelector" @click=${this.onListSelect}>
          </select>
        </h2>        
      </div>
    `;
  }

}
