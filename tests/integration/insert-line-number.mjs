#!/usr/bin/env node
import { AssemblyState } from "@defasm/core";
import process from "process";
import fs from "fs";
import { createExecutable } from "@defasm/cli/main.js";
import { linePositionStr } from "../../site/src/line-number.mjs";

const dump_filename = process.argv[2];
const asm_filename = process.argv[3];

const dump = fs.readFileSync(dump_filename).toString();
const rip_regex = /\(%rip was ([0-9a-zA-Z]+)\)/;
const rip_match = dump.match(rip_regex);
if (!rip_match) {
  // console.log("No rip in reg dump, skipping.");
  process.exit(0);
}
const rip = parseInt(rip_match[1], 16);

const assemblyConfig = { writableText: true };
let state = new AssemblyState(assemblyConfig);

try {
  const code = fs.readFileSync(asm_filename).toString();
  state.compile(code, { haltOnError: true });

  const outputFile = "/tmp/asm";
  // `createExecutable` mutates `state` because of course it does.
  // It adds the `programHeader` data (the segments) below.
  createExecutable(outputFile, state);
} catch (e) {
  console.error("Error in re-assembly to insert line number:");
  console.error(e);
  process.exit(1);
}

const pos = linePositionStr(state, rip);
fs.writeFileSync(
  dump_filename,
  dump.replace(rip_regex, (s) => `Signal: segmentation fault${pos} ${s}`)
);
