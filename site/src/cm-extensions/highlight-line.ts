import { StateField, StateEffect } from "@codemirror/state";
import { EditorView, Decoration } from "@codemirror/view";

export function highlightLineExt() {
  return [lineHighlightBaseTheme, lineHighlightField];
}

export function setHighlightedLines(lineNumbers: number[]) {
  return setHighlightedLinesEffect.of(lineNumbers);
}

const setHighlightedLinesEffect = StateEffect.define<number[]>();

const lineHighlightField = StateField.define({
  create() {
    return Decoration.none;
  },
  update(lines, tr) {
    lines = lines.map(tr.changes);
    for (let e of tr.effects) {
      if (e.is(setHighlightedLinesEffect)) {
        lines = Decoration.none;
        const decorations = [];
        for (const lineNumber of e.value) {
          console.log("lineNumber", lineNumber);
          const { from } = tr.newDoc.line(lineNumber);
          decorations.push(highlightLineDecoration.range(from));
        }
        lines = lines.update({ add: decorations });
      }
    }
    return lines;
  },
  provide: (f) => EditorView.decorations.from(f),
});

const lineHighlightBaseTheme = EditorView.baseTheme({
  "&dark .cm-highlight-line": { background: "#5C4300" }, // #976700 edges.
  "&light .cm-highlight-line": { background: "#FDF3AA" }, // #AD8D0F edges
});

const highlightLineDecoration = Decoration.line({
  class: "cm-highlight-line",
});
