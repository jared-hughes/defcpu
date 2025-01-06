/**
 * Char offset to RIP address of instruction.
 * @param {AssemblyState} state
 * @param {Number} from
 * @returns {Number | undefined}
 */
export function findInstructionFromOffset(state, from) {
  let node = state.head.next;
  while (node && node.statement.range.end < from) {
    node = node.next;
  }
  if (node) {
    const { statement } = node;
    const sectionStart = statement.section.programHeader.p_vaddr;
    const start = sectionStart + statement.address;
    // statement.length gives the length in bytes.
    return start;
  } else {
    return undefined;
  }
}

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
 * RIP address to section and instruction.
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
export function linePositionStr(state, address) {
  const pe = linePosition(state, address);
  if (pe === null) return "";
  const { pos, errLine } = pe;
  return ` ${pos} line ${errLine}`;
}

/**
 * RIP address to line number.
 * @param {AssemblyState} state
 * @param {Number} address
 * @returns {{pos: "on" | "after", errLine: number}  | null}
 */
export function linePosition(state, address) {
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
  return errLine === null ? null : { pos, errLine };
}
