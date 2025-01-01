import init, { run } from "./defcpu_web.js";
import { examples } from "./examples.js";
import { AssemblyState } from "@defasm/core";
import { createExecutable } from "defasm-cli-browser";

/** Assume $ always succeeds and returns an HTMLElement */
function $<MatchType extends HTMLElement>(selector: string) {
  return document.querySelector(selector) as MatchType;
}

let wasmReady = false;

const form = $<HTMLFormElement>("form");
form.onsubmit = () => {
  const { stdout, stderr } = compileAndRun();
  const outElem = $<HTMLPreElement>("pre#output");
  outElem.innerText = stdout ?? "";
  const errElem = $<HTMLPreElement>("pre#errors");
  errElem.innerText = stderr ?? "";
};

/** For now, just assume it's UTF-8 output. */
function arrToString(arr: Uint8Array): string {
  return new TextDecoder("utf-8").decode(arr);
}

function compileAndRun(): { stdout?: string; stderr?: string } {
  if (!wasmReady) {
    return { stderr: "Wasm module not yet loaded." };
  }
  const src = srcInput.value;

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

function saveToLocalStorage() {
  localStorage.setItem("saved-src", srcInput.value);
}

const srcInput = $<HTMLTextAreaElement>("textarea#src");

const saved = localStorage.getItem("saved-src");
if (saved === null || /^\s+$/.test(saved)) {
  srcInput.value = examples[0].source;
} else {
  srcInput.value = saved;
}

const examplesSpan = $<HTMLSpanElement>("#examples");
for (const example of examples) {
  const btn = document.createElement("button");
  btn.innerText = example.name;
  btn.addEventListener("click", () => {
    srcInput.value = example.source;
    saveToLocalStorage();
  });
  examplesSpan.appendChild(btn);
  examplesSpan.appendChild(document.createTextNode(" "));
}

srcInput.addEventListener("input", debounce(saveToLocalStorage, 500), false);

init().then(() => {
  wasmReady = true;
});
