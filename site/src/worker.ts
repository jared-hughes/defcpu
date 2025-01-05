import init, { OuterMachine } from "./defcpu_web.js";
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

globalThis.addEventListener("message", (fullMsg) => {
  const msg = fullMsg.data as MessageToWorker;
  switch (msg.type) {
    case "run":
      startRunningCode(msg);
      break;
    case "poll-status":
      if (!om) {
        console.warn("Poll while not running");
        return;
      }
      postMessageFromWorker({
        type: "status",
        status: getStatus(om),
      });
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

let om: OuterMachine | undefined;

function getStatus(om: OuterMachine): Status {
  // TODO: only send message if there's a change.
  return {
    stdout: arrToString(om.get_stdout()),
    stderr: arrToString(om.get_stderr()),
  };
}

function startRunningCode(data: MsgRunCode) {
  try {
    const src = data.src;
    const state = new AssemblyState();
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
      type: "done",
      status: {
        stdout: "",
        stderr: `Error when running: ${e}`,
      },
    });
  }
}

function continueRunningCode() {
  if (!om) return;
  try {
    for (let i = 0; i < 65536; i++) {
      if (om.is_done()) {
        postMessageFromWorker({
          type: "done",
          status: getStatus(om),
        });
        return;
      }
      om.step();
    }
    setTimeout(continueRunningCode, 0);
  } catch (e) {
    postMessageFromWorker({
      type: "done",
      status: {
        stdout: "",
        stderr: `Error when running: ${e}`,
      },
    });
  }
}
