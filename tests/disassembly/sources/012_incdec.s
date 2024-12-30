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
