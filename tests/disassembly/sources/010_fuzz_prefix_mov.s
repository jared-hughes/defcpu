88 b8 67 66 4f 66                   mov    BYTE PTR [rax+0x664f6667], bh
bf 4a 67 8a 88                      mov    edi, 0x888a674a
b8 4c 67 b0 b8                      mov    eax, 0xb8b0674c
c6 40 c6 67                         mov    BYTE PTR [rax-0x3a], 0x67
66 c6                               data16 (bad)
88 c6                               mov    dh, al
88 66 66                            mov    BYTE PTR [rsi+0x66], ah
66 40                               data16 rex
47                                  rex.RXB
4f 8a b3 4c c7 c6 67                rex.WRXB mov r14b, BYTE PTR [r11+0x67c6c74c]
c7 c6 47 67 66 bf                   mov    esi, 0xbf666747
66 c6 c7 88                         data16 mov bh, 0x88
4c c7                               rex.WR (bad)
66 66 67 66 67 c7 40 66 c6 4a       data16 data16 addr32 mov WORD PTR [eax+0x66], 0x4ac6
88 40 67                            mov    BYTE PTR [rax+0x67], al
8a 4f 4a                            mov    cl, BYTE PTR [rdi+0x4a]
66 88 c6                            data16 mov dh, al
47 b8 4a 47 c7 b3                   rex.RXB mov r8d, 0xb3c7474a
66 66 67 4a                         data16 data16 addr32 rex.WX
4c                                  rex.WR
4f b0 4a                            rex.WRXB mov r8b, 0x4a
8a 4c b3 8a                         mov    cl, BYTE PTR [rbx+rsi*4-0x76]
b0 66                               mov    al, 0x66
88 4c 67 66                         mov    BYTE PTR [rdi+riz*2+0x66], cl
b9 c7 4c c6 b3                      mov    ecx, 0xb3c64cc7
4f                                  rex.WRXB
66 b8 c6 40                         mov    ax, 0x40c6
40                                  rex
66 b0 bf                            data16 mov al, 0xbf
b8 4f c6 66 c6                      mov    eax, 0xc666c64f
b3 40                               mov    bl, 0x40
47 c6                               rex.RXB (bad)
67 66 66 b8 8a 40                   addr32 data16 mov ax, 0x408a
c6                                  (bad)
67 bf 8a b3 b3 c7                   addr32 mov edi, 0xc7b3b38a
c6                                  (bad)
4c                                  rex.WR
67 47 b3 67                         addr32 rex.RXB mov r11b, 0x67
88 66 67                            mov    BYTE PTR [rsi+0x67], ah
66 b3 c6                            data16 mov bl, 0xc6
4a                                  rex.WX
67 c7                               addr32 (bad)
b8 67 4a 88 47                      mov    eax, 0x47884a67
66 66 88 66 67                      data16 data16 mov BYTE PTR [rsi+0x67], ah
4c                                  rex.WR
67 b9 67 47 8a c7                   addr32 mov ecx, 0xc78a4767
40                                  rex
4a                                  rex.WX
67 c7                               addr32 (bad)
b0 c7                               mov    al, 0xc7
88 8a 67 c6 c7 66                   mov    BYTE PTR [rdx+0x66c7c667], cl
b3 c6                               mov    bl, 0xc6
4c 88 b9 66 47 c7 67                rex.WR mov BYTE PTR [rcx+0x67c74766], r15b
8a 67 b0                            mov    ah, BYTE PTR [rdi-0x50]
67 4f bf 67 66 c6 66 4a 40 c7 66    addr32 rex.WRXB movabs r15, 0x66c7404a66c66667
66 66 47 b8 4f 4a                   data16 rex.RXB mov r8w, 0x4a4f
40                                  rex
40 88 66 c7                         mov    BYTE PTR [rsi-0x39], spl
67 67 b8 4a c7 67 67                addr32 addr32 mov eax, 0x6767c74a
4f 8a b0 66 4f 4c c6                rex.WRXB mov r14b, BYTE PTR [r8-0x39b3b09a]
b0 67                               mov    al, 0x67
c6 47 4c 88                         mov    BYTE PTR [rdi+0x4c], 0x88
88 4f c6                            mov    BYTE PTR [rdi-0x3a], cl
b0 c7                               mov    al, 0xc7
c7                                  (bad)
67 b3 c7                            addr32 mov bl, 0xc7
66 bf c6 66                         mov    di, 0x66c6
c7                                  (bad)
b8 b8 8a 67 47                      mov    eax, 0x47678ab8
40                                  rex
66 8a 4f 66                         data16 mov cl, BYTE PTR [rdi+0x66]
c7                                  (bad)
b9 4c 66 c7 c7                      mov    ecx, 0xc7c7664c
66 4a                               data16 rex.WX
66 66 8a 88 b3 b0 b0 67             data16 data16 mov cl, BYTE PTR [rax+0x67b0b0b3]
4c                                  rex.WR
66 b0 8a                            data16 mov al, 0x8a
b3 66                               mov    bl, 0x66
b0 4c                               mov    al, 0x4c
b9 4a b3 b3 88                      mov    ecx, 0x88b3b34a
b0 67                               mov    al, 0x67
8a 8a b0 b9 c7 4c                   mov    cl, BYTE PTR [rdx+0x4cc7b9b0]
b9 b3 b0 67 4c                      mov    ecx, 0x4c67b0b3
c6                                  (bad)
66 c6                               data16 (bad)
4f                                  rex.WRXB
47                                  rex.RXB
47 c6                               rex.RXB (bad)
8a 88 40 8a b9 c6                   mov    cl, BYTE PTR [rax-0x394675c0]
40 bf b8 4a 67 66                   rex mov edi, 0x66674ab8
88 66 67                            mov    BYTE PTR [rsi+0x67], ah
c6                                  (bad)
b3 b8                               mov    bl, 0xb8
8a 66 40                            mov    ah, BYTE PTR [rsi+0x40]
b8 b8 4c b8 40                      mov    eax, 0x40b84cb8
bf 88 88 b3 4c                      mov    edi, 0x4cb38888
4c                                  rex.WR
66 40 b8 c7 66                      rex mov ax, 0x66c7
4c                                  rex.WR
4c                                  rex.WR
4a                                  rex.WX
67 b8 8a b0 b0 8a                   addr32 mov eax, 0x8ab0b08a
4a c6 c7 4c                         rex.WX mov dil, 0x4c
67 4c c7                            addr32 rex.WR (bad)
67 4a 88 88 b3 4f 67 8a             rex.WX mov BYTE PTR [eax-0x7598b04d], cl
4f                                  rex.WRXB
47                                  rex.RXB
67 bf 66 88 c7 4f                   addr32 mov edi, 0x4fc78866
66 66 47                            data16 data16 rex.RXB
47                                  rex.RXB
40                                  rex
47                                  rex.RXB
4c b9 4c 67 47 c6 4a 88 b9 8a       rex.WR movabs rcx, 0x8ab9884ac647674c
66 67 b9 4c 66                      addr32 mov cx, 0x664c
4a                                  rex.WX
66 c7                               data16 (bad)
8a bf 66 4a 88 66                   mov    bh, BYTE PTR [rdi+0x66884a66]
b8 4a 47 8a c7                      mov    eax, 0xc78a474a
88 40 66                            mov    BYTE PTR [rax+0x66], al
8a 67 b8                            mov    ah, BYTE PTR [rdi-0x48]
66 b9 40 bf                         mov    cx, 0xbf40
66 4f                               data16 rex.WRXB
67 40 c7                            addr32 rex (bad)
66 67 c7                            data16 addr32 (bad)
67 4a                               addr32 rex.WX
67 47                               addr32 rex.RXB
4a                                  rex.WX
67 88 66 88                         mov    BYTE PTR [esi-0x78], ah
b8 67 8a b3 67                      mov    eax, 0x67b38a67
67 88 66 4c                         mov    BYTE PTR [esi+0x4c], ah
66 67 88 88 b8 b3 88 88             data16 mov BYTE PTR [eax-0x77774c48], cl
67 66 67 67 40                      addr32 data16 addr32 addr32 rex
67 4c                               addr32 rex.WR
66 67 b0 4a                         data16 addr32 mov al, 0x4a
66 66 c6                            data16 data16 (bad)
67 bf 4f b9 4c 67                   addr32 mov edi, 0x674cb94f
88 b8 b8 c7 bf c6                   mov    BYTE PTR [rax-0x39403848], bh
67 66 c6                            addr32 data16 (bad)
b0 c7                               mov    al, 0xc7
67 4c                               addr32 rex.WR
4a bf b0 88 8a 66 c6 67 c7 c7       rex.WX movabs rdi, 0xc7c767c6668a88b0
8a 8a 47 88 c6 4a                   mov    cl, BYTE PTR [rdx+0x4ac68847]
67 b0 66                            addr32 mov al, 0x66
4c                                  rex.WR
66 4a                               data16 rex.WX
67 40                               addr32 rex
4c                                  rex.WR
4f                                  rex.WRXB
67 66 c6 47 8a c7                   data16 mov BYTE PTR [edi-0x76], 0xc7
67 b9 67 67 b3 8a                   addr32 mov ecx, 0x8ab36767
4f 8a c6                            rex.WRXB mov r8b, r14b
67 67 66 66 67 67 b3 66             addr32 addr32 data16 data16 addr32 addr32 mov bl, 0x66
c7                                  (bad)
b8 c7 66 b3 66                      mov    eax, 0x66b366c7
4a                                  rex.WX
67 bf 4a 66 8a 8a                   addr32 mov edi, 0x8a8a664a
c7                                  (bad)
bf 88 c7 4c 66                      mov    edi, 0x664cc788
67 4f 8a 88 8a bf 66 66             rex.WRXB mov r9b, BYTE PTR [r8d+0x6666bf8a]
67 4f c6                            addr32 rex.WRXB (bad)
bf 67 c6 b8 40                      mov    edi, 0x40b8c667
b0 66                               mov    al, 0x66
67 4a b3 66                         addr32 rex.WX mov bl, 0x66
b0 c7                               mov    al, 0xc7
bf c7 47 88 4f                      mov    edi, 0x4f8847c7
67 66 c6                            addr32 data16 (bad)
b0 4f                               mov    al, 0x4f
88 66 8a                            mov    BYTE PTR [rsi-0x76], ah
66 bf b3 88                         mov    di, 0x88b3
4a b0 40                            rex.WX mov al, 0x40
c7 c6 67 8a 67 40                   mov    esi, 0x40678a67
67 c7 c7 67 88 88 b9                addr32 mov edi, 0xb9888867
c6                                  (bad)
66 b0 47                            data16 mov al, 0x47
66 b3 4c                            data16 mov bl, 0x4c
67 66 66 c7                         addr32 data16 data16 (bad)
8a 8a 66 c7 8a 4a                   mov    cl, BYTE PTR [rdx+0x4a8ac766]
bf b0 67 bf 66                      mov    edi, 0x66bf67b0
66 4a                               data16 rex.WX
40 c6                               rex (bad)
b3 b3                               mov    bl, 0xb3
66 bf 67 67                         mov    di, 0x6767
c6                                  (bad)
4f                                  rex.WRXB
67 67 67 40                         addr32 addr32 addr32 rex
4c                                  rex.WR
67 c7                               addr32 (bad)
66 88 88 c6 40 67 66                data16 mov BYTE PTR [rax+0x666740c6], cl
4f b3 c6                            rex.WRXB mov r11b, 0xc6
66 b9 bf c7                         mov    cx, 0xc7bf
8a b3 67 c7 c7 8a                   mov    dh, BYTE PTR [rbx-0x75383899]
4f                                  rex.WRXB
67 40                               addr32 rex
67 4f                               addr32 rex.WRXB
67 88 8a c6 66 bf b9                mov    BYTE PTR [edx-0x4640993a], cl
b8 47 8a 8a 8a                      mov    eax, 0x8a8a8a47
66 66 67 b8 c7 67                   data16 addr32 mov ax, 0x67c7
b0 4a                               mov    al, 0x4a
66 c6 40 b9 8a                      data16 mov BYTE PTR [rax-0x47], 0x8a
88 67 47                            mov    BYTE PTR [rdi+0x47], ah
66 4a                               data16 rex.WX
66 88 c6                            data16 mov dh, al
47                                  rex.RXB
66 67 67 66 4a                      data16 addr32 addr32 data16 rex.WX
66 bf b9 88                         mov    di, 0x88b9
66 67 b9 67 67                      addr32 mov cx, 0x6767
b0 66                               mov    al, 0x66
b0 bf                               mov    al, 0xbf
4a                                  rex.WX
4f c6                               rex.WRXB (bad)
66 4c                               data16 rex.WR
40                                  rex
66 4a                               data16 rex.WX
66 c6                               data16 (bad)
88 67 67                            mov    BYTE PTR [rdi+0x67], ah
b0 c7                               mov    al, 0xc7
c6                                  (bad)
4c                                  rex.WR
47 c6                               rex.RXB (bad)
88 c6                               mov    dh, al
88 66 40                            mov    BYTE PTR [rsi+0x40], ah
66 b3 4f                            data16 mov bl, 0x4f
8a b3 88 67 c6 bf                   mov    dh, BYTE PTR [rbx-0x40399878]
66 b9 67 c6                         mov    cx, 0xc667
8a 8a 88 40 66 b0                   mov    cl, BYTE PTR [rdx-0x4f99bf78]
67 66 88 67 67                      data16 mov BYTE PTR [edi+0x67], ah
88 c7                               mov    bh, al
b0 88                               mov    al, 0x88
c6 c7 47                            mov    bh, 0x47
40                                  rex
67 c6                               addr32 (bad)
bf bf 66 88 66                      mov    edi, 0x668866bf
67 88 67 4c                         mov    BYTE PTR [edi+0x4c], ah
c7                                  (bad)
88 8a c6 66 4c b9                   mov    BYTE PTR [rdx-0x46b3993a], cl
47 c7 c6 b9 67 88 8a                rex.RXB mov r14d, 0x8a8867b9
4a                                  rex.WX
67 88 4c 67 66                      mov    BYTE PTR [edi+eiz*2+0x66], cl
66 b3 8a                            data16 mov bl, 0x8a
66 b8 4a 66                         mov    ax, 0x664a
67 c6                               addr32 (bad)
4a 8a 47 b3                         rex.WX mov al, BYTE PTR [rdi-0x4d]
b3 4a                               mov    bl, 0x4a
66 88 b8 47 67 8a 66                data16 mov BYTE PTR [rax+0x668a6747], bh
c6                                  (bad)
66 4c 8a 8a bf 67 4f c6             data16 rex.WR mov r9b, BYTE PTR [rdx-0x39b09841]
88 66 66                            mov    BYTE PTR [rsi+0x66], ah
67 8a 67 b9                         mov    ah, BYTE PTR [edi-0x47]
c6                                  (bad)
8a c7                               mov    al, bh
c7                                  (bad)
8a b9 67 bf 8a c7                   mov    bh, BYTE PTR [rcx-0x38754099]
88 40 b3                            mov    BYTE PTR [rax-0x4d], al
b0 b9                               mov    al, 0xb9
c6                                  (bad)
8a 4c 4c 88                         mov    cl, BYTE PTR [rsp+rcx*2-0x78]
c7 c7 c6 67 4c c7                   mov    edi, 0xc74c67c6
8a 66 b3                            mov    ah, BYTE PTR [rsi-0x4d]
b0 b8                               mov    al, 0xb8
4f b9 8a c6 88 b0 8a 47 b3 8a       rex.WRXB movabs r9, 0x8ab3478ab088c68a
4a c6                               rex.WX (bad)
67 88 c6                            addr32 mov dh, al
b3 4c                               mov    bl, 0x4c
b3 c6                               mov    bl, 0xc6
66 c7                               data16 (bad)
b0 66                               mov    al, 0x66
66 4a                               data16 rex.WX
67 8a c6                            addr32 mov al, dh
88 8a c7 67 67 c7                   mov    BYTE PTR [rdx-0x38989839], cl
66 66 c6                            data16 data16 (bad)
67 c6 c6 67                         addr32 mov dh, 0x67
67 8a 4c c7 67                      mov    cl, BYTE PTR [edi+eax*8+0x67]
c6                                  (bad)
67 47 b3 66                         addr32 rex.RXB mov r11b, 0x66
67 c7 40 b8 67 c7 8a 4c             mov    DWORD PTR [eax-0x48], 0x4c8ac767
67 88 66 88                         mov    BYTE PTR [esi-0x78], ah
67 67 88 8a 40 66 66 bf             addr32 mov BYTE PTR [edx-0x409999c0], cl
47 b3 4c                            rex.RXB mov r11b, 0x4c
4c                                  rex.WR
4c c7                               rex.WR (bad)
bf 67 4a 8a 67                      mov    edi, 0x678a4a67
c7 40 88 c6 66 66 4c                mov    DWORD PTR [rax-0x78], 0x4c6666c6
66 47                               data16 rex.RXB
47 c7                               rex.RXB (bad)
66 66 b9 67 66                      data16 mov cx, 0x6667
66 66 c6 c6 bf                      data16 data16 mov dh, 0xbf
c6                                  (bad)
66 8a b3 b0 66 88 c6                data16 mov dh, BYTE PTR [rbx-0x39779950]
bf 66 88 66 67                      mov    edi, 0x67668866
c6                                  (bad)
4c                                  rex.WR
66 66 4f c6 00 00                   data16 data16 rex.WRXB mov BYTE PTR [r8], 0x0
