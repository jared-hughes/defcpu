04 12                       add    al, 0x12
66 05 12 34                 add    ax, 0x3412
05 12 34 56 78              add    eax, 0x78563412
41 05 12 34 56 78           rex.B add eax, 0x78563412
41 05 12 34 56 88           rex.B add eax, 0x88563412
48 05 12 34 56 78           add    rax, 0x78563412
48 05 12 34 56 88           add    rax, 0xffffffff88563412
49 05 12 34 56 78           rex.WB add rax, 0x78563412
49 05 12 34 56 88           rex.WB add rax, 0xffffffff88563412
80 c0 12                    add    al, 0x12
80 c6 12                    add    dh, 0x12
40 80 c6 12                 add    sil, 0x12
66 81 c6 12 34              add    si, 0x3412
81 c6 12 34 56 78           add    esi, 0x78563412
48 81 c6 12 34 56 78        add    rsi, 0x78563412
48 81 c6 12 34 56 88        add    rsi, 0xffffffff88563412
66 83 c6 12                 add    si, 0x12
66 83 c6 82                 add    si, 0xff82
83 c6 12                    add    esi, 0x12
83 c6 82                    add    esi, 0xffffff82
41 83 c6 12                 add    r14d, 0x12
41 83 c6 82                 add    r14d, 0xffffff82
48 83 c6 12                 add    rsi, 0x12
48 83 c6 82                 add    rsi, 0xffffffffffffff82
49 83 c6 12                 add    r14, 0x12
49 83 c6 82                 add    r14, 0xffffffffffffff82
00 23                       add    BYTE PTR [rbx], ah
66 00 23                    data16 add BYTE PTR [rbx], ah
67 00 23                    add    BYTE PTR [ebx], ah
66 67 00 23                 data16 add BYTE PTR [ebx], ah
40 00 23                    add    BYTE PTR [rbx], spl
67 40 00 23                 add    BYTE PTR [ebx], spl
66 40 00 23                 data16 add BYTE PTR [rbx], spl
66 67 40 00 23              data16 add BYTE PTR [ebx], spl
41 00 23                    add    BYTE PTR [r11], spl
67 41 00 23                 add    BYTE PTR [r11d], spl
66 41 00 23                 data16 add BYTE PTR [r11], spl
66 67 41 00 23              data16 add BYTE PTR [r11d], spl
48 00 23                    rex.W add BYTE PTR [rbx], spl
67 48 00 23                 rex.W add BYTE PTR [ebx], spl
66 48 00 23                 data16 rex.W add BYTE PTR [rbx], spl
66 67 48 00 23              data16 rex.W add BYTE PTR [ebx], spl
01 23                       add    DWORD PTR [rbx], esp
66 01 23                    add    WORD PTR [rbx], sp
67 01 23                    add    DWORD PTR [ebx], esp
66 67 01 23                 add    WORD PTR [ebx], sp
40 01 23                    rex add DWORD PTR [rbx], esp
67 40 01 23                 rex add DWORD PTR [ebx], esp
66 40 01 23                 rex add WORD PTR [rbx], sp
66 67 40 01 23              rex add WORD PTR [ebx], sp
41 01 23                    add    DWORD PTR [r11], esp
67 41 01 23                 add    DWORD PTR [r11d], esp
66 41 01 23                 add    WORD PTR [r11], sp
66 67 41 01 23              add    WORD PTR [r11d], sp
48 01 23                    add    QWORD PTR [rbx], rsp
67 48 01 23                 add    QWORD PTR [ebx], rsp
66 48 01 23                 data16 add QWORD PTR [rbx], rsp
66 67 48 01 23              data16 add QWORD PTR [ebx], rsp
02 23                       add    ah, BYTE PTR [rbx]
66 02 23                    data16 add ah, BYTE PTR [rbx]
67 02 23                    add    ah, BYTE PTR [ebx]
66 67 02 23                 data16 add ah, BYTE PTR [ebx]
40 02 23                    add    spl, BYTE PTR [rbx]
67 40 02 23                 add    spl, BYTE PTR [ebx]
66 40 02 23                 data16 add spl, BYTE PTR [rbx]
66 67 40 02 23              data16 add spl, BYTE PTR [ebx]
41 02 23                    add    spl, BYTE PTR [r11]
67 41 02 23                 add    spl, BYTE PTR [r11d]
66 41 02 23                 data16 add spl, BYTE PTR [r11]
66 67 41 02 23              data16 add spl, BYTE PTR [r11d]
48 02 23                    rex.W add spl, BYTE PTR [rbx]
67 48 02 23                 rex.W add spl, BYTE PTR [ebx]
66 48 02 23                 data16 rex.W add spl, BYTE PTR [rbx]
66 67 48 02 23              data16 rex.W add spl, BYTE PTR [ebx]
03 23                       add    esp, DWORD PTR [rbx]
66 03 23                    add    sp, WORD PTR [rbx]
67 03 23                    add    esp, DWORD PTR [ebx]
66 67 03 23                 add    sp, WORD PTR [ebx]
40 03 23                    rex add esp, DWORD PTR [rbx]
67 40 03 23                 rex add esp, DWORD PTR [ebx]
66 40 03 23                 rex add sp, WORD PTR [rbx]
66 67 40 03 23              rex add sp, WORD PTR [ebx]
41 03 23                    add    esp, DWORD PTR [r11]
67 41 03 23                 add    esp, DWORD PTR [r11d]
66 41 03 23                 add    sp, WORD PTR [r11]
66 67 41 03 23              add    sp, WORD PTR [r11d]
48 03 23                    add    rsp, QWORD PTR [rbx]
67 48 03 23                 add    rsp, QWORD PTR [ebx]
0c 12                       or     al, 0x12
66 0d 12 34                 or     ax, 0x3412
0d 12 34 56 78              or     eax, 0x78563412
41 0d 12 34 56 78           rex.B or eax, 0x78563412
41 0d 12 34 56 88           rex.B or eax, 0x88563412
48 0d 12 34 56 78           or     rax, 0x78563412
48 0d 12 34 56 88           or     rax, 0xffffffff88563412
49 0d 12 34 56 78           rex.WB or rax, 0x78563412
49 0d 12 34 56 88           rex.WB or rax, 0xffffffff88563412
80 c8 12                    or     al, 0x12
80 ce 12                    or     dh, 0x12
40 80 ce 12                 or     sil, 0x12
66 81 ce 12 34              or     si, 0x3412
81 ce 12 34 56 78           or     esi, 0x78563412
48 81 ce 12 34 56 78        or     rsi, 0x78563412
48 81 ce 12 34 56 88        or     rsi, 0xffffffff88563412
66 83 ce 12                 or     si, 0x12
66 83 ce 82                 or     si, 0xff82
83 ce 12                    or     esi, 0x12
83 ce 82                    or     esi, 0xffffff82
41 83 ce 12                 or     r14d, 0x12
41 83 ce 82                 or     r14d, 0xffffff82
48 83 ce 12                 or     rsi, 0x12
48 83 ce 82                 or     rsi, 0xffffffffffffff82
49 83 ce 12                 or     r14, 0x12
49 83 ce 82                 or     r14, 0xffffffffffffff82
08 23                       or     BYTE PTR [rbx], ah
66 08 23                    data16 or BYTE PTR [rbx], ah
67 08 23                    or     BYTE PTR [ebx], ah
66 67 08 23                 data16 or BYTE PTR [ebx], ah
40 08 23                    or     BYTE PTR [rbx], spl
67 40 08 23                 or     BYTE PTR [ebx], spl
66 40 08 23                 data16 or BYTE PTR [rbx], spl
66 67 40 08 23              data16 or BYTE PTR [ebx], spl
41 08 23                    or     BYTE PTR [r11], spl
67 41 08 23                 or     BYTE PTR [r11d], spl
66 41 08 23                 data16 or BYTE PTR [r11], spl
66 67 41 08 23              data16 or BYTE PTR [r11d], spl
48 08 23                    rex.W or BYTE PTR [rbx], spl
67 48 08 23                 rex.W or BYTE PTR [ebx], spl
66 48 08 23                 data16 rex.W or BYTE PTR [rbx], spl
66 67 48 08 23              data16 rex.W or BYTE PTR [ebx], spl
09 23                       or     DWORD PTR [rbx], esp
66 09 23                    or     WORD PTR [rbx], sp
67 09 23                    or     DWORD PTR [ebx], esp
66 67 09 23                 or     WORD PTR [ebx], sp
40 09 23                    rex or DWORD PTR [rbx], esp
67 40 09 23                 rex or DWORD PTR [ebx], esp
66 40 09 23                 rex or WORD PTR [rbx], sp
66 67 40 09 23              rex or WORD PTR [ebx], sp
41 09 23                    or     DWORD PTR [r11], esp
67 41 09 23                 or     DWORD PTR [r11d], esp
66 41 09 23                 or     WORD PTR [r11], sp
66 67 41 09 23              or     WORD PTR [r11d], sp
48 09 23                    or     QWORD PTR [rbx], rsp
67 48 09 23                 or     QWORD PTR [ebx], rsp
66 48 09 23                 data16 or QWORD PTR [rbx], rsp
66 67 48 09 23              data16 or QWORD PTR [ebx], rsp
0a 23                       or     ah, BYTE PTR [rbx]
66 0a 23                    data16 or ah, BYTE PTR [rbx]
67 0a 23                    or     ah, BYTE PTR [ebx]
66 67 0a 23                 data16 or ah, BYTE PTR [ebx]
40 0a 23                    or     spl, BYTE PTR [rbx]
67 40 0a 23                 or     spl, BYTE PTR [ebx]
66 40 0a 23                 data16 or spl, BYTE PTR [rbx]
66 67 40 0a 23              data16 or spl, BYTE PTR [ebx]
41 0a 23                    or     spl, BYTE PTR [r11]
67 41 0a 23                 or     spl, BYTE PTR [r11d]
66 41 0a 23                 data16 or spl, BYTE PTR [r11]
66 67 41 0a 23              data16 or spl, BYTE PTR [r11d]
48 0a 23                    rex.W or spl, BYTE PTR [rbx]
67 48 0a 23                 rex.W or spl, BYTE PTR [ebx]
66 48 0a 23                 data16 rex.W or spl, BYTE PTR [rbx]
66 67 48 0a 23              data16 rex.W or spl, BYTE PTR [ebx]
0b 23                       or     esp, DWORD PTR [rbx]
66 0b 23                    or     sp, WORD PTR [rbx]
67 0b 23                    or     esp, DWORD PTR [ebx]
66 67 0b 23                 or     sp, WORD PTR [ebx]
40 0b 23                    rex or esp, DWORD PTR [rbx]
67 40 0b 23                 rex or esp, DWORD PTR [ebx]
66 40 0b 23                 rex or sp, WORD PTR [rbx]
66 67 40 0b 23              rex or sp, WORD PTR [ebx]
41 0b 23                    or     esp, DWORD PTR [r11]
67 41 0b 23                 or     esp, DWORD PTR [r11d]
66 41 0b 23                 or     sp, WORD PTR [r11]
66 67 41 0b 23              or     sp, WORD PTR [r11d]
48 0b 23                    or     rsp, QWORD PTR [rbx]
67 48 0b 23                 or     rsp, QWORD PTR [ebx]
14 12                       adc    al, 0x12
66 15 12 34                 adc    ax, 0x3412
15 12 34 56 78              adc    eax, 0x78563412
41 15 12 34 56 78           rex.B adc eax, 0x78563412
41 15 12 34 56 88           rex.B adc eax, 0x88563412
48 15 12 34 56 78           adc    rax, 0x78563412
48 15 12 34 56 88           adc    rax, 0xffffffff88563412
49 15 12 34 56 78           rex.WB adc rax, 0x78563412
49 15 12 34 56 88           rex.WB adc rax, 0xffffffff88563412
80 d0 12                    adc    al, 0x12
80 d6 12                    adc    dh, 0x12
40 80 d6 12                 adc    sil, 0x12
66 81 d6 12 34              adc    si, 0x3412
81 d6 12 34 56 78           adc    esi, 0x78563412
48 81 d6 12 34 56 78        adc    rsi, 0x78563412
48 81 d6 12 34 56 88        adc    rsi, 0xffffffff88563412
66 83 d6 12                 adc    si, 0x12
66 83 d6 82                 adc    si, 0xff82
83 d6 12                    adc    esi, 0x12
83 d6 82                    adc    esi, 0xffffff82
41 83 d6 12                 adc    r14d, 0x12
41 83 d6 82                 adc    r14d, 0xffffff82
48 83 d6 12                 adc    rsi, 0x12
48 83 d6 82                 adc    rsi, 0xffffffffffffff82
49 83 d6 12                 adc    r14, 0x12
49 83 d6 82                 adc    r14, 0xffffffffffffff82
10 23                       adc    BYTE PTR [rbx], ah
66 10 23                    data16 adc BYTE PTR [rbx], ah
67 10 23                    adc    BYTE PTR [ebx], ah
66 67 10 23                 data16 adc BYTE PTR [ebx], ah
40 10 23                    adc    BYTE PTR [rbx], spl
67 40 10 23                 adc    BYTE PTR [ebx], spl
66 40 10 23                 data16 adc BYTE PTR [rbx], spl
66 67 40 10 23              data16 adc BYTE PTR [ebx], spl
41 10 23                    adc    BYTE PTR [r11], spl
67 41 10 23                 adc    BYTE PTR [r11d], spl
66 41 10 23                 data16 adc BYTE PTR [r11], spl
66 67 41 10 23              data16 adc BYTE PTR [r11d], spl
48 10 23                    rex.W adc BYTE PTR [rbx], spl
67 48 10 23                 rex.W adc BYTE PTR [ebx], spl
66 48 10 23                 data16 rex.W adc BYTE PTR [rbx], spl
66 67 48 10 23              data16 rex.W adc BYTE PTR [ebx], spl
11 23                       adc    DWORD PTR [rbx], esp
66 11 23                    adc    WORD PTR [rbx], sp
67 11 23                    adc    DWORD PTR [ebx], esp
66 67 11 23                 adc    WORD PTR [ebx], sp
40 11 23                    rex adc DWORD PTR [rbx], esp
67 40 11 23                 rex adc DWORD PTR [ebx], esp
66 40 11 23                 rex adc WORD PTR [rbx], sp
66 67 40 11 23              rex adc WORD PTR [ebx], sp
41 11 23                    adc    DWORD PTR [r11], esp
67 41 11 23                 adc    DWORD PTR [r11d], esp
66 41 11 23                 adc    WORD PTR [r11], sp
66 67 41 11 23              adc    WORD PTR [r11d], sp
48 11 23                    adc    QWORD PTR [rbx], rsp
67 48 11 23                 adc    QWORD PTR [ebx], rsp
66 48 11 23                 data16 adc QWORD PTR [rbx], rsp
66 67 48 11 23              data16 adc QWORD PTR [ebx], rsp
12 23                       adc    ah, BYTE PTR [rbx]
66 12 23                    data16 adc ah, BYTE PTR [rbx]
67 12 23                    adc    ah, BYTE PTR [ebx]
66 67 12 23                 data16 adc ah, BYTE PTR [ebx]
40 12 23                    adc    spl, BYTE PTR [rbx]
67 40 12 23                 adc    spl, BYTE PTR [ebx]
66 40 12 23                 data16 adc spl, BYTE PTR [rbx]
66 67 40 12 23              data16 adc spl, BYTE PTR [ebx]
41 12 23                    adc    spl, BYTE PTR [r11]
67 41 12 23                 adc    spl, BYTE PTR [r11d]
66 41 12 23                 data16 adc spl, BYTE PTR [r11]
66 67 41 12 23              data16 adc spl, BYTE PTR [r11d]
48 12 23                    rex.W adc spl, BYTE PTR [rbx]
67 48 12 23                 rex.W adc spl, BYTE PTR [ebx]
66 48 12 23                 data16 rex.W adc spl, BYTE PTR [rbx]
66 67 48 12 23              data16 rex.W adc spl, BYTE PTR [ebx]
13 23                       adc    esp, DWORD PTR [rbx]
66 13 23                    adc    sp, WORD PTR [rbx]
67 13 23                    adc    esp, DWORD PTR [ebx]
66 67 13 23                 adc    sp, WORD PTR [ebx]
40 13 23                    rex adc esp, DWORD PTR [rbx]
67 40 13 23                 rex adc esp, DWORD PTR [ebx]
66 40 13 23                 rex adc sp, WORD PTR [rbx]
66 67 40 13 23              rex adc sp, WORD PTR [ebx]
41 13 23                    adc    esp, DWORD PTR [r11]
67 41 13 23                 adc    esp, DWORD PTR [r11d]
66 41 13 23                 adc    sp, WORD PTR [r11]
66 67 41 13 23              adc    sp, WORD PTR [r11d]
48 13 23                    adc    rsp, QWORD PTR [rbx]
67 48 13 23                 adc    rsp, QWORD PTR [ebx]
1c 12                       sbb    al, 0x12
66 1d 12 34                 sbb    ax, 0x3412
1d 12 34 56 78              sbb    eax, 0x78563412
41 1d 12 34 56 78           rex.B sbb eax, 0x78563412
41 1d 12 34 56 88           rex.B sbb eax, 0x88563412
48 1d 12 34 56 78           sbb    rax, 0x78563412
48 1d 12 34 56 88           sbb    rax, 0xffffffff88563412
49 1d 12 34 56 78           rex.WB sbb rax, 0x78563412
49 1d 12 34 56 88           rex.WB sbb rax, 0xffffffff88563412
80 d8 12                    sbb    al, 0x12
80 de 12                    sbb    dh, 0x12
40 80 de 12                 sbb    sil, 0x12
66 81 de 12 34              sbb    si, 0x3412
81 de 12 34 56 78           sbb    esi, 0x78563412
48 81 de 12 34 56 78        sbb    rsi, 0x78563412
48 81 de 12 34 56 88        sbb    rsi, 0xffffffff88563412
66 83 de 12                 sbb    si, 0x12
66 83 de 82                 sbb    si, 0xff82
83 de 12                    sbb    esi, 0x12
83 de 82                    sbb    esi, 0xffffff82
41 83 de 12                 sbb    r14d, 0x12
41 83 de 82                 sbb    r14d, 0xffffff82
48 83 de 12                 sbb    rsi, 0x12
48 83 de 82                 sbb    rsi, 0xffffffffffffff82
49 83 de 12                 sbb    r14, 0x12
49 83 de 82                 sbb    r14, 0xffffffffffffff82
18 23                       sbb    BYTE PTR [rbx], ah
66 18 23                    data16 sbb BYTE PTR [rbx], ah
67 18 23                    sbb    BYTE PTR [ebx], ah
66 67 18 23                 data16 sbb BYTE PTR [ebx], ah
40 18 23                    sbb    BYTE PTR [rbx], spl
67 40 18 23                 sbb    BYTE PTR [ebx], spl
66 40 18 23                 data16 sbb BYTE PTR [rbx], spl
66 67 40 18 23              data16 sbb BYTE PTR [ebx], spl
41 18 23                    sbb    BYTE PTR [r11], spl
67 41 18 23                 sbb    BYTE PTR [r11d], spl
66 41 18 23                 data16 sbb BYTE PTR [r11], spl
66 67 41 18 23              data16 sbb BYTE PTR [r11d], spl
48 18 23                    rex.W sbb BYTE PTR [rbx], spl
67 48 18 23                 rex.W sbb BYTE PTR [ebx], spl
66 48 18 23                 data16 rex.W sbb BYTE PTR [rbx], spl
66 67 48 18 23              data16 rex.W sbb BYTE PTR [ebx], spl
19 23                       sbb    DWORD PTR [rbx], esp
66 19 23                    sbb    WORD PTR [rbx], sp
67 19 23                    sbb    DWORD PTR [ebx], esp
66 67 19 23                 sbb    WORD PTR [ebx], sp
40 19 23                    rex sbb DWORD PTR [rbx], esp
67 40 19 23                 rex sbb DWORD PTR [ebx], esp
66 40 19 23                 rex sbb WORD PTR [rbx], sp
66 67 40 19 23              rex sbb WORD PTR [ebx], sp
41 19 23                    sbb    DWORD PTR [r11], esp
67 41 19 23                 sbb    DWORD PTR [r11d], esp
66 41 19 23                 sbb    WORD PTR [r11], sp
66 67 41 19 23              sbb    WORD PTR [r11d], sp
48 19 23                    sbb    QWORD PTR [rbx], rsp
67 48 19 23                 sbb    QWORD PTR [ebx], rsp
66 48 19 23                 data16 sbb QWORD PTR [rbx], rsp
66 67 48 19 23              data16 sbb QWORD PTR [ebx], rsp
1a 23                       sbb    ah, BYTE PTR [rbx]
66 1a 23                    data16 sbb ah, BYTE PTR [rbx]
67 1a 23                    sbb    ah, BYTE PTR [ebx]
66 67 1a 23                 data16 sbb ah, BYTE PTR [ebx]
40 1a 23                    sbb    spl, BYTE PTR [rbx]
67 40 1a 23                 sbb    spl, BYTE PTR [ebx]
66 40 1a 23                 data16 sbb spl, BYTE PTR [rbx]
66 67 40 1a 23              data16 sbb spl, BYTE PTR [ebx]
41 1a 23                    sbb    spl, BYTE PTR [r11]
67 41 1a 23                 sbb    spl, BYTE PTR [r11d]
66 41 1a 23                 data16 sbb spl, BYTE PTR [r11]
66 67 41 1a 23              data16 sbb spl, BYTE PTR [r11d]
48 1a 23                    rex.W sbb spl, BYTE PTR [rbx]
67 48 1a 23                 rex.W sbb spl, BYTE PTR [ebx]
66 48 1a 23                 data16 rex.W sbb spl, BYTE PTR [rbx]
66 67 48 1a 23              data16 rex.W sbb spl, BYTE PTR [ebx]
1b 23                       sbb    esp, DWORD PTR [rbx]
66 1b 23                    sbb    sp, WORD PTR [rbx]
67 1b 23                    sbb    esp, DWORD PTR [ebx]
66 67 1b 23                 sbb    sp, WORD PTR [ebx]
40 1b 23                    rex sbb esp, DWORD PTR [rbx]
67 40 1b 23                 rex sbb esp, DWORD PTR [ebx]
66 40 1b 23                 rex sbb sp, WORD PTR [rbx]
66 67 40 1b 23              rex sbb sp, WORD PTR [ebx]
41 1b 23                    sbb    esp, DWORD PTR [r11]
67 41 1b 23                 sbb    esp, DWORD PTR [r11d]
66 41 1b 23                 sbb    sp, WORD PTR [r11]
66 67 41 1b 23              sbb    sp, WORD PTR [r11d]
48 1b 23                    sbb    rsp, QWORD PTR [rbx]
67 48 1b 23                 sbb    rsp, QWORD PTR [ebx]
24 12                       and    al, 0x12
66 25 12 34                 and    ax, 0x3412
25 12 34 56 78              and    eax, 0x78563412
41 25 12 34 56 78           rex.B and eax, 0x78563412
41 25 12 34 56 88           rex.B and eax, 0x88563412
48 25 12 34 56 78           and    rax, 0x78563412
48 25 12 34 56 88           and    rax, 0xffffffff88563412
49 25 12 34 56 78           rex.WB and rax, 0x78563412
49 25 12 34 56 88           rex.WB and rax, 0xffffffff88563412
80 e0 12                    and    al, 0x12
80 e6 12                    and    dh, 0x12
40 80 e6 12                 and    sil, 0x12
66 81 e6 12 34              and    si, 0x3412
81 e6 12 34 56 78           and    esi, 0x78563412
48 81 e6 12 34 56 78        and    rsi, 0x78563412
48 81 e6 12 34 56 88        and    rsi, 0xffffffff88563412
66 83 e6 12                 and    si, 0x12
66 83 e6 82                 and    si, 0xff82
83 e6 12                    and    esi, 0x12
83 e6 82                    and    esi, 0xffffff82
41 83 e6 12                 and    r14d, 0x12
41 83 e6 82                 and    r14d, 0xffffff82
48 83 e6 12                 and    rsi, 0x12
48 83 e6 82                 and    rsi, 0xffffffffffffff82
49 83 e6 12                 and    r14, 0x12
49 83 e6 82                 and    r14, 0xffffffffffffff82
20 23                       and    BYTE PTR [rbx], ah
66 20 23                    data16 and BYTE PTR [rbx], ah
67 20 23                    and    BYTE PTR [ebx], ah
66 67 20 23                 data16 and BYTE PTR [ebx], ah
40 20 23                    and    BYTE PTR [rbx], spl
67 40 20 23                 and    BYTE PTR [ebx], spl
66 40 20 23                 data16 and BYTE PTR [rbx], spl
66 67 40 20 23              data16 and BYTE PTR [ebx], spl
41 20 23                    and    BYTE PTR [r11], spl
67 41 20 23                 and    BYTE PTR [r11d], spl
66 41 20 23                 data16 and BYTE PTR [r11], spl
66 67 41 20 23              data16 and BYTE PTR [r11d], spl
48 20 23                    rex.W and BYTE PTR [rbx], spl
67 48 20 23                 rex.W and BYTE PTR [ebx], spl
66 48 20 23                 data16 rex.W and BYTE PTR [rbx], spl
66 67 48 20 23              data16 rex.W and BYTE PTR [ebx], spl
21 23                       and    DWORD PTR [rbx], esp
66 21 23                    and    WORD PTR [rbx], sp
67 21 23                    and    DWORD PTR [ebx], esp
66 67 21 23                 and    WORD PTR [ebx], sp
40 21 23                    rex and DWORD PTR [rbx], esp
67 40 21 23                 rex and DWORD PTR [ebx], esp
66 40 21 23                 rex and WORD PTR [rbx], sp
66 67 40 21 23              rex and WORD PTR [ebx], sp
41 21 23                    and    DWORD PTR [r11], esp
67 41 21 23                 and    DWORD PTR [r11d], esp
66 41 21 23                 and    WORD PTR [r11], sp
66 67 41 21 23              and    WORD PTR [r11d], sp
48 21 23                    and    QWORD PTR [rbx], rsp
67 48 21 23                 and    QWORD PTR [ebx], rsp
66 48 21 23                 data16 and QWORD PTR [rbx], rsp
66 67 48 21 23              data16 and QWORD PTR [ebx], rsp
22 23                       and    ah, BYTE PTR [rbx]
66 22 23                    data16 and ah, BYTE PTR [rbx]
67 22 23                    and    ah, BYTE PTR [ebx]
66 67 22 23                 data16 and ah, BYTE PTR [ebx]
40 22 23                    and    spl, BYTE PTR [rbx]
67 40 22 23                 and    spl, BYTE PTR [ebx]
66 40 22 23                 data16 and spl, BYTE PTR [rbx]
66 67 40 22 23              data16 and spl, BYTE PTR [ebx]
41 22 23                    and    spl, BYTE PTR [r11]
67 41 22 23                 and    spl, BYTE PTR [r11d]
66 41 22 23                 data16 and spl, BYTE PTR [r11]
66 67 41 22 23              data16 and spl, BYTE PTR [r11d]
48 22 23                    rex.W and spl, BYTE PTR [rbx]
67 48 22 23                 rex.W and spl, BYTE PTR [ebx]
66 48 22 23                 data16 rex.W and spl, BYTE PTR [rbx]
66 67 48 22 23              data16 rex.W and spl, BYTE PTR [ebx]
23 23                       and    esp, DWORD PTR [rbx]
66 23 23                    and    sp, WORD PTR [rbx]
67 23 23                    and    esp, DWORD PTR [ebx]
66 67 23 23                 and    sp, WORD PTR [ebx]
40 23 23                    rex and esp, DWORD PTR [rbx]
67 40 23 23                 rex and esp, DWORD PTR [ebx]
66 40 23 23                 rex and sp, WORD PTR [rbx]
66 67 40 23 23              rex and sp, WORD PTR [ebx]
41 23 23                    and    esp, DWORD PTR [r11]
67 41 23 23                 and    esp, DWORD PTR [r11d]
66 41 23 23                 and    sp, WORD PTR [r11]
66 67 41 23 23              and    sp, WORD PTR [r11d]
48 23 23                    and    rsp, QWORD PTR [rbx]
67 48 23 23                 and    rsp, QWORD PTR [ebx]
66 48 23 23                 data16 and rsp, QWORD PTR [rbx]
66 67 48 23 23              data16 and rsp, QWORD PTR [ebx]
2c 12                       sub    al, 0x12
66 2d 12 34                 sub    ax, 0x3412
2d 12 34 56 78              sub    eax, 0x78563412
41 2d 12 34 56 78           rex.B sub eax, 0x78563412
41 2d 12 34 56 88           rex.B sub eax, 0x88563412
48 2d 12 34 56 78           sub    rax, 0x78563412
48 2d 12 34 56 88           sub    rax, 0xffffffff88563412
49 2d 12 34 56 78           rex.WB sub rax, 0x78563412
49 2d 12 34 56 88           rex.WB sub rax, 0xffffffff88563412
80 e8 12                    sub    al, 0x12
80 ee 12                    sub    dh, 0x12
40 80 ee 12                 sub    sil, 0x12
66 81 ee 12 34              sub    si, 0x3412
81 ee 12 34 56 78           sub    esi, 0x78563412
48 81 ee 12 34 56 78        sub    rsi, 0x78563412
48 81 ee 12 34 56 88        sub    rsi, 0xffffffff88563412
66 83 ee 12                 sub    si, 0x12
66 83 ee 82                 sub    si, 0xff82
83 ee 12                    sub    esi, 0x12
83 ee 82                    sub    esi, 0xffffff82
41 83 ee 12                 sub    r14d, 0x12
41 83 ee 82                 sub    r14d, 0xffffff82
48 83 ee 12                 sub    rsi, 0x12
48 83 ee 82                 sub    rsi, 0xffffffffffffff82
49 83 ee 12                 sub    r14, 0x12
49 83 ee 82                 sub    r14, 0xffffffffffffff82
28 23                       sub    BYTE PTR [rbx], ah
66 28 23                    data16 sub BYTE PTR [rbx], ah
67 28 23                    sub    BYTE PTR [ebx], ah
66 67 28 23                 data16 sub BYTE PTR [ebx], ah
40 28 23                    sub    BYTE PTR [rbx], spl
67 40 28 23                 sub    BYTE PTR [ebx], spl
66 40 28 23                 data16 sub BYTE PTR [rbx], spl
66 67 40 28 23              data16 sub BYTE PTR [ebx], spl
41 28 23                    sub    BYTE PTR [r11], spl
67 41 28 23                 sub    BYTE PTR [r11d], spl
66 41 28 23                 data16 sub BYTE PTR [r11], spl
66 67 41 28 23              data16 sub BYTE PTR [r11d], spl
48 28 23                    rex.W sub BYTE PTR [rbx], spl
67 48 28 23                 rex.W sub BYTE PTR [ebx], spl
66 48 28 23                 data16 rex.W sub BYTE PTR [rbx], spl
66 67 48 28 23              data16 rex.W sub BYTE PTR [ebx], spl
29 23                       sub    DWORD PTR [rbx], esp
66 29 23                    sub    WORD PTR [rbx], sp
67 29 23                    sub    DWORD PTR [ebx], esp
66 67 29 23                 sub    WORD PTR [ebx], sp
40 29 23                    rex sub DWORD PTR [rbx], esp
67 40 29 23                 rex sub DWORD PTR [ebx], esp
66 40 29 23                 rex sub WORD PTR [rbx], sp
66 67 40 29 23              rex sub WORD PTR [ebx], sp
41 29 23                    sub    DWORD PTR [r11], esp
67 41 29 23                 sub    DWORD PTR [r11d], esp
66 41 29 23                 sub    WORD PTR [r11], sp
66 67 41 29 23              sub    WORD PTR [r11d], sp
48 29 23                    sub    QWORD PTR [rbx], rsp
67 48 29 23                 sub    QWORD PTR [ebx], rsp
66 48 29 23                 data16 sub QWORD PTR [rbx], rsp
66 67 48 29 23              data16 sub QWORD PTR [ebx], rsp
2a 23                       sub    ah, BYTE PTR [rbx]
66 2a 23                    data16 sub ah, BYTE PTR [rbx]
67 2a 23                    sub    ah, BYTE PTR [ebx]
66 67 2a 23                 data16 sub ah, BYTE PTR [ebx]
40 2a 23                    sub    spl, BYTE PTR [rbx]
67 40 2a 23                 sub    spl, BYTE PTR [ebx]
66 40 2a 23                 data16 sub spl, BYTE PTR [rbx]
66 67 40 2a 23              data16 sub spl, BYTE PTR [ebx]
41 2a 23                    sub    spl, BYTE PTR [r11]
67 41 2a 23                 sub    spl, BYTE PTR [r11d]
66 41 2a 23                 data16 sub spl, BYTE PTR [r11]
66 67 41 2a 23              data16 sub spl, BYTE PTR [r11d]
48 2a 23                    rex.W sub spl, BYTE PTR [rbx]
67 48 2a 23                 rex.W sub spl, BYTE PTR [ebx]
66 48 2a 23                 data16 rex.W sub spl, BYTE PTR [rbx]
66 67 48 2a 23              data16 rex.W sub spl, BYTE PTR [ebx]
2b 23                       sub    esp, DWORD PTR [rbx]
66 2b 23                    sub    sp, WORD PTR [rbx]
67 2b 23                    sub    esp, DWORD PTR [ebx]
66 67 2b 23                 sub    sp, WORD PTR [ebx]
40 2b 23                    rex sub esp, DWORD PTR [rbx]
67 40 2b 23                 rex sub esp, DWORD PTR [ebx]
66 40 2b 23                 rex sub sp, WORD PTR [rbx]
66 67 40 2b 23              rex sub sp, WORD PTR [ebx]
41 2b 23                    sub    esp, DWORD PTR [r11]
67 41 2b 23                 sub    esp, DWORD PTR [r11d]
66 41 2b 23                 sub    sp, WORD PTR [r11]
66 67 41 2b 23              sub    sp, WORD PTR [r11d]
48 2b 23                    sub    rsp, QWORD PTR [rbx]
67 48 2b 23                 sub    rsp, QWORD PTR [ebx]
66 48 2b 23                 data16 sub rsp, QWORD PTR [rbx]
66 67 48 2b 23              data16 sub rsp, QWORD PTR [ebx]
3c 12                       cmp    al, 0x12
66 3d 12 34                 cmp    ax, 0x3412
3d 12 34 56 78              cmp    eax, 0x78563412
41 3d 12 34 56 78           rex.B cmp eax, 0x78563412
41 3d 12 34 56 88           rex.B cmp eax, 0x88563412
48 3d 12 34 56 78           cmp    rax, 0x78563412
48 3d 12 34 56 88           cmp    rax, 0xffffffff88563412
49 3d 12 34 56 78           rex.WB cmp rax, 0x78563412
49 3d 12 34 56 88           rex.WB cmp rax, 0xffffffff88563412
80 f8 12                    cmp    al, 0x12
80 fe 12                    cmp    dh, 0x12
40 80 fe 12                 cmp    sil, 0x12
66 81 fe 12 34              cmp    si, 0x3412
81 fe 12 34 56 78           cmp    esi, 0x78563412
48 81 fe 12 34 56 78        cmp    rsi, 0x78563412
48 81 fe 12 34 56 88        cmp    rsi, 0xffffffff88563412
66 83 fe 12                 cmp    si, 0x12
66 83 fe 82                 cmp    si, 0xff82
83 fe 12                    cmp    esi, 0x12
83 fe 82                    cmp    esi, 0xffffff82
41 83 fe 12                 cmp    r14d, 0x12
41 83 fe 82                 cmp    r14d, 0xffffff82
48 83 fe 12                 cmp    rsi, 0x12
48 83 fe 82                 cmp    rsi, 0xffffffffffffff82
49 83 fe 12                 cmp    r14, 0x12
49 83 fe 82                 cmp    r14, 0xffffffffffffff82
38 23                       cmp    BYTE PTR [rbx], ah
66 38 23                    data16 cmp BYTE PTR [rbx], ah
67 38 23                    cmp    BYTE PTR [ebx], ah
66 67 38 23                 data16 cmp BYTE PTR [ebx], ah
40 38 23                    cmp    BYTE PTR [rbx], spl
67 40 38 23                 cmp    BYTE PTR [ebx], spl
66 40 38 23                 data16 cmp BYTE PTR [rbx], spl
66 67 40 38 23              data16 cmp BYTE PTR [ebx], spl
41 38 23                    cmp    BYTE PTR [r11], spl
67 41 38 23                 cmp    BYTE PTR [r11d], spl
66 41 38 23                 data16 cmp BYTE PTR [r11], spl
66 67 41 38 23              data16 cmp BYTE PTR [r11d], spl
48 38 23                    rex.W cmp BYTE PTR [rbx], spl
67 48 38 23                 rex.W cmp BYTE PTR [ebx], spl
66 48 38 23                 data16 rex.W cmp BYTE PTR [rbx], spl
66 67 48 38 23              data16 rex.W cmp BYTE PTR [ebx], spl
39 23                       cmp    DWORD PTR [rbx], esp
66 39 23                    cmp    WORD PTR [rbx], sp
67 39 23                    cmp    DWORD PTR [ebx], esp
66 67 39 23                 cmp    WORD PTR [ebx], sp
40 39 23                    rex cmp DWORD PTR [rbx], esp
67 40 39 23                 rex cmp DWORD PTR [ebx], esp
66 40 39 23                 rex cmp WORD PTR [rbx], sp
66 67 40 39 23              rex cmp WORD PTR [ebx], sp
41 39 23                    cmp    DWORD PTR [r11], esp
67 41 39 23                 cmp    DWORD PTR [r11d], esp
66 41 39 23                 cmp    WORD PTR [r11], sp
66 67 41 39 23              cmp    WORD PTR [r11d], sp
48 39 23                    cmp    QWORD PTR [rbx], rsp
67 48 39 23                 cmp    QWORD PTR [ebx], rsp
66 48 39 23                 data16 cmp QWORD PTR [rbx], rsp
66 67 48 39 23              data16 cmp QWORD PTR [ebx], rsp
3a 23                       cmp    ah, BYTE PTR [rbx]
66 3a 23                    data16 cmp ah, BYTE PTR [rbx]
67 3a 23                    cmp    ah, BYTE PTR [ebx]
66 67 3a 23                 data16 cmp ah, BYTE PTR [ebx]
40 3a 23                    cmp    spl, BYTE PTR [rbx]
67 40 3a 23                 cmp    spl, BYTE PTR [ebx]
66 40 3a 23                 data16 cmp spl, BYTE PTR [rbx]
66 67 40 3a 23              data16 cmp spl, BYTE PTR [ebx]
41 3a 23                    cmp    spl, BYTE PTR [r11]
67 41 3a 23                 cmp    spl, BYTE PTR [r11d]
66 41 3a 23                 data16 cmp spl, BYTE PTR [r11]
66 67 41 3a 23              data16 cmp spl, BYTE PTR [r11d]
48 3a 23                    rex.W cmp spl, BYTE PTR [rbx]
67 48 3a 23                 rex.W cmp spl, BYTE PTR [ebx]
66 48 3a 23                 data16 rex.W cmp spl, BYTE PTR [rbx]
66 67 48 3a 23              data16 rex.W cmp spl, BYTE PTR [ebx]
3b 23                       cmp    esp, DWORD PTR [rbx]
66 3b 23                    cmp    sp, WORD PTR [rbx]
67 3b 23                    cmp    esp, DWORD PTR [ebx]
66 67 3b 23                 cmp    sp, WORD PTR [ebx]
40 3b 23                    rex cmp esp, DWORD PTR [rbx]
67 40 3b 23                 rex cmp esp, DWORD PTR [ebx]
66 40 3b 23                 rex cmp sp, WORD PTR [rbx]
66 67 40 3b 23              rex cmp sp, WORD PTR [ebx]
41 3b 23                    cmp    esp, DWORD PTR [r11]
67 41 3b 23                 cmp    esp, DWORD PTR [r11d]
66 41 3b 23                 cmp    sp, WORD PTR [r11]
66 67 41 3b 23              cmp    sp, WORD PTR [r11d]
48 3b 23                    cmp    rsp, QWORD PTR [rbx]
67 48 3b 23                 cmp    rsp, QWORD PTR [ebx]
66 48 3b 23                 data16 cmp rsp, QWORD PTR [rbx]
66 67 48 3b 23              data16 cmp rsp, QWORD PTR [ebx]
34 12                       xor    al, 0x12
66 35 12 34                 xor    ax, 0x3412
35 12 34 56 78              xor    eax, 0x78563412
41 35 12 34 56 78           rex.B xor eax, 0x78563412
41 35 12 34 56 88           rex.B xor eax, 0x88563412
48 35 12 34 56 78           xor    rax, 0x78563412
48 35 12 34 56 88           xor    rax, 0xffffffff88563412
49 35 12 34 56 78           rex.WB xor rax, 0x78563412
49 35 12 34 56 88           rex.WB xor rax, 0xffffffff88563412
80 f0 12                    xor    al, 0x12
80 f6 12                    xor    dh, 0x12
40 80 f6 12                 xor    sil, 0x12
66 81 f6 12 34              xor    si, 0x3412
81 f6 12 34 56 78           xor    esi, 0x78563412
48 81 f6 12 34 56 78        xor    rsi, 0x78563412
48 81 f6 12 34 56 88        xor    rsi, 0xffffffff88563412
66 83 f6 12                 xor    si, 0x12
66 83 f6 82                 xor    si, 0xff82
83 f6 12                    xor    esi, 0x12
83 f6 82                    xor    esi, 0xffffff82
41 83 f6 12                 xor    r14d, 0x12
41 83 f6 82                 xor    r14d, 0xffffff82
48 83 f6 12                 xor    rsi, 0x12
48 83 f6 82                 xor    rsi, 0xffffffffffffff82
49 83 f6 12                 xor    r14, 0x12
49 83 f6 82                 xor    r14, 0xffffffffffffff82
30 23                       xor    BYTE PTR [rbx], ah
66 30 23                    data16 xor BYTE PTR [rbx], ah
67 30 23                    xor    BYTE PTR [ebx], ah
66 67 30 23                 data16 xor BYTE PTR [ebx], ah
40 30 23                    xor    BYTE PTR [rbx], spl
67 40 30 23                 xor    BYTE PTR [ebx], spl
66 40 30 23                 data16 xor BYTE PTR [rbx], spl
66 67 40 30 23              data16 xor BYTE PTR [ebx], spl
41 30 23                    xor    BYTE PTR [r11], spl
67 41 30 23                 xor    BYTE PTR [r11d], spl
66 41 30 23                 data16 xor BYTE PTR [r11], spl
66 67 41 30 23              data16 xor BYTE PTR [r11d], spl
48 30 23                    rex.W xor BYTE PTR [rbx], spl
67 48 30 23                 rex.W xor BYTE PTR [ebx], spl
66 48 30 23                 data16 rex.W xor BYTE PTR [rbx], spl
66 67 48 30 23              data16 rex.W xor BYTE PTR [ebx], spl
31 23                       xor    DWORD PTR [rbx], esp
66 31 23                    xor    WORD PTR [rbx], sp
67 31 23                    xor    DWORD PTR [ebx], esp
66 67 31 23                 xor    WORD PTR [ebx], sp
40 31 23                    rex xor DWORD PTR [rbx], esp
67 40 31 23                 rex xor DWORD PTR [ebx], esp
66 40 31 23                 rex xor WORD PTR [rbx], sp
66 67 40 31 23              rex xor WORD PTR [ebx], sp
41 31 23                    xor    DWORD PTR [r11], esp
67 41 31 23                 xor    DWORD PTR [r11d], esp
66 41 31 23                 xor    WORD PTR [r11], sp
66 67 41 31 23              xor    WORD PTR [r11d], sp
48 31 23                    xor    QWORD PTR [rbx], rsp
67 48 31 23                 xor    QWORD PTR [ebx], rsp
66 48 31 23                 data16 xor QWORD PTR [rbx], rsp
66 67 48 31 23              data16 xor QWORD PTR [ebx], rsp
32 23                       xor    ah, BYTE PTR [rbx]
66 32 23                    data16 xor ah, BYTE PTR [rbx]
67 32 23                    xor    ah, BYTE PTR [ebx]
66 67 32 23                 data16 xor ah, BYTE PTR [ebx]
40 32 23                    xor    spl, BYTE PTR [rbx]
67 40 32 23                 xor    spl, BYTE PTR [ebx]
66 40 32 23                 data16 xor spl, BYTE PTR [rbx]
66 67 40 32 23              data16 xor spl, BYTE PTR [ebx]
41 32 23                    xor    spl, BYTE PTR [r11]
67 41 32 23                 xor    spl, BYTE PTR [r11d]
66 41 32 23                 data16 xor spl, BYTE PTR [r11]
66 67 41 32 23              data16 xor spl, BYTE PTR [r11d]
48 32 23                    rex.W xor spl, BYTE PTR [rbx]
67 48 32 23                 rex.W xor spl, BYTE PTR [ebx]
66 48 32 23                 data16 rex.W xor spl, BYTE PTR [rbx]
66 67 48 32 23              data16 rex.W xor spl, BYTE PTR [ebx]
33 23                       xor    esp, DWORD PTR [rbx]
66 33 23                    xor    sp, WORD PTR [rbx]
67 33 23                    xor    esp, DWORD PTR [ebx]
66 67 33 23                 xor    sp, WORD PTR [ebx]
40 33 23                    rex xor esp, DWORD PTR [rbx]
67 40 33 23                 rex xor esp, DWORD PTR [ebx]
66 40 33 23                 rex xor sp, WORD PTR [rbx]
66 67 40 33 23              rex xor sp, WORD PTR [ebx]
41 33 23                    xor    esp, DWORD PTR [r11]
67 41 33 23                 xor    esp, DWORD PTR [r11d]
66 41 33 23                 xor    sp, WORD PTR [r11]
66 67 41 33 23              xor    sp, WORD PTR [r11d]
48 33 23                    xor    rsp, QWORD PTR [rbx]
67 48 33 23                 xor    rsp, QWORD PTR [ebx]
66 48 33 23                 data16 xor rsp, QWORD PTR [rbx]
66 67 48 33 23              data16 xor rsp, QWORD PTR [ebx]
66 0f ba e6 12              bt     si, 0x12
66 0f ba e6 82              bt     si, 0x82
0f ba e6 12                 bt     esi, 0x12
0f ba e6 82                 bt     esi, 0x82
41 0f ba e6 12              bt     r14d, 0x12
41 0f ba e6 82              bt     r14d, 0x82
48 0f ba e6 12              bt     rsi, 0x12
48 0f ba e6 82              bt     rsi, 0x82
49 0f ba e6 12              bt     r14, 0x12
49 0f ba e6 82              bt     r14, 0x82
0f ba 23 12                 bt     DWORD PTR [rbx], 0x12
0f ba 23 82                 bt     DWORD PTR [rbx], 0x82
66 0f ba 23 12              bt     WORD PTR [rbx], 0x12
66 0f ba 23 82              bt     WORD PTR [rbx], 0x82
67 0f ba 23 12              bt     DWORD PTR [ebx], 0x12
67 0f ba 23 82              bt     DWORD PTR [ebx], 0x82
66 67 0f ba 23 12           bt     WORD PTR [ebx], 0x12
66 67 0f ba 23 82           bt     WORD PTR [ebx], 0x82
49 0f ba 23 12              bt     QWORD PTR [r11], 0x12
49 0f ba 23 82              bt     QWORD PTR [r11], 0x82
66 49 0f ba 23 12           data16 bt QWORD PTR [r11], 0x12
66 49 0f ba 23 82           data16 bt QWORD PTR [r11], 0x82
67 49 0f ba 23 12           bt     QWORD PTR [r11d], 0x12
67 49 0f ba 23 82           bt     QWORD PTR [r11d], 0x82
66 67 49 0f ba 23 12        data16 bt QWORD PTR [r11d], 0x12
66 67 49 0f ba 23 82        data16 bt QWORD PTR [r11d], 0x82
0f a3 e6                    bt     esi, esp
41 0f a3 e6                 bt     r14d, esp
49 0f a3 e6                 bt     r14, rsp
66 0f a3 e6                 bt     si, sp
0f a3 23                    bt     DWORD PTR [rbx], esp
66 0f a3 23                 bt     WORD PTR [rbx], sp
67 0f a3 23                 bt     DWORD PTR [ebx], esp
66 67 0f a3 23              bt     WORD PTR [ebx], sp
40 0f a3 23                 rex bt DWORD PTR [rbx], esp
67 40 0f a3 23              rex bt DWORD PTR [ebx], esp
66 40 0f a3 23              rex bt WORD PTR [rbx], sp
66 67 40 0f a3 23           rex bt WORD PTR [ebx], sp
41 0f a3 23                 bt     DWORD PTR [r11], esp
67 41 0f a3 23              bt     DWORD PTR [r11d], esp
66 41 0f a3 23              bt     WORD PTR [r11], sp
66 67 41 0f a3 23           bt     WORD PTR [r11d], sp
48 0f a3 23                 bt     QWORD PTR [rbx], rsp
67 48 0f a3 23              bt     QWORD PTR [ebx], rsp
66 48 0f a3 23              data16 bt QWORD PTR [rbx], rsp
66 67 48 0f a3 23           data16 bt QWORD PTR [ebx], rsp
