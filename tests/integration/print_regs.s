# This file is a modification of defasm/cli/debug.s, which is under the following license.
#
# Copyright (c) 2021, Alon Ran
# 
# Permission to use, copy, modify, and/or distribute this software for any
# purpose with or without fee is hereby granted, provided that the above
# copyright notice and this permission notice appear in all copies.
# 
# THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
# WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
# MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
# ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
# WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
# ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
# OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

# usage:
#
# movq $print1_ret, (printRegs_return)
# jmp printRegs
# print1_ret:

sys_write = 1
sys_open = 2
sys_close = 3
sys_fork = 57
sys_execve = 59
sys_exit = 60
sys_ptrace = 101
sys_prctl = 157
sys_waitid = 247

STDERR_FILENO = 2

# Tracing the file
.bss
# regs are in the order provided by sys_ptrace(PTRACE_GETREGS, ...), even though
# we don't currently use sys_ptrace to get regs since we're in the same process.
regs:
r_R15:      .quad 0
r_R14:      .quad 0
r_R13:      .quad 0
r_R12:      .quad 0
r_RBP:      .quad 0
r_RBX:      .quad 0
r_R11:      .quad 0
r_R10:      .quad 0
r_R9:       .quad 0
r_R8:       .quad 0
r_RAX:      .quad 0
r_RCX:      .quad 0
r_RDX:      .quad 0
r_RSI:      .quad 0
r_RDI:      .quad 0
r_ORIG_RAX: .quad 0
r_RIP:      .quad 0
r_CS:       .quad 0
r_FLAGS:    .quad 0
r_RSP:      .quad 0
r_SS:       .quad 0
r_FS_BASE:  .quad 0
r_GS_BASE:  .quad 0
r_DS:       .quad 0
r_ES:       .quad 0
r_FS:       .quad 0
r_GS:       .quad 0

# We only need 7 quadwords of stack space. A bit extra to be space.
mini_stack: .quad 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
mini_stack_end: .quad 0


.section .rodata
hexaChars: .string "0123456789ABCDEF"

.data
outputBuffer: .byte 0
dumpFile: .long STDERR_FILENO

.text

# Print the value of %rax in 16 hexadecimal characters
# Uses 6 quadwords of mini_stack space.
printQuad:
    push %rbx
    push %rdx
    push %rsi
    push %rdi
    push %rcx
    
    mov $16, %ebx
    mov $outputBuffer, %esi
    
    nibbleLoop:
        rol $4, %rax
        mov %eax, %edx
        and $0xF, %edx
        mov hexaChars(%rdx), %dl
        mov %dl, (%rsi)

        # Print the nibble
        push %rax
            mov $1, %dl
            mov $sys_write, %eax
            mov dumpFile, %edi
            syscall
        pop %rax

        dec %ebx
        jnz nibbleLoop
    
    pop %rcx
    pop %rdi
    pop %rsi
    pop %rdx
    pop %rbx
    jmp printQuad_done

.section .rodata

registerOrder:
.long r_RAX, r_R8,  r_RBX, r_R9,  r_RCX, r_R10, r_RDX, r_R11
.long r_RSI, r_R12, r_RDI, r_R13, r_RSP, r_R14, r_RBP, r_R15
.long r_FLAGS

flagOrder:
.byte 0, 6, 11, 7, 10, 2

dumpString: .string "\
Registers:
    %rax = \0        %r8  = \0
    %rbx = \0        %r9  = \0
    %rcx = \0        %r10 = \0
    %rdx = \0        %r11 = \0
    %rsi = \0        %r12 = \0
    %rdi = \0        %r13 = \0
    %rsp = \0        %r14 = \0
    %rbp = \0        %r15 = \0
Flags (\0):
    Carry     = \0   Zero   = \0
    Overflow  = \0   Sign   = \0
    Direction = \0   Parity = \0

\0"

flagMessages:
.string "0 (no carry)    ", "1 (carry)       "
.string "0 (isn't zero)  ", "1 (is zero)     "
.string "0 (no overflow) ", "1 (overflow)    "
.string "0 (positive)    ", "1 (negative)    "
.string "0 (up)          ", "1 (down)        "
.string "0 (odd)         ", "1 (even)        "
flagMsgLen = 16

.text
printRegs:
    mov %rax, (r_RAX)
    mov %rbx, (r_RBX)
    mov %rcx, (r_RCX)
    mov %rdx, (r_RDX)
    mov %rsi, (r_RSI)
    mov %rdi, (r_RDI)
    mov %rsp, (r_RSP)
    mov %rbp, (r_RBP)
    mov %r8, (r_R8)
    mov %r9, (r_R9)
    mov %r10, (r_R10)
    mov %r11, (r_R11)
    mov %r12, (r_R12)
    mov %r13, (r_R13)
    mov %r14, (r_R14)
    mov %r15, (r_R15)

    mov $mini_stack_end, %rsp

    pushfq
    pop %rax
    mov %rax, (r_FLAGS)

    xor %ebx, %ebx
    # bl = 0. Will be incremented on each read to count number of nulls filled in.
    mov $dumpString, %esi

    registerLoop:
        # start of loop: %esi is the start of $dumpString, or points to the last \0.
        mov %esi, %edi
        mov $-1, %ecx
        xor %al, %al
        # increment %rdi and decrement %ecx until (%rdi) = %al (=0), or %ecx = 0.
        repnz scasb
        # %rdi is now a pointer to the null byte; %ecx is negative of its 1-index in the string.

        push %rdi
        not %ecx
        dec %ecx
        # %ecx is length to print.
        mov %ecx, %edx
        mov $sys_write, %eax
        mov dumpFile, %edi
        syscall
        pop %rsi

        cmp $23, %bl # 17 registers + 6 flags. 
        jge endRegisterLoop

        cmp $17, %bl # 17 registers (including RFLAGS)
        jge printFlag
            # bl < 17: Select register and print it
            mov registerOrder(, %rbx, 4), %eax
            mov (%rax), %rax
            jmp printQuad
            printQuad_done: jmp nextRegister
        printFlag:
            # bl >= 17: Select appropriate flag message
            push %rsi

            mov %bl, %dl
            sub $17, %dl
            mov flagOrder(%rdx), %al
            add %dl, %dl # shl $1, %dl
            bt %eax, r_FLAGS
            adc $0, %dl

            imul $flagMsgLen, %edx, %edx
            lea flagMessages(%rdx), %esi

            mov $flagMsgLen, %edx
            mov $sys_write, %eax
            mov dumpFile, %edi
            syscall

            pop %rsi
        nextRegister:
        inc %ebx
        jmp registerLoop
    endRegisterLoop:

printRegsDone:
    mov (r_FLAGS), %rax
    push %rax
    popfq

    mov (r_RAX), %rax
    mov (r_RBX), %rbx
    mov (r_RCX), %rcx
    mov (r_RDX), %rdx
    mov (r_RSI), %rsi
    mov (r_RDI), %rdi
    mov (r_RSP), %rsp
    mov (r_RBP), %rbp
    mov (r_R8), %r8
    mov (r_R9), %r9
    mov (r_R10), %r10
    mov (r_R11), %r11
    mov (r_R12), %r12
    mov (r_R13), %r13
    mov (r_R14), %r14
    mov (r_R15), %r15

    jmp 0x0(%rip)
printRegs_return: .quad 0
