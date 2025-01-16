# hole: fibonacci

mov $0, %rsp

stc
print_regs

clc
print_regs

# --- 64-bit add

# plain
mov $0x0102030405060708, %rax
mov $0xA1B1C1D1E1F1A1B1, %rbx
mov %rax, %rcx
add %rbx, %rcx
print_regs

# carry
mov $0x6000000000000001, %rax
mov $0xA000000000000000, %rbx
mov %rax, %rcx
add %rbx, %rcx
print_regs

# overflow and carry
mov $0x9000000000000001, %rax
mov $0xA000000000000010, %rbx
mov %rax, %rcx
add %rbx, %rcx  
print_regs

# overflow
mov $0x7000000000000007, %rax
mov $0x7000000000000000, %rbx
mov %rax, %rcx
add %rbx, %rcx  
print_regs

# AF flag.
mov $0x7000000000000008, %rax
mov $0x7000000000000008, %rbx
mov %rax, %rcx
add %rbx, %rcx  
print_regs

# Zero.
mov $0x7FFFFFFFFFFFFFFF, %rax
mov $0x8000000000000001, %rbx
mov %rax, %rcx
add %rbx, %rcx  
print_regs

# --- 32-bit add

# plain
mov $0x01020304, %eax
mov $0xA1B1C1D1, %ebx
mov %eax, %ecx
add %ebx, %ecx
print_regs

# carry
mov $0x60000001, %eax
mov $0xA0000000, %ebx
mov %eax, %ecx
add %ebx, %ecx
print_regs

# overflow and carry
mov $0x90000001, %eax
mov $0xA0000010, %ebx
mov %eax, %ecx
add %ebx, %ecx  
print_regs

# overflow
mov $0x70000007, %eax
mov $0x70000000, %ebx
mov %eax, %ecx
add %ebx, %ecx  
print_regs

# AF flag.
mov $0x70000008, %eax
mov $0x70000008, %ebx
mov %eax, %ecx
add %ebx, %ecx  
print_regs

# Zero.
mov $0x7FFFFFFF, %eax
mov $0x80000001, %ebx
mov %eax, %ecx
add %ebx, %ecx  
print_regs

# --- 16-bit add

# plain
mov $0x0102, %ax
mov $0xA1B1, %bx
mov %ax, %cx
add %bx, %cx
print_regs

# carry
mov $0x6001, %ax
mov $0xA000, %bx
mov %ax, %cx
add %bx, %cx
print_regs

# overflow and carry
mov $0x9001, %ax
mov $0xA010, %bx
mov %ax, %cx
add %bx, %cx  
print_regs

# overflow
mov $0x7007, %ax
mov $0x7000, %bx
mov %ax, %cx
add %bx, %cx  
print_regs

# AF flag.
mov $0x7008, %ax
mov $0x7008, %bx
mov %ax, %cx
add %bx, %cx  
print_regs

# Zero.
mov $0x7FFF, %ax
mov $0x8001, %bx
mov %ax, %cx
add %bx, %cx  
print_regs

# --- 8-bit add

# plain
mov $0x01, %al
mov $0xA1, %bl
mov %al, %cl
add %bl, %cl
print_regs

# carry
mov $0x61, %al
mov $0xA0, %bl
mov %al, %cl
add %bl, %cl
print_regs

# overflow and carry
mov $0x91, %al
mov $0xA0, %bl
mov %al, %cl
add %bl, %cl  
print_regs

# overflow
mov $0x77, %al
mov $0x70, %bl
mov %al, %cl
add %bl, %cl  
print_regs

# AF flag.
mov $0x78, %al
mov $0x78, %bl
mov %al, %cl
add %bl, %cl  
print_regs

# Zero.
mov $0x7F, %al
mov $0x81, %bl
mov %al, %cl
add %bl, %cl  
print_regs

# --- 64-bit adc

