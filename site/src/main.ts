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

/**
 * idle = machine is empty.
 * running = currently running.
 * paused = machine is paused and can continue.
 * done = machine is paused and cannot continue.
 */
type State = "idle" | "running" | "paused" | "done";

let state: State = "idle";
let pollInterval: number | undefined;

setState("idle");

function setState(s: State) {
  state = s;
  $<HTMLSpanElement>("span#status").innerText = s;

  $<HTMLSpanElement>("span#step-count-container").classList.toggle(
    "inapplicable",
    state === "idle"
  );

  $<HTMLDivElement>("div#run-button-container").classList.toggle(
    "inapplicable",
    !canRun()
  );
  $<HTMLButtonElement>("button#run-button").disabled = !canRun();
  $<HTMLButtonElement>("button#continue-button").disabled = !canContinue();
  $<HTMLButtonElement>("button#step-button").disabled = !canStep();
  $<HTMLButtonElement>("button#pause-button").disabled = !canPause();
  $<HTMLButtonElement>("button#halt-button").disabled = !canHalt();
  if (s === "idle" || s === "done") {
    if (pollInterval) clearInterval(pollInterval);
    pollInterval = undefined;
  }
}

function setStatus(status: Status) {
  const { stdout, stderr, registersStr, fullStepCount } = status;
  $<HTMLPreElement>("pre#registers").innerText = registersStr ?? "";
  $<HTMLPreElement>("pre#output").innerText = stdout ?? "";
  $<HTMLPreElement>("pre#errors").innerText = stderr ?? "";
  $<HTMLSpanElement>("span#full-step-count").innerText =
    fullStepCount.toString() ?? "";
}

worker.addEventListener("message", (fullMsg) => {
  const msg = fullMsg.data as MessageFromWorker;
  if (state === "idle" || state === "done") {
    console.error("Unexpected message while not running", msg);
  }
  switch (msg.type) {
    case "status":
      setStatus(msg.status);
      return;
    case "done":
      setState("done");
      setStatus(msg.status);
      break;
    case "error":
      setState("done");
      setStatus({
        stdout: "",
        stderr: msg.error,
        registersStr: "",
        fullStepCount: 0n,
        rip: 0n,
      });
      break;
    default:
      msg satisfies never;
      console.error("Unrecognized message type", msg);
      break;
  }
});

$<HTMLButtonElement>("button#run-button").addEventListener("click", startRun);
$<HTMLButtonElement>("button#halt-button").addEventListener("click", haltRun);
$<HTMLButtonElement>("button#pause-button").addEventListener("click", pauseRun);
$<HTMLButtonElement>("button#continue-button").addEventListener(
  "click",
  continueRun
);
$<HTMLButtonElement>("button#step-button").addEventListener("click", stepRun);

function canHalt() {
  return state !== "idle";
}

function haltRun() {
  if (!canHalt()) return;
  setState("idle");
  setStatus({
    stdout: "",
    stderr: "",
    registersStr: "",
    fullStepCount: 0n,
    rip: 0n,
  });
  postMessageToWorker({
    type: "halt",
  });
}

function canPause() {
  return state === "running";
}

function pauseRun() {
  if (!canPause()) return;
  setState("paused");
  postMessageToWorker({
    type: "pause",
  });
}

function canContinue() {
  return state === "paused";
}

function continueRun() {
  if (!canContinue()) return;
  setState("running");
  postMessageToWorker({
    type: "continue",
  });
}

function canStep() {
  return state === "paused";
}

function stepRun() {
  if (!canStep()) return;
  postMessageToWorker({
    type: "single-step",
  });
}

document.documentElement.addEventListener("keypress", (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key == "Enter") {
    // Ctrl-Enter
    startRun();
  }
});

function canRun() {
  return state === "idle";
}

function startRun() {
  if (!canRun()) return;
  setState("running");
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
