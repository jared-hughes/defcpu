58                          pop    rax
59                          pop    rcx
5a                          pop    rdx
5b                          pop    rbx
5c                          pop    rsp
5d                          pop    rbp
5e                          pop    rsi
5f                          pop    rdi
66 58                       pop    ax
66 59                       pop    cx
66 5a                       pop    dx
66 5b                       pop    bx
66 5c                       pop    sp
66 5d                       pop    bp
66 5e                       pop    si
66 5f                       pop    di
41 58                       pop    r8
41 59                       pop    r9
41 5a                       pop    r10
41 5b                       pop    r11
41 5c                       pop    r12
41 5d                       pop    r13
41 5e                       pop    r14
41 5f                       pop    r15
66 41 58                    pop    r8w
66 41 59                    pop    r9w
66 41 5a                    pop    r10w
66 41 5b                    pop    r11w
66 41 5c                    pop    r12w
66 41 5d                    pop    r13w
66 41 5e                    pop    r14w
66 41 5f                    pop    r15w
48 58                       rex.W pop rax
48 59                       rex.W pop rcx
48 5a                       rex.W pop rdx
48 5b                       rex.W pop rbx
48 5c                       rex.W pop rsp
48 5d                       rex.W pop rbp
48 5e                       rex.W pop rsi
48 5f                       rex.W pop rdi
66 48 58                    data16 rex.W pop rax
66 48 59                    data16 rex.W pop rcx
66 48 5a                    data16 rex.W pop rdx
66 48 5b                    data16 rex.W pop rbx
66 48 5c                    data16 rex.W pop rsp
66 48 5d                    data16 rex.W pop rbp
66 48 5e                    data16 rex.W pop rsi
66 48 5f                    data16 rex.W pop rdi
49 58                       rex.WB pop r8
49 59                       rex.WB pop r9
49 5a                       rex.WB pop r10
49 5b                       rex.WB pop r11
49 5c                       rex.WB pop r12
49 5d                       rex.WB pop r13
49 5e                       rex.WB pop r14
49 5f                       rex.WB pop r15
66 49 58                    data16 rex.WB pop r8
66 49 59                    data16 rex.WB pop r9
66 49 5a                    data16 rex.WB pop r10
66 49 5b                    data16 rex.WB pop r11
66 49 5c                    data16 rex.WB pop r12
66 49 5d                    data16 rex.WB pop r13
66 49 5e                    data16 rex.WB pop r14
66 49 5f                    data16 rex.WB pop r15
8f 00                       pop    QWORD PTR [rax]
8f 04 f4                    pop    QWORD PTR [rsp+rsi*8]
8f 05 f4 f4 f4 f4           pop    QWORD PTR [rip+0xfffffffff4f4f4f4]
8f 06                       pop    QWORD PTR [rsi]
8f 40 f4                    pop    QWORD PTR [rax-0xc]
8f c3                       pop    rbx
8f c4                       pop    rsp
41 8f 00                    pop    QWORD PTR [r8]
41 8f 04 12                 pop    QWORD PTR [r10+rdx*1]
41 8f 05 12 34 56 78        pop    QWORD PTR [rip+0x78563412]
41 8f 06                    pop    QWORD PTR [r14]
66 8f 00                    pop    WORD PTR [rax]
66 8f 04 f4                 pop    WORD PTR [rsp+rsi*8]
66 8f 05 f4 f4 f4 f4        pop    WORD PTR [rip+0xfffffffff4f4f4f4]
66 8f 06                    pop    WORD PTR [rsi]
66 8f 40 f4                 pop    WORD PTR [rax-0xc]
66 8f c3                    pop    bx
66 8f c4                    pop    sp
66 41 8f 00                 pop    WORD PTR [r8]
66 41 8f 04 12              pop    WORD PTR [r10+rdx*1]
66 41 8f 05 12 34 56 78     pop    WORD PTR [rip+0x78563412]
66 41 8f 06                 pop    WORD PTR [r14]
67 8f 00                    pop    QWORD PTR [eax]
67 8f 04 f4                 pop    QWORD PTR [esp+esi*8]
67 8f 05 f4 f4 f4 f4        pop    QWORD PTR [eip+0xfffffffff4f4f4f4]
67 8f 06                    pop    QWORD PTR [esi]
67 8f 40 f4                 pop    QWORD PTR [eax-0xc]
67 8f c3                    addr32 pop rbx
67 8f c4                    addr32 pop rsp
67 41 8f 00                 pop    QWORD PTR [r8d]
67 41 8f 04 12              pop    QWORD PTR [r10d+edx*1]
67 41 8f 05 12 34 56 78     pop    QWORD PTR [eip+0x78563412]
67 41 8f 06                 pop    QWORD PTR [r14d]
66 67 8f 00                 pop    WORD PTR [eax]
66 67 8f 04 f4              pop    WORD PTR [esp+esi*8]
66 67 8f 05 f4 f4 f4 f4     pop    WORD PTR [eip+0xfffffffff4f4f4f4]
66 67 8f 06                 pop    WORD PTR [esi]
66 67 8f 40 f4              pop    WORD PTR [eax-0xc]
66 67 8f c3                 addr32 pop bx
66 67 8f c4                 addr32 pop sp
66 67 41 8f 00              pop    WORD PTR [r8d]
66 67 41 8f 04 12           pop    WORD PTR [r10d+edx*1]
66 67 41 8f 05 12 34 56 78  pop    WORD PTR [eip+0x78563412]
66 67 41 8f 06              pop    WORD PTR [r14d]
66 67 48 8f 00              data16 rex.W pop QWORD PTR [eax]
66 67 48 8f 04 f4           data16 rex.W pop QWORD PTR [esp+esi*8]
66 67 48 8f 05 f4 f4 f4 f4  data16 rex.W pop QWORD PTR [eip+0xfffffffff4f4f4f4]
48 8f 06                    rex.W pop QWORD PTR [rsi]
48 8f 40 f4                 rex.W pop QWORD PTR [rax-0xc]
48 8f c3                    rex.W pop rbx
48 8f c4                    rex.W pop rsp
49 8f 00                    rex.WB pop QWORD PTR [r8]
49 8f 04 f4                 rex.WB pop QWORD PTR [r12+rsi*8]
49 8f 05 f4 f4 f4 f4        rex.WB pop QWORD PTR [rip+0xfffffffff4f4f4f4]
49 8f 06                    rex.WB pop QWORD PTR [r14]
49 8f 40 f4                 rex.WB pop QWORD PTR [r8-0xc]
49 8f c3                    rex.WB pop r11
49 8f c4                    rex.WB pop r12
67 48 8f 06                 rex.W pop QWORD PTR [esi]
67 48 8f 40 f4              rex.W pop QWORD PTR [eax-0xc]
67 48 8f c3                 addr32 rex.W pop rbx
67 48 8f c4                 addr32 rex.W pop rsp
67 49 8f 00                 rex.WB pop QWORD PTR [r8d]
67 49 8f 04 f4              rex.WB pop QWORD PTR [r12d+esi*8]
67 49 8f 05 f4 f4 f4 f4     rex.WB pop QWORD PTR [eip+0xfffffffff4f4f4f4]
67 49 8f 06                 rex.WB pop QWORD PTR [r14d]
67 49 8f 40 f4              rex.WB pop QWORD PTR [r8d-0xc]
67 49 8f c3                 addr32 rex.WB pop r11
67 49 8f c4                 addr32 rex.WB pop r12
9d                          popf
66 9d                       popfw
67 9d                       addr32 popf
66 67 9d                    addr32 popfw
48 9d                       rex.W popf
66 48 9d                    data16 rex.W popf