# plain
mov $0x0102030405060708, %rax
mov $0xA1B1C1D1E1F1A1B1, %rbx
mov %rax, %rcx
stc; adc %rbx, %rcx
print_regs

# carry
mov $0x6000000000000001, %rax
mov $0xA000000000000000, %rbx
mov %rax, %rcx
stc; adc %rbx, %rcx
print_regs

# overflow and carry
mov $0x9000000000000001, %rax
mov $0xA000000000000010, %rbx
mov %rax, %rcx
stc; adc %rbx, %rcx  
print_regs

# overflow
mov $0x7000000000000007, %rax
mov $0x7000000000000000, %rbx
mov %rax, %rcx
stc; adc %rbx, %rcx  
print_regs

# AF flag.
mov $0x7000000000000008, %rax
mov $0x7000000000000008, %rbx
mov %rax, %rcx
stc; adc %rbx, %rcx  
print_regs

# Zero.
mov $0x7FFFFFFFFFFFFFFF, %rax
mov $0x8000000000000001, %rbx
mov %rax, %rcx
stc; adc %rbx, %rcx  
print_regs

# --- 32-bit adc

# plain
mov $0x01020304, %eax
mov $0xA1B1C1D1, %ebx
mov %eax, %ecx
stc; adc %ebx, %ecx
print_regs

# carry
mov $0x60000001, %eax
mov $0xA0000000, %ebx
mov %eax, %ecx
stc; adc %ebx, %ecx
print_regs

# overflow and carry
mov $0x90000001, %eax
mov $0xA0000010, %ebx
mov %eax, %ecx
stc; adc %ebx, %ecx  
print_regs

# overflow
mov $0x70000007, %eax
mov $0x70000000, %ebx
mov %eax, %ecx
stc; adc %ebx, %ecx  
print_regs

# AF flag.
mov $0x70000008, %eax
mov $0x70000008, %ebx
mov %eax, %ecx
stc; adc %ebx, %ecx  
print_regs

# Zero.
mov $0x7FFFFFFF, %eax
mov $0x80000001, %ebx
mov %eax, %ecx
stc; adc %ebx, %ecx  
print_regs

# --- 16-bit adc

# plain
mov $0x0102, %ax
mov $0xA1B1, %bx
mov %ax, %cx
stc; adc %bx, %cx
print_regs

# carry
mov $0x6001, %ax
mov $0xA000, %bx
mov %ax, %cx
stc; adc %bx, %cx
print_regs

# overflow and carry
mov $0x9001, %ax
mov $0xA010, %bx
mov %ax, %cx
stc; adc %bx, %cx  
print_regs

# overflow
mov $0x7007, %ax
mov $0x7000, %bx
mov %ax, %cx
stc; adc %bx, %cx  
print_regs

# AF flag.
mov $0x7008, %ax
mov $0x7008, %bx
mov %ax, %cx
stc; adc %bx, %cx  
print_regs

# Zero.
mov $0x7FFF, %ax
mov $0x8001, %bx
mov %ax, %cx
stc; adc %bx, %cx  
print_regs

# --- 8-bit adc

# plain
mov $0x01, %al
mov $0xA1, %bl
mov %al, %cl
stc; adc %bl, %cl
print_regs

# carry
mov $0x61, %al
mov $0xA0, %bl
mov %al, %cl
stc; adc %bl, %cl
print_regs

# overflow and carry
mov $0x91, %al
mov $0xA0, %bl
mov %al, %cl
stc; adc %bl, %cl  
print_regs

# overflow
mov $0x77, %al
mov $0x70, %bl
mov %al, %cl
stc; adc %bl, %cl  
print_regs

# AF flag.
mov $0x78, %al
mov $0x78, %bl
mov %al, %cl
stc; adc %bl, %cl  
print_regs

# Zero.
mov $0x7F, %al
mov $0x81, %bl
mov %al, %cl
stc; adc %bl, %cl  
print_regs

# --- 64-bit sub

