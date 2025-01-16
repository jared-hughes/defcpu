# hole: fibonacci

.data

stack: .quad 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 
stack_top: .quad 0

.text
mov $stack_top, %rsp

mov $1, %rax
call lbl1
mov $5, %rdi
print_regs
call exit

# unreachable:
mov $12345, %r12

lbl1:
    mov $2, %rbx
    call lbl2
    mov $4, %rdx
    ret

# unreachable:
mov $23456, %r13

lbl2:
    mov $3, %rcx
    ret

# unreachable:
mov $34567, %r15

exit:
    # --- Clean exit
    mov $60, %eax
    mov $0, %edi
    syscall
