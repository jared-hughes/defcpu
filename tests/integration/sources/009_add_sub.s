# hole: fibonacci

mov $0, %rsp

mov $0x0102030405060708, %rax
mov $0xA1B1C1D1E1F1A1B1, %rbx
print_regs

add %rbx, %rax
print_regs

sub %rbx, %rax
print_regs

sub %rbx, %rax
hlt
