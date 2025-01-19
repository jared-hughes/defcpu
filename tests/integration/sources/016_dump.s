# hole: rock-paper-scissors-spock-lizard
SYS_WRITE = 1
SYS_EXIT = 60
STDOUT_FILENO = 1

.text
dump_initial

print_regs

mov $argc, %rsi
call printsz

pop %rax
push %rax
call printDec # argc (one eighbyte)
pop %rbx

mov $nl, %rsi
call printsz

# always at least one arg because $0 = /tmp/asm
argLoop:  
	pop %rsi # pop the next arg pointer
	push %rsi
	mov $ox, %rsi; call printsz;
	pop %rax; push %rax; call printQuad;
	mov $colon_space, %rsi; call printsz;
	pop %rsi; call printsz;
	mov $nl, %rsi; call printsz
  
	dec %rbx
	jnz argLoop
endArgLoop:

call print_stack_quad # .quad 0
# one eightbyte each of environment pointers (none on code.golf)
call print_stack_quad # .quad 0
# begin auxiliary vector entries:
call print_auxv # a_type=33 (AT_SYSINFO_EHDR), a_un = pointer to start of ELF file? Always &0xFFF = 0x000
call print_auxv # a_type=51 (AT_MINSIGSTKSZ), a_val = 0xD30
call print_auxv # a_type=16 (AT_HWCAP), a_val = 0x178BFBFF
call print_auxv # a_type=6 (AT_PAGESZ), a_val = 0x1000
call print_auxv # a_type=17 (AT_CLKTCK), a_val = 0x64
call print_auxv # a_type=3 (AT_PHDR), a_ptr = 0x0
call print_auxv # a_type=4 (AT_PHENT), a_val = 0x38
call print_auxv # a_type=5 (AT_PHNUM), a_val = 0x3
call print_auxv # a_type=7 (AT_BASE), a_ptr = 0x0
call print_auxv # a_type=8 (AT_FLAGS), a_val = 0x0
call print_auxv # a_type=9 (AT_ENTRY), a_ptr = 0x400000
call print_auxv # a_type=11 (AT_UID), a_val = 0xFFFE
call print_auxv # a_type=12 (AT_EUID), a_val = 0xFFFE
call print_auxv # a_type=13 (AT_GID), a_val = 0xFFFE
call print_auxv # a_type=14 (AT_EGID), a_val = 0xFFFE
call print_auxv # a_type=23 (AT_SECURE), a_val = 0x0
call print_auxv # a_type=25 (AT_RANDOM), a_un = stack pointer 
# (to 16 random bytes. always +0x1 or +0x9 on the info_block_end, whatever ends in 0x9)
call print_auxv # a_type=26 (AT_HWCAP2), a_val = 0x2 
# (docs say it should be zero)
call print_auxv # a_type=31 (AT_EXECFN), a_ptr = stack pointer
# (to char* filename "/tmp/asm". always 0xFEF in address on a page.
# Either the page after info_block_end, or the same page,
# precisely ((info_block_end + [0x0 or 0x1000]) & ~0xFFF) | 0xFEF)
call print_auxv # a_type=15 (AT_PLATFORM), a_ptr = stack pointer
# (to char* platform "x86_64". always +0x11 or +0x19 on the info_block_end, whatever ends in 0x9)
call print_auxv # null auxiliary vector entry
# unspecified:
# call print_stack_quad # .quad 0

mov $finished_aux_vec, %rsi; call printsz

mov %rsp, info_block_end
mov %rsp, %rbx
mov at_random, %rcx; sub %rbx, %rcx; mov $at_random_str, %rsi; call printsz;
mov %rcx, %rax; call printQuad; mov $nl, %rsi; call printsz;
mov at_platform, %rcx; sub %rbx, %rcx; mov $at_platform_str, %rsi; call printsz;
mov %rcx, %rax; call printQuad; mov $nl, %rsi; call printsz;
mov at_execfn, %rcx; sub %rbx, %rcx; mov $at_execfn_str, %rsi; call printsz;
mov %rcx, %rax; call printQuad; mov $nl, %rsi; call printsz;
mov at_33, %rcx; sub %rbx, %rcx; mov $at_33_str, %rsi; call printsz;
mov %rcx, %rax; call printQuad; mov $nl, %rsi; call printsz;


