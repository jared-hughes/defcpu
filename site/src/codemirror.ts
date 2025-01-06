import { Compartment, EditorState } from "@codemirror/state";
import {
  EditorView,
  keymap,
  lineNumbers,
  drawSelection,
  ViewUpdate,
} from "@codemirror/view";
export { EditorState, EditorView };
import { assembly } from "@defasm/codemirror";
import {
  history,
  historyKeymap,
  indentLess,
  insertNewline,
  insertTab,
  standardKeymap,
  toggleComment,
} from "@codemirror/commands";
import { tags } from "@lezer/highlight";
import {
  bracketMatching,
  defaultHighlightStyle,
  HighlightStyle,
  syntaxHighlighting,
} from "@codemirror/language";
import {
  oneDarkTheme,
  oneDarkHighlightStyle,
} from "@codemirror/theme-one-dark";
import { highlightLineExt } from "./cm-extensions/highlight-line";

const themeCompartment = new Compartment();

export function getExtensions(onViewUpdate: (vu: ViewUpdate) => void) {
  const asmErrorTooltip = {
    "&:before": { borderTopColor: "var(--color)" },
    background: "var(--color)",
    color: "var(--background)",
  };

  const fontFamily = "'Source Code Pro', monospace";

  // Enable character-wise wrapping whenever possible.
  // This was disabled in the upstream due to the old Safari issue (codemirror/dev#524).
  const lineWrapping: any = CSS.supports("overflow-wrap", "anywhere")
    ? { wordBreak: "break-all" }
    : {};

  const extensions = [
    // requires _codemirror_unprintable: showUnprintables,
    drawSelection(),
    assembly(),
    bracketMatching(),
    EditorView.updateListener.of(onViewUpdate),
    EditorView.theme({
      ".cm-lineWrapping": lineWrapping,
      ".cm-gutters": { fontFamily },
      ".cm-tooltip": { fontFamily },
      ".cm-asm-error": {
        textDecoration: "underline var(--asm-error)",
      },
      ".cm-asm-error-tooltip": asmErrorTooltip,
      ".cm-content": { fontFamily },
      ".cm-tooltip-autocomplete": { fontFamily },
    }),
    themeCompartment.of([]),
    highlightLineExt(),
  ];

  // More extensions that we won't need for output boxes.
  extensions.push(
    history(),
    keymap.of([
      // Replace "enter" with a non auto indenting action.
      ...historyKeymap,
      ...standardKeymap.filter((k) => k.key != "Enter"),
      { key: "Enter", run: insertNewline },
      { key: "Tab", run: insertTab, shift: indentLess },
      { key: "Mod-/", run: toggleComment },
    ]),
    lineNumbers()
  );

  return extensions;
}

export function reconfigureTheme(theme: "dark" | "light") {
  return themeCompartment.reconfigure(themeExtensions(theme));
}

function themeExtensions(theme: "dark" | "light") {
  return theme === "dark"
    ? [
        EditorView.theme(
          {
            "&": { background: "var(--background)", color: "var(--color)" },
            ".cm-asm-dump": { color: "var(--asm-dump)" },
            ".cm-gutters": { background: "var(--light-grey)" },
            ".cm-content": { caretColor: "var(--color)" },
            ".cm-cursor, .cm-dropCursor": {
              borderLeftColor: "var(--color)",
            },
          },
          { dark: true }
        ),
        syntaxHighlighting(
          HighlightStyle.define([
            { color: "#98c379", tag: tags.literal },
            { color: "#e06c75", tag: tags.regexp },
          ])
        ),
        oneDarkTheme,
        syntaxHighlighting(oneDarkHighlightStyle),
      ]
    : [syntaxHighlighting(defaultHighlightStyle)];
}
