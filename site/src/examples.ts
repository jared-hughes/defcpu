interface Example {
  name: string;
  source: string;
  args?: string[];
}

const _examples: Example[] = [
  {
    name: "code.golf starter",
    source: String.raw`SYS_WRITE = 1
      SYS_EXIT = 60
      STDOUT_FILENO = 1

      # Printing
      .data
      buffer: .string "Hello, World!\n"
      bufferLen = . - buffer

      .text
      mov $SYS_WRITE, %eax
      mov $STDOUT_FILENO, %edi
      mov $buffer, %esi
      mov $bufferLen, %edx
      syscall

      # Looping
      .data
      digit: .byte   '0', '\n'

      .text
      mov $10, %bl
      numberLoop:
          mov $SYS_WRITE, %eax
          mov $STDOUT_FILENO, %edi
          mov $digit, %esi
          mov $2, %edx
          syscall

          incb (%rsi)
          dec %bl
          jnz numberLoop

      # Accessing arguments
      pop %rbx
      pop %rax

      argLoop:
          dec %rbx
          jz endArgLoop

          pop %rsi
          mov %rsi, %rdi

          mov $-1, %ecx
          xor %al, %al
          repnz scasb

          not %ecx
          movb $'\n', -1(%rsi, %rcx)

          mov %ecx, %edx
          mov $SYS_WRITE, %eax
          mov $STDOUT_FILENO, %edi
          syscall

          jmp argLoop
      endArgLoop:

      mov $SYS_EXIT, %eax
      mov $0, %edi
      syscall`,
    args: [
      "__46____1",
      "_8__4367_",
      "____2____",
      "__5______",
      "8__1_47_2",
      "__7_68__5",
      "97_31_2_4",
      "416_8__9_",
      "_52__91__",
    ],
  },
  {
    name: "64-bit add",
    source: String.raw`mov $0xABCD1234, %eax
      mov $0x11115432, %ebx
      add %eax, %ebx
      hlt`,
  },
  {
    name: "Hello World",
    source: String.raw`SYS_WRITE = 1
      SYS_EXIT = 60
      STDOUT_FILENO = 1

      .data
      buffer: .string "Hello, World!\n"
      bufferLen = . - buffer

      .text
      mov $SYS_WRITE, %eax
      mov $STDOUT_FILENO, %edi
      mov $buffer, %esi
      mov $bufferLen, %edx
      syscall

      mov $SYS_EXIT, %eax
      mov $0, %edi
      syscall`,
  },
  {
    name: "Many hellos",
    source: String.raw`SYS_WRITE = 1
      STDOUT_FILENO = 1

      .data
      buffer: .string "Hello, World!\n"
      bufferLen = . - buffer

      .text
      hello_world:
      mov $SYS_WRITE, %eax
      mov $STDOUT_FILENO, %edi
      mov $buffer, %esi
      mov $bufferLen, %edx
      syscall

      mov $0x3FFFF, %r9d
      busy_loop:
      dec %r9d
      jnz busy_loop
      jmp hello_world`,
  },
];

export const examples: Example[] = _examples.map((ex) => ({
  ...ex,
  source: dedent(ex.source),
}));

function dedent(s: string): string {
  const lines = s.split("\n");
  if (lines.length <= 1) return s;
  let commonWS = leadingWhitespace(lines[1]);
  for (const line of lines.slice(2)) {
    if (/^\s*$/.test(line)) {
      // Fully blank line; skip
      continue;
    }
    const ws = leadingWhitespace(line);
    commonWS = commonPrefix(commonWS, ws);
    if (commonWS.length === 0) return s;
  }
  if (commonWS.length === 0) return s;
  const wsLen = commonWS.length;
  let ret = lines[0];
  for (const line of lines.slice(1)) {
    ret += "\n" + line.slice(wsLen);
  }
  return ret;
}

function leadingWhitespace(line: string): string {
  return line.match(/^\s*/)?.[0] ?? "";
}

function commonPrefix(a: string, b: string) {
  const m = Math.min(a.length, b.length);
  for (let i = 0; i < m; i++) {
    if (a[i] !== b[i]) return a.slice(0, i);
  }
  return a.slice(0, m);
}