# plain
mov $0xA1B1C1D1E1F1A1B1, %rax
mov $0x0102030405060708, %rbx
mov %rax, %rcx
sub %rbx, %rcx
print_regs

# carry (borrow)
mov $0x2000000000000001, %rax
mov $0x4000000000000000, %rbx
mov %rax, %rcx
sub %rbx, %rcx
print_regs

# overflow and carry
mov $0x7000000000000007, %rax
mov $0xA000000000000000, %rbx
mov %rax, %rcx
sub %rbx, %rcx  
print_regs

# overflow
mov $0xA000000000000001, %rax
mov $0x7000000000000010, %rbx
mov %rax, %rcx
sub %rbx, %rcx  
print_regs

# AF flag.
mov $0x7000000000000007, %rax
mov $0x7000000000000008, %rbx
mov %rax, %rcx
sub %rbx, %rcx  
print_regs

# Zero.
mov $0x8000000000000001, %rax
mov $0x8000000000000001, %rbx
mov %rax, %rcx
sub %rbx, %rcx  
print_regs

# --- 32-bit sub

# plain
mov $0xA1B1C1D1, %eax
mov $0x01020304, %ebx
mov %eax, %ecx
sub %ebx, %ecx
print_regs

# carry (borrow)
mov $0x20000001, %eax
mov $0x40000000, %ebx
mov %eax, %ecx
sub %ebx, %ecx
print_regs

# overflow and carry
mov $0x70000007, %eax
mov $0xA0000000, %ebx
mov %eax, %ecx
sub %ebx, %ecx  
print_regs

# overflow
mov $0xA0000001, %eax
mov $0x70000010, %ebx
mov %eax, %ecx
sub %ebx, %ecx  
print_regs

# AF flag.
mov $0x70000007, %eax
mov $0x70000008, %ebx
mov %eax, %ecx
sub %ebx, %ecx  
print_regs

# Zero.
mov $0x80000001, %eax
mov $0x80000001, %ebx
mov %eax, %ecx
sub %ebx, %ecx  
print_regs

# --- 16-bit sub

# plain
mov $0xA1B1, %ax
mov $0x0102, %bx
mov %ax, %cx
sub %bx, %cx
print_regs

# carry (borrow)
mov $0x2001, %ax
mov $0x4000, %bx
mov %ax, %cx
sub %bx, %cx
print_regs

# overflow and carry
mov $0x7007, %ax
mov $0xA000, %bx
mov %ax, %cx
sub %bx, %cx  
print_regs

# overflow
mov $0xA001, %ax
mov $0x7010, %bx
mov %ax, %cx
sub %bx, %cx  
print_regs

# AF flag.
mov $0x7007, %ax
mov $0x7008, %bx
mov %ax, %cx
sub %bx, %cx  
print_regs

# Zero.
mov $0x8001, %ax
mov $0x8001, %bx
mov %ax, %cx
sub %bx, %cx  
print_regs

# --- 8-bit sub

# plain
mov $0xA1, %al
mov $0x01, %bl
mov %al, %cl
sub %bl, %cl
print_regs

# carry (borrow)
mov $0x21, %al
mov $0x40, %bl
mov %al, %cl
sub %bl, %cl
print_regs

# overflow and carry
mov $0x77, %al
mov $0xA0, %bl
mov %al, %cl
sub %bl, %cl  
print_regs

# overflow
mov $0xA1, %al
mov $0x70, %bl
mov %al, %cl
sub %bl, %cl  
print_regs

# AF flag.
mov $0x77, %al
mov $0x78, %bl
mov %al, %cl
sub %bl, %cl  
print_regs

# Zero.
mov $0x81, %al
mov $0x81, %bl
mov %al, %cl
sub %bl, %cl  
print_regs

# --- 64-bit sbb

# plain
mov $0xA1B1C1D1E1F1A1B1, %rax
mov $0x0102030405060708, %rbx
mov %rax, %rcx
stc; sbb %rbx, %rcx
print_regs

