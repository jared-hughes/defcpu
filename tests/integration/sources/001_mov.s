# hole: fibonacci
mov $0x00007FFCD4A9D8F0, %rsp
mov $0xDEADBEEF, %ebx

mov $0x1023456789ABCDEF, %rax
mov $0xF1023456789ABCDE, %rcx
mov $0xEF1023456789ABCD, %rdx
mov $0xDEF1023456789ABC, %r8
mov $0x12340567, %eax
mov $0x1234, %cx
mov $0x45, %dl
mov $0x23, %dh
hlt
