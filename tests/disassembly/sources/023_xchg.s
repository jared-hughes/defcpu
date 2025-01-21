90          nop
91          xchg   ecx, eax
92          xchg   edx, eax
93          xchg   ebx, eax
94          xchg   esp, eax
95          xchg   ebp, eax
96          xchg   esi, eax
97          xchg   edi, eax
66 90       xchg   ax, ax
66 91       xchg   cx, ax
66 92       xchg   dx, ax
66 93       xchg   bx, ax
66 94       xchg   sp, ax
66 95       xchg   bp, ax
66 96       xchg   si, ax
66 97       xchg   di, ax
41 90       xchg   r8d, eax
41 91       xchg   r9d, eax
41 92       xchg   r10d, eax
41 93       xchg   r11d, eax
41 94       xchg   r12d, eax
41 95       xchg   r13d, eax
41 96       xchg   r14d, eax
41 97       xchg   r15d, eax
66 41 90    xchg   r8w, ax
66 41 91    xchg   r9w, ax
66 41 92    xchg   r10w, ax
66 41 93    xchg   r11w, ax
66 41 94    xchg   r12w, ax
66 41 95    xchg   r13w, ax
66 41 96    xchg   r14w, ax
66 41 97    xchg   r15w, ax
48 90       rex.W nop
48 91       xchg   rcx, rax
48 92       xchg   rdx, rax
48 93       xchg   rbx, rax
48 94       xchg   rsp, rax
48 95       xchg   rbp, rax
48 96       xchg   rsi, rax
48 97       xchg   rdi, rax
49 90       xchg   r8, rax
49 91       xchg   r9, rax
49 92       xchg   r10, rax
49 93       xchg   r11, rax
49 94       xchg   r12, rax
49 95       xchg   r13, rax
49 96       xchg   r14, rax
49 97       xchg   r15, rax
86 13       xchg   BYTE PTR [rbx], dl
66 48 90    xchg   rax, rax
66 48 91    data16 xchg rcx, rax
66 48 92    data16 xchg rdx, rax
66 48 93    data16 xchg rbx, rax
66 48 94    data16 xchg rsp, rax
66 48 95    data16 xchg rbp, rax
66 48 96    data16 xchg rsi, rax
66 48 97    data16 xchg rdi, rax
66 49 90    xchg   r8, rax
66 49 91    data16 xchg r9, rax
66 49 92    data16 xchg r10, rax
66 49 93    data16 xchg r11, rax
66 49 94    data16 xchg r12, rax
66 49 95    data16 xchg r13, rax
66 49 96    data16 xchg r14, rax
66 49 97    data16 xchg r15, rax
86 13       xchg   BYTE PTR [rbx], dl
66 86 13    data16 xchg BYTE PTR [rbx], dl
67 86 13    xchg   BYTE PTR [ebx], dl
40 86 13    rex xchg BYTE PTR [rbx], dl
41 86 13    xchg   BYTE PTR [r11], dl
48 86 13    rex.W xchg BYTE PTR [rbx], dl
49 86 13    rex.WB xchg BYTE PTR [r11], dl
87 13       xchg   DWORD PTR [rbx], edx
66 87 13    xchg   WORD PTR [rbx], dx
67 87 13    xchg   DWORD PTR [ebx], edx
40 87 13    rex xchg DWORD PTR [rbx], edx
41 87 13    xchg   DWORD PTR [r11], edx
48 87 13    xchg   QWORD PTR [rbx], rdx
49 87 13    xchg   QWORD PTR [r11], rdx
