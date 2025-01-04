# hole: fibonacci

.data

stack: .quad 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 
stack_top: .quad 0

.text
mov $stack_top, %rsp

# --- 64-bit div

mov $0x123456789ABCDEF0, %rdx
mov $0x1122334455667788, %rax
mov $0xA1B2C3D4E5F67890, %rcx
div %rcx

print_regs

# --- 32-bit div

push $0x0000000000010ad7; popfq

mov $0x9ABCDEF0, %edx
mov $0x55667788, %eax
mov $0xE5F67890, %ecx
div %ecx

print_regs

# --- 16-bit div

push $0x0000000000010ad7; popfq

mov $0x9ABC, %dx
mov $0x5566, %ax
mov $0xE5F6, %cx
div %cx

print_regs

# --- 8-bit div

push $0x0000000000010ad7; popfq

mov $0x9ABC, %ax
mov $0xE5, %cl
div %ch

print_regs

# --- Clean exit
mov $60, %eax
mov $0, %edi
syscall
