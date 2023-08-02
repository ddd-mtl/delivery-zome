import {css, html} from "lit";
import {property, state, customElement} from "lit/decorators.js";
import { ZomeElement } from "@ddd-qc/lit-happ";
import {DeliveryPerspective, DeliveryZvm} from "../viewModels/delivery.zvm";


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
            ([eh, state]) => {
                //console.log("MembraneLi", MembraneLi)
                return html `
              <li style="margin-top:10px;" title=${eh}>
                  <b>${eh}</b>: ${JSON.stringify(state)}
              </li>`
            }
        )


        /* Li */
        //console.log("signals", this.perspective.myDistributions);
        const signalsLi = {}
        // Object.entries(this.si.myDistributions).map(
        //     ([eh, state]) => {
        //         //console.log("MembraneLi", MembraneLi)
        //         return html `
        //       <li style="margin-top:10px;" title=${eh}>
        //           <b>${eh}</b>: ${JSON.stringify(state)}
        //       </li>`
        //     }
        // )


        /** render all */
        return html`
      <div>
        <h1>Delivery Dashboard</h1>
        <h2>My Distributions</h2>
        <ul>${distribsLi}</ul>
        <h2>Signals</h2>
        <ul>${signalsLi}</ul>          
      </div>
    `;
    }
}