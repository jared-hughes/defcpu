90          nop
91          xchg   %eax, %ecx
92          xchg   %eax, %edx
93          xchg   %eax, %ebx
94          xchg   %eax, %esp
95          xchg   %eax, %ebp
96          xchg   %eax, %esi
97          xchg   %eax, %edi
66 90       xchg   %ax, %ax
66 91       xchg   %ax, %cx
66 92       xchg   %ax, %dx
66 93       xchg   %ax, %bx
66 94       xchg   %ax, %sp
66 95       xchg   %ax, %bp
66 96       xchg   %ax, %si
66 97       xchg   %ax, %di
41 90       xchg   %eax, %r8d
41 91       xchg   %eax, %r9d
41 92       xchg   %eax, %r10d
41 93       xchg   %eax, %r11d
41 94       xchg   %eax, %r12d
41 95       xchg   %eax, %r13d
41 96       xchg   %eax, %r14d
41 97       xchg   %eax, %r15d
66 41 90    xchg   %ax, %r8w
66 41 91    xchg   %ax, %r9w
66 41 92    xchg   %ax, %r10w
66 41 93    xchg   %ax, %r11w
66 41 94    xchg   %ax, %r12w
66 41 95    xchg   %ax, %r13w
66 41 96    xchg   %ax, %r14w
66 41 97    xchg   %ax, %r15w
48 90       rex.W nop
48 91       xchg   %rax, %rcx
48 92       xchg   %rax, %rdx
48 93       xchg   %rax, %rbx
48 94       xchg   %rax, %rsp
48 95       xchg   %rax, %rbp
48 96       xchg   %rax, %rsi
48 97       xchg   %rax, %rdi
49 90       xchg   %rax, %r8
49 91       xchg   %rax, %r9
49 92       xchg   %rax, %r10
49 93       xchg   %rax, %r11
49 94       xchg   %rax, %r12
49 95       xchg   %rax, %r13
49 96       xchg   %rax, %r14
49 97       xchg   %rax, %r15
86 13       xchg   %dl, (%rbx)
66 48 90    xchg   %rax, %rax
66 48 91    data16 xchg %rax, %rcx
66 48 92    data16 xchg %rax, %rdx
66 48 93    data16 xchg %rax, %rbx
66 48 94    data16 xchg %rax, %rsp
66 48 95    data16 xchg %rax, %rbp
66 48 96    data16 xchg %rax, %rsi
66 48 97    data16 xchg %rax, %rdi
66 49 90    xchg   %rax, %r8
66 49 91    data16 xchg %rax, %r9
66 49 92    data16 xchg %rax, %r10
66 49 93    data16 xchg %rax, %r11
66 49 94    data16 xchg %rax, %r12
66 49 95    data16 xchg %rax, %r13
66 49 96    data16 xchg %rax, %r14
66 49 97    data16 xchg %rax, %r15
86 13       xchg   %dl, (%rbx)
66 86 13    data16 xchg %dl, (%rbx)
67 86 13    xchg   %dl, (%ebx)
40 86 13    rex xchg %dl, (%rbx)
41 86 13    xchg   %dl, (%r11)
48 86 13    rex.W xchg %dl, (%rbx)
49 86 13    rex.WB xchg %dl, (%r11)
87 13       xchg   %edx, (%rbx)
66 87 13    xchg   %dx, (%rbx)
67 87 13    xchg   %edx, (%ebx)
40 87 13    rex xchg %edx, (%rbx)
41 87 13    xchg   %edx, (%r11)
48 87 13    xchg   %rdx, (%rbx)
49 87 13    xchg   %rdx, (%r11)
