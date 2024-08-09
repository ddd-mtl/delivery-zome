import { html } from "lit";
import {state, customElement} from "lit/decorators.js";
import { SecretDvm } from "./viewModels/secret.dvm";
import {HvmDef, HappElement, Cell} from "@ddd-qc/lit-happ";
import {DnaDefinition} from "@holochain/client";
import {Dictionary, EntryDef} from "@ddd-qc/cell-proxy";


/**
 *
 */
@customElement("secret-app")
export class SecretApp extends HappElement {

  /** Ctor */
  constructor() {
    const adminUrl = process.env.ADMIN_PORT? new URL(`ws://localhost:${process.env.ADMIN_PORT}`) : undefined;
    console.log("SecretApp.ctor()", adminUrl);
    super(Number(process.env.HC_PORT), undefined, adminUrl);
  }


  /** HvmDef */
  static override readonly HVM_DEF: HvmDef = {
    id: "hSecret",
    dvmDefs: [{ctor: SecretDvm, isClonable: true}],
  };

  /** QoL */
  get secret(): SecretDvm { return this.hvm.getDvm(SecretDvm.DEFAULT_BASE_ROLE_NAME)! as SecretDvm }

  /** -- Fields -- */

  @state() private _loaded = false;

  private _pageDisplayIndex: number = 0;
  /** ZomeName -> (AppEntryDefName, isPublic) */
  private _allAppEntryTypes: Dictionary<Dictionary<EntryDef>> = {};


  @state() private _cell?: Cell;


  private _dnaDef?: DnaDefinition;

  /** */
  override async hvmConstructed() {
    console.log("hvmConstructed()")
    /** Probe */
    this._cell = this.secret.cell;
    //await this.hvm.probeAll();
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
  override render() {
    console.log("*** <secret-app> render()", this._loaded, this.secret.secretZvm.perspective)
    if (!this._loaded) {
      return html`<span>Loading...</span>`;
    }
    //let knownAgents: AgentId[] = this.secret.agentDirectoryZvm.perspective.agents;
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
        <span><abbr title=${this.secret.cell.address.agentId.b64}>Agent</abbr>: <b>${this.secret.cell.address.agentId.short}</b></span>
        <!--<dvm-inspect .dnaViewModel=${this.secret}></dvm-inspect> -->          
        <hr class="solid">      
        ${page}
      </cell-context>        
    `
  }

}
