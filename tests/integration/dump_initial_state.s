# Dump the intial state of the program execution.
# It's not strictly a memory dump -- I'm trying to print
# the minimal necessary information to reconstruct the state.
# This should just be the program arguments, plus any offsets needed
# to determine the consequences of address-space layout randomization (ASLR)

# usage:
#
# jmp dump_initial
# dump_initial_ret:
#
# Don't call with `call` since that goobles the stack.

.data

# Note we make sure it's impossible to have trailing spaces,
# since code.golf trims those.
dump_contents:
.asciz "Unpredictables:"
.asciz "\nrand16"
# contents of the 16 bytes at entry 25 (AT_RANDOM) of the auxv.
# 16 bytes is 32 nibbles, so 4 quadwords.
dump_random_16: .quad 0, 0, 0, 0
.asciz "\nVDSO\0\0"
dump_vdso_ptr: .quad 0,0 # entry 33 (AT_SYSINFO_EHDR) of the auxv.
.asciz "\nexecfn"
dump_execfn_ptr: .quad 0,0 # entry 31 (AT_EXECFN) of the auxv.
.asciz "\npltoff"
dump_platform_offset: .quad 0,0 # distance from entry 15 (AT_PLATFORM) of the auxv to argv[0].
.asciz "\nargc\0\0"
dump_argc: .quad 0,0 # number of arguments
.asciz "\narglen"
dump_arglen: .quad 0,0 # total number of bytes arguments take up, including null terminators.
dump_nlnl: .ascii "\n\n"
dump_len = . - dump_contents

dump_hexChars: .ascii "0123456789ABCDEF"
dump_execfn_ptr_val: .quad 0

dump_printQuad_ret: .quad 0

.text
dump_initial:
    mov %rsp, %r8 # %r8 = initial %rsp.
	pushfq

	mov %r8, %rsp
    
    # Below here, don't push to the stack.
    # Using %rsp sneakily to look into the middle of the stack.

    # get argc
    pop %rax
    mov %rax, %rbx
    mov $dump_argc, %rdi
    movq $done_printing_argc, dump_printQuad_ret
    jmp dump_printQuad
    done_printing_argc:

    pop %rbp # rbp points to the first argument /tmp/asm.

    # Skip the arg pointers.
    # The null separator after argv (before envp)
	# is at 0x08 + 8 * argc + initial %rsp.
	# There's no envp, so the null separtor after envp (before auxv)
	# is at 0x10 + 8 * argc + initial %rsp.
    # so the first env pointer is 0x08 after that.
    lea 0x18(%r8, %rax, 8), %rsp

    # rsp is now the pointer to the first env pointer.
    # get the relevant environment pointers
    dump_at_loop:
        pop %rax # rax is the at_type.
        pop %rbx # rbx is the a_un (a_ptr or a_val).
        cmp $0, %rax
        jz dump_at_loop_done
        cmp $25, %rax; jz dump_at_type_25
        cmp $31, %rax; jz dump_at_type_31
        cmp $33, %rax; jz dump_at_type_33
        cmp $15, %rax; jnz dump_at_loop
        dump_at_type_15: # AT_PLATFORM
			mov $0x100, %ecx
            mov %rbx, %rdi
            mov $0, %al
            repne scasb
            # %rdi now points to the nul terminator of the platform string
            mov %rbp, %rbx
            sub %rdi, %rbx
            # rbx is now the distance from the nul terminator of the
            # platform string "x86_64", to argv[0].
            # decrement due to growing down.
            dec %rbx
            mov $dump_platform_offset, %edi
            movq $dump_at_loop, dump_printQuad_ret
            jmp dump_printQuad
        dump_at_type_25: # AT_RANDOM
            mov %rbx, %rax
            movq (%rax), %rbx
            bswap %rbx
            movq $after_first_quad_25, dump_printQuad_ret
            movq $dump_random_16, %edi
            jmp dump_printQuad

            after_first_quad_25:
            movq 0x8(%rax), %rbx
            bswap %rbx
            movq $dump_at_loop, dump_printQuad_ret
            jmp dump_printQuad
        dump_at_type_33: # AT_SYSINFO_EHDR
            mov $dump_vdso_ptr, %edi
            movq $dump_at_loop, dump_printQuad_ret
            jmp dump_printQuad
        dump_at_type_31: # AT_EXECFN
			mov %rbx, dump_execfn_ptr_val
            mov $dump_execfn_ptr, %edi
            movq $dump_at_loop, dump_printQuad_ret
            jmp dump_printQuad
    dump_at_loop_done:
    # all auxv entries popped, including the null entry.


    # calculate arglen
    mov dump_execfn_ptr_val, %rbx
    mov %rbp, %rsi # rsi = pointer to first arg
    sub %rsi, %rbx # rbx = arglen (bytes)

    # dump arglen
    movq $dump_arglen, %rdi
    movq $done_printing_arglen, dump_printQuad_ret
    jmp dump_printQuad
    done_printing_arglen:

	mov $dump_contents, %rsi
	mov $dump_len, %edx
	mov $1, %eax # sys_write
	mov $1, %edi # stdout
	syscall

    # calculate arglen
    mov dump_execfn_ptr_val, %rdx
    mov %rbp, %rsi # rsi = pointer to first arg
    sub %rsi, %rdx # rdx = arglen = count (bytes)
    mov $1, %eax # sys_write
    mov $1, %edi # stdout
    syscall

    mov $dump_nlnl, %esi
    mov $2, %edx
    mov $1, %eax # sys_write
    mov $1, %edi #stdout

	# Reset all registers to 0, except %rsp which should be back to normal. 
    mov %r8, %rsp
	# but we also need to zero the extra quadword written to store flags
	add $-0x8, %rsp
	popfq
	xor %eax, %eax
	mov %rax, -0x8(%rsp)
	xor %ecx, %ecx
	xor %edx, %edx
	xor %esi, %esi
	xor %edi, %edi
	xor %rbp, %rbp
	xor %r11, %r11
	xor %r8, %r8
    jmp dump_initial_ret

# print %rbx (8 bytes) as nibbles, to %rdi (16 bytes of nibbles)
# returns to dump_printQuad_ret
dump_printQuad:
    mov $16, %ecx
    xor %edx, %edx
    dump_random_nibble_loop:
        rol $4, %rbx
        mov %bl, %dl
        and $0xF, %dl
        mov dump_hexChars(%rdx), %dl
        mov %dl, (%rdi)

        inc %rdi
        dec %ecx
        jnz dump_random_nibble_loop
    jmp *dump_printQuad_ret
