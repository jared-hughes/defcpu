88 23               mov    %ah, (%rbx)
66 88 23            data16 mov %ah, (%rbx)
67 88 23            mov    %ah, (%ebx)
66 67 88 23         data16 mov %ah, (%ebx)
40 88 23            mov    %spl, (%rbx)
67 40 88 23         mov    %spl, (%ebx)
66 40 88 23         data16 mov %spl, (%rbx)
66 67 40 88 23      data16 mov %spl, (%ebx)
41 88 23            mov    %spl, (%r11)
67 41 88 23         mov    %spl, (%r11d)
66 41 88 23         data16 mov %spl, (%r11)
66 67 41 88 23      data16 mov %spl, (%r11d)
48 88 23            rex.W mov %spl, (%rbx)
67 48 88 23         rex.W mov %spl, (%ebx)
66 48 88 23         data16 rex.W mov %spl, (%rbx)
66 67 48 88 23      data16 rex.W mov %spl, (%ebx)
49 88 23            rex.WB mov %spl, (%r11)
67 49 88 23         rex.WB mov %spl, (%r11d)
66 49 88 23         data16 rex.WB mov %spl, (%r11)
66 67 49 88 23      data16 rex.WB mov %spl, (%r11d)
88 c0               mov    %al, %al
88 c1               mov    %al, %cl
88 c2               mov    %al, %dl
88 c3               mov    %al, %bl
88 c4               mov    %al, %ah
88 c5               mov    %al, %ch
88 c6               mov    %al, %dh
88 c7               mov    %al, %bh
66 88 c1            data16 mov %al, %cl
67 88 c1            addr32 mov %al, %cl
66 67 88 c1         data16 addr32 mov %al, %cl
40 88 c1            rex mov %al, %cl
67 40 88 c1         addr32 rex mov %al, %cl
66 40 88 c1         data16 rex mov %al, %cl
66 67 40 88 c1      data16 addr32 rex mov %al, %cl
41 88 c1            mov    %al, %r9b
67 41 88 c1         addr32 mov %al, %r9b
66 41 88 c1         data16 mov %al, %r9b
66 67 41 88 c1      data16 addr32 mov %al, %r9b
48 88 c1            rex.W mov %al, %cl
67 48 88 c1         addr32 rex.W mov %al, %cl
66 48 88 c1         data16 rex.W mov %al, %cl
66 67 48 88 c1      data16 addr32 rex.W mov %al, %cl
49 88 c1            rex.WB mov %al, %r9b
67 49 88 c1         addr32 rex.WB mov %al, %r9b
66 49 88 c1         data16 rex.WB mov %al, %r9b
66 67 49 88 c1      data16 addr32 rex.WB mov %al, %r9b
8a 23               mov    (%rbx), %ah
66 8a 23            data16 mov (%rbx), %ah
67 8a 23            mov    (%ebx), %ah
66 67 8a 23         data16 mov (%ebx), %ah
40 8a 23            mov    (%rbx), %spl
67 40 8a 23         mov    (%ebx), %spl
66 40 8a 23         data16 mov (%rbx), %spl
66 67 40 8a 23      data16 mov (%ebx), %spl
41 8a 23            mov    (%r11), %spl
67 41 8a 23         mov    (%r11d), %spl
66 41 8a 23         data16 mov (%r11), %spl
66 67 41 8a 23      data16 mov (%r11d), %spl
48 8a 23            rex.W mov (%rbx), %spl
67 48 8a 23         rex.W mov (%ebx), %spl
66 48 8a 23         data16 rex.W mov (%rbx), %spl
66 67 48 8a 23      data16 rex.W mov (%ebx), %spl
49 8a 23            rex.WB mov (%r11), %spl
67 49 8a 23         rex.WB mov (%r11d), %spl
66 49 8a 23         data16 rex.WB mov (%r11), %spl
66 67 49 8a 23      data16 rex.WB mov (%r11d), %spl
89 23               mov    %esp, (%rbx)
66 89 23            mov    %sp, (%rbx)
67 89 23            mov    %esp, (%ebx)
66 67 89 23         mov    %sp, (%ebx)
40 89 23            rex mov %esp, (%rbx)
67 40 89 23         rex mov %esp, (%ebx)
66 40 89 23         rex mov %sp, (%rbx)
66 67 40 89 23      rex mov %sp, (%ebx)
41 89 23            mov    %esp, (%r11)
67 41 89 23         mov    %esp, (%r11d)
66 41 89 23         mov    %sp, (%r11)
66 67 41 89 23      mov    %sp, (%r11d)
48 89 23            mov    %rsp, (%rbx)
67 48 89 23         mov    %rsp, (%ebx)
66 48 89 23         data16 mov %rsp, (%rbx)
66 67 48 89 23      data16 mov %rsp, (%ebx)
49 89 23            mov    %rsp, (%r11)
67 49 89 23         mov    %rsp, (%r11d)
66 49 89 23         data16 mov %rsp, (%r11)
66 67 49 89 23      data16 mov %rsp, (%r11d)
89 c0               mov    %eax, %eax
89 c1               mov    %eax, %ecx
89 c2               mov    %eax, %edx
89 c3               mov    %eax, %ebx
89 c4               mov    %eax, %esp
89 c5               mov    %eax, %ebp
89 c6               mov    %eax, %esi
89 c7               mov    %eax, %edi
66 89 c1            mov    %ax, %cx
67 89 c1            addr32 mov %eax, %ecx
66 67 89 c1         addr32 mov %ax, %cx
40 89 c1            rex mov %eax, %ecx
67 40 89 c1         addr32 rex mov %eax, %ecx
66 40 89 c1         rex mov %ax, %cx
66 67 40 89 c1      addr32 rex mov %ax, %cx
41 89 c1            mov    %eax, %r9d
67 41 89 c1         addr32 mov %eax, %r9d
66 41 89 c1         mov    %ax, %r9w
66 67 41 89 c1      addr32 mov %ax, %r9w
48 89 c1            mov    %rax, %rcx
67 48 89 c1         addr32 mov %rax, %rcx
66 48 89 c1         data16 mov %rax, %rcx
66 67 48 89 c1      data16 addr32 mov %rax, %rcx
49 89 c1            mov    %rax, %r9
67 49 89 c1         addr32 mov %rax, %r9
66 49 89 c1         data16 mov %rax, %r9
66 67 49 89 c1      data16 addr32 mov %rax, %r9
8b 23               mov    (%rbx), %esp
66 8b 23            mov    (%rbx), %sp
67 8b 23            mov    (%ebx), %esp
66 67 8b 23         mov    (%ebx), %sp
40 8b 23            rex mov (%rbx), %esp
67 40 8b 23         rex mov (%ebx), %esp
66 40 8b 23         rex mov (%rbx), %sp
66 67 40 8b 23      rex mov (%ebx), %sp
41 8b 23            mov    (%r11), %esp
67 41 8b 23         mov    (%r11d), %esp
66 41 8b 23         mov    (%r11), %sp
66 67 41 8b 23      mov    (%r11d), %sp
48 8b 23            mov    (%rbx), %rsp
67 48 8b 23         mov    (%ebx), %rsp
66 48 8b 23         data16 mov (%rbx), %rsp
66 67 48 8b 23      data16 mov (%ebx), %rsp
49 8b 23            mov    (%r11), %rsp
67 49 8b 23         mov    (%r11d), %rsp
66 49 8b 23         data16 mov (%r11), %rsp
66 67 49 8b 23      data16 mov (%r11d), %rsp
