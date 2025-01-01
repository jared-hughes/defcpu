import init, { run } from "./defcpu_web.js";
import { examples } from "./examples.js";
import { AssemblyState } from "@defasm/core";
import { createExecutable } from "defasm-cli-browser";
import { getExtensions, reconfigureTheme } from "./codemirror";
import { $ } from "./util.js";
import { EditorState, EditorView } from "./codemirror";
import { ViewUpdate } from "@codemirror/view";

let wasmReady = false;

const form = $<HTMLFormElement>("form");
form.addEventListener("submit", runAndUpdateView);

document.documentElement.addEventListener("keypress", (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key == "Enter") {
    // Ctrl-Enter
    runAndUpdateView();
  }
});

function runAndUpdateView() {
  const { stdout, stderr } = compileAndRun();
  const outElem = $<HTMLPreElement>("pre#output");
  outElem.innerText = stdout ?? "";
  const errElem = $<HTMLPreElement>("pre#errors");
  errElem.innerText = stderr ?? "";
}

/** For now, just assume it's UTF-8 output. */
function arrToString(arr: Uint8Array): string {
  return new TextDecoder("utf-8").decode(arr);
}

function compileAndRun(): { stdout?: string; stderr?: string } {
  if (!wasmReady) {
    return { stderr: "Wasm module not yet loaded." };
  }
  const src = editor.state.sliceDoc();

  const state = new AssemblyState();
  try {
    state.compile(src, { haltOnError: true });
  } catch (e) {
    console.error(e);
    return { stderr: `Error in compiling: ${e}.` };
  }

  const elf = createExecutable(state);

  try {
    const output = run(elf);
    return {
      stdout: arrToString(output.get_stdout()),
      stderr: arrToString(output.get_stderr()),
    };
  } catch (e) {
    console.error(e);
    return { stderr: `Error when running: ${e}` };
  }
}

function debounce<T extends Function>(cb: T, timeout = 20) {
  let tm = 0;
  const debounced = (...args: any) => {
    clearTimeout(tm);
    tm = setTimeout(() => cb(...args), timeout);
  };
  Object.defineProperty(debounced, "name", { value: `debounced_${cb.name}` });
  return debounced as any as T;
}

const editorContainer = $<HTMLDivElement>("div#editor");

const editor = new EditorView({
  parent: editorContainer,
});

function saveToLocalStorage() {
  const code = editor.state.sliceDoc();
  localStorage.setItem("saved-src", code);
}

const debouncedSave = debounce(saveToLocalStorage, 250);

function onViewUpdate(vu: ViewUpdate) {
  if (!vu.docChanged) return;
  debouncedSave();
}

function getDefaultSource() {
  const saved = localStorage.getItem("saved-src");
  if (saved === null || /^\s+$/.test(saved)) {
    return examples[0].source;
  } else {
    return saved;
  }
}

editor.setState(
  EditorState.create({
    doc: getDefaultSource(),
    extensions: getExtensions(onViewUpdate),
  })
);

const themeMatch = matchMedia("(prefers-color-scheme: light)");

function setTheme() {
  const theme = themeMatch.matches ? "light" : "dark";
  editor.dispatch({
    effects: reconfigureTheme(theme),
  });
}

themeMatch.addEventListener("change", () => {
  setTheme();
});

setTheme();

// Disable Grammarly.
editor.contentDOM.setAttribute("data-gramm", "false");

const examplesSpan = $<HTMLSpanElement>("#examples");
for (const example of examples) {
  const btn = document.createElement("button");
  btn.innerText = example.name;
  btn.addEventListener("click", () => {
    editor.dispatch({
      changes: { from: 0, to: editor.state.doc.length, insert: example.source },
    });
  });
  examplesSpan.appendChild(btn);
  examplesSpan.appendChild(document.createTextNode(" "));
}

init().then(() => {
  wasmReady = true;
});
