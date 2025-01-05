import { examples } from "./examples.js";
import { getExtensions, reconfigureTheme } from "./codemirror";
import { $ } from "./util.js";
import { EditorState, EditorView } from "./codemirror";
import { ViewUpdate } from "@codemirror/view";
import { MessageFromWorker, MessageToWorker } from "./messages.js";

const worker = new Worker(new URL("worker.js", import.meta.url), {
  type: "module",
  credentials: "omit",
  name: "defcpu-main-worker",
});

function postMessageToWorker(msg: MessageToWorker) {
  worker.postMessage(msg);
}

worker.addEventListener("message", (msg) => {
  const data = msg.data as MessageFromWorker;
  switch (data.type) {
    case "result":
      const { stdout, stderr } = data;
      const outElem = $<HTMLPreElement>("pre#output");
      outElem.innerText = stdout ?? "";
      const errElem = $<HTMLPreElement>("pre#errors");
      errElem.innerText = stderr ?? "";
      break;
    default:
      console.error("Unrecognized message type", data);
      break;
  }
});

const form = $<HTMLFormElement>("form");
form.addEventListener("submit", startRun);

document.documentElement.addEventListener("keypress", (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key == "Enter") {
    // Ctrl-Enter
    startRun();
  }
});

function startRun() {
  const src = editor.state.sliceDoc();

  postMessageToWorker({
    type: "run",
    src,
  });
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