mov %rsp, %rsi
call hexdump_page


mov at_execfn, %rbx
cmp %rbx, %rsi
jge already_dumped_execfn
	call hexdump_page
already_dumped_execfn:
# the pages after at_execfn are not readable.

mov at_33, %rsi
call hexdump_page

# segfault expected here. seems to only have one page at 33.
add $0x1000, %rsi
call hexdump_page

jmp exit

.data
at_33: .quad 0
at_random: .quad 0
at_execfn: .quad 0
at_platform: .quad 0
info_block_end: .quad 0

.text

print_stack_quad:
	pop %r8 # return rip

	mov $ox, %rsi; call printsz
	pop %rax; call printQuad
	mov $nl, %rsi; call printsz

	push %r8
	ret


# Pops two quadwords, treats it as auxv_t (see https://gitlab.com/x86-psABIs/x86-64-ABI)
# The first quadword is a_type. The second is a_un, which may be anything (pointer or value)
print_auxv:
	pop %r8 # return rip

	mov $a_type_eq, %rsi
	call printsz

	pop %rax
	mov %rax, %r12
	call printDec

	mov $auxv_mid, %rsi
	call printsz

	pop %rax
	mov %rax, %r13
	call printQuad

	cmp $33, %r12
	je char_star_33
	cmp $31, %r12
	je char_star_31
	cmp $15, %r12
	je char_star_15
	cmp $25, %r12
	je sixteen_random
	jmp done_branch
	sixteen_random:
		mov $at_random, %eax; mov %r13, (%eax);
		mov $sixteen_open, %rsi; call printsz
		mov $nl, %rsi; call printsz
		mov %r13, %rsi; call print16
		mov $sixteen_close, %rsi; call printsz
		jmp done_branch
	char_star_33: mov $at_33, %eax; mov %r13, (%eax); jmp char_star
	char_star_31: mov $at_execfn, %eax; mov %r13, (%eax); jmp char_star
	char_star_15: mov $at_platform, %eax; mov %r13, (%eax); jmp char_star
	char_star:
		mov $str_open, %rsi; call printsz
		
  		mov %r13, %rsi

		call printsz
		
		mov $str_close, %rsi
		call printsz
	done_branch:

	mov $nl, %rsi
	call printsz

	push %r8
	ret


.data
a_type_eq: .asciz "a_type="
colon_space: .asciz ": "
nl: .asciz "\n"
auxv_mid: .asciz ",\ta_un=0x"
ox: .asciz "0x"
argc: .asciz "argc="
str_open: .asciz " (char*) (\""
str_close: .asciz "\")"
sixteen_open: .asciz " (char[16]) (\""
sixteen_close: .asciz "\")"
finished_aux_vec: .asciz "Finished reading aux vector entries. Offsets:\n"
at_33_str: .asciz "AT 33 (??):\t+0x"
at_random_str: .asciz "AT_RANDOM:\t+0x"
at_execfn_str: .asciz "AT_EXECFN:\t+0x"
at_platform_str: .asciz "AT_PLATFORM:\t+0x"

.text
# print the null-terminated string in %rsi
printsz:
	push %rax; push %rcx
	mov %rsi, %rdi

    mov $-1, %ecx
    xor %al, %al
    repnz scasb

    not %ecx
	dec %ecx
	# length in %ecx

    mov %ecx, %edx
    mov $SYS_WRITE, %eax
    mov $STDOUT_FILENO, %edi
    syscall
	pop %rcx; pop %rax
	ret

.data
dec_buf: .quad 0,0,0
dec_buf_end: .byte '\n'
hexdump_buf: .ascii ": "
hexdump_buf_hex: .ascii "0000 0000 0000 0000 0000 0000 0000 0000  "
hexdump_buf_chars: .ascii "aaaaaaaaaaaaaaaa\n"
hexdump_buf_len = . - hexdump_buf

