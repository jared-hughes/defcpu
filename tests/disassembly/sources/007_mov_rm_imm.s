48 c7 c0 12 34 56 78        mov    rax, 0x78563412
c6 c0 12                    mov    al, 0x12
66 67 88 23                 data16 mov BYTE PTR [ebx], ah
