6a 12                       push   $0x12
6a a2                       push   $0xffffffffffffffa2
66 6a 12                    pushw  $0x12
67 6a 12                    addr32 push $0x12
66 67 6a 12                 addr32 pushw $0x12
68 12 34 56 78              push   $0x78563412
68 a2 b4 c6 d8              push   $0xffffffffd8c6b4a2
66 68 12 34                 pushw  $0x3412
66 68 a2 b4                 pushw  $0xb4a2
67 68 12 34 56 78           addr32 push $0x78563412
67 68 a2 b4 c6 d8           addr32 push $0xffffffffd8c6b4a2
67 66 68 12 34              addr32 pushw $0x3412
67 66 68 a2 b4              addr32 pushw $0xb4a2
48 6a 12                    rex.W push $0x12
48 6a a2                    rex.W push $0xffffffffffffffa2
66 48 6a 12                 data16 rex.W push $0x12
67 48 6a 12                 addr32 rex.W push $0x12
66 67 48 6a 12              data16 addr32 rex.W push $0x12
48 68 12 34 56 78           rex.W push $0x78563412
48 68 a2 b4 c6 d8           rex.W push $0xffffffffd8c6b4a2
66 48 68 12 34 56 78        data16 rex.W push $0x78563412
66 48 68 a2 b4 c6 d8        data16 rex.W push $0xffffffffd8c6b4a2
67 48 68 12 34 56 78        addr32 rex.W push $0x78563412
67 48 68 a2 b4 c6 d8        addr32 rex.W push $0xffffffffd8c6b4a2
67 66 48 68 12 34 56 78     addr32 data16 rex.W push $0x78563412
67 66 48 68 a2 b4 c6 d8     addr32 data16 rex.W push $0xffffffffd8c6b4a2
50                          push   %rax
51                          push   %rcx
52                          push   %rdx
53                          push   %rbx
54                          push   %rsp
55                          push   %rbp
56                          push   %rsi
57                          push   %rdi
66 50                       push   %ax
66 51                       push   %cx
66 52                       push   %dx
66 53                       push   %bx
66 54                       push   %sp
66 55                       push   %bp
66 56                       push   %si
66 57                       push   %di
41 50                       push   %r8
41 51                       push   %r9
41 52                       push   %r10
41 53                       push   %r11
41 54                       push   %r12
41 55                       push   %r13
41 56                       push   %r14
41 57                       push   %r15
66 41 50                    push   %r8w
66 41 51                    push   %r9w
66 41 52                    push   %r10w
66 41 53                    push   %r11w
66 41 54                    push   %r12w
66 41 55                    push   %r13w
66 41 56                    push   %r14w
66 41 57                    push   %r15w
48 50                       rex.W push %rax
48 51                       rex.W push %rcx
48 52                       rex.W push %rdx
48 53                       rex.W push %rbx
48 54                       rex.W push %rsp
48 55                       rex.W push %rbp
48 56                       rex.W push %rsi
48 57                       rex.W push %rdi
66 48 50                    data16 rex.W push %rax
66 48 51                    data16 rex.W push %rcx
66 48 52                    data16 rex.W push %rdx
66 48 53                    data16 rex.W push %rbx
66 48 54                    data16 rex.W push %rsp
66 48 55                    data16 rex.W push %rbp
66 48 56                    data16 rex.W push %rsi
66 48 57                    data16 rex.W push %rdi
49 50                       rex.WB push %r8
49 51                       rex.WB push %r9
49 52                       rex.WB push %r10
49 53                       rex.WB push %r11
49 54                       rex.WB push %r12
49 55                       rex.WB push %r13
49 56                       rex.WB push %r14
49 57                       rex.WB push %r15
66 49 50                    data16 rex.WB push %r8
66 49 51                    data16 rex.WB push %r9
66 49 52                    data16 rex.WB push %r10
66 49 53                    data16 rex.WB push %r11
66 49 54                    data16 rex.WB push %r12
66 49 55                    data16 rex.WB push %r13
66 49 56                    data16 rex.WB push %r14
66 49 57                    data16 rex.WB push %r15
ff 30                       push   (%rax)
ff 34 f4                    push   (%rsp, %rsi, 8)
ff 35 f4 f4 f4 f4           push   -0xb0b0b0c(%rip)
ff 36                       push   (%rsi)
ff 70 f4                    push   -0xc(%rax)
ff f3                       push   %rbx
ff f4                       push   %rsp
41 ff 30                    push   (%r8)
41 ff 34 12                 push   (%r10, %rdx, 1)
41 ff 35 12 34 56 78        push   0x78563412(%rip)
41 ff 36                    push   (%r14)
66 ff 30                    pushw  (%rax)
66 ff 34 f4                 pushw  (%rsp, %rsi, 8)
66 ff 35 f4 f4 f4 f4        pushw  -0xb0b0b0c(%rip)
66 ff 36                    pushw  (%rsi)
66 ff 70 f4                 pushw  -0xc(%rax)
66 ff f3                    push   %bx
66 ff f4                    push   %sp
66 41 ff 30                 pushw  (%r8)
66 41 ff 34 12              pushw  (%r10, %rdx, 1)
66 41 ff 35 12 34 56 78     pushw  0x78563412(%rip)
66 41 ff 36                 pushw  (%r14)
67 ff 30                    push   (%eax)
67 ff 34 f4                 push   (%esp, %esi, 8)
67 ff 35 f4 f4 f4 f4        push   -0xb0b0b0c(%eip)
67 ff 36                    push   (%esi)
67 ff 70 f4                 push   -0xc(%eax)
67 ff f3                    addr32 push %rbx
67 ff f4                    addr32 push %rsp
67 41 ff 30                 push   (%r8d)
67 41 ff 34 12              push   (%r10d, %edx, 1)
67 41 ff 35 12 34 56 78     push   0x78563412(%eip)
67 41 ff 36                 push   (%r14d)
66 67 ff 30                 pushw  (%eax)
66 67 ff 34 f4              pushw  (%esp, %esi, 8)
66 67 ff 35 f4 f4 f4 f4     pushw  -0xb0b0b0c(%eip)
66 67 ff 36                 pushw  (%esi)
66 67 ff 70 f4              pushw  -0xc(%eax)
66 67 ff f3                 addr32 push %bx
66 67 ff f4                 addr32 push %sp
66 67 41 ff 30              pushw  (%r8d)
66 67 41 ff 34 12           pushw  (%r10d, %edx, 1)
66 67 41 ff 35 12 34 56 78  pushw  0x78563412(%eip)
66 67 41 ff 36              pushw  (%r14d)
66 67 48 ff 30              data16 rex.W push (%eax)
66 67 48 ff 34 f4           data16 rex.W push (%esp, %esi, 8)
66 67 48 ff 35 f4 f4 f4 f4  data16 rex.W push -0xb0b0b0c(%eip)
48 ff 36                    rex.W push (%rsi)
48 ff 70 f4                 rex.W push -0xc(%rax)
48 ff f3                    rex.W push %rbx
48 ff f4                    rex.W push %rsp
49 ff 30                    rex.WB push (%r8)
49 ff 34 f4                 rex.WB push (%r12, %rsi, 8)
49 ff 35 f4 f4 f4 f4        rex.WB push -0xb0b0b0c(%rip)
49 ff 36                    rex.WB push (%r14)
49 ff 70 f4                 rex.WB push -0xc(%r8)
49 ff f3                    rex.WB push %r11
49 ff f4                    rex.WB push %r12
67 48 ff 36                 rex.W push (%esi)
67 48 ff 70 f4              rex.W push -0xc(%eax)
67 48 ff f3                 addr32 rex.W push %rbx
67 48 ff f4                 addr32 rex.W push %rsp
67 49 ff 30                 rex.WB push (%r8d)
67 49 ff 34 f4              rex.WB push (%r12d, %esi, 8)
67 49 ff 35 f4 f4 f4 f4     rex.WB push -0xb0b0b0c(%eip)
67 49 ff 36                 rex.WB push (%r14d)
67 49 ff 70 f4              rex.WB push -0xc(%r8d)
67 49 ff f3                 addr32 rex.WB push %r11
67 49 ff f4                 addr32 rex.WB push %r12
9c                          pushf
66 9c                       pushfw
67 9c                       addr32 pushf
66 67 9c                    addr32 pushfw
48 9c                       rex.W pushf
66 48 9c                    data16 rex.W pushf
