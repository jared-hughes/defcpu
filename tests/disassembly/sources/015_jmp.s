eb 82                       jmp    0x3fff84
eb 12                       jmp    0x400016
e8 12 34 56 88              call   0xffffffff8896341b
e9 12 34 56 88              jmp    0xffffffff88963420
e8 12 34 56 78              call   0x78963425
e9 12 34 56 78              jmp    0x7896342a
66 eb 82                    data16 jmp 0x3fff9d
66 eb 12                    data16 jmp 0x400030
67 eb 82                    addr32 jmp 0x3fffa3
67 eb 12                    addr32 jmp 0x400036
66 67 eb 82                 data16 addr32 jmp 0x3fffaa
66 67 eb 12                 data16 addr32 jmp 0x40003e
41 eb 82                    rex.B jmp 0x3fffb1
48 eb 82                    rex.W jmp 0x3fffb4
49 eb 82                    rex.WB jmp 0x3fffb7
41 e8 12 34 56 78           rex.B call 0x7896344d
41 e9 12 34 56 78           rex.B jmp 0x78963453
48 e8 12 34 56 78           rex.W call 0x78963459
48 e9 12 34 56 78           rex.W jmp 0x7896345f
49 e8 12 34 56 78           rex.WB call 0x78963465
49 e9 12 34 56 78           rex.WB jmp 0x7896346b
ff 20                       jmp    *(%rax)
ff 24 18                    jmp    *(%rax, %rbx, 1)
ff 24 58                    jmp    *(%rax, %rbx, 2)
ff 24 98                    jmp    *(%rax, %rbx, 4)
ff 24 d8                    jmp    *(%rax, %rbx, 8)
67 ff 24 18                 jmp    *(%eax, %ebx, 1)
67 ff 24 58                 jmp    *(%eax, %ebx, 2)
67 ff 24 98                 jmp    *(%eax, %ebx, 4)
67 ff 24 d8                 jmp    *(%eax, %ebx, 8)
43 ff 24 23                 jmp    *(%r11, %r12, 1)
43 ff 24 63                 jmp    *(%r11, %r12, 2)
43 ff 24 a3                 jmp    *(%r11, %r12, 4)
43 ff 24 e3                 jmp    *(%r11, %r12, 8)
67 43 ff 24 23              jmp    *(%r11d, %r12d, 1)
67 43 ff 24 63              jmp    *(%r11d, %r12d, 2)
67 43 ff 24 a3              jmp    *(%r11d, %r12d, 4)
67 43 ff 24 e3              jmp    *(%r11d, %r12d, 8)
ff 64 18 79                 jmp    *0x79(%rax, %rbx, 1)
ff 64 58 80                 jmp    *-0x80(%rax, %rbx, 2)
ff a4 98 80 00 00 00        jmp    *0x80(%rax, %rbx, 4)
ff a4 d8 7f ff ff ff        jmp    *-0x81(%rax, %rbx, 8)
ff 24 1c                    jmp    *(%rsp, %rbx, 1)
ff 25 12 34 56 78           jmp    *0x78563412(%rip)
67 ff 25 12 34 56 78        jmp    *0x78563412(%eip)
ff 24 20                    jmp    *(%rax, %riz, 1)
ff a4 20 12 34 56 78        jmp    *0x78563412(%rax, %riz, 1)
ff 24 20                    jmp    *(%rax, %riz, 1)
ff 24 21                    jmp    *(%rcx, %riz, 1)
ff 24 22                    jmp    *(%rdx, %riz, 1)
ff 24 23                    jmp    *(%rbx, %riz, 1)
ff 24 24                    jmp    *(%rsp)
ff 24 25 12 34 56 78        jmp    *0x78563412
ff 24 65 12 34 56 78        jmp    *0x78563412(, %riz, 2)
ff 24 a5 12 34 56 78        jmp    *0x78563412(, %riz, 4)
ff 24 e5 12 34 56 78        jmp    *0x78563412(, %riz, 8)
ff 24 26                    jmp    *(%rsi, %riz, 1)
ff 24 27                    jmp    *(%rdi, %riz, 1)
ff 24 64                    jmp    *(%rsp, %riz, 2)
ff 24 a4                    jmp    *(%rsp, %riz, 4)
ff 24 e4                    jmp    *(%rsp, %riz, 8)
ff 24 ac                    jmp    *(%rsp, %rbp, 4)
ff 64 20 3a                 jmp    *0x3a(%rax, %riz, 1)
ff 64 64 3a                 jmp    *0x3a(%rsp, %riz, 2)
ff 64 a4 3a                 jmp    *0x3a(%rsp, %riz, 4)
ff 64 e4 3a                 jmp    *0x3a(%rsp, %riz, 8)
ff 64 25 3a                 jmp    *0x3a(%rbp, %riz, 1)
ff 64 65 3a                 jmp    *0x3a(%rbp, %riz, 2)
ff 64 a5 3a                 jmp    *0x3a(%rbp, %riz, 4)
ff 64 e5 3a                 jmp    *0x3a(%rbp, %riz, 8)
c3                          ret
