# hole: fibonacci

mov $0, %rsp

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
mov $0x2000000000000001, %rax
mov $0xA000000000000010, %rbx
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

# --- Clean exit
mov $60, %eax
mov $0, %edi
syscall
