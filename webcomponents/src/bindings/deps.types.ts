import {HoloHash, Timestamp} from "@holochain/client";
//import {ZomeIndex} from "@ddd-qc/cell-proxy";

export type ZomeIndex = number;
export type EntryDefIndex = number;

/** Link defs from holochain */

export type LinkTag = number[];

export type LinkType = number;

export interface Link {
  author: HoloHash
  target: HoloHash
  timestamp: Timestamp
  zome_index: ZomeIndex
  link_type: LinkType
  tag: LinkTag
  create_link_hash: HoloHash
}