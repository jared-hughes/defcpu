import { Facet, StateField } from "@codemirror/state";

export const voidFacet = Facet.define<unknown>();

export function onState<T>(field: StateField<T>, cb: (value: T) => void) {
  return voidFacet.compute([field], (state) => cb(state.field(field)));
}
