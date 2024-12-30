#!/usr/bin/env node
import { AssemblyState } from "@defasm/core";
import process from "process";
import fs from "fs";
import { createExecutable } from "@defasm/cli/main.js";

const dump_filename = process.argv[2];
const asm_filename = process.argv[3];

const dump = fs.readFileSync(dump_filename).toString();
const rip_match = dump.match(/rip was ([0-9a-zA-Z]+)/);
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

const pos = linePosition(state, rip);
fs.writeFileSync(dump_filename, `Signal: segmentation fault${pos} ` + dump);

// The following function `findInstruction` is from @defasm/core,
// and `linePosition` is modified from `debug` from @defasm/core as well,
// under the following license.
// -
// Copyright (c) 2021, Alon Ran
// -
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
// -
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
/**
 * @param {AssemblyState} state
 * @param {Number} address
 */
function findInstruction(state, address) {
  for (const section of state.sections) {
    const segment = section.programHeader;
    if (
      address >= segment.p_vaddr &&
      address < Math.ceil((segment.p_vaddr + segment.p_memsz) / 0x1000) * 0x1000
    ) {
      const localAddress = address - segment.p_vaddr;
      let node = section.head.next;
      while (node) {
        if (node.statement.length > 0 && node.statement.address >= localAddress)
          return { section, instr: node.statement };
        node = node.next;
      }
      return { section, instr: null };
    }
  }

  return { section: null, instr: null };
}
/**
 * @param {AssemblyState} state
 * @param {Number} address
 */
function linePosition(state, address) {
  let { instr: crashedInstr, section: crashedSection } = findInstruction(
    state,
    address
  );
  let pos = "on";
  let errLine = null;
  if (crashedInstr === null) {
    if (crashedSection !== null) {
      pos = "after";
      state.iterate((instr, line) => {
        if (instr.section === crashedSection) errLine = line;
      });
    }
  } else {
    state.iterate((instr, line) => {
      if (errLine) return;
      if (instr == crashedInstr) errLine = line;
    });
  }
  return errLine !== null ? ` ${pos} line ${errLine}` : "";
}
