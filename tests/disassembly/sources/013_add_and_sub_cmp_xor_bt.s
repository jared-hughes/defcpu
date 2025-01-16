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
0c 12                       or     $0x12, %al
66 0d 12 34                 or     $0x3412, %ax
0d 12 34 56 78              or     $0x78563412, %eax
41 0d 12 34 56 78           rex.B or $0x78563412, %eax
41 0d 12 34 56 88           rex.B or $0x88563412, %eax
48 0d 12 34 56 78           or     $0x78563412, %rax
48 0d 12 34 56 88           or     $0xffffffff88563412, %rax
49 0d 12 34 56 78           rex.WB or $0x78563412, %rax
49 0d 12 34 56 88           rex.WB or $0xffffffff88563412, %rax
80 c8 12                    or     $0x12, %al
80 ce 12                    or     $0x12, %dh
40 80 ce 12                 or     $0x12, %sil
66 81 ce 12 34              or     $0x3412, %si
81 ce 12 34 56 78           or     $0x78563412, %esi
48 81 ce 12 34 56 78        or     $0x78563412, %rsi
48 81 ce 12 34 56 88        or     $0xffffffff88563412, %rsi
66 83 ce 12                 or     $0x12, %si
66 83 ce 82                 or     $0xff82, %si
83 ce 12                    or     $0x12, %esi
83 ce 82                    or     $0xffffff82, %esi
41 83 ce 12                 or     $0x12, %r14d
41 83 ce 82                 or     $0xffffff82, %r14d
48 83 ce 12                 or     $0x12, %rsi
48 83 ce 82                 or     $0xffffffffffffff82, %rsi
49 83 ce 12                 or     $0x12, %r14
49 83 ce 82                 or     $0xffffffffffffff82, %r14
08 23                       or     %ah, (%rbx)
66 08 23                    data16 or %ah, (%rbx)
67 08 23                    or     %ah, (%ebx)
66 67 08 23                 data16 or %ah, (%ebx)
40 08 23                    or     %spl, (%rbx)
67 40 08 23                 or     %spl, (%ebx)
66 40 08 23                 data16 or %spl, (%rbx)
66 67 40 08 23              data16 or %spl, (%ebx)
41 08 23                    or     %spl, (%r11)
67 41 08 23                 or     %spl, (%r11d)
66 41 08 23                 data16 or %spl, (%r11)
66 67 41 08 23              data16 or %spl, (%r11d)
48 08 23                    rex.W or %spl, (%rbx)
67 48 08 23                 rex.W or %spl, (%ebx)
66 48 08 23                 data16 rex.W or %spl, (%rbx)
66 67 48 08 23              data16 rex.W or %spl, (%ebx)
09 23                       or     %esp, (%rbx)
66 09 23                    or     %sp, (%rbx)
67 09 23                    or     %esp, (%ebx)
66 67 09 23                 or     %sp, (%ebx)
40 09 23                    rex or %esp, (%rbx)
67 40 09 23                 rex or %esp, (%ebx)
66 40 09 23                 rex or %sp, (%rbx)
66 67 40 09 23              rex or %sp, (%ebx)
41 09 23                    or     %esp, (%r11)
67 41 09 23                 or     %esp, (%r11d)
66 41 09 23                 or     %sp, (%r11)
66 67 41 09 23              or     %sp, (%r11d)
48 09 23                    or     %rsp, (%rbx)
67 48 09 23                 or     %rsp, (%ebx)
66 48 09 23                 data16 or %rsp, (%rbx)
66 67 48 09 23              data16 or %rsp, (%ebx)
0a 23                       or     (%rbx), %ah
66 0a 23                    data16 or (%rbx), %ah
67 0a 23                    or     (%ebx), %ah
66 67 0a 23                 data16 or (%ebx), %ah
40 0a 23                    or     (%rbx), %spl
67 40 0a 23                 or     (%ebx), %spl
66 40 0a 23                 data16 or (%rbx), %spl
66 67 40 0a 23              data16 or (%ebx), %spl
41 0a 23                    or     (%r11), %spl
67 41 0a 23                 or     (%r11d), %spl
66 41 0a 23                 data16 or (%r11), %spl
66 67 41 0a 23              data16 or (%r11d), %spl
48 0a 23                    rex.W or (%rbx), %spl
67 48 0a 23                 rex.W or (%ebx), %spl
66 48 0a 23                 data16 rex.W or (%rbx), %spl
66 67 48 0a 23              data16 rex.W or (%ebx), %spl
0b 23                       or     (%rbx), %esp
66 0b 23                    or     (%rbx), %sp
67 0b 23                    or     (%ebx), %esp
66 67 0b 23                 or     (%ebx), %sp
40 0b 23                    rex or (%rbx), %esp
67 40 0b 23                 rex or (%ebx), %esp
66 40 0b 23                 rex or (%rbx), %sp
66 67 40 0b 23              rex or (%ebx), %sp
41 0b 23                    or     (%r11), %esp
67 41 0b 23                 or     (%r11d), %esp
66 41 0b 23                 or     (%r11), %sp
66 67 41 0b 23              or     (%r11d), %sp
48 0b 23                    or     (%rbx), %rsp
67 48 0b 23                 or     (%ebx), %rsp
14 12                       adc    $0x12, %al
66 15 12 34                 adc    $0x3412, %ax
15 12 34 56 78              adc    $0x78563412, %eax
41 15 12 34 56 78           rex.B adc $0x78563412, %eax
41 15 12 34 56 88           rex.B adc $0x88563412, %eax
48 15 12 34 56 78           adc    $0x78563412, %rax
48 15 12 34 56 88           adc    $0xffffffff88563412, %rax
49 15 12 34 56 78           rex.WB adc $0x78563412, %rax
49 15 12 34 56 88           rex.WB adc $0xffffffff88563412, %rax
80 d0 12                    adc    $0x12, %al
80 d6 12                    adc    $0x12, %dh
40 80 d6 12                 adc    $0x12, %sil
66 81 d6 12 34              adc    $0x3412, %si
81 d6 12 34 56 78           adc    $0x78563412, %esi
48 81 d6 12 34 56 78        adc    $0x78563412, %rsi
48 81 d6 12 34 56 88        adc    $0xffffffff88563412, %rsi
66 83 d6 12                 adc    $0x12, %si
66 83 d6 82                 adc    $0xff82, %si
83 d6 12                    adc    $0x12, %esi
83 d6 82                    adc    $0xffffff82, %esi
41 83 d6 12                 adc    $0x12, %r14d
41 83 d6 82                 adc    $0xffffff82, %r14d
48 83 d6 12                 adc    $0x12, %rsi
48 83 d6 82                 adc    $0xffffffffffffff82, %rsi
49 83 d6 12                 adc    $0x12, %r14
49 83 d6 82                 adc    $0xffffffffffffff82, %r14
10 23                       adc    %ah, (%rbx)
66 10 23                    data16 adc %ah, (%rbx)
67 10 23                    adc    %ah, (%ebx)
66 67 10 23                 data16 adc %ah, (%ebx)
40 10 23                    adc    %spl, (%rbx)
67 40 10 23                 adc    %spl, (%ebx)
66 40 10 23                 data16 adc %spl, (%rbx)
66 67 40 10 23              data16 adc %spl, (%ebx)
41 10 23                    adc    %spl, (%r11)
67 41 10 23                 adc    %spl, (%r11d)
66 41 10 23                 data16 adc %spl, (%r11)
66 67 41 10 23              data16 adc %spl, (%r11d)
48 10 23                    rex.W adc %spl, (%rbx)
67 48 10 23                 rex.W adc %spl, (%ebx)
66 48 10 23                 data16 rex.W adc %spl, (%rbx)
66 67 48 10 23              data16 rex.W adc %spl, (%ebx)
11 23                       adc    %esp, (%rbx)
66 11 23                    adc    %sp, (%rbx)
67 11 23                    adc    %esp, (%ebx)
66 67 11 23                 adc    %sp, (%ebx)
40 11 23                    rex adc %esp, (%rbx)
67 40 11 23                 rex adc %esp, (%ebx)
66 40 11 23                 rex adc %sp, (%rbx)
66 67 40 11 23              rex adc %sp, (%ebx)
41 11 23                    adc    %esp, (%r11)
67 41 11 23                 adc    %esp, (%r11d)
66 41 11 23                 adc    %sp, (%r11)
66 67 41 11 23              adc    %sp, (%r11d)
48 11 23                    adc    %rsp, (%rbx)
67 48 11 23                 adc    %rsp, (%ebx)
66 48 11 23                 data16 adc %rsp, (%rbx)
66 67 48 11 23              data16 adc %rsp, (%ebx)
12 23                       adc    (%rbx), %ah
66 12 23                    data16 adc (%rbx), %ah
67 12 23                    adc    (%ebx), %ah
66 67 12 23                 data16 adc (%ebx), %ah
40 12 23                    adc    (%rbx), %spl
67 40 12 23                 adc    (%ebx), %spl
66 40 12 23                 data16 adc (%rbx), %spl
66 67 40 12 23              data16 adc (%ebx), %spl
41 12 23                    adc    (%r11), %spl
67 41 12 23                 adc    (%r11d), %spl
66 41 12 23                 data16 adc (%r11), %spl
66 67 41 12 23              data16 adc (%r11d), %spl
48 12 23                    rex.W adc (%rbx), %spl
67 48 12 23                 rex.W adc (%ebx), %spl
66 48 12 23                 data16 rex.W adc (%rbx), %spl
66 67 48 12 23              data16 rex.W adc (%ebx), %spl
13 23                       adc    (%rbx), %esp
66 13 23                    adc    (%rbx), %sp
67 13 23                    adc    (%ebx), %esp
66 67 13 23                 adc    (%ebx), %sp
40 13 23                    rex adc (%rbx), %esp
67 40 13 23                 rex adc (%ebx), %esp
66 40 13 23                 rex adc (%rbx), %sp
66 67 40 13 23              rex adc (%ebx), %sp
41 13 23                    adc    (%r11), %esp
67 41 13 23                 adc    (%r11d), %esp
66 41 13 23                 adc    (%r11), %sp
66 67 41 13 23              adc    (%r11d), %sp
48 13 23                    adc    (%rbx), %rsp
67 48 13 23                 adc    (%ebx), %rsp
1c 12                       sbb    $0x12, %al
66 1d 12 34                 sbb    $0x3412, %ax
1d 12 34 56 78              sbb    $0x78563412, %eax
41 1d 12 34 56 78           rex.B sbb $0x78563412, %eax
41 1d 12 34 56 88           rex.B sbb $0x88563412, %eax
48 1d 12 34 56 78           sbb    $0x78563412, %rax
48 1d 12 34 56 88           sbb    $0xffffffff88563412, %rax
49 1d 12 34 56 78           rex.WB sbb $0x78563412, %rax
49 1d 12 34 56 88           rex.WB sbb $0xffffffff88563412, %rax
80 d8 12                    sbb    $0x12, %al
80 de 12                    sbb    $0x12, %dh
40 80 de 12                 sbb    $0x12, %sil
66 81 de 12 34              sbb    $0x3412, %si
81 de 12 34 56 78           sbb    $0x78563412, %esi
48 81 de 12 34 56 78        sbb    $0x78563412, %rsi
48 81 de 12 34 56 88        sbb    $0xffffffff88563412, %rsi
66 83 de 12                 sbb    $0x12, %si
66 83 de 82                 sbb    $0xff82, %si
83 de 12                    sbb    $0x12, %esi
83 de 82                    sbb    $0xffffff82, %esi
41 83 de 12                 sbb    $0x12, %r14d
41 83 de 82                 sbb    $0xffffff82, %r14d
48 83 de 12                 sbb    $0x12, %rsi
48 83 de 82                 sbb    $0xffffffffffffff82, %rsi
49 83 de 12                 sbb    $0x12, %r14
49 83 de 82                 sbb    $0xffffffffffffff82, %r14
18 23                       sbb    %ah, (%rbx)
66 18 23                    data16 sbb %ah, (%rbx)
67 18 23                    sbb    %ah, (%ebx)
66 67 18 23                 data16 sbb %ah, (%ebx)
40 18 23                    sbb    %spl, (%rbx)
67 40 18 23                 sbb    %spl, (%ebx)
66 40 18 23                 data16 sbb %spl, (%rbx)
66 67 40 18 23              data16 sbb %spl, (%ebx)
41 18 23                    sbb    %spl, (%r11)
67 41 18 23                 sbb    %spl, (%r11d)
66 41 18 23                 data16 sbb %spl, (%r11)
66 67 41 18 23              data16 sbb %spl, (%r11d)
48 18 23                    rex.W sbb %spl, (%rbx)
67 48 18 23                 rex.W sbb %spl, (%ebx)
66 48 18 23                 data16 rex.W sbb %spl, (%rbx)
66 67 48 18 23              data16 rex.W sbb %spl, (%ebx)
19 23                       sbb    %esp, (%rbx)
66 19 23                    sbb    %sp, (%rbx)
67 19 23                    sbb    %esp, (%ebx)
66 67 19 23                 sbb    %sp, (%ebx)
40 19 23                    rex sbb %esp, (%rbx)
67 40 19 23                 rex sbb %esp, (%ebx)
66 40 19 23                 rex sbb %sp, (%rbx)
66 67 40 19 23              rex sbb %sp, (%ebx)
41 19 23                    sbb    %esp, (%r11)
67 41 19 23                 sbb    %esp, (%r11d)
66 41 19 23                 sbb    %sp, (%r11)
66 67 41 19 23              sbb    %sp, (%r11d)
48 19 23                    sbb    %rsp, (%rbx)
67 48 19 23                 sbb    %rsp, (%ebx)
66 48 19 23                 data16 sbb %rsp, (%rbx)
66 67 48 19 23              data16 sbb %rsp, (%ebx)
1a 23                       sbb    (%rbx), %ah
66 1a 23                    data16 sbb (%rbx), %ah
67 1a 23                    sbb    (%ebx), %ah
66 67 1a 23                 data16 sbb (%ebx), %ah
40 1a 23                    sbb    (%rbx), %spl
67 40 1a 23                 sbb    (%ebx), %spl
66 40 1a 23                 data16 sbb (%rbx), %spl
66 67 40 1a 23              data16 sbb (%ebx), %spl
41 1a 23                    sbb    (%r11), %spl
67 41 1a 23                 sbb    (%r11d), %spl
66 41 1a 23                 data16 sbb (%r11), %spl
66 67 41 1a 23              data16 sbb (%r11d), %spl
48 1a 23                    rex.W sbb (%rbx), %spl
67 48 1a 23                 rex.W sbb (%ebx), %spl
66 48 1a 23                 data16 rex.W sbb (%rbx), %spl
66 67 48 1a 23              data16 rex.W sbb (%ebx), %spl
1b 23                       sbb    (%rbx), %esp
66 1b 23                    sbb    (%rbx), %sp
67 1b 23                    sbb    (%ebx), %esp
66 67 1b 23                 sbb    (%ebx), %sp
40 1b 23                    rex sbb (%rbx), %esp
67 40 1b 23                 rex sbb (%ebx), %esp
66 40 1b 23                 rex sbb (%rbx), %sp
66 67 40 1b 23              rex sbb (%ebx), %sp
41 1b 23                    sbb    (%r11), %esp
67 41 1b 23                 sbb    (%r11d), %esp
66 41 1b 23                 sbb    (%r11), %sp
66 67 41 1b 23              sbb    (%r11d), %sp
48 1b 23                    sbb    (%rbx), %rsp
67 48 1b 23                 sbb    (%ebx), %rsp
24 12                       and    $0x12, %al
66 25 12 34                 and    $0x3412, %ax
25 12 34 56 78              and    $0x78563412, %eax
41 25 12 34 56 78           rex.B and $0x78563412, %eax
41 25 12 34 56 88           rex.B and $0x88563412, %eax
48 25 12 34 56 78           and    $0x78563412, %rax
48 25 12 34 56 88           and    $0xffffffff88563412, %rax
49 25 12 34 56 78           rex.WB and $0x78563412, %rax
49 25 12 34 56 88           rex.WB and $0xffffffff88563412, %rax
80 e0 12                    and    $0x12, %al
80 e6 12                    and    $0x12, %dh
40 80 e6 12                 and    $0x12, %sil
66 81 e6 12 34              and    $0x3412, %si
81 e6 12 34 56 78           and    $0x78563412, %esi
48 81 e6 12 34 56 78        and    $0x78563412, %rsi
48 81 e6 12 34 56 88        and    $0xffffffff88563412, %rsi
66 83 e6 12                 and    $0x12, %si
66 83 e6 82                 and    $0xff82, %si
83 e6 12                    and    $0x12, %esi
83 e6 82                    and    $0xffffff82, %esi
41 83 e6 12                 and    $0x12, %r14d
41 83 e6 82                 and    $0xffffff82, %r14d
48 83 e6 12                 and    $0x12, %rsi
48 83 e6 82                 and    $0xffffffffffffff82, %rsi
49 83 e6 12                 and    $0x12, %r14
49 83 e6 82                 and    $0xffffffffffffff82, %r14
20 23                       and    %ah, (%rbx)
66 20 23                    data16 and %ah, (%rbx)
67 20 23                    and    %ah, (%ebx)
66 67 20 23                 data16 and %ah, (%ebx)
40 20 23                    and    %spl, (%rbx)
67 40 20 23                 and    %spl, (%ebx)
66 40 20 23                 data16 and %spl, (%rbx)
66 67 40 20 23              data16 and %spl, (%ebx)
41 20 23                    and    %spl, (%r11)
67 41 20 23                 and    %spl, (%r11d)
66 41 20 23                 data16 and %spl, (%r11)
66 67 41 20 23              data16 and %spl, (%r11d)
48 20 23                    rex.W and %spl, (%rbx)
67 48 20 23                 rex.W and %spl, (%ebx)
66 48 20 23                 data16 rex.W and %spl, (%rbx)
66 67 48 20 23              data16 rex.W and %spl, (%ebx)
21 23                       and    %esp, (%rbx)
66 21 23                    and    %sp, (%rbx)
67 21 23                    and    %esp, (%ebx)
66 67 21 23                 and    %sp, (%ebx)
40 21 23                    rex and %esp, (%rbx)
67 40 21 23                 rex and %esp, (%ebx)
66 40 21 23                 rex and %sp, (%rbx)
66 67 40 21 23              rex and %sp, (%ebx)
41 21 23                    and    %esp, (%r11)
67 41 21 23                 and    %esp, (%r11d)
66 41 21 23                 and    %sp, (%r11)
66 67 41 21 23              and    %sp, (%r11d)
48 21 23                    and    %rsp, (%rbx)
67 48 21 23                 and    %rsp, (%ebx)
66 48 21 23                 data16 and %rsp, (%rbx)
66 67 48 21 23              data16 and %rsp, (%ebx)
22 23                       and    (%rbx), %ah
66 22 23                    data16 and (%rbx), %ah
67 22 23                    and    (%ebx), %ah
66 67 22 23                 data16 and (%ebx), %ah
40 22 23                    and    (%rbx), %spl
67 40 22 23                 and    (%ebx), %spl
66 40 22 23                 data16 and (%rbx), %spl
66 67 40 22 23              data16 and (%ebx), %spl
41 22 23                    and    (%r11), %spl
67 41 22 23                 and    (%r11d), %spl
66 41 22 23                 data16 and (%r11), %spl
66 67 41 22 23              data16 and (%r11d), %spl
48 22 23                    rex.W and (%rbx), %spl
67 48 22 23                 rex.W and (%ebx), %spl
66 48 22 23                 data16 rex.W and (%rbx), %spl
66 67 48 22 23              data16 rex.W and (%ebx), %spl
23 23                       and    (%rbx), %esp
66 23 23                    and    (%rbx), %sp
67 23 23                    and    (%ebx), %esp
66 67 23 23                 and    (%ebx), %sp
40 23 23                    rex and (%rbx), %esp
67 40 23 23                 rex and (%ebx), %esp
66 40 23 23                 rex and (%rbx), %sp
66 67 40 23 23              rex and (%ebx), %sp
41 23 23                    and    (%r11), %esp
67 41 23 23                 and    (%r11d), %esp
66 41 23 23                 and    (%r11), %sp
66 67 41 23 23              and    (%r11d), %sp
48 23 23                    and    (%rbx), %rsp
67 48 23 23                 and    (%ebx), %rsp
66 48 23 23                 data16 and (%rbx), %rsp
66 67 48 23 23              data16 and (%ebx), %rsp
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
34 12                       xor    $0x12, %al
66 35 12 34                 xor    $0x3412, %ax
35 12 34 56 78              xor    $0x78563412, %eax
41 35 12 34 56 78           rex.B xor $0x78563412, %eax
41 35 12 34 56 88           rex.B xor $0x88563412, %eax
48 35 12 34 56 78           xor    $0x78563412, %rax
48 35 12 34 56 88           xor    $0xffffffff88563412, %rax
49 35 12 34 56 78           rex.WB xor $0x78563412, %rax
49 35 12 34 56 88           rex.WB xor $0xffffffff88563412, %rax
80 f0 12                    xor    $0x12, %al
80 f6 12                    xor    $0x12, %dh
40 80 f6 12                 xor    $0x12, %sil
66 81 f6 12 34              xor    $0x3412, %si
81 f6 12 34 56 78           xor    $0x78563412, %esi
48 81 f6 12 34 56 78        xor    $0x78563412, %rsi
48 81 f6 12 34 56 88        xor    $0xffffffff88563412, %rsi
66 83 f6 12                 xor    $0x12, %si
66 83 f6 82                 xor    $0xff82, %si
83 f6 12                    xor    $0x12, %esi
83 f6 82                    xor    $0xffffff82, %esi
41 83 f6 12                 xor    $0x12, %r14d
41 83 f6 82                 xor    $0xffffff82, %r14d
48 83 f6 12                 xor    $0x12, %rsi
48 83 f6 82                 xor    $0xffffffffffffff82, %rsi
49 83 f6 12                 xor    $0x12, %r14
49 83 f6 82                 xor    $0xffffffffffffff82, %r14
30 23                       xor    %ah, (%rbx)
66 30 23                    data16 xor %ah, (%rbx)
67 30 23                    xor    %ah, (%ebx)
66 67 30 23                 data16 xor %ah, (%ebx)
40 30 23                    xor    %spl, (%rbx)
67 40 30 23                 xor    %spl, (%ebx)
66 40 30 23                 data16 xor %spl, (%rbx)
66 67 40 30 23              data16 xor %spl, (%ebx)
41 30 23                    xor    %spl, (%r11)
67 41 30 23                 xor    %spl, (%r11d)
66 41 30 23                 data16 xor %spl, (%r11)
66 67 41 30 23              data16 xor %spl, (%r11d)
48 30 23                    rex.W xor %spl, (%rbx)
67 48 30 23                 rex.W xor %spl, (%ebx)
66 48 30 23                 data16 rex.W xor %spl, (%rbx)
66 67 48 30 23              data16 rex.W xor %spl, (%ebx)
31 23                       xor    %esp, (%rbx)
66 31 23                    xor    %sp, (%rbx)
67 31 23                    xor    %esp, (%ebx)
66 67 31 23                 xor    %sp, (%ebx)
40 31 23                    rex xor %esp, (%rbx)
67 40 31 23                 rex xor %esp, (%ebx)
66 40 31 23                 rex xor %sp, (%rbx)
66 67 40 31 23              rex xor %sp, (%ebx)
41 31 23                    xor    %esp, (%r11)
67 41 31 23                 xor    %esp, (%r11d)
66 41 31 23                 xor    %sp, (%r11)
66 67 41 31 23              xor    %sp, (%r11d)
48 31 23                    xor    %rsp, (%rbx)
67 48 31 23                 xor    %rsp, (%ebx)
66 48 31 23                 data16 xor %rsp, (%rbx)
66 67 48 31 23              data16 xor %rsp, (%ebx)
32 23                       xor    (%rbx), %ah
66 32 23                    data16 xor (%rbx), %ah
67 32 23                    xor    (%ebx), %ah
66 67 32 23                 data16 xor (%ebx), %ah
40 32 23                    xor    (%rbx), %spl
67 40 32 23                 xor    (%ebx), %spl
66 40 32 23                 data16 xor (%rbx), %spl
66 67 40 32 23              data16 xor (%ebx), %spl
41 32 23                    xor    (%r11), %spl
67 41 32 23                 xor    (%r11d), %spl
66 41 32 23                 data16 xor (%r11), %spl
66 67 41 32 23              data16 xor (%r11d), %spl
48 32 23                    rex.W xor (%rbx), %spl
67 48 32 23                 rex.W xor (%ebx), %spl
66 48 32 23                 data16 rex.W xor (%rbx), %spl
66 67 48 32 23              data16 rex.W xor (%ebx), %spl
33 23                       xor    (%rbx), %esp
66 33 23                    xor    (%rbx), %sp
67 33 23                    xor    (%ebx), %esp
66 67 33 23                 xor    (%ebx), %sp
40 33 23                    rex xor (%rbx), %esp
67 40 33 23                 rex xor (%ebx), %esp
66 40 33 23                 rex xor (%rbx), %sp
66 67 40 33 23              rex xor (%ebx), %sp
41 33 23                    xor    (%r11), %esp
67 41 33 23                 xor    (%r11d), %esp
66 41 33 23                 xor    (%r11), %sp
66 67 41 33 23              xor    (%r11d), %sp
48 33 23                    xor    (%rbx), %rsp
67 48 33 23                 xor    (%ebx), %rsp
66 48 33 23                 data16 xor (%rbx), %rsp
66 67 48 33 23              data16 xor (%ebx), %rsp
66 0f ba e6 12              bt     $0x12, %si
66 0f ba e6 82              bt     $0x82, %si
0f ba e6 12                 bt     $0x12, %esi
0f ba e6 82                 bt     $0x82, %esi
41 0f ba e6 12              bt     $0x12, %r14d
41 0f ba e6 82              bt     $0x82, %r14d
48 0f ba e6 12              bt     $0x12, %rsi
48 0f ba e6 82              bt     $0x82, %rsi
49 0f ba e6 12              bt     $0x12, %r14
49 0f ba e6 82              bt     $0x82, %r14
0f ba 23 12                 btl    $0x12, (%rbx)
0f ba 23 82                 btl    $0x82, (%rbx)
66 0f ba 23 12              btw    $0x12, (%rbx)
66 0f ba 23 82              btw    $0x82, (%rbx)
67 0f ba 23 12              btl    $0x12, (%ebx)
67 0f ba 23 82              btl    $0x82, (%ebx)
66 67 0f ba 23 12           btw    $0x12, (%ebx)
66 67 0f ba 23 82           btw    $0x82, (%ebx)
49 0f ba 23 12              btq    $0x12, (%r11)
49 0f ba 23 82              btq    $0x82, (%r11)
66 49 0f ba 23 12           data16 btq $0x12, (%r11)
66 49 0f ba 23 82           data16 btq $0x82, (%r11)
67 49 0f ba 23 12           btq    $0x12, (%r11d)
67 49 0f ba 23 82           btq    $0x82, (%r11d)
66 67 49 0f ba 23 12        data16 btq $0x12, (%r11d)
66 67 49 0f ba 23 82        data16 btq $0x82, (%r11d)
0f a3 e6                    bt     %esp, %esi
41 0f a3 e6                 bt     %esp, %r14d
49 0f a3 e6                 bt     %rsp, %r14
66 0f a3 e6                 bt     %sp, %si
0f a3 23                    bt     %esp, (%rbx)
66 0f a3 23                 bt     %sp, (%rbx)
67 0f a3 23                 bt     %esp, (%ebx)
66 67 0f a3 23              bt     %sp, (%ebx)
40 0f a3 23                 rex bt %esp, (%rbx)
67 40 0f a3 23              rex bt %esp, (%ebx)
66 40 0f a3 23              rex bt %sp, (%rbx)
66 67 40 0f a3 23           rex bt %sp, (%ebx)
41 0f a3 23                 bt     %esp, (%r11)
67 41 0f a3 23              bt     %esp, (%r11d)
66 41 0f a3 23              bt     %sp, (%r11)
66 67 41 0f a3 23           bt     %sp, (%r11d)
48 0f a3 23                 bt     %rsp, (%rbx)
67 48 0f a3 23              bt     %rsp, (%ebx)
66 48 0f a3 23              data16 bt %rsp, (%rbx)
66 67 48 0f a3 23           data16 bt %rsp, (%ebx)
