import {StateChange, StateChangeType} from "./bindings/delivery.types";

/** */
export function getVariantByIndex(enumType: any, index: number): string {
  const keys = Object.keys(enumType);
  if (index >= 0 && index < keys.length) {
    const key = keys[index];
    return enumType[key];
  }
  throw Error("Out of bounds index");
}


/** */
export function prettyState(state: StateChange): string {
  if (StateChangeType.Create in state) {
    return state.Create? "Create NEW" : "Create";
  }
  if (StateChangeType.Update in state) {
    return state.Update? "Update NEW" : "Update";
  }
  if (StateChangeType.Delete in state) {
    return state.Delete? "Delete NEW" : "Delete";
  }
  throw Error("Unknown stateChange type");
}
