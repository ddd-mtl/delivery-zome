import {html} from "lit";
import {customElement, state} from "lit/decorators.js";
import {SecretDvm} from "./viewModels/secret.dvm";
import {
    AppProxy,
    BaseRoleName,
    Cell,
    CloneId,
    DnaViewModel,
    DvmDef,
    EntryId,
    HappElement,
    HCL,
    HvmDef
} from "@ddd-qc/lit-happ";
// @ts-ignore
import {AdminWebsocket, AppWebsocket, DnaDefinition, InstalledAppId, ZomeName} from "@holochain/client";
import {ProfilesDvm} from "@ddd-qc/profiles-dvm";
import {AppletId, AppletView, GroupProfile, WeaveServices} from "@theweave/api";
import {ContextProvider, createContext} from "@lit/context";
import {HC_ADMIN_PORT, HC_APP_PORT} from "./globals";
import {GetStrategy} from "@holochain-open-dev/core-types";

const weClientContext = createContext<WeaveServices>('weave_client');


/**
 *
 */
@customElement("secret-app")
export class SecretApp extends HappElement {

  /** Ctor */
  // constructor() {
  //   const adminUrl = process.env.HC_ADMIN_PORT? new URL(`ws://localhost:${process.env.HC_ADMIN_PORT}`) : undefined;
  //   console.log("SecretApp.ctor()", adminUrl, Number(process.env.HC_APP_PORT));
  //   super(Number(process.env.HC_APP_PORT), undefined, adminUrl);
  // }

  /** All arguments should be provided when constructed explicitly */
  // @ts-ignore
  constructor(appWs?: AppWebsocket, private adminWs?: AdminWebsocket, readonly appId?: InstalledAppId, public _appletView?: AppletView) {
    console.log("PlaygroundApp.ctor()", HC_ADMIN_PORT, HC_APP_PORT, appWs, adminWs, appId);
    /** Figure out arguments for super() */
    const appPort: number = Number(HC_APP_PORT);
    const adminUrl = adminWs
      ? undefined
      : HC_ADMIN_PORT
        ? new URL(`ws://localhost:${HC_ADMIN_PORT}`)
        : undefined;
    super(appWs? appWs : appPort, appId, adminUrl, 10 * 1000);
  }



  /** -- We-applet specifics -- */

  private _weProfilesDvm?: ProfilesDvm;
  protected _weProvider?: unknown; // FIXME type: ContextProvider<this.getContext()> ?

  public appletId?: AppletId;
  public groupProfiles?: GroupProfile[];
  // protected _attachmentsProvider?: unknown;


  /**  */
  static async fromWe(
    appWs: AppWebsocket,
    adminWs: AdminWebsocket | undefined,
    _canAuthorizeZfns: boolean,
    appId: InstalledAppId,
    profilesAppId: InstalledAppId,
    profilesBaseRoleName: BaseRoleName,
    profilesCloneId: CloneId | undefined,
    profilesZomeName: ZomeName,
    profilesProxy: AppProxy,
    weServices: WeaveServices,
    thisAppletHash: EntryId,
    //showEntryOnly?: boolean,
    appletView: AppletView,
    groupProfiles: GroupProfile[],
  ) : Promise<SecretApp> {
    const app = new SecretApp(appWs, adminWs, appId, appletView);
    /** Provide it as context */
    console.log(`\t\tProviding context "${weClientContext}" | in host `, app);
    app._weProvider = new ContextProvider(app, weClientContext, weServices);
    app.appletId = thisAppletHash.b64;
    app.groupProfiles = groupProfiles;
    /** Create Profiles Dvm from provided AppProxy */
    console.log("<example-app>.ctor()", profilesProxy);
    await app.createWeProfilesDvm(profilesProxy, profilesAppId, profilesBaseRoleName, profilesCloneId, profilesZomeName);
    return app;
  }


