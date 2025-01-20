import {
  StateField,
  StateEffect,
  RangeSet,
  EditorState,
} from "@codemirror/state";
import { gutterLineClass, GutterMarker, lineNumbers } from "@codemirror/view";
import { EditorView } from "codemirror";
import { onState } from "./cm-utils";

const breakpointEffect = StateEffect.define<{ pos: number; on: boolean }>({
  map: (val, mapping) => ({ pos: mapping.mapPos(val.pos), on: val.on }),
});

export const breakpointField = StateField.define<RangeSet<GutterMarker>>({
  create() {
    return RangeSet.empty;
  },
  update(set, transaction) {
    set = set.map(transaction.changes);
    for (const e of transaction.effects) {
      if (e.is(breakpointEffect)) {
        if (e.value.on) {
          set = set.update({ add: [breakpointMarker.range(e.value.pos)] });
        } else {
          set = set.update({ filter: (from) => from != e.value.pos });
        }
      }
    }
    return set;
  },
  toJSON(value) {
    return breakpointFroms(value);
  },
  fromJSON(obj: unknown) {
    if (!Array.isArray(obj) || obj.some((x) => typeof x !== "number")) {
      return RangeSet.empty;
    }
    const breakpointFroms = obj as number[];
    const markers = [];
    for (const from of breakpointFroms) {
      markers.push(breakpointMarker.range(from));
    }
    return RangeSet.of(markers);
  },
});

// Ref https://codemirror.net/examples/gutter/
function toggleBreakpoint(view: EditorView, pos: number) {
  const breakpoints = view.state.field(breakpointField);
  let hasBreakpoint = false;
  breakpoints.between(pos, pos, () => {
    hasBreakpoint = true;
  });
  view.dispatch({
    effects: breakpointEffect.of({ pos, on: !hasBreakpoint }),
  });
}

const breakpointMarker = new (class extends GutterMarker {
  elementClass = "cm-breakpoint";
})();

function breakpointFroms(breakpoints: RangeSet<GutterMarker>) {
  const iter = breakpoints.iter();
  let froms: number[] = [];
  while (iter.value !== null) {
    froms.push(iter.from);
    iter.next();
  }
  return froms;
}

export function getBreakpointFroms(state: EditorState): number[] {
  const breakpoints = state.field(breakpointField);
  return breakpointFroms(breakpoints);
}

export function breakpointGutterExt(onNewGutters: (froms: number[]) => void) {
  return [
    breakpointField,
    lineNumbers({
      domEventHandlers: {
        mousedown(view, line, event) {
          if (
            !(event.target instanceof HTMLElement) ||
            event.target.closest(".cm-gutterElement") === null
          ) {
            // The click is not actually on a line number, but rather on
            // blank space in the gutter.
            return false;
          }
          toggleBreakpoint(view, line.from);
          return true;
        },
      },
    }),
    gutterLineClass.compute([breakpointField], (state) =>
      state.field(breakpointField)
    ),
    onState(breakpointField, (breakpoints) => {
      onNewGutters(breakpointFroms(breakpoints));
    }),
    EditorView.baseTheme({
      "&dark .cm-lineNumbers .cm-gutterElement.cm-breakpoint": {
        background: "#5186EC", // #1a73e8 edges
        color: "white",
      },
      "&light .cm-lineNumbers .cm-gutterElement.cm-breakpoint": {
        background: "#4285F4", //  #1a73e8 edges
        color: "white",
      },
      ".cm-lineNumbers .cm-gutterElement": {
        paddingLeft: "20px",
        cursor: "pointer",
      },
    }),
  ];
}
