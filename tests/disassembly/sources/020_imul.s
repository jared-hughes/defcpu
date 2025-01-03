f6 28                       imulb  (%rax)
f6 e8                       imul   %al
66 f7 28                    imulw  (%rax)
66 f7 e8                    imul   %ax
f7 28                       imull  (%rax)
f7 e8                       imul   %eax
48 f7 28                    imulq  (%rax)
48 f7 e8                    imul   %rax
6b 23 12                    imul   $0x12, (%rbx), %esp
66 0f af 23                 imul   (%rbx), %sp
67 0f af 23                 imul   (%ebx), %esp
66 67 0f af 23              imul   (%ebx), %sp
40 0f af 23                 rex imul (%rbx), %esp
67 40 0f af 23              rex imul (%ebx), %esp
66 40 0f af 23              rex imul (%rbx), %sp
66 67 40 0f af 23           rex imul (%ebx), %sp
41 0f af 23                 imul   (%r11), %esp
67 41 0f af 23              imul   (%r11d), %esp
66 41 0f af 23              imul   (%r11), %sp
66 67 41 0f af 23           imul   (%r11d), %sp
48 0f af 23                 imul   (%rbx), %rsp
67 48 0f af 23              imul   (%ebx), %rsp
66 48 0f af 23              data16 imul (%rbx), %rsp
66 67 48 0f af 23           data16 imul (%ebx), %rsp
6b 23 12                    imul   $0x12, (%rbx), %esp
66 6b 23 12                 imul   $0x12, (%rbx), %sp
67 6b 23 12                 imul   $0x12, (%ebx), %esp
66 67 6b 23 12              imul   $0x12, (%ebx), %sp
40 6b 23 12                 rex imul $0x12, (%rbx), %esp
67 40 6b 23 12              rex imul $0x12, (%ebx), %esp
66 40 6b 23 12              rex imul $0x12, (%rbx), %sp
66 67 40 6b 23 12           rex imul $0x12, (%ebx), %sp
41 6b 23 12                 imul   $0x12, (%r11), %esp
67 41 6b 23 12              imul   $0x12, (%r11d), %esp
66 41 6b 23 12              imul   $0x12, (%r11), %sp
66 67 41 6b 23 12           imul   $0x12, (%r11d), %sp
48 6b 23 12                 imul   $0x12, (%rbx), %rsp
67 48 6b 23 12              imul   $0x12, (%ebx), %rsp
66 48 6b 23 12              data16 imul $0x12, (%rbx), %rsp
66 67 48 6b 23 12           data16 imul $0x12, (%ebx), %rsp
69 23 12 34 56 78           imul   $0x78563412, (%rbx), %esp
66 69 23 12 34              imul   $0x3412, (%rbx), %sp
67 69 23 12 34 56 78        imul   $0x78563412, (%ebx), %esp
66 67 69 23 12 34           imul   $0x3412, (%ebx), %sp
40 69 23 12 34 56 78        rex imul $0x78563412, (%rbx), %esp
67 40 69 23 12 34 56 78     rex imul $0x78563412, (%ebx), %esp
66 40 69 23 12 34           rex imul $0x3412, (%rbx), %sp
66 67 40 69 23 12 34        rex imul $0x3412, (%ebx), %sp
41 69 23 12 34 56 78        imul   $0x78563412, (%r11), %esp
67 41 69 23 12 34 56 78     imul   $0x78563412, (%r11d), %esp
66 41 69 23 12 34           imul   $0x3412, (%r11), %sp
66 67 41 69 23 12 34        imul   $0x3412, (%r11d), %sp
48 69 23 12 34 56 78        imul   $0x78563412, (%rbx), %rsp
67 48 69 23 12 34 56 78     imul   $0x78563412, (%ebx), %rsp
66 48 69 23 12 34 56 78     data16 imul $0x78563412, (%rbx), %rsp
66 67 48 69 23 12 34 56 78  data16 imul $0x78563412, (%ebx), %rsp
6b 23 82                    imul   $0xffffff82, (%rbx), %esp
66 6b 23 82                 imul   $0xff82, (%rbx), %sp
67 6b 23 82                 imul   $0xffffff82, (%ebx), %esp
66 67 6b 23 82              imul   $0xff82, (%ebx), %sp
40 6b 23 82                 rex imul $0xffffff82, (%rbx), %esp
67 40 6b 23 82              rex imul $0xffffff82, (%ebx), %esp
66 40 6b 23 82              rex imul $0xff82, (%rbx), %sp
66 67 40 6b 23 82           rex imul $0xff82, (%ebx), %sp
41 6b 23 82                 imul   $0xffffff82, (%r11), %esp
67 41 6b 23 82              imul   $0xffffff82, (%r11d), %esp
66 41 6b 23 82              imul   $0xff82, (%r11), %sp
66 67 41 6b 23 82           imul   $0xff82, (%r11d), %sp
48 6b 23 82                 imul   $0xffffffffffffff82, (%rbx), %rsp
67 48 6b 23 82              imul   $0xffffffffffffff82, (%ebx), %rsp
66 48 6b 23 82              data16 imul $0xffffffffffffff82, (%rbx), %rsp
66 67 48 6b 23 82           data16 imul $0xffffffffffffff82, (%ebx), %rsp
69 23 12 34 56 88           imul   $0x88563412, (%rbx), %esp
66 69 23 12 84              imul   $0x8412, (%rbx), %sp
67 69 23 12 34 56 88        imul   $0x88563412, (%ebx), %esp
66 67 69 23 12 84           imul   $0x8412, (%ebx), %sp
40 69 23 12 34 56 88        rex imul $0x88563412, (%rbx), %esp
67 40 69 23 12 34 56 88     rex imul $0x88563412, (%ebx), %esp
66 40 69 23 12 84           rex imul $0x8412, (%rbx), %sp
66 67 40 69 23 12 84        rex imul $0x8412, (%ebx), %sp
41 69 23 12 34 56 88        imul   $0x88563412, (%r11), %esp
67 41 69 23 12 34 56 88     imul   $0x88563412, (%r11d), %esp
66 41 69 23 12 84           imul   $0x8412, (%r11), %sp
66 67 41 69 23 12 84        imul   $0x8412, (%r11d), %sp
48 69 23 12 34 56 88        imul   $0xffffffff88563412, (%rbx), %rsp
67 48 69 23 12 34 56 88     imul   $0xffffffff88563412, (%ebx), %rsp
66 48 69 23 12 34 56 88     data16 imul $0xffffffff88563412, (%rbx), %rsp
66 67 48 69 23 12 34 56 88  data16 imul $0xffffffff88563412, (%ebx), %rsp