  /** Create a Profiles DVM out of a different happ */
  async createWeProfilesDvm(profilesProxy: AppProxy, profilesAppId: InstalledAppId, profilesBaseRoleName: BaseRoleName,
                            profilesCloneId: CloneId | undefined,
                            _profilesZomeName: ZomeName): Promise<void> {
    const profilesAppInfo = await profilesProxy.appInfo();
    if (!profilesAppInfo) {
      throw Promise.reject("Profiles AppInfo not found");
    }
    const profilesDef: DvmDef = {ctor: ProfilesDvm, baseRoleName: profilesBaseRoleName, isClonable: false};
    const cell_infos = Object.values(profilesAppInfo.cell_info);
    console.log("createProfilesDvm() cell_infos:", cell_infos);
    /** Create Profiles DVM */
      //const profilesZvmDef: ZvmDef = [ProfilesZvm, profilesZomeName];
    const dvm: DnaViewModel = new profilesDef.ctor(this, profilesProxy, new HCL(profilesAppId, profilesBaseRoleName, profilesCloneId), false);
    console.log("createProfilesDvm() dvm", dvm);
    console.log("createProfilesDvm() profilesAppInfo", profilesAppInfo);
    await this.setupWeProfilesDvm(dvm as ProfilesDvm);
  }


  /** */
  async setupWeProfilesDvm(dvm: ProfilesDvm): Promise<void> {
    this._weProfilesDvm = dvm as ProfilesDvm;
    /** Load My profile */
      //const maybeProfiles = await this._weProfilesDvm.profilesZvm.zomeProxy.getAgentsWithProfile();
      //const maybeAgents = maybeProfiles.map((eh) => encodeHashToBase64(eh));
      //console.log("maybeAgents", maybeAgents);
    const maybeMyProfile = await this._weProfilesDvm.profilesZvm.probeProfile(dvm.profilesZvm.cell.address.agentId.b64);
    console.log("setupWeProfilesDvm() maybeMyProfile", maybeMyProfile);
    if (maybeMyProfile) {
      const maybeLang = maybeMyProfile.fields['lang'];
      if (maybeLang) {
        console.log("Setting locale from We Profile", maybeLang);
        //setLocale(maybeLang);
      }
      //this._hasWeProfile = true;
    }
    // else {
    //   /** Create Guest profile */
    //   const profile = { nickname: "guest_" + Math.floor(Math.random() * 100), fields: {}};
    //   console.log("setupWeProfilesDvm() createMyProfile", this.filesDvm.profilesZvm.cell.agentId);
    //   await this.filesDvm.profilesZvm.createMyProfile(profile);
    // }
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

  @state() private _cell?: Cell;


  private _dnaDef?: DnaDefinition;

  /** */
  override async hvmConstructed() {
    console.log("hvmConstructed()")
    this.appProxy.getCellProxy(this.secret.deliveryZvm.cell.address).setCanThrottle(false);
    /** Probe */
    this._cell = this.secret.cell;
    // TODO: Fix issue: zTasker entry_defs() not found. Maybe confusion with integrity zome name?
    /** Done */
    this._loaded = true;
  }


  /** */
  async refresh(_e?: any) {
    console.log("secret-app.refresh() called")
    await this.hvm.probeAll(GetStrategy.Network);
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
          <div style="display: flex; flex-direction: row; gap: 5px; padding: 5px;">
          <view-cell-context></view-cell-context>          
          <span> - <abbr title=${this.secret.cell.address.agentId.b64}>Agent</abbr>: <b>${this.secret.cell.address.agentId.short}</b></span>
              </div>
          <div style="display: flex; flex-direction: row;  padding: 5px;">
          <div>
            <input type="button" value="Secret" @click=${() => {this._pageDisplayIndex = 0; this.requestUpdate()}} >
            <input type="button" value="Delivery" @click=${() => {this._pageDisplayIndex = 1; this.requestUpdate()}} >          
            <input type="button" value="Agent Directory" @click=${() => {this._pageDisplayIndex = 2; this.requestUpdate()}} >
          </div>
              <div style="flex: 1"></div>
        <div>
          <button type="button" @click=${this.refresh}>Refresh</button>
        </div>
          </div>
        <!--<dvm-inspect .dnaViewModel=${this.secret}></dvm-inspect> -->          
        <hr class="solid">      
        ${page}
      </cell-context>        
    `
  }

}
