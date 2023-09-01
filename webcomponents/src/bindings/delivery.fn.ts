/* This file is generated by zits. Do not edit manually */

import {ZomeName, FunctionName} from '@holochain/client';


/** Array of all zome function names in "delivery" */
export const deliveryFunctionNames: FunctionName[] = [
	"entry_defs", 
	"get_zome_info", 
	"get_dna_info",



	"check_manifest",
	"commit_chunks",
	"commit_parcel_chunk",
	"commit_parcel_manifest",
	"commit_pending_item",
	"distribute_parcel",
	"fetch_chunk",
	"get_all_local_parcels",
	"get_delivery_state",
	"get_distribution_state",
	"get_notice",
	"get_parcel_notice",
	"get_notice_state",
	"get_enc_key",
	"get_my_enc_key",
	"test_encryption",
	"pull_inbox",
	"query_all_Distribution",
	"query_Distribution",
	"query_all_DeliveryNotice",
	"query_DeliveryNotice",
	"query_all_NoticeReceived",
	"query_NoticeReceived",
	"query_all_DeliveryReply",
	"query_all_ReplyReceived",
	"query_all_ParcelReceived",
	"query_ParcelReceived",
	"query_all_DeliveryReceipt",
	"respond_to_notice",
	"receive_delivery_dm",
	"commit_parcel",
	"commit_NoticeReceived",
	"commit_ParcelReceived",
	"fetch_parcel",];


/** Generate tuple array of function names with given zomeName */
export function generateDeliveryZomeFunctionsArray(zomeName: ZomeName): [ZomeName, FunctionName][] {
   const fns: [ZomeName, FunctionName][] = [];
   for (const fn of deliveryFunctionNames) {
      fns.push([zomeName, fn]);
   }
   return fns;
}


/** Tuple array of all zome function names with default zome name "delivery" */
export const deliveryZomeFunctions: [ZomeName, FunctionName][] = generateDeliveryZomeFunctionsArray("delivery");
