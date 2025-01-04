# hole: fibonacci

.data

stack: .quad 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 
stack_top: .quad 0

.text

mov $stack_top, %rsp
push $0x123

print_regs

pushfq

print_regs

pop %rax
pop %rbx

print_regs

# TODO: setting the rflags. It's a mess of bus errors and sigtrap currently.
# mov $0xFFFFFFFFFFFFFFFFFF, %rbx
# push %rbx
# popfq
# print_regs

# --- Clean exit
mov $60, %eax
mov $0, %edi
syscall
