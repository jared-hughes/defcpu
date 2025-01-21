88 23               mov    BYTE PTR [rbx], ah
66 88 23            data16 mov BYTE PTR [rbx], ah
67 88 23            mov    BYTE PTR [ebx], ah
66 67 88 23         data16 mov BYTE PTR [ebx], ah
40 88 23            mov    BYTE PTR [rbx], spl
67 40 88 23         mov    BYTE PTR [ebx], spl
66 40 88 23         data16 mov BYTE PTR [rbx], spl
66 67 40 88 23      data16 mov BYTE PTR [ebx], spl
41 88 23            mov    BYTE PTR [r11], spl
67 41 88 23         mov    BYTE PTR [r11d], spl
66 41 88 23         data16 mov BYTE PTR [r11], spl
66 67 41 88 23      data16 mov BYTE PTR [r11d], spl
48 88 23            rex.W mov BYTE PTR [rbx], spl
67 48 88 23         rex.W mov BYTE PTR [ebx], spl
66 48 88 23         data16 rex.W mov BYTE PTR [rbx], spl
66 67 48 88 23      data16 rex.W mov BYTE PTR [ebx], spl
49 88 23            rex.WB mov BYTE PTR [r11], spl
67 49 88 23         rex.WB mov BYTE PTR [r11d], spl
66 49 88 23         data16 rex.WB mov BYTE PTR [r11], spl
66 67 49 88 23      data16 rex.WB mov BYTE PTR [r11d], spl
88 c0               mov    al, al
88 c1               mov    cl, al
88 c2               mov    dl, al
88 c3               mov    bl, al
88 c4               mov    ah, al
88 c5               mov    ch, al
88 c6               mov    dh, al
88 c7               mov    bh, al
66 88 c1            data16 mov cl, al
67 88 c1            addr32 mov cl, al
66 67 88 c1         data16 addr32 mov cl, al
40 88 c1            rex mov cl, al
67 40 88 c1         addr32 rex mov cl, al
66 40 88 c1         data16 rex mov cl, al
66 67 40 88 c1      data16 addr32 rex mov cl, al
41 88 c1            mov    r9b, al
67 41 88 c1         addr32 mov r9b, al
66 41 88 c1         data16 mov r9b, al
66 67 41 88 c1      data16 addr32 mov r9b, al
48 88 c1            rex.W mov cl, al
67 48 88 c1         addr32 rex.W mov cl, al
66 48 88 c1         data16 rex.W mov cl, al
66 67 48 88 c1      data16 addr32 rex.W mov cl, al
49 88 c1            rex.WB mov r9b, al
67 49 88 c1         addr32 rex.WB mov r9b, al
66 49 88 c1         data16 rex.WB mov r9b, al
66 67 49 88 c1      data16 addr32 rex.WB mov r9b, al
8a 23               mov    ah, BYTE PTR [rbx]
66 8a 23            data16 mov ah, BYTE PTR [rbx]
67 8a 23            mov    ah, BYTE PTR [ebx]
66 67 8a 23         data16 mov ah, BYTE PTR [ebx]
40 8a 23            mov    spl, BYTE PTR [rbx]
67 40 8a 23         mov    spl, BYTE PTR [ebx]
66 40 8a 23         data16 mov spl, BYTE PTR [rbx]
66 67 40 8a 23      data16 mov spl, BYTE PTR [ebx]
41 8a 23            mov    spl, BYTE PTR [r11]
67 41 8a 23         mov    spl, BYTE PTR [r11d]
66 41 8a 23         data16 mov spl, BYTE PTR [r11]
66 67 41 8a 23      data16 mov spl, BYTE PTR [r11d]
48 8a 23            rex.W mov spl, BYTE PTR [rbx]
67 48 8a 23         rex.W mov spl, BYTE PTR [ebx]
66 48 8a 23         data16 rex.W mov spl, BYTE PTR [rbx]
66 67 48 8a 23      data16 rex.W mov spl, BYTE PTR [ebx]
49 8a 23            rex.WB mov spl, BYTE PTR [r11]
67 49 8a 23         rex.WB mov spl, BYTE PTR [r11d]
66 49 8a 23         data16 rex.WB mov spl, BYTE PTR [r11]
66 67 49 8a 23      data16 rex.WB mov spl, BYTE PTR [r11d]
89 23               mov    DWORD PTR [rbx], esp
66 89 23            mov    WORD PTR [rbx], sp
67 89 23            mov    DWORD PTR [ebx], esp
66 67 89 23         mov    WORD PTR [ebx], sp
40 89 23            rex mov DWORD PTR [rbx], esp
67 40 89 23         rex mov DWORD PTR [ebx], esp
66 40 89 23         rex mov WORD PTR [rbx], sp
66 67 40 89 23      rex mov WORD PTR [ebx], sp
41 89 23            mov    DWORD PTR [r11], esp
67 41 89 23         mov    DWORD PTR [r11d], esp
66 41 89 23         mov    WORD PTR [r11], sp
66 67 41 89 23      mov    WORD PTR [r11d], sp
48 89 23            mov    QWORD PTR [rbx], rsp
67 48 89 23         mov    QWORD PTR [ebx], rsp
66 48 89 23         data16 mov QWORD PTR [rbx], rsp
66 67 48 89 23      data16 mov QWORD PTR [ebx], rsp
49 89 23            mov    QWORD PTR [r11], rsp
67 49 89 23         mov    QWORD PTR [r11d], rsp
66 49 89 23         data16 mov QWORD PTR [r11], rsp
66 67 49 89 23      data16 mov QWORD PTR [r11d], rsp
89 c0               mov    eax, eax
89 c1               mov    ecx, eax
89 c2               mov    edx, eax
89 c3               mov    ebx, eax
89 c4               mov    esp, eax
89 c5               mov    ebp, eax
89 c6               mov    esi, eax
89 c7               mov    edi, eax
66 89 c1            mov    cx, ax
67 89 c1            addr32 mov ecx, eax
66 67 89 c1         addr32 mov cx, ax
40 89 c1            rex mov ecx, eax
67 40 89 c1         addr32 rex mov ecx, eax
66 40 89 c1         rex mov cx, ax
66 67 40 89 c1      addr32 rex mov cx, ax
41 89 c1            mov    r9d, eax
67 41 89 c1         addr32 mov r9d, eax
66 41 89 c1         mov    r9w, ax
66 67 41 89 c1      addr32 mov r9w, ax
48 89 c1            mov    rcx, rax
67 48 89 c1         addr32 mov rcx, rax
66 48 89 c1         data16 mov rcx, rax
66 67 48 89 c1      data16 addr32 mov rcx, rax
49 89 c1            mov    r9, rax
67 49 89 c1         addr32 mov r9, rax
66 49 89 c1         data16 mov r9, rax
66 67 49 89 c1      data16 addr32 mov r9, rax
8b 23               mov    esp, DWORD PTR [rbx]
66 8b 23            mov    sp, WORD PTR [rbx]
67 8b 23            mov    esp, DWORD PTR [ebx]
66 67 8b 23         mov    sp, WORD PTR [ebx]
40 8b 23            rex mov esp, DWORD PTR [rbx]
67 40 8b 23         rex mov esp, DWORD PTR [ebx]
66 40 8b 23         rex mov sp, WORD PTR [rbx]
66 67 40 8b 23      rex mov sp, WORD PTR [ebx]
41 8b 23            mov    esp, DWORD PTR [r11]
67 41 8b 23         mov    esp, DWORD PTR [r11d]
66 41 8b 23         mov    sp, WORD PTR [r11]
66 67 41 8b 23      mov    sp, WORD PTR [r11d]
48 8b 23            mov    rsp, QWORD PTR [rbx]
67 48 8b 23         mov    rsp, QWORD PTR [ebx]
66 48 8b 23         data16 mov rsp, QWORD PTR [rbx]
66 67 48 8b 23      data16 mov rsp, QWORD PTR [ebx]
49 8b 23            mov    rsp, QWORD PTR [r11]
67 49 8b 23         mov    rsp, QWORD PTR [r11d]
66 49 8b 23         data16 mov rsp, QWORD PTR [r11]
66 67 49 8b 23      data16 mov rsp, QWORD PTR [r11d]
