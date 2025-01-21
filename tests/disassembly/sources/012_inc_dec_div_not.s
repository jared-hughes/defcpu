fe 00                       inc    BYTE PTR [rax]
fe 04 f4                    inc    BYTE PTR [rsp+rsi*8]
fe 05 f4 f4 f4 f4           inc    BYTE PTR [rip+0xfffffffff4f4f4f4]
fe 06                       inc    BYTE PTR [rsi]
fe 40 f4                    inc    BYTE PTR [rax-0xc]
fe c3                       inc    bl
fe c4                       inc    ah
41 fe 00                    inc    BYTE PTR [r8]
41 fe 04 12                 inc    BYTE PTR [r10+rdx*1]
41 fe 05 12 34 56 78        inc    BYTE PTR [rip+0x78563412]
41 fe 06                    inc    BYTE PTR [r14]
ff 00                       inc    DWORD PTR [rax]
ff 04 f4                    inc    DWORD PTR [rsp+rsi*8]
ff 05 f4 f4 f4 f4           inc    DWORD PTR [rip+0xfffffffff4f4f4f4]
ff 06                       inc    DWORD PTR [rsi]
ff 40 f4                    inc    DWORD PTR [rax-0xc]
ff c3                       inc    ebx
ff c4                       inc    esp
41 ff 00                    inc    DWORD PTR [r8]
41 ff 04 12                 inc    DWORD PTR [r10+rdx*1]
41 ff 05 12 34 56 78        inc    DWORD PTR [rip+0x78563412]
41 ff 06                    inc    DWORD PTR [r14]
66 ff 00                    inc    WORD PTR [rax]
66 ff 04 f4                 inc    WORD PTR [rsp+rsi*8]
66 ff 05 f4 f4 f4 f4        inc    WORD PTR [rip+0xfffffffff4f4f4f4]
66 ff 06                    inc    WORD PTR [rsi]
66 ff 40 f4                 inc    WORD PTR [rax-0xc]
66 ff c3                    inc    bx
66 ff c4                    inc    sp
66 41 ff 00                 inc    WORD PTR [r8]
66 41 ff 04 12              inc    WORD PTR [r10+rdx*1]
66 41 ff 05 12 34 56 78     inc    WORD PTR [rip+0x78563412]
66 41 ff 06                 inc    WORD PTR [r14]
67 ff 00                    inc    DWORD PTR [eax]
67 ff 04 f4                 inc    DWORD PTR [esp+esi*8]
67 ff 05 f4 f4 f4 f4        inc    DWORD PTR [eip+0xfffffffff4f4f4f4]
67 ff 06                    inc    DWORD PTR [esi]
67 ff 40 f4                 inc    DWORD PTR [eax-0xc]
67 ff c3                    addr32 inc ebx
67 ff c4                    addr32 inc esp
67 41 ff 00                 inc    DWORD PTR [r8d]
67 41 ff 04 12              inc    DWORD PTR [r10d+edx*1]
67 41 ff 05 12 34 56 78     inc    DWORD PTR [eip+0x78563412]
67 41 ff 06                 inc    DWORD PTR [r14d]
66 67 ff 00                 inc    WORD PTR [eax]
66 67 ff 04 f4              inc    WORD PTR [esp+esi*8]
66 67 ff 05 f4 f4 f4 f4     inc    WORD PTR [eip+0xfffffffff4f4f4f4]
66 67 ff 06                 inc    WORD PTR [esi]
66 67 ff 40 f4              inc    WORD PTR [eax-0xc]
66 67 ff c3                 addr32 inc bx
66 67 ff c4                 addr32 inc sp
66 67 41 ff 00              inc    WORD PTR [r8d]
66 67 41 ff 04 12           inc    WORD PTR [r10d+edx*1]
66 67 41 ff 05 12 34 56 78  inc    WORD PTR [eip+0x78563412]
66 67 41 ff 06              inc    WORD PTR [r14d]
66 67 48 ff 00              data16 inc QWORD PTR [eax]
66 67 48 ff 04 f4           data16 inc QWORD PTR [esp+esi*8]
66 67 48 ff 05 f4 f4 f4 f4  data16 inc QWORD PTR [eip+0xfffffffff4f4f4f4]
48 ff 06                    inc    QWORD PTR [rsi]
48 ff 40 f4                 inc    QWORD PTR [rax-0xc]
48 ff c3                    inc    rbx
48 ff c4                    inc    rsp
49 ff 00                    inc    QWORD PTR [r8]
49 ff 04 f4                 inc    QWORD PTR [r12+rsi*8]
49 ff 05 f4 f4 f4 f4        inc    QWORD PTR [rip+0xfffffffff4f4f4f4]
49 ff 06                    inc    QWORD PTR [r14]
49 ff 40 f4                 inc    QWORD PTR [r8-0xc]
49 ff c3                    inc    r11
49 ff c4                    inc    r12
67 48 ff 06                 inc    QWORD PTR [esi]
67 48 ff 40 f4              inc    QWORD PTR [eax-0xc]
67 48 ff c3                 addr32 inc rbx
67 48 ff c4                 addr32 inc rsp
67 49 ff 00                 inc    QWORD PTR [r8d]
67 49 ff 04 f4              inc    QWORD PTR [r12d+esi*8]
67 49 ff 05 f4 f4 f4 f4     inc    QWORD PTR [eip+0xfffffffff4f4f4f4]
67 49 ff 06                 inc    QWORD PTR [r14d]
67 49 ff 40 f4              inc    QWORD PTR [r8d-0xc]
67 49 ff c3                 addr32 inc r11
67 49 ff c4                 addr32 inc r12
fe 08                       dec    BYTE PTR [rax]
fe 0c f4                    dec    BYTE PTR [rsp+rsi*8]
fe 0d f4 f4 f4 f4           dec    BYTE PTR [rip+0xfffffffff4f4f4f4]
fe 0e                       dec    BYTE PTR [rsi]
fe 48 f4                    dec    BYTE PTR [rax-0xc]
fe cb                       dec    bl
fe cc                       dec    ah
41 fe 08                    dec    BYTE PTR [r8]
41 fe 0c 12                 dec    BYTE PTR [r10+rdx*1]
41 fe 0d 12 34 56 78        dec    BYTE PTR [rip+0x78563412]
41 fe 0e                    dec    BYTE PTR [r14]
ff 08                       dec    DWORD PTR [rax]
ff 0c f4                    dec    DWORD PTR [rsp+rsi*8]
ff 0d f4 f4 f4 f4           dec    DWORD PTR [rip+0xfffffffff4f4f4f4]
ff 0e                       dec    DWORD PTR [rsi]
ff 48 f4                    dec    DWORD PTR [rax-0xc]
ff cb                       dec    ebx
ff cc                       dec    esp
41 ff 08                    dec    DWORD PTR [r8]
41 ff 0c 12                 dec    DWORD PTR [r10+rdx*1]
41 ff 0d 12 34 56 78        dec    DWORD PTR [rip+0x78563412]
41 ff 0e                    dec    DWORD PTR [r14]
66 ff 08                    dec    WORD PTR [rax]
66 ff 0c f4                 dec    WORD PTR [rsp+rsi*8]
66 ff 0d f4 f4 f4 f4        dec    WORD PTR [rip+0xfffffffff4f4f4f4]
66 ff 0e                    dec    WORD PTR [rsi]
66 ff 48 f4                 dec    WORD PTR [rax-0xc]
66 ff cb                    dec    bx
66 ff cc                    dec    sp
66 41 ff 08                 dec    WORD PTR [r8]
66 41 ff 0c 12              dec    WORD PTR [r10+rdx*1]
66 41 ff 0d 12 34 56 78     dec    WORD PTR [rip+0x78563412]
66 41 ff 0e                 dec    WORD PTR [r14]
67 ff 08                    dec    DWORD PTR [eax]
67 ff 0c f4                 dec    DWORD PTR [esp+esi*8]
67 ff 0d f4 f4 f4 f4        dec    DWORD PTR [eip+0xfffffffff4f4f4f4]
67 ff 0e                    dec    DWORD PTR [esi]
67 ff 48 f4                 dec    DWORD PTR [eax-0xc]
67 ff cb                    addr32 dec ebx
67 ff cc                    addr32 dec esp
67 41 ff 08                 dec    DWORD PTR [r8d]
67 41 ff 0c 12              dec    DWORD PTR [r10d+edx*1]
67 41 ff 0d 12 34 56 78     dec    DWORD PTR [eip+0x78563412]
67 41 ff 0e                 dec    DWORD PTR [r14d]
66 67 ff 08                 dec    WORD PTR [eax]
66 67 ff 0c f4              dec    WORD PTR [esp+esi*8]
66 67 ff 0d f4 f4 f4 f4     dec    WORD PTR [eip+0xfffffffff4f4f4f4]
66 67 ff 0e                 dec    WORD PTR [esi]
66 67 ff 48 f4              dec    WORD PTR [eax-0xc]
66 67 ff cb                 addr32 dec bx
66 67 ff cc                 addr32 dec sp
66 67 41 ff 08              dec    WORD PTR [r8d]
66 67 41 ff 0c 12           dec    WORD PTR [r10d+edx*1]
66 67 41 ff 0d 12 34 56 78  dec    WORD PTR [eip+0x78563412]
66 67 41 ff 0e              dec    WORD PTR [r14d]
66 67 48 ff 08              data16 dec QWORD PTR [eax]
66 67 48 ff 0c f4           data16 dec QWORD PTR [esp+esi*8]
66 67 48 ff 0d f4 f4 f4 f4  data16 dec QWORD PTR [eip+0xfffffffff4f4f4f4]
48 ff 0e                    dec    QWORD PTR [rsi]
48 ff 48 f4                 dec    QWORD PTR [rax-0xc]
48 ff cb                    dec    rbx
48 ff cc                    dec    rsp
49 ff 08                    dec    QWORD PTR [r8]
49 ff 0c f4                 dec    QWORD PTR [r12+rsi*8]
49 ff 0d f4 f4 f4 f4        dec    QWORD PTR [rip+0xfffffffff4f4f4f4]
49 ff 0e                    dec    QWORD PTR [r14]
49 ff 48 f4                 dec    QWORD PTR [r8-0xc]
49 ff cb                    dec    r11
49 ff cc                    dec    r12
67 48 ff 0e                 dec    QWORD PTR [esi]
67 48 ff 48 f4              dec    QWORD PTR [eax-0xc]
67 48 ff cb                 addr32 dec rbx
67 48 ff cc                 addr32 dec rsp
67 49 ff 08                 dec    QWORD PTR [r8d]
67 49 ff 0c f4              dec    QWORD PTR [r12d+esi*8]
67 49 ff 0d f4 f4 f4 f4     dec    QWORD PTR [eip+0xfffffffff4f4f4f4]
67 49 ff 0e                 dec    QWORD PTR [r14d]
67 49 ff 48 f4              dec    QWORD PTR [r8d-0xc]
67 49 ff cb                 addr32 dec r11
67 49 ff cc                 addr32 dec r12
f6 30                       div    BYTE PTR [rax]
f6 34 f4                    div    BYTE PTR [rsp+rsi*8]
f6 35 f4 f4 f4 f4           div    BYTE PTR [rip+0xfffffffff4f4f4f4]
f6 36                       div    BYTE PTR [rsi]
f6 70 f4                    div    BYTE PTR [rax-0xc]
f6 f3                       div    bl
f6 f4                       div    ah
41 f6 30                    div    BYTE PTR [r8]
41 f6 34 12                 div    BYTE PTR [r10+rdx*1]
41 f6 35 12 34 56 78        div    BYTE PTR [rip+0x78563412]
41 f6 36                    div    BYTE PTR [r14]
f7 30                       div    DWORD PTR [rax]
f7 34 f4                    div    DWORD PTR [rsp+rsi*8]
f7 35 f4 f4 f4 f4           div    DWORD PTR [rip+0xfffffffff4f4f4f4]
f7 36                       div    DWORD PTR [rsi]
f7 70 f4                    div    DWORD PTR [rax-0xc]
f7 f3                       div    ebx
f7 f4                       div    esp
41 f7 30                    div    DWORD PTR [r8]
41 f7 34 12                 div    DWORD PTR [r10+rdx*1]
41 f7 35 12 34 56 78        div    DWORD PTR [rip+0x78563412]
41 f7 36                    div    DWORD PTR [r14]
66 f7 30                    div    WORD PTR [rax]
66 f7 34 f4                 div    WORD PTR [rsp+rsi*8]
66 f7 35 f4 f4 f4 f4        div    WORD PTR [rip+0xfffffffff4f4f4f4]
66 f7 36                    div    WORD PTR [rsi]
66 f7 70 f4                 div    WORD PTR [rax-0xc]
66 f7 f3                    div    bx
66 f7 f4                    div    sp
66 41 f7 30                 div    WORD PTR [r8]
66 41 f7 34 12              div    WORD PTR [r10+rdx*1]
66 41 f7 35 12 34 56 78     div    WORD PTR [rip+0x78563412]
66 41 f7 36                 div    WORD PTR [r14]
67 f7 30                    div    DWORD PTR [eax]
67 f7 34 f4                 div    DWORD PTR [esp+esi*8]
67 f7 35 f4 f4 f4 f4        div    DWORD PTR [eip+0xfffffffff4f4f4f4]
67 f7 36                    div    DWORD PTR [esi]
67 f7 70 f4                 div    DWORD PTR [eax-0xc]
67 f7 f3                    addr32 div ebx
67 f7 f4                    addr32 div esp
67 41 f7 30                 div    DWORD PTR [r8d]
67 41 f7 34 12              div    DWORD PTR [r10d+edx*1]
67 41 f7 35 12 34 56 78     div    DWORD PTR [eip+0x78563412]
67 41 f7 36                 div    DWORD PTR [r14d]
66 67 f7 30                 div    WORD PTR [eax]
66 67 f7 34 f4              div    WORD PTR [esp+esi*8]
66 67 f7 35 f4 f4 f4 f4     div    WORD PTR [eip+0xfffffffff4f4f4f4]
66 67 f7 36                 div    WORD PTR [esi]
66 67 f7 70 f4              div    WORD PTR [eax-0xc]
66 67 f7 f3                 addr32 div bx
66 67 f7 f4                 addr32 div sp
66 67 41 f7 30              div    WORD PTR [r8d]
66 67 41 f7 34 12           div    WORD PTR [r10d+edx*1]
66 67 41 f7 35 12 34 56 78  div    WORD PTR [eip+0x78563412]
66 67 41 f7 36              div    WORD PTR [r14d]
66 67 48 f7 30              data16 div QWORD PTR [eax]
66 67 48 f7 34 f4           data16 div QWORD PTR [esp+esi*8]
66 67 48 f7 35 f4 f4 f4 f4  data16 div QWORD PTR [eip+0xfffffffff4f4f4f4]
48 f7 36                    div    QWORD PTR [rsi]
48 f7 70 f4                 div    QWORD PTR [rax-0xc]
48 f7 f3                    div    rbx
48 f7 f4                    div    rsp
49 f7 30                    div    QWORD PTR [r8]
49 f7 34 f4                 div    QWORD PTR [r12+rsi*8]
49 f7 35 f4 f4 f4 f4        div    QWORD PTR [rip+0xfffffffff4f4f4f4]
49 f7 36                    div    QWORD PTR [r14]
49 f7 70 f4                 div    QWORD PTR [r8-0xc]
49 f7 f3                    div    r11
49 f7 f4                    div    r12
67 48 f7 36                 div    QWORD PTR [esi]
67 48 f7 70 f4              div    QWORD PTR [eax-0xc]
67 48 f7 f3                 addr32 div rbx
67 48 f7 f4                 addr32 div rsp
67 49 f7 30                 div    QWORD PTR [r8d]
67 49 f7 34 f4              div    QWORD PTR [r12d+esi*8]
67 49 f7 35 f4 f4 f4 f4     div    QWORD PTR [eip+0xfffffffff4f4f4f4]
67 49 f7 36                 div    QWORD PTR [r14d]
67 49 f7 70 f4              div    QWORD PTR [r8d-0xc]
67 49 f7 f3                 addr32 div r11
67 49 f7 f4                 addr32 div r12
f6 10                       not    BYTE PTR [rax]
f6 14 f4                    not    BYTE PTR [rsp+rsi*8]
f6 15 f4 f4 f4 f4           not    BYTE PTR [rip+0xfffffffff4f4f4f4]
f6 16                       not    BYTE PTR [rsi]
f6 50 f4                    not    BYTE PTR [rax-0xc]
f6 d3                       not    bl
f6 d4                       not    ah
41 f6 10                    not    BYTE PTR [r8]
41 f6 14 12                 not    BYTE PTR [r10+rdx*1]
41 f6 15 12 34 56 78        not    BYTE PTR [rip+0x78563412]
41 f6 16                    not    BYTE PTR [r14]
f7 10                       not    DWORD PTR [rax]
f7 14 f4                    not    DWORD PTR [rsp+rsi*8]
f7 15 f4 f4 f4 f4           not    DWORD PTR [rip+0xfffffffff4f4f4f4]
f7 16                       not    DWORD PTR [rsi]
f7 50 f4                    not    DWORD PTR [rax-0xc]
f7 d3                       not    ebx
f7 d4                       not    esp
41 f7 10                    not    DWORD PTR [r8]
41 f7 14 12                 not    DWORD PTR [r10+rdx*1]
41 f7 15 12 34 56 78        not    DWORD PTR [rip+0x78563412]
41 f7 16                    not    DWORD PTR [r14]
66 f7 10                    not    WORD PTR [rax]
66 f7 14 f4                 not    WORD PTR [rsp+rsi*8]
66 f7 15 f4 f4 f4 f4        not    WORD PTR [rip+0xfffffffff4f4f4f4]
66 f7 16                    not    WORD PTR [rsi]
66 f7 50 f4                 not    WORD PTR [rax-0xc]
66 f7 d3                    not    bx
66 f7 d4                    not    sp
66 41 f7 10                 not    WORD PTR [r8]
66 41 f7 14 12              not    WORD PTR [r10+rdx*1]
66 41 f7 15 12 34 56 78     not    WORD PTR [rip+0x78563412]
66 41 f7 16                 not    WORD PTR [r14]
67 f7 10                    not    DWORD PTR [eax]
67 f7 14 f4                 not    DWORD PTR [esp+esi*8]
67 f7 15 f4 f4 f4 f4        not    DWORD PTR [eip+0xfffffffff4f4f4f4]
67 f7 16                    not    DWORD PTR [esi]
67 f7 50 f4                 not    DWORD PTR [eax-0xc]
67 f7 d3                    addr32 not ebx
67 f7 d4                    addr32 not esp
67 41 f7 10                 not    DWORD PTR [r8d]
67 41 f7 14 12              not    DWORD PTR [r10d+edx*1]
67 41 f7 15 12 34 56 78     not    DWORD PTR [eip+0x78563412]
67 41 f7 16                 not    DWORD PTR [r14d]
66 67 f7 10                 not    WORD PTR [eax]
66 67 f7 14 f4              not    WORD PTR [esp+esi*8]
66 67 f7 15 f4 f4 f4 f4     not    WORD PTR [eip+0xfffffffff4f4f4f4]
66 67 f7 16                 not    WORD PTR [esi]
66 67 f7 50 f4              not    WORD PTR [eax-0xc]
66 67 f7 d3                 addr32 not bx
66 67 f7 d4                 addr32 not sp
66 67 41 f7 10              not    WORD PTR [r8d]
66 67 41 f7 14 12           not    WORD PTR [r10d+edx*1]
66 67 41 f7 15 12 34 56 78  not    WORD PTR [eip+0x78563412]
66 67 41 f7 16              not    WORD PTR [r14d]
66 67 48 f7 10              data16 not QWORD PTR [eax]
66 67 48 f7 14 f4           data16 not QWORD PTR [esp+esi*8]
66 67 48 f7 15 f4 f4 f4 f4  data16 not QWORD PTR [eip+0xfffffffff4f4f4f4]
48 f7 16                    not    QWORD PTR [rsi]
48 f7 50 f4                 not    QWORD PTR [rax-0xc]
48 f7 d3                    not    rbx
48 f7 d4                    not    rsp
49 f7 10                    not    QWORD PTR [r8]
49 f7 14 f4                 not    QWORD PTR [r12+rsi*8]
49 f7 15 f4 f4 f4 f4        not    QWORD PTR [rip+0xfffffffff4f4f4f4]
49 f7 16                    not    QWORD PTR [r14]
49 f7 50 f4                 not    QWORD PTR [r8-0xc]
49 f7 d3                    not    r11
49 f7 d4                    not    r12
67 48 f7 16                 not    QWORD PTR [esi]
67 48 f7 50 f4              not    QWORD PTR [eax-0xc]
67 48 f7 d3                 addr32 not rbx
67 48 f7 d4                 addr32 not rsp
67 49 f7 10                 not    QWORD PTR [r8d]
67 49 f7 14 f4              not    QWORD PTR [r12d+esi*8]
67 49 f7 15 f4 f4 f4 f4     not    QWORD PTR [eip+0xfffffffff4f4f4f4]
67 49 f7 16                 not    QWORD PTR [r14d]
67 49 f7 50 f4              not    QWORD PTR [r8d-0xc]
67 49 f7 d3                 addr32 not r11
67 49 f7 d4                 addr32 not r12
