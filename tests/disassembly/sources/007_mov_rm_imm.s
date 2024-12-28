48 c7 c0 12 34 56 78        mov    $0x78563412, %rax
c6 c0 12                    mov    $0x12, %al
66 67 88 23                 data16 mov %ah, (%ebx)
