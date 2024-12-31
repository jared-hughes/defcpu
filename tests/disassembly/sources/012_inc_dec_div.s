fe 00                       incb   (%rax)
fe 04 f4                    incb   (%rsp, %rsi, 8)
fe 05 f4 f4 f4 f4           incb   -0xb0b0b0c(%rip)
fe 06                       incb   (%rsi)
fe 40 f4                    incb   -0xc(%rax)
fe c3                       inc    %bl
fe c4                       inc    %ah
41 fe 00                    incb   (%r8)
41 fe 04 12                 incb   (%r10, %rdx, 1)
41 fe 05 12 34 56 78        incb   0x78563412(%rip)
41 fe 06                    incb   (%r14)
ff 00                       incl   (%rax)
ff 04 f4                    incl   (%rsp, %rsi, 8)
ff 05 f4 f4 f4 f4           incl   -0xb0b0b0c(%rip)
ff 06                       incl   (%rsi)
ff 40 f4                    incl   -0xc(%rax)
ff c3                       inc    %ebx
ff c4                       inc    %esp
41 ff 00                    incl   (%r8)
41 ff 04 12                 incl   (%r10, %rdx, 1)
41 ff 05 12 34 56 78        incl   0x78563412(%rip)
41 ff 06                    incl   (%r14)
66 ff 00                    incw   (%rax)
66 ff 04 f4                 incw   (%rsp, %rsi, 8)
66 ff 05 f4 f4 f4 f4        incw   -0xb0b0b0c(%rip)
66 ff 06                    incw   (%rsi)
66 ff 40 f4                 incw   -0xc(%rax)
66 ff c3                    inc    %bx
66 ff c4                    inc    %sp
66 41 ff 00                 incw   (%r8)
66 41 ff 04 12              incw   (%r10, %rdx, 1)
66 41 ff 05 12 34 56 78     incw   0x78563412(%rip)
66 41 ff 06                 incw   (%r14)
67 ff 00                    incl   (%eax)
67 ff 04 f4                 incl   (%esp, %esi, 8)
67 ff 05 f4 f4 f4 f4        incl   -0xb0b0b0c(%eip)
67 ff 06                    incl   (%esi)
67 ff 40 f4                 incl   -0xc(%eax)
67 ff c3                    addr32 inc %ebx
67 ff c4                    addr32 inc %esp
67 41 ff 00                 incl   (%r8d)
67 41 ff 04 12              incl   (%r10d, %edx, 1)
67 41 ff 05 12 34 56 78     incl   0x78563412(%eip)
67 41 ff 06                 incl   (%r14d)
66 67 ff 00                 incw   (%eax)
66 67 ff 04 f4              incw   (%esp, %esi, 8)
66 67 ff 05 f4 f4 f4 f4     incw   -0xb0b0b0c(%eip)
66 67 ff 06                 incw   (%esi)
66 67 ff 40 f4              incw   -0xc(%eax)
66 67 ff c3                 addr32 inc %bx
66 67 ff c4                 addr32 inc %sp
66 67 41 ff 00              incw   (%r8d)
66 67 41 ff 04 12           incw   (%r10d, %edx, 1)
66 67 41 ff 05 12 34 56 78  incw   0x78563412(%eip)
66 67 41 ff 06              incw   (%r14d)
66 67 48 ff 00              data16 incq (%eax)
66 67 48 ff 04 f4           data16 incq (%esp, %esi, 8)
66 67 48 ff 05 f4 f4 f4 f4  data16 incq -0xb0b0b0c(%eip)
48 ff 06                    incq   (%rsi)
48 ff 40 f4                 incq   -0xc(%rax)
48 ff c3                    inc    %rbx
48 ff c4                    inc    %rsp
49 ff 00                    incq   (%r8)
49 ff 04 f4                 incq   (%r12, %rsi, 8)
49 ff 05 f4 f4 f4 f4        incq   -0xb0b0b0c(%rip)
49 ff 06                    incq   (%r14)
49 ff 40 f4                 incq   -0xc(%r8)
49 ff c3                    inc    %r11
49 ff c4                    inc    %r12
67 48 ff 06                 incq   (%esi)
67 48 ff 40 f4              incq   -0xc(%eax)
67 48 ff c3                 addr32 inc %rbx
67 48 ff c4                 addr32 inc %rsp
67 49 ff 00                 incq   (%r8d)
67 49 ff 04 f4              incq   (%r12d, %esi, 8)
67 49 ff 05 f4 f4 f4 f4     incq   -0xb0b0b0c(%eip)
67 49 ff 06                 incq   (%r14d)
67 49 ff 40 f4              incq   -0xc(%r8d)
67 49 ff c3                 addr32 inc %r11
67 49 ff c4                 addr32 inc %r12
fe 08                       decb   (%rax)
fe 0c f4                    decb   (%rsp, %rsi, 8)
fe 0d f4 f4 f4 f4           decb   -0xb0b0b0c(%rip)
fe 0e                       decb   (%rsi)
fe 48 f4                    decb   -0xc(%rax)
fe cb                       dec    %bl
fe cc                       dec    %ah
41 fe 08                    decb   (%r8)
41 fe 0c 12                 decb   (%r10, %rdx, 1)
41 fe 0d 12 34 56 78        decb   0x78563412(%rip)
41 fe 0e                    decb   (%r14)
ff 08                       decl   (%rax)
ff 0c f4                    decl   (%rsp, %rsi, 8)
ff 0d f4 f4 f4 f4           decl   -0xb0b0b0c(%rip)
ff 0e                       decl   (%rsi)
ff 48 f4                    decl   -0xc(%rax)
ff cb                       dec    %ebx
ff cc                       dec    %esp
41 ff 08                    decl   (%r8)
41 ff 0c 12                 decl   (%r10, %rdx, 1)
41 ff 0d 12 34 56 78        decl   0x78563412(%rip)
41 ff 0e                    decl   (%r14)
66 ff 08                    decw   (%rax)
66 ff 0c f4                 decw   (%rsp, %rsi, 8)
66 ff 0d f4 f4 f4 f4        decw   -0xb0b0b0c(%rip)
66 ff 0e                    decw   (%rsi)
66 ff 48 f4                 decw   -0xc(%rax)
66 ff cb                    dec    %bx
66 ff cc                    dec    %sp
66 41 ff 08                 decw   (%r8)
66 41 ff 0c 12              decw   (%r10, %rdx, 1)
66 41 ff 0d 12 34 56 78     decw   0x78563412(%rip)
66 41 ff 0e                 decw   (%r14)
67 ff 08                    decl   (%eax)
67 ff 0c f4                 decl   (%esp, %esi, 8)
67 ff 0d f4 f4 f4 f4        decl   -0xb0b0b0c(%eip)
67 ff 0e                    decl   (%esi)
67 ff 48 f4                 decl   -0xc(%eax)
67 ff cb                    addr32 dec %ebx
67 ff cc                    addr32 dec %esp
67 41 ff 08                 decl   (%r8d)
67 41 ff 0c 12              decl   (%r10d, %edx, 1)
67 41 ff 0d 12 34 56 78     decl   0x78563412(%eip)
67 41 ff 0e                 decl   (%r14d)
66 67 ff 08                 decw   (%eax)
66 67 ff 0c f4              decw   (%esp, %esi, 8)
66 67 ff 0d f4 f4 f4 f4     decw   -0xb0b0b0c(%eip)
66 67 ff 0e                 decw   (%esi)
66 67 ff 48 f4              decw   -0xc(%eax)
66 67 ff cb                 addr32 dec %bx
66 67 ff cc                 addr32 dec %sp
66 67 41 ff 08              decw   (%r8d)
66 67 41 ff 0c 12           decw   (%r10d, %edx, 1)
66 67 41 ff 0d 12 34 56 78  decw   0x78563412(%eip)
66 67 41 ff 0e              decw   (%r14d)
66 67 48 ff 08              data16 decq (%eax)
66 67 48 ff 0c f4           data16 decq (%esp, %esi, 8)
66 67 48 ff 0d f4 f4 f4 f4  data16 decq -0xb0b0b0c(%eip)
48 ff 0e                    decq   (%rsi)
48 ff 48 f4                 decq   -0xc(%rax)
48 ff cb                    dec    %rbx
48 ff cc                    dec    %rsp
49 ff 08                    decq   (%r8)
49 ff 0c f4                 decq   (%r12, %rsi, 8)
49 ff 0d f4 f4 f4 f4        decq   -0xb0b0b0c(%rip)
49 ff 0e                    decq   (%r14)
49 ff 48 f4                 decq   -0xc(%r8)
49 ff cb                    dec    %r11
49 ff cc                    dec    %r12
67 48 ff 0e                 decq   (%esi)
67 48 ff 48 f4              decq   -0xc(%eax)
67 48 ff cb                 addr32 dec %rbx
67 48 ff cc                 addr32 dec %rsp
67 49 ff 08                 decq   (%r8d)
67 49 ff 0c f4              decq   (%r12d, %esi, 8)
67 49 ff 0d f4 f4 f4 f4     decq   -0xb0b0b0c(%eip)
67 49 ff 0e                 decq   (%r14d)
67 49 ff 48 f4              decq   -0xc(%r8d)
67 49 ff cb                 addr32 dec %r11
67 49 ff cc                 addr32 dec %r12
f6 30                       divb   (%rax)
f6 34 f4                    divb   (%rsp, %rsi, 8)
f6 35 f4 f4 f4 f4           divb   -0xb0b0b0c(%rip)
f6 36                       divb   (%rsi)
f6 70 f4                    divb   -0xc(%rax)
f6 f3                       div    %bl
f6 f4                       div    %ah
41 f6 30                    divb   (%r8)
41 f6 34 12                 divb   (%r10, %rdx, 1)
41 f6 35 12 34 56 78        divb   0x78563412(%rip)
41 f6 36                    divb   (%r14)
f7 30                       divl   (%rax)
f7 34 f4                    divl   (%rsp, %rsi, 8)
f7 35 f4 f4 f4 f4           divl   -0xb0b0b0c(%rip)
f7 36                       divl   (%rsi)
f7 70 f4                    divl   -0xc(%rax)
f7 f3                       div    %ebx
f7 f4                       div    %esp
41 f7 30                    divl   (%r8)
41 f7 34 12                 divl   (%r10, %rdx, 1)
41 f7 35 12 34 56 78        divl   0x78563412(%rip)
41 f7 36                    divl   (%r14)
66 f7 30                    divw   (%rax)
66 f7 34 f4                 divw   (%rsp, %rsi, 8)
66 f7 35 f4 f4 f4 f4        divw   -0xb0b0b0c(%rip)
66 f7 36                    divw   (%rsi)
66 f7 70 f4                 divw   -0xc(%rax)
66 f7 f3                    div    %bx
66 f7 f4                    div    %sp
66 41 f7 30                 divw   (%r8)
66 41 f7 34 12              divw   (%r10, %rdx, 1)
66 41 f7 35 12 34 56 78     divw   0x78563412(%rip)
66 41 f7 36                 divw   (%r14)
67 f7 30                    divl   (%eax)
67 f7 34 f4                 divl   (%esp, %esi, 8)
67 f7 35 f4 f4 f4 f4        divl   -0xb0b0b0c(%eip)
67 f7 36                    divl   (%esi)
67 f7 70 f4                 divl   -0xc(%eax)
67 f7 f3                    addr32 div %ebx
67 f7 f4                    addr32 div %esp
67 41 f7 30                 divl   (%r8d)
67 41 f7 34 12              divl   (%r10d, %edx, 1)
67 41 f7 35 12 34 56 78     divl   0x78563412(%eip)
67 41 f7 36                 divl   (%r14d)
66 67 f7 30                 divw   (%eax)
66 67 f7 34 f4              divw   (%esp, %esi, 8)
66 67 f7 35 f4 f4 f4 f4     divw   -0xb0b0b0c(%eip)
66 67 f7 36                 divw   (%esi)
66 67 f7 70 f4              divw   -0xc(%eax)
66 67 f7 f3                 addr32 div %bx
66 67 f7 f4                 addr32 div %sp
66 67 41 f7 30              divw   (%r8d)
66 67 41 f7 34 12           divw   (%r10d, %edx, 1)
66 67 41 f7 35 12 34 56 78  divw   0x78563412(%eip)
66 67 41 f7 36              divw   (%r14d)
66 67 48 f7 30              data16 divq (%eax)
66 67 48 f7 34 f4           data16 divq (%esp, %esi, 8)
66 67 48 f7 35 f4 f4 f4 f4  data16 divq -0xb0b0b0c(%eip)
48 f7 36                    divq   (%rsi)
48 f7 70 f4                 divq   -0xc(%rax)
48 f7 f3                    div    %rbx
48 f7 f4                    div    %rsp
49 f7 30                    divq   (%r8)
49 f7 34 f4                 divq   (%r12, %rsi, 8)
49 f7 35 f4 f4 f4 f4        divq   -0xb0b0b0c(%rip)
49 f7 36                    divq   (%r14)
49 f7 70 f4                 divq   -0xc(%r8)
49 f7 f3                    div    %r11
49 f7 f4                    div    %r12
67 48 f7 36                 divq   (%esi)
67 48 f7 70 f4              divq   -0xc(%eax)
67 48 f7 f3                 addr32 div %rbx
67 48 f7 f4                 addr32 div %rsp
67 49 f7 30                 divq   (%r8d)
67 49 f7 34 f4              divq   (%r12d, %esi, 8)
67 49 f7 35 f4 f4 f4 f4     divq   -0xb0b0b0c(%eip)
67 49 f7 36                 divq   (%r14d)
67 49 f7 70 f4              divq   -0xc(%r8d)
67 49 f7 f3                 addr32 div %r11
67 49 f7 f4                 addr32 div %r12
