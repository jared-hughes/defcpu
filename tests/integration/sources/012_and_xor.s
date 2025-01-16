# hole: fibonacci

.data

stack: .quad 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 
stack_top: .quad 0

.text
mov $stack_top, %rsp

# --- 64-bit and

mov $0x1122334455667788, %rax
mov $0xA1B2C3D4E5F67890, %rbx
and %rax, %rbx
print_regs

# --- 32-bit and

push $0x0000000000010ad7; popfq

mov $0x11223344, %eax
mov $0xA1B2C3D4, %ebx
and %eax, %ebx
print_regs

and $0xA1F3FFE6, %ebx
and $0xA1F3FFE6, %eax
print_regs

# --- 16-bit and

push $0x0000000000010ad7; popfq

mov $0xA555, %ax
mov $0xA1B2, %bx
and %ax, %bx
print_regs

and $0x81F3, %bx
and $0xA1F3, %ax
print_regs

# --- 8-bit and

push $0x0000000000010ad7; popfq

mov $0x77, %al
mov $0xA1, %ah
and %al, %ah
print_regs

and $0x77, %al
and $0xF3, %ah
print_regs

and $0x04, %ah
print_regs

# --- 64-bit xor

mov $0x1122334455667788, %rax
mov $0xA1B2C3D4E5F67890, %rbx
xor %rax, %rbx
print_regs

# --- 32-bit xor

push $0x0000000000010ad7; popfq

mov $0x11223344, %eax
mov $0xA1B2C3D4, %ebx
xor %eax, %ebx
print_regs

xor $0xA1F3FFE6, %ebx
xor $0xA1F3FFE6, %eax
print_regs

# --- 16-bit xor

push $0x0000000000010ad7; popfq

mov $0xA555, %ax
mov $0xA1B2, %bx
xor %ax, %bx
print_regs

xor $0x81F3, %bx
xor $0xA1F3, %ax
print_regs

# --- 8-bit xor

push $0x0000000000010ad7; popfq

mov $0x77, %al
mov $0xA1, %ah
xor %al, %ah
print_regs

xor $0x77, %al
xor $0xF3, %ah
print_regs

xor $0x04, %ah
print_regs

# --- 64-bit or

mov $0x1122334455667788, %rax
mov $0xA1B2C3D4E5F67890, %rbx
or %rax, %rbx
print_regs

# --- 32-bit or

push $0x0000000000010ad7; popfq

mov $0x11223344, %eax
mov $0xA1B2C3D4, %ebx
or %eax, %ebx
print_regs

or $0xA1F3FFE6, %ebx
or $0xA1F3FFE6, %eax
print_regs

# --- 16-bit or

push $0x0000000000010ad7; popfq

mov $0xA555, %ax
mov $0xA1B2, %bx
or %ax, %bx
print_regs

or $0x81F3, %bx
or $0xA1F3, %ax
print_regs

# --- 8-bit or

push $0x0000000000010ad7; popfq

mov $0x77, %al
mov $0xA1, %ah
or %al, %ah
print_regs

or $0x77, %al
or $0xF3, %ah
print_regs

or $0x04, %ah
print_regs


# --- Clean exit
mov $60, %eax
mov $0, %edi
syscall
