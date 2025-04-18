import { InputConfigJSON } from "./cm-extensions/input-config.js";
import init, { OuterMachine, WebUnpredictables } from "./defcpu_web.js";
import { findInstructionFromOffset, linePosition } from "./line-number.mjs";
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
let breakpoints: Set<bigint> = new Set();
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
      continueRunningCode(true);
      break;
    case "single-step":
      singleStep();
      break;
    case "set-breakpoints":
      setBreakpoints(msg.breakpointFroms);
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

function setBreakpoints(breakpointFroms: number[]) {
  if (!state) return;
  breakpoints = new Set();
  for (const from of breakpointFroms) {
    const addr = findInstructionFromOffset(state, from);
    if (addr === undefined) continue;
    breakpoints.add(BigInt(addr));
  }
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
    const ds = data.state;
    state = new AssemblyState();
    // state.compile may throw
    state.compile(ds.doc, { haltOnError: true });
    const elf = createExecutable(state);

    // This clearly may throw
    if (!wasmReady) {
      throw new Error("Wasm module not yet loaded");
    }

    setBreakpoints(ds.breakpoints);

    if (om) {
      om.free();
    }
    om = OuterMachine.init(
      elf,
      // argv
      getArgv(ds.inputConfig),
      // envp
      getEnvp(ds.inputConfig),
      // TODO-seed: proper seed
      getInitUnp(ds.inputConfig)
    );
    setTimeout(() => continueRunningCode(false), 0);
  } catch (e) {
    postMessageFromWorker({
      type: "error",
      error: `Error when running: ${e}`,
    });
  }
}

function getArgv(ic: InputConfigJSON) {
  let argv;
  try {
    argv = JSON.parse(ic.argv);
  } catch (e) {
    throw new Error("Invalid argv JSON.");
  }
  if (!Array.isArray(argv) || argv.some((e) => typeof e !== "string")) {
    throw new Error("Argv should be a JSON array of strings.");
  }
  argv.unshift(ic.arg0);
  return argv;
}

function getEnvp(ic: InputConfigJSON) {
  let envp;
  try {
    envp = JSON.parse(ic.envp);
  } catch (e) {
    throw new Error("Invalid envp JSON.");
  }
  if (!Array.isArray(envp) || envp.some((e) => typeof e !== "string")) {
    throw new Error("Envp should be a JSON array of strings.");
  }
  return envp;
}

function getInitUnp(ic: InputConfigJSON) {
  if (!ic.useFixed) {
    return WebUnpredictables.from_random_seed(BigInt(ic.randomSeed));
  }
  return WebUnpredictables.from_fixed(
    ic.vdsoPtr,
    ic.rand16,
    ic.execfnPtr,
    ic.platformOffset
  );
}

function checkDone() {
  if (!om || !state) return;
  if (om.is_done()) {
    running = false;
    postMessageFromWorker({
      type: "done",
      status: getStatus(om, state),
    });
    return true;
  }
}

function checkBreakpoint() {
  if (!om || !state) return;
  if (breakpoints.has(om.get_rip())) {
    running = false;
    postMessageFromWorker({
      type: "pause",
      status: getStatus(om, state),
    });
    return true;
  }
}

function singleStep() {
  if (!om || !state) return;
  if (checkDone()) return;
  om.step();
  if (checkDone()) return;
  postMessageFromWorker({
    type: "pause",
    status: getStatus(om, state),
  });
}

function continueRunningCode(firstAfterContinue: boolean) {
  if (!om) return;
  if (!running) {
    return;
  }
  try {
    if (checkDone()) return;
    if (!firstAfterContinue) {
      if (checkBreakpoint()) return;
    }
    for (let i = 0; i < 65536; i++) {
      om.step();
      if (checkDone()) return;
      if (checkBreakpoint()) return;
    }
    setTimeout(() => continueRunningCode(false), 0);
  } catch (e) {
    postMessageFromWorker({
      type: "error",
      error: `Error when running: ${e}`,
    });
  }
}
