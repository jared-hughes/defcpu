# hole: fibonacci

mov $0, %rsp

mov $0x1234567812345678, %rdx
mov $0x0000, %ax
cwtd
print_regs

mov $0x1234567812345678, %rdx
mov $0x8000, %ax
cwtd
print_regs

mov $0x1234567812345678, %rdx
mov $0x00000000, %eax
cltd
print_regs

mov $0x1234567812345678, %rdx
mov $0x80000000, %eax
cltd
print_regs

mov $0x1234567812345678, %rdx
mov $0x0000000000000, %rax
cqto
print_regs

mov $0x1234567812345678, %rdx
mov $0x8000000000000, %rax
cqto
print_regs

mov $0x1234567812345678, %rdx
mov $0x00, %al
cbtw
print_regs

mov $0x1234567812345678, %rdx
mov $0x80, %al
cbtw
print_regs

mov $0x1234567812345678, %rdx
mov $0x0000, %ax
cwtl
print_regs

mov $0x1234567812345678, %rdx
mov $0x8000, %ax
cwtl
print_regs

mov $0x1234567812345678, %rdx
mov $0x00000000, %eax
cltq
print_regs

mov $0x1234567812345678, %rdx
mov $0x80000000, %eax
cltq
print_regs