.text
# Print the value of %rax in decimal. Uses X quadwords of  stack.
printDec:
	mov $dec_buf_end, %esi
	push %rsi # push start ptr

	mov $10, %ebx # ebx = 10
	xor %edx, %edx

    for_digit_in_num:
    dec %esi;
    div %ebx # divide %edx:%eax by %ebx=10, %eax=quotient, %edx=remainder.
    add $'0', %dl
	mov %dl, (%rsi) # P
	cltd # %edx = 0.
    cmp $0, %eax
    jnz for_digit_in_num 

	pop %rdx
	sub %esi, %edx # edx = count
    mov $SYS_WRITE, %eax
    mov $STDOUT_FILENO, %edi
    syscall

	ret

# write %al as two nibble chars
printByte:
	mov $hex_buf, %rdi
	call writeByte
	
	mov $hex_buf, %esi
	mov $2, %edx
    mov $SYS_WRITE, %eax
    mov $STDOUT_FILENO, %edi
    syscall

# move %al as two nibble chars to (%rdi) and 0x1(%rdi)
writeByte:
	xor %edx, %edx
	
    rol $4, %al
	mov %al, %dl
    and $0xF, %dl
	mov hexaChars(%rdx), %dl
	mov %dl, (%rdi)

    rol $4, %al
	mov %al, %dl
    and $0xF, %dl
	mov hexaChars(%rdx), %dl
	mov %dl, 0x1(%rdi)

	ret

# hexdump starting from %rsi until the end of the page.
hexdump_page:
  	push %rsi; mov $nl, %rsi; call printsz; call printsz; call printsz; pop %rsi
	and $~0xF, %rsi
	hexdump_loop:
  		call print16;add $16, %rsi
  		mov %esi, %eax
		and $0xFF0, %eax
  		jnz hexdump_loop
	ret

# Print 16 chars, starting at %rsi, in hexdump format
print16:
	mov %rsi, %r13
	mov %rsi, %rax;	call printQuad

	mov %r13, %rsi
    mov $hexdump_buf_hex, %rdi
	mov $16, %ecx
	for_byte_start:
    	mov (%rsi), %al;
    	call writeByte
		bt $0, %ecx
		adc $2, %rdi
		inc %rsi
		dec %ecx
	jnz for_byte_start

	mov %r13, %rsi
    mov $hexdump_buf_chars, %rdi
	mov $16, %ecx
	for_char_start:
    	mov (%rsi), %al;
		cmp $32, %al
		jb bad
		cmp $126, %al
		ja bad
		jmp good
		bad:
			mov $'.', %al
		good:
		mov %al, (%rdi)
		inc %rdi
		inc %rsi
		dec %ecx
	jnz for_char_start


	# print the hexdump
	mov $hexdump_buf, %esi
	mov $hexdump_buf_len, %edx
    mov $SYS_WRITE, %eax
    mov $STDOUT_FILENO, %edi
    syscall

	mov %r13, %rsi
	ret
	

.data
hexaChars: .string "0123456789ABCDEF"
hex_buf: .byte 0,0

.text
# Print the value of %rax in 16 hexadecimal characters. Uses 6 quadwords of stack.
printQuad:
    push %rbx
    push %rdx
    push %rsi
    push %rdi
    push %rcx
    
    mov $16, %ebx
    mov $hex_buf, %esi
    
    nibbleLoop:
        rol $4, %rax
        mov %eax, %edx
        and $0xF, %edx
        mov hexaChars(%rdx), %dl
        mov %dl, (%rsi)

        # Print the nibble
        push %rax
            mov $1, %dl
            mov $SYS_WRITE, %eax
            mov $STDOUT_FILENO, %edi
            syscall
        pop %rax

        dec %ebx
        jnz nibbleLoop
    
    pop %rcx
    pop %rdi
    pop %rsi
    pop %rdx
    pop %rbx
    
	ret

.text
exit:
	mov $SYS_EXIT, %eax
	mov $0, %edi
	syscall
