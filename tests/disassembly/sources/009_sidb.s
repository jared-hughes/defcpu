8a 0c 18            mov    (%rax, %rbx, 1), %cl
8a 0c 58            mov    (%rax, %rbx, 2), %cl
8a 0c 98            mov    (%rax, %rbx, 4), %cl
8a 0c d8            mov    (%rax, %rbx, 8), %cl
67 8a 0c 18         mov    (%eax, %ebx, 1), %cl
67 8a 0c 58         mov    (%eax, %ebx, 2), %cl
67 8a 0c 98         mov    (%eax, %ebx, 4), %cl
67 8a 0c d8         mov    (%eax, %ebx, 8), %cl
43 8a 0c 23         mov    (%r11, %r12, 1), %cl
43 8a 0c 63         mov    (%r11, %r12, 2), %cl
43 8a 0c a3         mov    (%r11, %r12, 4), %cl
43 8a 0c e3         mov    (%r11, %r12, 8), %cl
67 43 8a 0c 23      mov    (%r11d, %r12d, 1), %cl
67 43 8a 0c 63      mov    (%r11d, %r12d, 2), %cl
67 43 8a 0c a3      mov    (%r11d, %r12d, 4), %cl
67 43 8a 0c e3      mov    (%r11d, %r12d, 8), %cl