# carry (borrow)
mov $0x2000000000000001, %rax
mov $0x4000000000000000, %rbx
mov %rax, %rcx
stc; sbb %rbx, %rcx
print_regs

# overflow and carry
mov $0x7000000000000007, %rax
mov $0xA000000000000000, %rbx
mov %rax, %rcx
stc; sbb %rbx, %rcx  
print_regs

# overflow
mov $0xA000000000000001, %rax
mov $0x7000000000000010, %rbx
mov %rax, %rcx
stc; sbb %rbx, %rcx  
print_regs

# AF flag.
mov $0x7000000000000007, %rax
mov $0x7000000000000008, %rbx
mov %rax, %rcx
stc; sbb %rbx, %rcx  
print_regs

# Zero.
mov $0x8000000000000001, %rax
mov $0x8000000000000001, %rbx
mov %rax, %rcx
stc; sbb %rbx, %rcx  
print_regs

# --- 32-bit sbb

# plain
mov $0xA1B1C1D1, %eax
mov $0x01020304, %ebx
mov %eax, %ecx
stc; sbb %ebx, %ecx
print_regs

# carry (borrow)
mov $0x20000001, %eax
mov $0x40000000, %ebx
mov %eax, %ecx
stc; sbb %ebx, %ecx
print_regs

# overflow and carry
mov $0x70000007, %eax
mov $0xA0000000, %ebx
mov %eax, %ecx
stc; sbb %ebx, %ecx  
print_regs

# overflow
mov $0xA0000001, %eax
mov $0x70000010, %ebx
mov %eax, %ecx
stc; sbb %ebx, %ecx  
print_regs

# AF flag.
mov $0x70000007, %eax
mov $0x70000008, %ebx
mov %eax, %ecx
stc; sbb %ebx, %ecx  
print_regs

# Zero.
mov $0x80000001, %eax
mov $0x80000001, %ebx
mov %eax, %ecx
stc; sbb %ebx, %ecx  
print_regs

# --- 16-bit sbb

# plain
mov $0xA1B1, %ax
mov $0x0102, %bx
mov %ax, %cx
stc; sbb %bx, %cx
print_regs

# carry (borrow)
mov $0x2001, %ax
mov $0x4000, %bx
mov %ax, %cx
stc; sbb %bx, %cx
print_regs

# overflow and carry
mov $0x7007, %ax
mov $0xA000, %bx
mov %ax, %cx
stc; sbb %bx, %cx  
print_regs

# overflow
mov $0xA001, %ax
mov $0x7010, %bx
mov %ax, %cx
stc; sbb %bx, %cx  
print_regs

# AF flag.
mov $0x7007, %ax
mov $0x7008, %bx
mov %ax, %cx
stc; sbb %bx, %cx  
print_regs

# Zero.
mov $0x8001, %ax
mov $0x8001, %bx
mov %ax, %cx
stc; sbb %bx, %cx  
print_regs

# --- 8-bit sbb

# plain
mov $0xA1, %al
mov $0x01, %bl
mov %al, %cl
stc; sbb %bl, %cl
print_regs

# carry (borrow)
mov $0x21, %al
mov $0x40, %bl
mov %al, %cl
stc; sbb %bl, %cl
print_regs

# overflow and carry
mov $0x77, %al
mov $0xA0, %bl
mov %al, %cl
stc; sbb %bl, %cl  
print_regs

# overflow
mov $0xA1, %al
mov $0x70, %bl
mov %al, %cl
stc; sbb %bl, %cl  
print_regs

# AF flag.
mov $0x77, %al
mov $0x78, %bl
mov %al, %cl
stc; sbb %bl, %cl  
print_regs

# Zero.
mov $0x81, %al
mov $0x81, %bl
mov %al, %cl
stc; sbb %bl, %cl  
print_regs

# --- 64-bit cmp

