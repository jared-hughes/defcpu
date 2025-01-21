8d 0c 18                    lea    ecx, [rax+rbx*1]
8d 4c 18 12                 lea    ecx, [rax+rbx*1+0x12]
8d 8c 18 12 34 56 78        lea    ecx, [rax+rbx*1+0x78563412]
8d 4c 18 82                 lea    ecx, [rax+rbx*1-0x7e]
8d 8c 18 12 34 56 88        lea    ecx, [rax+rbx*1-0x77a9cbee]
8d                          (bad)
f4                          hlt
