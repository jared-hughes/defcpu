#!/usr/bin/env node
import fs from "fs";
import process from "process";

const input_filename = process.argv[2];
const output_filename = process.argv[3];
if (!input_filename || !output_filename) {
  console.error("Invalid args");
  process.exit(1);
}

let source = fs.readFileSync(input_filename).toString();
let i = 0;
let dumpCount = 0;
let lineNum = 1;
const lines = [];
let hex = "0123456789abcdef";
for (let line of source.split("\n")) {
  if (/^\s*print_regs\s*(#*)?$/.test(line)) {
    let lineStr = lineNum.toString(10) + ":\n";
    if (lineStr.length > 7) {
      throw "Why did you make such a long file? Smh.";
    }
    let lineLen = lineStr.length;
    let sub_lines = [];
    let lineStrQuad = "0x";
    for (let j = lineLen - 1; j >= 0; j--) {
      let c = lineStr.charCodeAt(j);
      lineStrQuad += hex[c >> 4];
      lineStrQuad += hex[c % 16];
      if (j % 4 == 0) {
        sub_lines.push(`movl $${lineStrQuad}, (${j}+line_num)`);
        lineStrQuad = "0x";
      }
    }

    line = [
      `movb $${lineLen}, (line_num_len)`,
      ...sub_lines,
      `movq $print${i}_ret, (printRegs_return)`,
      `jmp printRegs`,
      `print${i}_ret:`,
    ].join("; ");
    i++;
  } else if (/^\s*dump_initial\s*(#*)?$/.test(line)) {
    line = "jmp dump_initial; dump_initial_ret:";
    dumpCount++;
  }
  lines.push(line);
  lineNum++;
}
source = lines.join("\n");
if (i > 0) {
  const print_regs_s = fs.readFileSync("./print_regs.s").toString();
  source += "\n\n\n" + print_regs_s;
}
if (dumpCount > 1) {
  throw new Error("There should only be one dump_initial macro.");
}
if (dumpCount > 0) {
  const dump_initial_state_s = fs
    .readFileSync("./dump_initial_state.s")
    .toString();
  source += "\n\n\n" + dump_initial_state_s;
}

fs.writeFileSync(output_filename, source);
