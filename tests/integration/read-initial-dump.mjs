#!/usr/bin/env node
import fs from "fs";
import process from "process";

const input_filename = process.argv[2];
if (!input_filename) {
  console.error("Invalid args");
  process.exit(1);
}

const fd = fs.openSync(input_filename, "r");

function give_up() {
  // TODO-seed: proper seed.
  process.stdout.write(`--random-seed\f123456\f`);
  fs.closeSync(fd);
  process.exit(0);
}

function read_buf(offset, len) {
  if (len > 1024 * 1024) {
    throw new Error(`Length ${len} bigger than a megabyte, wtf.`);
  }
  const buf = Buffer.alloc(len);
  const bytesRead = fs.readSync(fd, buf, offset, len);
  if (bytesRead < len) {
    // Slow filesystem or short file, not handled.
    give_up();
  }
  return buf;
}

const bin = [
  ["header", 16, "sz"],
  ["rand16_yap", 8, "sz"],
  ["rand16", 32, "s"],
  ["vdso_yap", 8, "sz"],
  ["vdso_ptr", 16, "n"],
  ["execfn_yap", 8, "sz"],
  ["execfn_ptr", 16, "n"],
  ["pltoff_yap", 8, "sz"],
  ["platform_offset", 16, "n"],
  ["argc_yap", 8, "sz"],
  ["argc", 16, "n"],
  ["arglen_yap", 8, "sz"],
  ["arglen", 16, "n"],
  ["tail", 2, "s"],
];
const bin_len = bin.map((x) => +x[1]).reduce((a, b) => a + b, 0);
const bin_buf = read_buf(0, bin_len);
const map = {};
let offset = 0;
for (const [key, len, type] of bin) {
  const buf = bin_buf.subarray(offset, offset + len);
  switch (type) {
    case "sz":
      map[key] = from_cstr(buf);
      break;
    case "s":
      map[key] = buf.toString();
      break;
    case "n":
      map[key] = "0x" + buf.toString();
      break;
    default:
      throw new Error(`Invalid type ${type}.`);
  }
  offset += len;
}
function from_cstr(buf) {
  const nullIndex = buf.indexOf(0);
  if (nullIndex === -1) throw new Error("Missing trailing null");
  return buf.subarray(0, nullIndex).toString();
}
const header = map["header"];
if (header !== "Unpredictables:") {
  give_up();
}

// 16 bytes, convert to 32 hex nibbles
const rand16 = map["rand16"];
let rand16_str = "";
for (let i = 0; i < 16; i++) {
  rand16_str += (rand16[i] >> 4).toString(16);
  rand16_str += (rand16[i] % 16).toString(16);
}

const argc = parseInt(map["argc"]);
const arglen = parseInt(map["arglen"]);

let argv_buf = read_buf(0, arglen);
const argv = [];
for (let i = 0; i < argc; i++) {
  let next_null = argv_buf.findIndex((v) => v === 0);
  if (next_null === -1) {
    throw new Error("Not enough args, or missing null byte.");
  }
  const arg = argv_buf.subarray(0, next_null).toString();
  argv.push(arg);
  argv_buf = argv_buf.subarray(next_null + 1);
}
if (argv_buf.byteLength > 0) {
  throw new Error("Excess null byte inside args.");
}

fs.closeSync(fd);

/** Escape string for bash. Inside single quotes, but single quotes inserted need to be special. */
// function escape_str(s) {
//   return "'" + s.replace("'", `'"'"'`) + "'";
// }

process.stdout.write(
  [
    "--rand16",
    map["rand16"],
    "--vdso-ptr",
    map["vdso_ptr"],
    "--execfn-ptr",
    map["execfn_ptr"],
    "--platform-offset",
    map["platform_offset"],
    "--argv",
    ...argv,
  ].join("\f") + "\f"
);
