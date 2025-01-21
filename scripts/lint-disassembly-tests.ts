// Ensure tests/disassembly/sources/*.s represent correct disassembly.
// The source of truth is the hexdump bytes on the left.
// The instructions on the right are the disassembly of the bytes on the right.

import { exec, execStdout, sha256sum } from "./script-helpers";
import fs from "node:fs/promises";

const elfsDir = "./tests/disassembly/elfs";
const expectedDir = "./tests/disassembly/expected";
const shasDir = "./tests/disassembly/shas";
const sourcesDir = "./tests/disassembly/sources";

export async function lintDisassemblyTests(fix: boolean) {
  console.log("[Linting disassembly tests]");
  // catch: be okay with no dir
  await fs.rm(elfsDir, { recursive: true }).catch(() => {});
  await fs.mkdir(elfsDir);
  await fs.rm(expectedDir, { recursive: true }).catch(() => {});
  await fs.mkdir(expectedDir);
  await fs.mkdir(shasDir, { recursive: true });
  const filenames = await fs.readdir(sourcesDir);
  let bad = false;
  for (const filename of filenames) {
    if (filename.endsWith(".s")) {
      const base = filename.slice(0, -".s".length);
      const thisBad = await lintSingleFile(base, fix);
      if (thisBad) bad = true;
    }
  }
  if (bad) {
    process.exit(1);
  }
}

/** Returns true if the file needs fixing but `fix` is false. */
async function lintSingleFile(base: string, fix: boolean): Promise<boolean> {
  const sourceFile = `${sourcesDir}/${base}.s`;
  const sha = await sha256sum(sourceFile);
  const shaFile = `${shasDir}/${base}.sha256sum`;
  const old_sha = await fs.readFile(shaFile).catch((_) => Buffer.from(""));
  if (sha.equals(old_sha)) {
    return false;
  }
  await fs.rm(shaFile).catch(() => {});

  // Now we know the file's SHA has changed.

  const tempSourceFile = (await execStdout("mktemp", []))
    .toString("utf-8")
    .trimEnd();

  // Now tempSourceFile is a temporary filename

  const oldDis = await fs.readFile(sourceFile, { encoding: "utf-8" });
  const asm = disSourceToAsm(oldDis);
  await fs.writeFile(tempSourceFile, asm);

  // Now tempSourceFile contains the assembly.

  const elfFile = `${elfsDir}/${base}.elf`;
  await exec(
    "./node_modules/.bin/defasm",
    [tempSourceFile, ...["-w", "-x", "-o"], elfFile],
    { quiet: true }
  );

  await fs.rm(tempSourceFile);

  // Now the ELF file is created

  const objdump = (await execStdout("objdump", ["-x", elfFile])).toString();
  const startAddr = objdump.match(/start address (.*)/)![1];
  const objdumpLines = objdump.split("\n");
  const sectionStartLine = objdumpLines.findIndex((line) =>
    line.includes("vaddr " + startAddr)
  );
  const segmentLenLine = objdumpLines[sectionStartLine + 1];
  const segmentLen = segmentLenLine.match(/memsz ([^ ]+)/)![1];

  // Now startAddr looks like 0x0000000000400000
  // Now segmentLen looks like 0x0000000000000049

  const gdbOut = (
    await execStdout("gdb", [
      "-e",
      elfFile,
      "-batch",
      ...["-ex", `b *${startAddr}`],
      ...["-ex", "r"],
      // https://visualgdb.com/gdbreference/commands/disassemble
      ...["-ex", `set disassembly-flavor intel`],
      ...["-ex", `disassemble /r ${startAddr}, +${segmentLen}`],
    ])
  ).toString("utf-8");
  const expDis = normalizeDis(gdbOut);

  // Now gdbOutNorm is the expected output

  if (oldDis == expDis) {
    fs.writeFile(shaFile, sha);
    return false;
  }

  console.log(`Difference in '${sourceFile}'`);

  const expectedFile = `${expectedDir}/${base}.s`;
  fs.writeFile(expectedFile, expDis);

  await exec(
    "git",
    ["--no-pager", "diff", "--no-index", sourceFile, expectedFile],
    { allowNonzeroExit: true }
  );

  if (fix) {
    console.log(`Applying new contents for '${sourceFile}'...`);
    console.log(
      "If the file is open in your editor, you may wish to revert it to the version on disk."
    );
    fs.writeFile(sourceFile, expDis);
    // Only update SHA if successful
    fs.writeFile(shaFile, sha);
    return false;
  } else {
    console.log(`Run as './make build -w' to modify the file in-place`);
    return true;
  }
}

function disSourceToAsm(s: string) {
  let out = ".byte ";
  out += s
    .replace(/([0-9a-fA-F]{2}( [0-9a-fA-F]{2})*)( .*)?$/gm, "$1")
    .replace(/\n/g, " ")
    .replace(/ +/g, " ")
    .replace(/ /g, ", ")
    .replace(/([0-9a-fA-F]{2})/g, "0x$1")
    .replace(/, $/gm, "");
  out += "\n";
  return out;
}

function normalizeDis(s: string) {
  s = s.trim().split("\n").slice(4, -1).join("\n");
  s = s
    .replace(/(=>)? *0x[0-9a-fA-F]+:\s*/g, "")
    .replace(/,/g, ", ")
    .replace(/,  /g, ", ")
    .replace(/ *$/gm, "")
    .replace(/ *#.*/g, "");
  const longestStart = Math.max(
    ...s.split("\n").map((x) => x.split("\t")[0].length)
  );
  const tabLen = (((longestStart + 4) >> 3) << 3) + 4;
  s = s.replace(
    /^([^\t]+)\t/gm,
    (_, m1) => m1 + " ".repeat(tabLen - m1.length)
  );
  return s + "\n";
}
