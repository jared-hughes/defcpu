04 12                       add    $0x12, %al
66 05 12 34                 add    $0x3412, %ax
05 12 34 56 78              add    $0x78563412, %eax
41 05 12 34 56 78           rex.B add $0x78563412, %eax
41 05 12 34 56 88           rex.B add $0x88563412, %eax
48 05 12 34 56 78           add    $0x78563412, %rax
48 05 12 34 56 88           add    $0xffffffff88563412, %rax
49 05 12 34 56 78           rex.WB add $0x78563412, %rax
49 05 12 34 56 88           rex.WB add $0xffffffff88563412, %rax
80 c0 12                    add    $0x12, %al
80 c6 12                    add    $0x12, %dh
40 80 c6 12                 add    $0x12, %sil
66 81 c6 12 34              add    $0x3412, %si
81 c6 12 34 56 78           add    $0x78563412, %esi
48 81 c6 12 34 56 78        add    $0x78563412, %rsi
48 81 c6 12 34 56 88        add    $0xffffffff88563412, %rsi
66 83 c6 12                 add    $0x12, %si
66 83 c6 82                 add    $0xff82, %si
83 c6 12                    add    $0x12, %esi
83 c6 82                    add    $0xffffff82, %esi
41 83 c6 12                 add    $0x12, %r14d
41 83 c6 82                 add    $0xffffff82, %r14d
48 83 c6 12                 add    $0x12, %rsi
48 83 c6 82                 add    $0xffffffffffffff82, %rsi
49 83 c6 12                 add    $0x12, %r14
49 83 c6 82                 add    $0xffffffffffffff82, %r14
00 23                       add    %ah, (%rbx)
66 00 23                    data16 add %ah, (%rbx)
67 00 23                    add    %ah, (%ebx)
66 67 00 23                 data16 add %ah, (%ebx)
40 00 23                    add    %spl, (%rbx)
67 40 00 23                 add    %spl, (%ebx)
66 40 00 23                 data16 add %spl, (%rbx)
66 67 40 00 23              data16 add %spl, (%ebx)
41 00 23                    add    %spl, (%r11)
67 41 00 23                 add    %spl, (%r11d)
66 41 00 23                 data16 add %spl, (%r11)
66 67 41 00 23              data16 add %spl, (%r11d)
48 00 23                    rex.W add %spl, (%rbx)
67 48 00 23                 rex.W add %spl, (%ebx)
66 48 00 23                 data16 rex.W add %spl, (%rbx)
66 67 48 00 23              data16 rex.W add %spl, (%ebx)
01 23                       add    %esp, (%rbx)
66 01 23                    add    %sp, (%rbx)
67 01 23                    add    %esp, (%ebx)
66 67 01 23                 add    %sp, (%ebx)
40 01 23                    rex add %esp, (%rbx)
67 40 01 23                 rex add %esp, (%ebx)
66 40 01 23                 rex add %sp, (%rbx)
66 67 40 01 23              rex add %sp, (%ebx)
41 01 23                    add    %esp, (%r11)
67 41 01 23                 add    %esp, (%r11d)
66 41 01 23                 add    %sp, (%r11)
66 67 41 01 23              add    %sp, (%r11d)
48 01 23                    add    %rsp, (%rbx)
67 48 01 23                 add    %rsp, (%ebx)
66 48 01 23                 data16 add %rsp, (%rbx)
66 67 48 01 23              data16 add %rsp, (%ebx)
02 23                       add    (%rbx), %ah
66 02 23                    data16 add (%rbx), %ah
67 02 23                    add    (%ebx), %ah
66 67 02 23                 data16 add (%ebx), %ah
40 02 23                    add    (%rbx), %spl
67 40 02 23                 add    (%ebx), %spl
66 40 02 23                 data16 add (%rbx), %spl
66 67 40 02 23              data16 add (%ebx), %spl
41 02 23                    add    (%r11), %spl
67 41 02 23                 add    (%r11d), %spl
66 41 02 23                 data16 add (%r11), %spl
66 67 41 02 23              data16 add (%r11d), %spl
48 02 23                    rex.W add (%rbx), %spl
67 48 02 23                 rex.W add (%ebx), %spl
66 48 02 23                 data16 rex.W add (%rbx), %spl
66 67 48 02 23              data16 rex.W add (%ebx), %spl
03 23                       add    (%rbx), %esp
66 03 23                    add    (%rbx), %sp
67 03 23                    add    (%ebx), %esp
66 67 03 23                 add    (%ebx), %sp
40 03 23                    rex add (%rbx), %esp
67 40 03 23                 rex add (%ebx), %esp
66 40 03 23                 rex add (%rbx), %sp
66 67 40 03 23              rex add (%ebx), %sp
41 03 23                    add    (%r11), %esp
67 41 03 23                 add    (%r11d), %esp
66 41 03 23                 add    (%r11), %sp
66 67 41 03 23              add    (%r11d), %sp
48 03 23                    add    (%rbx), %rsp
67 48 03 23                 add    (%ebx), %rsp
66 48 03 23                 data16 add (%rbx), %rsp
66 67 48 03 23              data16 add (%ebx), %rsp
