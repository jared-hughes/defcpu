8a 0c 18                    mov    cl, BYTE PTR [rax+rbx*1]
8a 0c 58                    mov    cl, BYTE PTR [rax+rbx*2]
8a 0c 98                    mov    cl, BYTE PTR [rax+rbx*4]
8a 0c d8                    mov    cl, BYTE PTR [rax+rbx*8]
67 8a 0c 18                 mov    cl, BYTE PTR [eax+ebx*1]
67 8a 0c 58                 mov    cl, BYTE PTR [eax+ebx*2]
67 8a 0c 98                 mov    cl, BYTE PTR [eax+ebx*4]
67 8a 0c d8                 mov    cl, BYTE PTR [eax+ebx*8]
43 8a 0c 23                 mov    cl, BYTE PTR [r11+r12*1]
43 8a 0c 63                 mov    cl, BYTE PTR [r11+r12*2]
43 8a 0c a3                 mov    cl, BYTE PTR [r11+r12*4]
43 8a 0c e3                 mov    cl, BYTE PTR [r11+r12*8]
67 43 8a 0c 23              mov    cl, BYTE PTR [r11d+r12d*1]
67 43 8a 0c 63              mov    cl, BYTE PTR [r11d+r12d*2]
67 43 8a 0c a3              mov    cl, BYTE PTR [r11d+r12d*4]
67 43 8a 0c e3              mov    cl, BYTE PTR [r11d+r12d*8]
8a 4c 18 79                 mov    cl, BYTE PTR [rax+rbx*1+0x79]
8a 4c 58 80                 mov    cl, BYTE PTR [rax+rbx*2-0x80]
8a 8c 98 80 00 00 00        mov    cl, BYTE PTR [rax+rbx*4+0x80]
8a 8c d8 7f ff ff ff        mov    cl, BYTE PTR [rax+rbx*8-0x81]
8a 0c 1c                    mov    cl, BYTE PTR [rsp+rbx*1]
8a 35 12 34 56 78           mov    dh, BYTE PTR [rip+0x78563412]
67 8a 35 12 34 56 78        mov    dh, BYTE PTR [eip+0x78563412]
8a 04 20                    mov    al, BYTE PTR [rax+riz*1]
8a 84 20 12 34 56 78        mov    al, BYTE PTR [rax+riz*1+0x78563412]
8a 04 20                    mov    al, BYTE PTR [rax+riz*1]
8a 0c 20                    mov    cl, BYTE PTR [rax+riz*1]
8a 14 20                    mov    dl, BYTE PTR [rax+riz*1]
8a 1c 20                    mov    bl, BYTE PTR [rax+riz*1]
8a 24 20                    mov    ah, BYTE PTR [rax+riz*1]
8a 2c 20                    mov    ch, BYTE PTR [rax+riz*1]
8a 34 20                    mov    dh, BYTE PTR [rax+riz*1]
8a 3c 20                    mov    bh, BYTE PTR [rax+riz*1]
8a 04 21                    mov    al, BYTE PTR [rcx+riz*1]
8a 04 22                    mov    al, BYTE PTR [rdx+riz*1]
8a 04 23                    mov    al, BYTE PTR [rbx+riz*1]
8a 04 24                    mov    al, BYTE PTR [rsp]
8a 04 25 12 34 56 78        mov    al, BYTE PTR ds:0x78563412
8a 04 65 12 34 56 78        mov    al, BYTE PTR [riz*2+0x78563412]
8a 04 a5 12 34 56 78        mov    al, BYTE PTR [riz*4+0x78563412]
8a 04 e5 12 34 56 78        mov    al, BYTE PTR [riz*8+0x78563412]
8a 04 26                    mov    al, BYTE PTR [rsi+riz*1]
8a 04 27                    mov    al, BYTE PTR [rdi+riz*1]
8a 04 64                    mov    al, BYTE PTR [rsp+riz*2]
8a 04 a4                    mov    al, BYTE PTR [rsp+riz*4]
8a 04 e4                    mov    al, BYTE PTR [rsp+riz*8]
8a 04 ac                    mov    al, BYTE PTR [rsp+rbp*4]
8a 44 20 3a                 mov    al, BYTE PTR [rax+riz*1+0x3a]
8a 4c 20 3a                 mov    cl, BYTE PTR [rax+riz*1+0x3a]
8a 54 20 3a                 mov    dl, BYTE PTR [rax+riz*1+0x3a]
8a 5c 20 3a                 mov    bl, BYTE PTR [rax+riz*1+0x3a]
8a 64 20 3a                 mov    ah, BYTE PTR [rax+riz*1+0x3a]
8a 6c 20 3a                 mov    ch, BYTE PTR [rax+riz*1+0x3a]
8a 74 20 3a                 mov    dh, BYTE PTR [rax+riz*1+0x3a]
8a 7c 20 3a                 mov    bh, BYTE PTR [rax+riz*1+0x3a]
8a 44 24 3a                 mov    al, BYTE PTR [rsp+0x3a]
8a 44 64 3a                 mov    al, BYTE PTR [rsp+riz*2+0x3a]
8a 44 a4 3a                 mov    al, BYTE PTR [rsp+riz*4+0x3a]
8a 44 e4 3a                 mov    al, BYTE PTR [rsp+riz*8+0x3a]
8a 44 25 3a                 mov    al, BYTE PTR [rbp+riz*1+0x3a]
8a 44 65 3a                 mov    al, BYTE PTR [rbp+riz*2+0x3a]
8a 44 a5 3a                 mov    al, BYTE PTR [rbp+riz*4+0x3a]
8a 44 e5 3a                 mov    al, BYTE PTR [rbp+riz*8+0x3a]
66 8a 4c 18 79              data16 mov cl, BYTE PTR [rax+rbx*1+0x79]
66 8a 4c 58 80              data16 mov cl, BYTE PTR [rax+rbx*2-0x80]
66 8a 8c 98 80 00 00 00     data16 mov cl, BYTE PTR [rax+rbx*4+0x80]
66 8a 8c d8 7f ff ff ff     data16 mov cl, BYTE PTR [rax+rbx*8-0x81]
66 8a 0c 1c                 data16 mov cl, BYTE PTR [rsp+rbx*1]
66 8a 35 12 34 56 78        data16 mov dh, BYTE PTR [rip+0x78563412]
66 67 8a 35 12 34 56 78     data16 mov dh, BYTE PTR [eip+0x78563412]
66 8a 04 20                 data16 mov al, BYTE PTR [rax+riz*1]
66 8a 84 20 12 34 56 78     data16 mov al, BYTE PTR [rax+riz*1+0x78563412]
66 8a 04 20                 data16 mov al, BYTE PTR [rax+riz*1]
66 8a 0c 20                 data16 mov cl, BYTE PTR [rax+riz*1]
66 8a 14 20                 data16 mov dl, BYTE PTR [rax+riz*1]
66 8a 1c 20                 data16 mov bl, BYTE PTR [rax+riz*1]
66 8a 24 20                 data16 mov ah, BYTE PTR [rax+riz*1]
66 8a 2c 20                 data16 mov ch, BYTE PTR [rax+riz*1]
66 8a 34 20                 data16 mov dh, BYTE PTR [rax+riz*1]
66 8a 3c 20                 data16 mov bh, BYTE PTR [rax+riz*1]
66 8a 04 21                 data16 mov al, BYTE PTR [rcx+riz*1]
66 8a 04 22                 data16 mov al, BYTE PTR [rdx+riz*1]
66 8a 04 23                 data16 mov al, BYTE PTR [rbx+riz*1]
66 8a 04 24                 data16 mov al, BYTE PTR [rsp]
66 8a 04 25 12 34 56 78     data16 mov al, BYTE PTR ds:0x78563412
66 8a 04 65 12 34 56 78     data16 mov al, BYTE PTR [riz*2+0x78563412]
66 8a 04 a5 12 34 56 78     data16 mov al, BYTE PTR [riz*4+0x78563412]
66 8a 04 e5 12 34 56 78     data16 mov al, BYTE PTR [riz*8+0x78563412]
66 8a 04 26                 data16 mov al, BYTE PTR [rsi+riz*1]
66 8a 04 27                 data16 mov al, BYTE PTR [rdi+riz*1]
66 8a 04 64                 data16 mov al, BYTE PTR [rsp+riz*2]
66 8a 04 a4                 data16 mov al, BYTE PTR [rsp+riz*4]
66 8a 04 e4                 data16 mov al, BYTE PTR [rsp+riz*8]
66 8a 44 20 3a              data16 mov al, BYTE PTR [rax+riz*1+0x3a]
66 8a 4c 20 3a              data16 mov cl, BYTE PTR [rax+riz*1+0x3a]
66 8a 54 20 3a              data16 mov dl, BYTE PTR [rax+riz*1+0x3a]
66 8a 5c 20 3a              data16 mov bl, BYTE PTR [rax+riz*1+0x3a]
66 8a 64 20 3a              data16 mov ah, BYTE PTR [rax+riz*1+0x3a]
66 8a 6c 20 3a              data16 mov ch, BYTE PTR [rax+riz*1+0x3a]
66 8a 74 20 3a              data16 mov dh, BYTE PTR [rax+riz*1+0x3a]
66 8a 7c 20 3a              data16 mov bh, BYTE PTR [rax+riz*1+0x3a]
66 8a 44 24 3a              data16 mov al, BYTE PTR [rsp+0x3a]
66 8a 44 64 3a              data16 mov al, BYTE PTR [rsp+riz*2+0x3a]
66 8a 44 a4 3a              data16 mov al, BYTE PTR [rsp+riz*4+0x3a]
66 8a 44 e4 3a              data16 mov al, BYTE PTR [rsp+riz*8+0x3a]
66 8a 44 25 3a              data16 mov al, BYTE PTR [rbp+riz*1+0x3a]
66 8a 44 65 3a              data16 mov al, BYTE PTR [rbp+riz*2+0x3a]
66 8a 44 a5 3a              data16 mov al, BYTE PTR [rbp+riz*4+0x3a]
66 8a 44 e5 3a              data16 mov al, BYTE PTR [rbp+riz*8+0x3a]
67 8a 4c 18 79              mov    cl, BYTE PTR [eax+ebx*1+0x79]
67 8a 4c 58 80              mov    cl, BYTE PTR [eax+ebx*2-0x80]
67 8a 8c 98 80 00 00 00     mov    cl, BYTE PTR [eax+ebx*4+0x80]
67 8a 8c d8 7f ff ff ff     mov    cl, BYTE PTR [eax+ebx*8-0x81]
67 8a 0c 1c                 mov    cl, BYTE PTR [esp+ebx*1]
67 8a 35 12 34 56 78        mov    dh, BYTE PTR [eip+0x78563412]
67 67 8a 35 12 34 56 78     addr32 mov dh, BYTE PTR [eip+0x78563412]
67 8a 04 20                 mov    al, BYTE PTR [eax+eiz*1]
67 8a 84 20 12 34 56 78     mov    al, BYTE PTR [eax+eiz*1+0x78563412]
67 8a 04 20                 mov    al, BYTE PTR [eax+eiz*1]
67 8a 0c 20                 mov    cl, BYTE PTR [eax+eiz*1]
67 8a 14 20                 mov    dl, BYTE PTR [eax+eiz*1]
67 8a 1c 20                 mov    bl, BYTE PTR [eax+eiz*1]
67 8a 24 20                 mov    ah, BYTE PTR [eax+eiz*1]
67 8a 2c 20                 mov    ch, BYTE PTR [eax+eiz*1]
67 8a 34 20                 mov    dh, BYTE PTR [eax+eiz*1]
67 8a 3c 20                 mov    bh, BYTE PTR [eax+eiz*1]
67 8a 04 21                 mov    al, BYTE PTR [ecx+eiz*1]
67 8a 04 22                 mov    al, BYTE PTR [edx+eiz*1]
67 8a 04 23                 mov    al, BYTE PTR [ebx+eiz*1]
67 8a 04 24                 mov    al, BYTE PTR [esp]
67 8a 04 25 12 34 56 78     mov    al, BYTE PTR [eiz*1+0x78563412]
67 8a 04 65 12 34 56 78     mov    al, BYTE PTR [eiz*2+0x78563412]
67 8a 04 a5 12 34 56 78     mov    al, BYTE PTR [eiz*4+0x78563412]
67 8a 04 e5 12 34 56 78     mov    al, BYTE PTR [eiz*8+0x78563412]
67 8a 04 26                 mov    al, BYTE PTR [esi+eiz*1]
67 8a 04 27                 mov    al, BYTE PTR [edi+eiz*1]
67 8a 04 64                 mov    al, BYTE PTR [esp+eiz*2]
67 8a 04 a4                 mov    al, BYTE PTR [esp+eiz*4]
67 8a 04 e4                 mov    al, BYTE PTR [esp+eiz*8]
67 8a 44 20 3a              mov    al, BYTE PTR [eax+eiz*1+0x3a]
67 8a 4c 20 3a              mov    cl, BYTE PTR [eax+eiz*1+0x3a]
67 8a 54 20 3a              mov    dl, BYTE PTR [eax+eiz*1+0x3a]
67 8a 5c 20 3a              mov    bl, BYTE PTR [eax+eiz*1+0x3a]
67 8a 64 20 3a              mov    ah, BYTE PTR [eax+eiz*1+0x3a]
67 8a 6c 20 3a              mov    ch, BYTE PTR [eax+eiz*1+0x3a]
67 8a 74 20 3a              mov    dh, BYTE PTR [eax+eiz*1+0x3a]
67 8a 7c 20 3a              mov    bh, BYTE PTR [eax+eiz*1+0x3a]
67 8a 44 24 3a              mov    al, BYTE PTR [esp+0x3a]
67 8a 44 64 3a              mov    al, BYTE PTR [esp+eiz*2+0x3a]
67 8a 44 a4 3a              mov    al, BYTE PTR [esp+eiz*4+0x3a]
67 8a 44 e4 3a              mov    al, BYTE PTR [esp+eiz*8+0x3a]
67 8a 44 25 3a              mov    al, BYTE PTR [ebp+eiz*1+0x3a]
67 8a 44 65 3a              mov    al, BYTE PTR [ebp+eiz*2+0x3a]
67 8a 44 a5 3a              mov    al, BYTE PTR [ebp+eiz*4+0x3a]
67 8a 44 e5 3a              mov    al, BYTE PTR [ebp+eiz*8+0x3a]
8a 44 24 12                 mov    al, BYTE PTR [rsp+0x12]
8a 44 25 12                 mov    al, BYTE PTR [rbp+riz*1+0x12]
8a 44 65 12                 mov    al, BYTE PTR [rbp+riz*2+0x12]
8a 44 a5 12                 mov    al, BYTE PTR [rbp+riz*4+0x12]
8a 44 e5 12                 mov    al, BYTE PTR [rbp+riz*8+0x12]
8a 84 24 12 34 56 78        mov    al, BYTE PTR [rsp+0x78563412]
8a 84 25 12 34 56 78        mov    al, BYTE PTR [rbp+riz*1+0x78563412]
8a 84 65 12 34 56 78        mov    al, BYTE PTR [rbp+riz*2+0x78563412]
8a 84 a5 12 34 56 78        mov    al, BYTE PTR [rbp+riz*4+0x78563412]
8a 84 e5 12 34 56 78        mov    al, BYTE PTR [rbp+riz*8+0x78563412]
8a c4                       mov    al, ah
8a 05 12 34 56 78           mov    al, BYTE PTR [rip+0x78563412]
8a 0d 12 34 56 78           mov    cl, BYTE PTR [rip+0x78563412]
8a 15 12 34 56 78           mov    dl, BYTE PTR [rip+0x78563412]
8a 1d 12 34 56 78           mov    bl, BYTE PTR [rip+0x78563412]
8a 25 12 34 56 78           mov    ah, BYTE PTR [rip+0x78563412]
8a 2d 12 34 56 78           mov    ch, BYTE PTR [rip+0x78563412]
8a 35 12 34 56 78           mov    dh, BYTE PTR [rip+0x78563412]
8a 3d 12 34 56 78           mov    bh, BYTE PTR [rip+0x78563412]
8a 45 12                    mov    al, BYTE PTR [rbp+0x12]
