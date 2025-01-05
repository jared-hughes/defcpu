import { examples } from "./examples.js";
import { getExtensions, reconfigureTheme } from "./codemirror";
import { $ } from "./util.js";
import { EditorState, EditorView } from "./codemirror";
import { ViewUpdate } from "@codemirror/view";
import { MessageFromWorker, MessageToWorker, Status } from "./messages.js";

const worker = new Worker(new URL("worker.js", import.meta.url), {
  type: "module",
  credentials: "omit",
  name: "defcpu-main-worker",
});

function postMessageToWorker(msg: MessageToWorker) {
  worker.postMessage(msg);
}

let running = false;
let pollInterval: number | undefined;

function setStatus(status: Status) {
  const { stdout, stderr } = status;
  const outElem = $<HTMLPreElement>("pre#output");
  outElem.innerText = stdout ?? "";
  const errElem = $<HTMLPreElement>("pre#errors");
  errElem.innerText = stderr ?? "";
}

worker.addEventListener("message", (fullMsg) => {
  const msg = fullMsg.data as MessageFromWorker;
  if (!running) {
    console.error("Unexpected message while not running", msg);
  }
  switch (msg.type) {
    case "status":
      setStatus(msg.status);
      return;
    case "done":
      setStatus(msg.status);
      setRunning(false);
      break;
    default:
      msg satisfies never;
      console.error("Unrecognized message type", msg);
      break;
  }
});

$<HTMLButtonElement>("button#run-button").addEventListener("click", startRun);

document.documentElement.addEventListener("keypress", (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key == "Enter") {
    // Ctrl-Enter
    startRun();
  }
});

function setRunning(r: boolean) {
  running = r;
  $<HTMLSpanElement>("span#status").innerText = r ? "running" : "idle";
  $<HTMLBodyElement>("body").classList.toggle("running", r);
  $<HTMLButtonElement>("button#run-button").disabled = r;
  if (!r) {
    clearInterval(pollInterval);
    pollInterval = undefined;
  }
}

function startRun() {
  if (running) {
    return;
  }
  setRunning(true);
  const src = editor.state.sliceDoc();

  postMessageToWorker({
    type: "run",
    src,
  });
  pollInterval = setInterval(() => {
    postMessageToWorker({
      type: "poll-status",
    });
  }, 250);
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
