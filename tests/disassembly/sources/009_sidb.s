8a 0c 18                    mov    (%rax, %rbx, 1), %cl
8a 0c 58                    mov    (%rax, %rbx, 2), %cl
8a 0c 98                    mov    (%rax, %rbx, 4), %cl
8a 0c d8                    mov    (%rax, %rbx, 8), %cl
67 8a 0c 18                 mov    (%eax, %ebx, 1), %cl
67 8a 0c 58                 mov    (%eax, %ebx, 2), %cl
67 8a 0c 98                 mov    (%eax, %ebx, 4), %cl
67 8a 0c d8                 mov    (%eax, %ebx, 8), %cl
43 8a 0c 23                 mov    (%r11, %r12, 1), %cl
43 8a 0c 63                 mov    (%r11, %r12, 2), %cl
43 8a 0c a3                 mov    (%r11, %r12, 4), %cl
43 8a 0c e3                 mov    (%r11, %r12, 8), %cl
67 43 8a 0c 23              mov    (%r11d, %r12d, 1), %cl
67 43 8a 0c 63              mov    (%r11d, %r12d, 2), %cl
67 43 8a 0c a3              mov    (%r11d, %r12d, 4), %cl
67 43 8a 0c e3              mov    (%r11d, %r12d, 8), %cl
8a 4c 18 79                 mov    0x79(%rax, %rbx, 1), %cl
8a 4c 58 80                 mov    -0x80(%rax, %rbx, 2), %cl
8a 8c 98 80 00 00 00        mov    0x80(%rax, %rbx, 4), %cl
8a 8c d8 7f ff ff ff        mov    -0x81(%rax, %rbx, 8), %cl
8a 0c 1c                    mov    (%rsp, %rbx, 1), %cl
8a 35 12 34 56 78           mov    0x78563412(%rip), %dh
67 8a 35 12 34 56 78        mov    0x78563412(%eip), %dh
8a 04 20                    mov    (%rax, %riz, 1), %al
8a 84 20 12 34 56 78        mov    0x78563412(%rax, %riz, 1), %al
8a 04 20                    mov    (%rax, %riz, 1), %al
8a 0c 20                    mov    (%rax, %riz, 1), %cl
8a 14 20                    mov    (%rax, %riz, 1), %dl
8a 1c 20                    mov    (%rax, %riz, 1), %bl
8a 24 20                    mov    (%rax, %riz, 1), %ah
8a 2c 20                    mov    (%rax, %riz, 1), %ch
8a 34 20                    mov    (%rax, %riz, 1), %dh
8a 3c 20                    mov    (%rax, %riz, 1), %bh
8a 04 21                    mov    (%rcx, %riz, 1), %al
8a 04 22                    mov    (%rdx, %riz, 1), %al
8a 04 23                    mov    (%rbx, %riz, 1), %al
8a 04 24                    mov    (%rsp), %al
8a 04 25 12 34 56 78        mov    0x78563412, %al
8a 04 65 12 34 56 78        mov    0x78563412(, %riz, 2), %al
8a 04 a5 12 34 56 78        mov    0x78563412(, %riz, 4), %al
8a 04 e5 12 34 56 78        mov    0x78563412(, %riz, 8), %al
8a 04 26                    mov    (%rsi, %riz, 1), %al
8a 04 27                    mov    (%rdi, %riz, 1), %al
8a 04 64                    mov    (%rsp, %riz, 2), %al
8a 04 a4                    mov    (%rsp, %riz, 4), %al
8a 04 e4                    mov    (%rsp, %riz, 8), %al
8a 04 ac                    mov    (%rsp, %rbp, 4), %al
8a 44 20 3a                 mov    0x3a(%rax, %riz, 1), %al
8a 4c 20 3a                 mov    0x3a(%rax, %riz, 1), %cl
8a 54 20 3a                 mov    0x3a(%rax, %riz, 1), %dl
8a 5c 20 3a                 mov    0x3a(%rax, %riz, 1), %bl
8a 64 20 3a                 mov    0x3a(%rax, %riz, 1), %ah
8a 6c 20 3a                 mov    0x3a(%rax, %riz, 1), %ch
8a 74 20 3a                 mov    0x3a(%rax, %riz, 1), %dh
8a 7c 20 3a                 mov    0x3a(%rax, %riz, 1), %bh
8a 44 24 3a                 mov    0x3a(%rsp), %al
8a 44 64 3a                 mov    0x3a(%rsp, %riz, 2), %al
8a 44 a4 3a                 mov    0x3a(%rsp, %riz, 4), %al
8a 44 e4 3a                 mov    0x3a(%rsp, %riz, 8), %al
8a 44 25 3a                 mov    0x3a(%rbp, %riz, 1), %al
8a 44 65 3a                 mov    0x3a(%rbp, %riz, 2), %al
8a 44 a5 3a                 mov    0x3a(%rbp, %riz, 4), %al
8a 44 e5 3a                 mov    0x3a(%rbp, %riz, 8), %al
66 8a 4c 18 79              data16 mov 0x79(%rax, %rbx, 1), %cl
66 8a 4c 58 80              data16 mov -0x80(%rax, %rbx, 2), %cl
66 8a 8c 98 80 00 00 00     data16 mov 0x80(%rax, %rbx, 4), %cl
66 8a 8c d8 7f ff ff ff     data16 mov -0x81(%rax, %rbx, 8), %cl
66 8a 0c 1c                 data16 mov (%rsp, %rbx, 1), %cl
66 8a 35 12 34 56 78        data16 mov 0x78563412(%rip), %dh
66 67 8a 35 12 34 56 78     data16 mov 0x78563412(%eip), %dh
66 8a 04 20                 data16 mov (%rax, %riz, 1), %al
66 8a 84 20 12 34 56 78     data16 mov 0x78563412(%rax, %riz, 1), %al
66 8a 04 20                 data16 mov (%rax, %riz, 1), %al
66 8a 0c 20                 data16 mov (%rax, %riz, 1), %cl
66 8a 14 20                 data16 mov (%rax, %riz, 1), %dl
66 8a 1c 20                 data16 mov (%rax, %riz, 1), %bl
66 8a 24 20                 data16 mov (%rax, %riz, 1), %ah
66 8a 2c 20                 data16 mov (%rax, %riz, 1), %ch
66 8a 34 20                 data16 mov (%rax, %riz, 1), %dh
66 8a 3c 20                 data16 mov (%rax, %riz, 1), %bh
66 8a 04 21                 data16 mov (%rcx, %riz, 1), %al
66 8a 04 22                 data16 mov (%rdx, %riz, 1), %al
66 8a 04 23                 data16 mov (%rbx, %riz, 1), %al
66 8a 04 24                 data16 mov (%rsp), %al
66 8a 04 25 12 34 56 78     data16 mov 0x78563412, %al
66 8a 04 65 12 34 56 78     data16 mov 0x78563412(, %riz, 2), %al
66 8a 04 a5 12 34 56 78     data16 mov 0x78563412(, %riz, 4), %al
66 8a 04 e5 12 34 56 78     data16 mov 0x78563412(, %riz, 8), %al
66 8a 04 26                 data16 mov (%rsi, %riz, 1), %al
66 8a 04 27                 data16 mov (%rdi, %riz, 1), %al
66 8a 04 64                 data16 mov (%rsp, %riz, 2), %al
66 8a 04 a4                 data16 mov (%rsp, %riz, 4), %al
66 8a 04 e4                 data16 mov (%rsp, %riz, 8), %al
66 8a 44 20 3a              data16 mov 0x3a(%rax, %riz, 1), %al
66 8a 4c 20 3a              data16 mov 0x3a(%rax, %riz, 1), %cl
66 8a 54 20 3a              data16 mov 0x3a(%rax, %riz, 1), %dl
66 8a 5c 20 3a              data16 mov 0x3a(%rax, %riz, 1), %bl
66 8a 64 20 3a              data16 mov 0x3a(%rax, %riz, 1), %ah
66 8a 6c 20 3a              data16 mov 0x3a(%rax, %riz, 1), %ch
66 8a 74 20 3a              data16 mov 0x3a(%rax, %riz, 1), %dh
66 8a 7c 20 3a              data16 mov 0x3a(%rax, %riz, 1), %bh
66 8a 44 24 3a              data16 mov 0x3a(%rsp), %al
66 8a 44 64 3a              data16 mov 0x3a(%rsp, %riz, 2), %al
66 8a 44 a4 3a              data16 mov 0x3a(%rsp, %riz, 4), %al
66 8a 44 e4 3a              data16 mov 0x3a(%rsp, %riz, 8), %al
66 8a 44 25 3a              data16 mov 0x3a(%rbp, %riz, 1), %al
66 8a 44 65 3a              data16 mov 0x3a(%rbp, %riz, 2), %al
66 8a 44 a5 3a              data16 mov 0x3a(%rbp, %riz, 4), %al
66 8a 44 e5 3a              data16 mov 0x3a(%rbp, %riz, 8), %al
67 8a 4c 18 79              mov    0x79(%eax, %ebx, 1), %cl
67 8a 4c 58 80              mov    -0x80(%eax, %ebx, 2), %cl
67 8a 8c 98 80 00 00 00     mov    0x80(%eax, %ebx, 4), %cl
67 8a 8c d8 7f ff ff ff     mov    -0x81(%eax, %ebx, 8), %cl
67 8a 0c 1c                 mov    (%esp, %ebx, 1), %cl
67 8a 35 12 34 56 78        mov    0x78563412(%eip), %dh
67 67 8a 35 12 34 56 78     addr32 mov 0x78563412(%eip), %dh
67 8a 04 20                 mov    (%eax, %eiz, 1), %al
67 8a 84 20 12 34 56 78     mov    0x78563412(%eax, %eiz, 1), %al
67 8a 04 20                 mov    (%eax, %eiz, 1), %al
67 8a 0c 20                 mov    (%eax, %eiz, 1), %cl
67 8a 14 20                 mov    (%eax, %eiz, 1), %dl
67 8a 1c 20                 mov    (%eax, %eiz, 1), %bl
67 8a 24 20                 mov    (%eax, %eiz, 1), %ah
67 8a 2c 20                 mov    (%eax, %eiz, 1), %ch
67 8a 34 20                 mov    (%eax, %eiz, 1), %dh
67 8a 3c 20                 mov    (%eax, %eiz, 1), %bh
67 8a 04 21                 mov    (%ecx, %eiz, 1), %al
67 8a 04 22                 mov    (%edx, %eiz, 1), %al
67 8a 04 23                 mov    (%ebx, %eiz, 1), %al
67 8a 04 24                 mov    (%esp), %al
67 8a 04 25 12 34 56 78     mov    0x78563412(, %eiz, 1), %al
67 8a 04 65 12 34 56 78     mov    0x78563412(, %eiz, 2), %al
67 8a 04 a5 12 34 56 78     mov    0x78563412(, %eiz, 4), %al
67 8a 04 e5 12 34 56 78     mov    0x78563412(, %eiz, 8), %al
67 8a 04 26                 mov    (%esi, %eiz, 1), %al
67 8a 04 27                 mov    (%edi, %eiz, 1), %al
67 8a 04 64                 mov    (%esp, %eiz, 2), %al
67 8a 04 a4                 mov    (%esp, %eiz, 4), %al
67 8a 04 e4                 mov    (%esp, %eiz, 8), %al
67 8a 44 20 3a              mov    0x3a(%eax, %eiz, 1), %al
67 8a 4c 20 3a              mov    0x3a(%eax, %eiz, 1), %cl
67 8a 54 20 3a              mov    0x3a(%eax, %eiz, 1), %dl
67 8a 5c 20 3a              mov    0x3a(%eax, %eiz, 1), %bl
67 8a 64 20 3a              mov    0x3a(%eax, %eiz, 1), %ah
67 8a 6c 20 3a              mov    0x3a(%eax, %eiz, 1), %ch
67 8a 74 20 3a              mov    0x3a(%eax, %eiz, 1), %dh
67 8a 7c 20 3a              mov    0x3a(%eax, %eiz, 1), %bh
67 8a 44 24 3a              mov    0x3a(%esp), %al
67 8a 44 64 3a              mov    0x3a(%esp, %eiz, 2), %al
67 8a 44 a4 3a              mov    0x3a(%esp, %eiz, 4), %al
67 8a 44 e4 3a              mov    0x3a(%esp, %eiz, 8), %al
67 8a 44 25 3a              mov    0x3a(%ebp, %eiz, 1), %al
67 8a 44 65 3a              mov    0x3a(%ebp, %eiz, 2), %al
67 8a 44 a5 3a              mov    0x3a(%ebp, %eiz, 4), %al
67 8a 44 e5 3a              mov    0x3a(%ebp, %eiz, 8), %al
8a 44 24 12                 mov    0x12(%rsp), %al
8a 44 25 12                 mov    0x12(%rbp, %riz, 1), %al
8a 44 65 12                 mov    0x12(%rbp, %riz, 2), %al
8a 44 a5 12                 mov    0x12(%rbp, %riz, 4), %al
8a 44 e5 12                 mov    0x12(%rbp, %riz, 8), %al
8a 84 24 12 34 56 78        mov    0x78563412(%rsp), %al
8a 84 25 12 34 56 78        mov    0x78563412(%rbp, %riz, 1), %al
8a 84 65 12 34 56 78        mov    0x78563412(%rbp, %riz, 2), %al
8a 84 a5 12 34 56 78        mov    0x78563412(%rbp, %riz, 4), %al
8a 84 e5 12 34 56 78        mov    0x78563412(%rbp, %riz, 8), %al
8a c4                       mov    %ah, %al
8a 05 12 34 56 78           mov    0x78563412(%rip), %al
8a 0d 12 34 56 78           mov    0x78563412(%rip), %cl
8a 15 12 34 56 78           mov    0x78563412(%rip), %dl
8a 1d 12 34 56 78           mov    0x78563412(%rip), %bl
8a 25 12 34 56 78           mov    0x78563412(%rip), %ah
8a 2d 12 34 56 78           mov    0x78563412(%rip), %ch
8a 35 12 34 56 78           mov    0x78563412(%rip), %dh
8a 3d 12 34 56 78           mov    0x78563412(%rip), %bh
8a 45 12                    mov    0x12(%rbp), %al
