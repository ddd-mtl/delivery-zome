import { html } from "lit";
import {property, state} from "lit/decorators.js";
import {SecretPage} from "./elements/secret-page";
import {AgentDirectoryList} from "@ddd-qc/agent-directory";
import { SecretDvm } from "./viewModels/secret.dvm";
import {
  HvmDef, HappElement, HCL, ViewCellContext, CellDef, CellContext, delay, Cell
} from "@ddd-qc/lit-happ";
import {AdminWebsocket, AgentPubKeyB64, DnaDefinition, RoleName} from "@holochain/client";


/**
 *
 */
export class SecretApp extends HappElement {

  /** Ctor */
  constructor() {
    super(Number(process.env.HC_PORT));
  }

  /** HvmDef */
  static readonly HVM_DEF: HvmDef = {
    id: "hSecret",
    dvmDefs: [{ctor: SecretDvm, isClonable: true}],
  };

  /** QoL */
  get secret(): SecretDvm { return this.hvm.getDvm(SecretDvm.DEFAULT_BASE_ROLE_NAME)! as SecretDvm }

  /** -- Fields -- */

  @state() private _loaded = false;

  private _pageDisplayIndex: number = 0;
  /** ZomeName -> (AppEntryDefName, isPublic) */
  private _allAppEntryTypes: Record<string, [string, boolean][]> = {};


  @state() private _cell?: Cell;


  private _dnaDef?: DnaDefinition;

  /** */
  async hvmConstructed() {
    console.log("hvmConstructed()")
    //new ContextProvider(this, cellContext, this.taskerDvm.cell);
    /** Authorize all zome calls */
    const adminWs = await AdminWebsocket.connect(`ws://localhost:${process.env.ADMIN_PORT}`);
    console.log({adminWs});
    await this.hvm.authorizeAllZomeCalls(adminWs);
    console.log("*** Zome call authorization complete");
    this._dnaDef = await adminWs.getDnaDefinition(this.secret.cell.id[0]);
    console.log("happInitialized() dnaDef", this._dnaDef);
    /** Probe */    
    this._cell = this.secret.cell;
    await this.hvm.probeAll();
    this._allAppEntryTypes = await this.secret.fetchAllEntryDefs();
    console.log("happInitialized(), _allAppEntryTypes", this._allAppEntryTypes);
    // TODO: Fix issue: zTasker entry_defs() not found. Maybe confusion with integrity zome name?
    /** Done */
    this._loaded = true;
  }


  /** */
  async refresh(_e?: any) {
    console.log("secret-app.refresh() called")
    await this.hvm.probeAll();
  }



  /** */
  render() {
    console.log("*** <secret-app> render()", this._loaded, this.secret.secretZvm.perspective)
    if (!this._loaded) {
      return html`<span>Loading...</span>`;
    }
    let knownAgents: AgentPubKeyB64[] = this.secret.AgentDirectoryZvm.perspective.agents;
    //console.log({coordinator_zomes: this._dnaDef?.coordinator_zomes})
    const zomeNames = this._dnaDef?.coordinator_zomes.map((zome) => { return zome[0]; });
    console.log({zomeNames})
    let page;
    switch (this._pageDisplayIndex) {
      case 0: page = html`<secret-page style="flex: 1;"></secret-page>` ; break;
      case 1: page = html`<delivery-dashboard style="flex: 1;"></delivery-dashboard>`; break;
      case 2: page = html`<agent-directory-list style="flex: 1;"></agent-directory-list>`; break;

      default: page = html`unknown page index`;
    };

    /* render all */
    return html`
      <cell-context .cell="${this._cell}">
        <div>
          <view-cell-context></view-cell-context>
          <input type="button" value="Secret" @click=${() => {this._pageDisplayIndex = 0; this.requestUpdate()}} >
          <input type="button" value="Delivery" @click=${() => {this._pageDisplayIndex = 1; this.requestUpdate()}} >          
          <input type="button" value="Agent Directory" @click=${() => {this._pageDisplayIndex = 2; this.requestUpdate()}} >
        </div>
        <button type="button" @click=${this.refresh}>Refresh</button>
        <span><abbr title=${this.secret.cell.agentPubKey}>Agent</abbr>: <b>${this.secret.cell.agentPubKey.slice(-5)}</b></span>
        <hr class="solid">      
        ${page}
      </cell-context>        
    `
  }

}
