8d 0c 18                    lea    (%rax, %rbx, 1), %ecx
8d 4c 18 12                 lea    0x12(%rax, %rbx, 1), %ecx
8d 8c 18 12 34 56 78        lea    0x78563412(%rax, %rbx, 1), %ecx
8d 4c 18 82                 lea    -0x7e(%rax, %rbx, 1), %ecx
8d 8c 18 12 34 56 88        lea    -0x77a9cbee(%rax, %rbx, 1), %ecx
8d                          (bad)
f4                          hlt
