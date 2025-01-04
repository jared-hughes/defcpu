#!/usr/bin/env node
import fs from "fs";
import process from "process";

const input_filename = process.argv[2];
const output_filename = process.argv[3];
if (!input_filename || !output_filename) {
  console.error("Invalid args");
  process.exit(1);
}

const print_regs_s = fs.readFileSync("./print_regs.s").toString();

let source = fs.readFileSync(input_filename).toString();
let i = 0;
let lineNum = 1;
const lines = [];
for (let line of source.split("\n")) {
  if (/^\s*print_regs\s*(#*)?$/.test(line)) {
    let lineStr = lineNum.toString(10) + ":\n";
    if (lineStr.length > 7) {
      throw "Why did you make such a long file? Smh.";
    }
    let lineLen = lineStr.length;
    let lineStrQuad = 0;
    for (let j = 7; j >= 0; j--) {
      lineStrQuad <<= 8;
      if (j < lineLen) {
        lineStrQuad += lineStr.charCodeAt(j);
      }
    }

    line = [
      `movb $${lineLen}, (line_num_len)`,
      `movq $0x${lineStrQuad.toString(16)}, (line_num)`,
      `movq $print${i}_ret, (printRegs_return)`,
      `jmp printRegs`,
      `print${i}_ret:`,
    ].join("; ");
    i++;
  }
  lines.push(line);
  lineNum++;
}
source = lines.join("\n");
if (i > 0) {
  source += "\n\n\n" + print_regs_s;
}

fs.writeFileSync(output_filename, source);
