import init, { run } from "./defcpu_web.js";
import { MessageFromWorker, MessageToWorker, RunCode } from "./messages.js";
import { AssemblyState } from "@defasm/core";
import { createExecutable } from "defasm-cli-browser";

let wasmReady = false;

init().then(() => {
  wasmReady = true;
});

globalThis.addEventListener("message", (msg) => {
  const data = msg.data as MessageToWorker;
  switch (data.type) {
    case "run":
      tryRunCode(data);
      break;
    default:
      console.error("Unrecognized message type", data);
      break;
  }
});

function postMessageFromWorker(msg: MessageFromWorker) {
  globalThis.postMessage(msg);
}

/** For now, just assume it's UTF-8 output. */
function arrToString(arr: Uint8Array): string {
  return new TextDecoder("utf-8").decode(arr);
}

function runCode(data: RunCode) {
  const src = data.src;
  const state = new AssemblyState();
  // state.compile may throw
  state.compile(src, { haltOnError: true });
  const elf = createExecutable(state);

  // This clearly may throw
  if (!wasmReady) {
    throw new Error("Wasm module not yet loaded");
  }

  // run() may throw.
  const output = run(elf);
  postMessageFromWorker({
    type: "result",
    stdout: arrToString(output.get_stdout()),
    stderr: arrToString(output.get_stderr()),
  });
}

function tryRunCode(data: RunCode) {
  try {
    runCode(data);
  } catch (e) {
    postMessageFromWorker({
      type: "result",
      stdout: "",
      stderr: `Error when running: ${e}`,
    });
  }
}
