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
source = source.replace(/print_regs/g, () => {
  i++;
  return [
    `movq $print${i}_ret, (printRegs_return)`,
    `jmp printRegs`,
    `print${i}_ret:`,
  ].join("; ");
});
if (i > 0) {
  source += "\n\n\n" + print_regs_s;
}

fs.writeFileSync(output_filename, source);
