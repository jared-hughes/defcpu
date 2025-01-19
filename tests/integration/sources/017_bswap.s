# hole: fibonacci

xor %rsp, %rsp

mov $0x0102030405060708, %rax
mov $0xA1B2C3D4E5F6A7B8, %rbx
bswap %rax
bswap %rbx
print_regs

mov $0x0102030405060708, %rax
mov $0xA1B2C3D4E5F6A7B8, %rbx
bswap %eax
bswap %ebx
print_regs

# --- Clean exit
mov $60, %eax
mov $0, %edi
syscall
