import init, { OuterMachine } from "./defcpu_web.js";
import { linePosition } from "./line-number.mjs";
import {
  MessageFromWorker,
  MessageToWorker,
  MsgRunCode,
  Status,
} from "./messages.js";
import { AssemblyState } from "@defasm/core";
import { createExecutable } from "defasm-cli-browser";

let wasmReady = false;

init().then(() => {
  wasmReady = true;
});

type AssemblyStateT = any;

let om: OuterMachine | undefined;
let state: AssemblyStateT | undefined;
let running = false;

globalThis.addEventListener("message", (fullMsg) => {
  const msg = fullMsg.data as MessageToWorker;
  switch (msg.type) {
    case "run":
      startRunningCode(msg);
      break;
    case "poll-status":
      if (!om || !state) {
        console.warn("Poll while not running");
        return;
      }
      postMessageFromWorker({
        type: "status",
        status: getStatus(om, state),
      });
      break;
    case "halt":
      if (!om) {
        console.warn("Halt while not running");
        return;
      }
      om.free();
      om = undefined;
      state = undefined;
      break;
    case "pause":
      // Will really pause on the next `continueRunningCode`,
      // which is probably the next queued event in the event loop.
      running = false;
      break;
    case "continue":
      running = true;
      continueRunningCode();
      break;
    case "single-step":
      singleStep();
      break;
    default:
      msg satisfies never;
      console.error("Unrecognized message type", msg);
      break;
  }
});

function postMessageFromWorker(msg: MessageFromWorker) {
  globalThis.postMessage(msg);
}

/** For now, just assume it's UTF-8 output. */
function arrToString(arr: Uint8Array): string {
  const maxLen = 128 * 1024; // 128 KB
  if (arr.length > maxLen) {
    arr = arr.slice(0, maxLen);
  }
  return new TextDecoder("utf-8").decode(arr);
}

function getStatus(om: OuterMachine, state: AssemblyStateT): Status {
  // TODO: only send message if there's a change.
  const rip = om.get_rip();
  const linePos =
    rip < Number.MAX_SAFE_INTEGER ? linePosition(state, Number(rip)) : null;
  return {
    stdout: arrToString(om.get_stdout()),
    stderr: arrToString(om.get_stderr()),
    registersStr: arrToString(om.get_registers_str()),
    linePos,
    fullStepCount: om.get_full_step_count(),
  };
}

function startRunningCode(data: MsgRunCode) {
  try {
    running = true;
    const src = data.src;
    state = new AssemblyState();
    // state.compile may throw
    state.compile(src, { haltOnError: true });
    const elf = createExecutable(state);

    // This clearly may throw
    if (!wasmReady) {
      throw new Error("Wasm module not yet loaded");
    }

    if (om) {
      om.free();
    }
    om = OuterMachine.init(elf);
    setTimeout(continueRunningCode, 0);
  } catch (e) {
    postMessageFromWorker({
      type: "error",
      error: `Error when running: ${e}`,
    });
  }
}

function checkDone() {
  if (!om || !state) return;
  if (om.is_done()) {
    running = false;
    postMessageFromWorker({
      type: "done",
      status: getStatus(om, state),
    });
  }
}

function singleStep() {
  checkDone();
  if (om && !om.is_done()) {
    om.step();
    checkDone();
  }
}

function continueRunningCode() {
  if (!om) return;
  if (!running) {
    return;
  }
  try {
    checkDone();
    for (let i = 0; i < 65536; i++) {
      om.step();
      checkDone();
      if (om.is_done()) return;
    }
    setTimeout(continueRunningCode, 0);
  } catch (e) {
    postMessageFromWorker({
      type: "error",
      error: `Error when running: ${e}`,
    });
  }
}