# plain
mov $0xA1B1C1D1E1F1A1B1, %rax
mov $0x0102030405060708, %rbx
mov %rax, %rcx
cmp %rbx, %rcx
print_regs

# carry (borrow)
mov $0x2000000000000001, %rax
mov $0x4000000000000000, %rbx
mov %rax, %rcx
cmp %rbx, %rcx
print_regs

# overflow and carry
mov $0x7000000000000007, %rax
mov $0xA000000000000000, %rbx
mov %rax, %rcx
cmp %rbx, %rcx  
print_regs

# overflow
mov $0xA000000000000001, %rax
mov $0x7000000000000010, %rbx
mov %rax, %rcx
cmp %rbx, %rcx  
print_regs

# AF flag.
mov $0x7000000000000007, %rax
mov $0x7000000000000008, %rbx
mov %rax, %rcx
cmp %rbx, %rcx  
print_regs

# Zero.
mov $0x8000000000000001, %rax
mov $0x8000000000000001, %rbx
mov %rax, %rcx
cmp %rbx, %rcx  
print_regs

# --- 32-bit cmp

# plain
mov $0xA1B1C1D1, %eax
mov $0x01020304, %ebx
mov %eax, %ecx
cmp %ebx, %ecx
print_regs

# carry (borrow)
mov $0x20000001, %eax
mov $0x40000000, %ebx
mov %eax, %ecx
cmp %ebx, %ecx
print_regs

# overflow and carry
mov $0x70000007, %eax
mov $0xA0000000, %ebx
mov %eax, %ecx
cmp %ebx, %ecx  
print_regs

# overflow
mov $0xA0000001, %eax
mov $0x70000010, %ebx
mov %eax, %ecx
cmp %ebx, %ecx  
print_regs

# AF flag.
mov $0x70000007, %eax
mov $0x70000008, %ebx
mov %eax, %ecx
cmp %ebx, %ecx  
print_regs

# Zero.
mov $0x80000001, %eax
mov $0x80000001, %ebx
mov %eax, %ecx
cmp %ebx, %ecx  
print_regs

# --- 16-bit cmp

# plain
mov $0xA1B1, %ax
mov $0x0102, %bx
mov %ax, %cx
cmp %bx, %cx
print_regs

# carry (borrow)
mov $0x2001, %ax
mov $0x4000, %bx
mov %ax, %cx
cmp %bx, %cx
print_regs

# overflow and carry
mov $0x7007, %ax
mov $0xA000, %bx
mov %ax, %cx
cmp %bx, %cx  
print_regs

# overflow
mov $0xA001, %ax
mov $0x7010, %bx
mov %ax, %cx
cmp %bx, %cx  
print_regs

# AF flag.
mov $0x7007, %ax
mov $0x7008, %bx
mov %ax, %cx
cmp %bx, %cx  
print_regs

# Zero.
mov $0x8001, %ax
mov $0x8001, %bx
mov %ax, %cx
cmp %bx, %cx  
print_regs

# --- 8-bit cmp

# plain
mov $0xA1, %al
mov $0x01, %bl
mov %al, %cl
cmp %bl, %cl
print_regs

# carry (borrow)
mov $0x21, %al
mov $0x40, %bl
mov %al, %cl
cmp %bl, %cl
print_regs

# overflow and carry
mov $0x77, %al
mov $0xA0, %bl
mov %al, %cl
cmp %bl, %cl  
print_regs

# overflow
mov $0xA1, %al
mov $0x70, %bl
mov %al, %cl
cmp %bl, %cl  
print_regs

# AF flag.
mov $0x77, %al
mov $0x78, %bl
mov %al, %cl
cmp %bl, %cl  
print_regs

# Zero.
mov $0x81, %al
mov $0x81, %bl
mov %al, %cl
cmp %bl, %cl  
print_regs

# --- Clean exit
mov $60, %eax
mov $0, %edi
syscall
