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
2c 12                       sub    $0x12, %al
66 2d 12 34                 sub    $0x3412, %ax
2d 12 34 56 78              sub    $0x78563412, %eax
41 2d 12 34 56 78           rex.B sub $0x78563412, %eax
41 2d 12 34 56 88           rex.B sub $0x88563412, %eax
48 2d 12 34 56 78           sub    $0x78563412, %rax
48 2d 12 34 56 88           sub    $0xffffffff88563412, %rax
49 2d 12 34 56 78           rex.WB sub $0x78563412, %rax
49 2d 12 34 56 88           rex.WB sub $0xffffffff88563412, %rax
80 e8 12                    sub    $0x12, %al
80 ee 12                    sub    $0x12, %dh
40 80 ee 12                 sub    $0x12, %sil
66 81 ee 12 34              sub    $0x3412, %si
81 ee 12 34 56 78           sub    $0x78563412, %esi
48 81 ee 12 34 56 78        sub    $0x78563412, %rsi
48 81 ee 12 34 56 88        sub    $0xffffffff88563412, %rsi
66 83 ee 12                 sub    $0x12, %si
66 83 ee 82                 sub    $0xff82, %si
83 ee 12                    sub    $0x12, %esi
83 ee 82                    sub    $0xffffff82, %esi
41 83 ee 12                 sub    $0x12, %r14d
41 83 ee 82                 sub    $0xffffff82, %r14d
48 83 ee 12                 sub    $0x12, %rsi
48 83 ee 82                 sub    $0xffffffffffffff82, %rsi
49 83 ee 12                 sub    $0x12, %r14
49 83 ee 82                 sub    $0xffffffffffffff82, %r14
28 23                       sub    %ah, (%rbx)
66 28 23                    data16 sub %ah, (%rbx)
67 28 23                    sub    %ah, (%ebx)
66 67 28 23                 data16 sub %ah, (%ebx)
40 28 23                    sub    %spl, (%rbx)
67 40 28 23                 sub    %spl, (%ebx)
66 40 28 23                 data16 sub %spl, (%rbx)
66 67 40 28 23              data16 sub %spl, (%ebx)
41 28 23                    sub    %spl, (%r11)
67 41 28 23                 sub    %spl, (%r11d)
66 41 28 23                 data16 sub %spl, (%r11)
66 67 41 28 23              data16 sub %spl, (%r11d)
48 28 23                    rex.W sub %spl, (%rbx)
67 48 28 23                 rex.W sub %spl, (%ebx)
66 48 28 23                 data16 rex.W sub %spl, (%rbx)
66 67 48 28 23              data16 rex.W sub %spl, (%ebx)
29 23                       sub    %esp, (%rbx)
66 29 23                    sub    %sp, (%rbx)
67 29 23                    sub    %esp, (%ebx)
66 67 29 23                 sub    %sp, (%ebx)
40 29 23                    rex sub %esp, (%rbx)
67 40 29 23                 rex sub %esp, (%ebx)
66 40 29 23                 rex sub %sp, (%rbx)
66 67 40 29 23              rex sub %sp, (%ebx)
41 29 23                    sub    %esp, (%r11)
67 41 29 23                 sub    %esp, (%r11d)
66 41 29 23                 sub    %sp, (%r11)
66 67 41 29 23              sub    %sp, (%r11d)
48 29 23                    sub    %rsp, (%rbx)
67 48 29 23                 sub    %rsp, (%ebx)
66 48 29 23                 data16 sub %rsp, (%rbx)
66 67 48 29 23              data16 sub %rsp, (%ebx)
2a 23                       sub    (%rbx), %ah
66 2a 23                    data16 sub (%rbx), %ah
67 2a 23                    sub    (%ebx), %ah
66 67 2a 23                 data16 sub (%ebx), %ah
40 2a 23                    sub    (%rbx), %spl
67 40 2a 23                 sub    (%ebx), %spl
66 40 2a 23                 data16 sub (%rbx), %spl
66 67 40 2a 23              data16 sub (%ebx), %spl
41 2a 23                    sub    (%r11), %spl
67 41 2a 23                 sub    (%r11d), %spl
66 41 2a 23                 data16 sub (%r11), %spl
66 67 41 2a 23              data16 sub (%r11d), %spl
48 2a 23                    rex.W sub (%rbx), %spl
67 48 2a 23                 rex.W sub (%ebx), %spl
66 48 2a 23                 data16 rex.W sub (%rbx), %spl
66 67 48 2a 23              data16 rex.W sub (%ebx), %spl
2b 23                       sub    (%rbx), %esp
66 2b 23                    sub    (%rbx), %sp
67 2b 23                    sub    (%ebx), %esp
66 67 2b 23                 sub    (%ebx), %sp
40 2b 23                    rex sub (%rbx), %esp
67 40 2b 23                 rex sub (%ebx), %esp
66 40 2b 23                 rex sub (%rbx), %sp
66 67 40 2b 23              rex sub (%ebx), %sp
41 2b 23                    sub    (%r11), %esp
67 41 2b 23                 sub    (%r11d), %esp
66 41 2b 23                 sub    (%r11), %sp
66 67 41 2b 23              sub    (%r11d), %sp
48 2b 23                    sub    (%rbx), %rsp
67 48 2b 23                 sub    (%ebx), %rsp
66 48 2b 23                 data16 sub (%rbx), %rsp
66 67 48 2b 23              data16 sub (%ebx), %rsp
3c 12                       cmp    $0x12, %al
66 3d 12 34                 cmp    $0x3412, %ax
3d 12 34 56 78              cmp    $0x78563412, %eax
41 3d 12 34 56 78           rex.B cmp $0x78563412, %eax
41 3d 12 34 56 88           rex.B cmp $0x88563412, %eax
48 3d 12 34 56 78           cmp    $0x78563412, %rax
48 3d 12 34 56 88           cmp    $0xffffffff88563412, %rax
49 3d 12 34 56 78           rex.WB cmp $0x78563412, %rax
49 3d 12 34 56 88           rex.WB cmp $0xffffffff88563412, %rax
80 f8 12                    cmp    $0x12, %al
80 fe 12                    cmp    $0x12, %dh
40 80 fe 12                 cmp    $0x12, %sil
66 81 fe 12 34              cmp    $0x3412, %si
81 fe 12 34 56 78           cmp    $0x78563412, %esi
48 81 fe 12 34 56 78        cmp    $0x78563412, %rsi
48 81 fe 12 34 56 88        cmp    $0xffffffff88563412, %rsi
66 83 fe 12                 cmp    $0x12, %si
66 83 fe 82                 cmp    $0xff82, %si
83 fe 12                    cmp    $0x12, %esi
83 fe 82                    cmp    $0xffffff82, %esi
41 83 fe 12                 cmp    $0x12, %r14d
41 83 fe 82                 cmp    $0xffffff82, %r14d
48 83 fe 12                 cmp    $0x12, %rsi
48 83 fe 82                 cmp    $0xffffffffffffff82, %rsi
49 83 fe 12                 cmp    $0x12, %r14
49 83 fe 82                 cmp    $0xffffffffffffff82, %r14
38 23                       cmp    %ah, (%rbx)
66 38 23                    data16 cmp %ah, (%rbx)
67 38 23                    cmp    %ah, (%ebx)
66 67 38 23                 data16 cmp %ah, (%ebx)
40 38 23                    cmp    %spl, (%rbx)
67 40 38 23                 cmp    %spl, (%ebx)
66 40 38 23                 data16 cmp %spl, (%rbx)
66 67 40 38 23              data16 cmp %spl, (%ebx)
41 38 23                    cmp    %spl, (%r11)
67 41 38 23                 cmp    %spl, (%r11d)
66 41 38 23                 data16 cmp %spl, (%r11)
66 67 41 38 23              data16 cmp %spl, (%r11d)
48 38 23                    rex.W cmp %spl, (%rbx)
67 48 38 23                 rex.W cmp %spl, (%ebx)
66 48 38 23                 data16 rex.W cmp %spl, (%rbx)
66 67 48 38 23              data16 rex.W cmp %spl, (%ebx)
39 23                       cmp    %esp, (%rbx)
66 39 23                    cmp    %sp, (%rbx)
67 39 23                    cmp    %esp, (%ebx)
66 67 39 23                 cmp    %sp, (%ebx)
40 39 23                    rex cmp %esp, (%rbx)
67 40 39 23                 rex cmp %esp, (%ebx)
66 40 39 23                 rex cmp %sp, (%rbx)
66 67 40 39 23              rex cmp %sp, (%ebx)
41 39 23                    cmp    %esp, (%r11)
67 41 39 23                 cmp    %esp, (%r11d)
66 41 39 23                 cmp    %sp, (%r11)
66 67 41 39 23              cmp    %sp, (%r11d)
48 39 23                    cmp    %rsp, (%rbx)
67 48 39 23                 cmp    %rsp, (%ebx)
66 48 39 23                 data16 cmp %rsp, (%rbx)
66 67 48 39 23              data16 cmp %rsp, (%ebx)
3a 23                       cmp    (%rbx), %ah
66 3a 23                    data16 cmp (%rbx), %ah
67 3a 23                    cmp    (%ebx), %ah
66 67 3a 23                 data16 cmp (%ebx), %ah
40 3a 23                    cmp    (%rbx), %spl
67 40 3a 23                 cmp    (%ebx), %spl
66 40 3a 23                 data16 cmp (%rbx), %spl
66 67 40 3a 23              data16 cmp (%ebx), %spl
41 3a 23                    cmp    (%r11), %spl
67 41 3a 23                 cmp    (%r11d), %spl
66 41 3a 23                 data16 cmp (%r11), %spl
66 67 41 3a 23              data16 cmp (%r11d), %spl
48 3a 23                    rex.W cmp (%rbx), %spl
67 48 3a 23                 rex.W cmp (%ebx), %spl
66 48 3a 23                 data16 rex.W cmp (%rbx), %spl
66 67 48 3a 23              data16 rex.W cmp (%ebx), %spl
3b 23                       cmp    (%rbx), %esp
66 3b 23                    cmp    (%rbx), %sp
67 3b 23                    cmp    (%ebx), %esp
66 67 3b 23                 cmp    (%ebx), %sp
40 3b 23                    rex cmp (%rbx), %esp
67 40 3b 23                 rex cmp (%ebx), %esp
66 40 3b 23                 rex cmp (%rbx), %sp
66 67 40 3b 23              rex cmp (%ebx), %sp
41 3b 23                    cmp    (%r11), %esp
67 41 3b 23                 cmp    (%r11d), %esp
66 41 3b 23                 cmp    (%r11), %sp
66 67 41 3b 23              cmp    (%r11d), %sp
48 3b 23                    cmp    (%rbx), %rsp
67 48 3b 23                 cmp    (%ebx), %rsp
66 48 3b 23                 data16 cmp (%rbx), %rsp
66 67 48 3b 23              data16 cmp (%ebx), %rsp
