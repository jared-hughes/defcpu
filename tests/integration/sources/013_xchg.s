# hole: fibonacci

mov $0, %rsp

mov $0x0102030405060708, %rax
mov $0xA1B1C1D1E1F1A1B1, %rbx
xchg %rax, %rbx
xchg %rax, %r11
xchg %r11, %rcx
xchg %rbx, %r8
xchg %r8d, %edx
nop
print_regs

xchg %r11w, %si
xchg %ax, %dx
xchg %bl, %dl
print_regs

mov $x, %rax
mov $y, %rbx
xchg (%rax), %dl
xchg (%rax), %cx
xchg (%rbx), %edx
xchg (%rbx), %rbx
print_regs

# --- Clean exit
mov $60, %eax
mov $0, %edi
syscall

.data
x: .quad 0xabcdef12345678ab
y: .quad 0xab12345678abcdef
