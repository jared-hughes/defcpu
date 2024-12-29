88 b8 67 66 4f 66                   mov    %bh, 0x664f6667(%rax)
bf 4a 67 8a 88                      mov    $0x888a674a, %edi
b8 4c 67 b0 b8                      mov    $0xb8b0674c, %eax
c6 40 c6 67                         movb   $0x67, -0x3a(%rax)
66 c6                               data16 (bad)
88 c6                               mov    %al, %dh
88 66 66                            mov    %ah, 0x66(%rsi)
66 40                               data16 rex
47                                  rex.RXB
4f 8a b3 4c c7 c6 67                rex.WRXB mov 0x67c6c74c(%r11), %r14b
c7 c6 47 67 66 bf                   mov    $0xbf666747, %esi
66 c6 c7 88                         data16 mov $0x88, %bh
4c c7                               rex.WR (bad)
66 66 67 66 67 c7 40 66 c6 4a       data16 data16 addr32 movw $0x4ac6, 0x66(%eax)
88 40 67                            mov    %al, 0x67(%rax)
8a 4f 4a                            mov    0x4a(%rdi), %cl
66 88 c6                            data16 mov %al, %dh
47 b8 4a 47 c7 b3                   rex.RXB mov $0xb3c7474a, %r8d
66 66 67 4a                         data16 data16 addr32 rex.WX
4c                                  rex.WR
4f b0 4a                            rex.WRXB mov $0x4a, %r8b
8a 4c b3 8a                         mov    -0x76(%rbx, %rsi, 4), %cl
b0 66                               mov    $0x66, %al
88 4c 67 66                         mov    %cl, 0x66(%rdi, %riz, 2)
b9 c7 4c c6 b3                      mov    $0xb3c64cc7, %ecx
4f                                  rex.WRXB
66 b8 c6 40                         mov    $0x40c6, %ax
40                                  rex
66 b0 bf                            data16 mov $0xbf, %al
b8 4f c6 66 c6                      mov    $0xc666c64f, %eax
b3 40                               mov    $0x40, %bl
47 c6                               rex.RXB (bad)
67 66 66 b8 8a 40                   addr32 data16 mov $0x408a, %ax
c6                                  (bad)
67 bf 8a b3 b3 c7                   addr32 mov $0xc7b3b38a, %edi
c6                                  (bad)
4c                                  rex.WR
67 47 b3 67                         addr32 rex.RXB mov $0x67, %r11b
88 66 67                            mov    %ah, 0x67(%rsi)
66 b3 c6                            data16 mov $0xc6, %bl
4a                                  rex.WX
67 c7                               addr32 (bad)
b8 67 4a 88 47                      mov    $0x47884a67, %eax
66 66 88 66 67                      data16 data16 mov %ah, 0x67(%rsi)
4c                                  rex.WR
67 b9 67 47 8a c7                   addr32 mov $0xc78a4767, %ecx
40                                  rex
4a                                  rex.WX
67 c7                               addr32 (bad)
b0 c7                               mov    $0xc7, %al
88 8a 67 c6 c7 66                   mov    %cl, 0x66c7c667(%rdx)
b3 c6                               mov    $0xc6, %bl
4c 88 b9 66 47 c7 67                rex.WR mov %r15b, 0x67c74766(%rcx)
8a 67 b0                            mov    -0x50(%rdi), %ah
67 4f bf 67 66 c6 66 4a 40 c7 66    addr32 rex.WRXB movabs $0x66c7404a66c66667, %r15
66 66 47 b8 4f 4a                   data16 rex.RXB mov $0x4a4f, %r8w
40                                  rex
40 88 66 c7                         mov    %spl, -0x39(%rsi)
67 67 b8 4a c7 67 67                addr32 addr32 mov $0x6767c74a, %eax
4f 8a b0 66 4f 4c c6                rex.WRXB mov -0x39b3b09a(%r8), %r14b
b0 67                               mov    $0x67, %al
c6 47 4c 88                         movb   $0x88, 0x4c(%rdi)
88 4f c6                            mov    %cl, -0x3a(%rdi)
b0 c7                               mov    $0xc7, %al
c7                                  (bad)
67 b3 c7                            addr32 mov $0xc7, %bl
66 bf c6 66                         mov    $0x66c6, %di
c7                                  (bad)
b8 b8 8a 67 47                      mov    $0x47678ab8, %eax
40                                  rex
66 8a 4f 66                         data16 mov 0x66(%rdi), %cl
c7                                  (bad)
b9 4c 66 c7 c7                      mov    $0xc7c7664c, %ecx
66 4a                               data16 rex.WX
66 66 8a 88 b3 b0 b0 67             data16 data16 mov 0x67b0b0b3(%rax), %cl
4c                                  rex.WR
66 b0 8a                            data16 mov $0x8a, %al
b3 66                               mov    $0x66, %bl
b0 4c                               mov    $0x4c, %al
b9 4a b3 b3 88                      mov    $0x88b3b34a, %ecx
b0 67                               mov    $0x67, %al
8a 8a b0 b9 c7 4c                   mov    0x4cc7b9b0(%rdx), %cl
b9 b3 b0 67 4c                      mov    $0x4c67b0b3, %ecx
c6                                  (bad)
66 c6                               data16 (bad)
4f                                  rex.WRXB
47                                  rex.RXB
47 c6                               rex.RXB (bad)
8a 88 40 8a b9 c6                   mov    -0x394675c0(%rax), %cl
40 bf b8 4a 67 66                   rex mov $0x66674ab8, %edi
88 66 67                            mov    %ah, 0x67(%rsi)
c6                                  (bad)
b3 b8                               mov    $0xb8, %bl
8a 66 40                            mov    0x40(%rsi), %ah
b8 b8 4c b8 40                      mov    $0x40b84cb8, %eax
bf 88 88 b3 4c                      mov    $0x4cb38888, %edi
4c                                  rex.WR
66 40 b8 c7 66                      rex mov $0x66c7, %ax
4c                                  rex.WR
4c                                  rex.WR
4a                                  rex.WX
67 b8 8a b0 b0 8a                   addr32 mov $0x8ab0b08a, %eax
4a c6 c7 4c                         rex.WX mov $0x4c, %dil
67 4c c7                            addr32 rex.WR (bad)
67 4a 88 88 b3 4f 67 8a             rex.WX mov %cl, -0x7598b04d(%eax)
4f                                  rex.WRXB
47                                  rex.RXB
67 bf 66 88 c7 4f                   addr32 mov $0x4fc78866, %edi
66 66 47                            data16 data16 rex.RXB
47                                  rex.RXB
40                                  rex
47                                  rex.RXB
4c b9 4c 67 47 c6 4a 88 b9 8a       rex.WR movabs $0x8ab9884ac647674c, %rcx
66 67 b9 4c 66                      addr32 mov $0x664c, %cx
4a                                  rex.WX
66 c7                               data16 (bad)
8a bf 66 4a 88 66                   mov    0x66884a66(%rdi), %bh
b8 4a 47 8a c7                      mov    $0xc78a474a, %eax
88 40 66                            mov    %al, 0x66(%rax)
8a 67 b8                            mov    -0x48(%rdi), %ah
66 b9 40 bf                         mov    $0xbf40, %cx
66 4f                               data16 rex.WRXB
67 40 c7                            addr32 rex (bad)
66 67 c7                            data16 addr32 (bad)
67 4a                               addr32 rex.WX
67 47                               addr32 rex.RXB
4a                                  rex.WX
67 88 66 88                         mov    %ah, -0x78(%esi)
b8 67 8a b3 67                      mov    $0x67b38a67, %eax
67 88 66 4c                         mov    %ah, 0x4c(%esi)
66 67 88 88 b8 b3 88 88             data16 mov %cl, -0x77774c48(%eax)
67 66 67 67 40                      addr32 data16 addr32 addr32 rex
67 4c                               addr32 rex.WR
66 67 b0 4a                         data16 addr32 mov $0x4a, %al
66 66 c6                            data16 data16 (bad)
67 bf 4f b9 4c 67                   addr32 mov $0x674cb94f, %edi
88 b8 b8 c7 bf c6                   mov    %bh, -0x39403848(%rax)
67 66 c6                            addr32 data16 (bad)
b0 c7                               mov    $0xc7, %al
67 4c                               addr32 rex.WR
4a bf b0 88 8a 66 c6 67 c7 c7       rex.WX movabs $0xc7c767c6668a88b0, %rdi
8a 8a 47 88 c6 4a                   mov    0x4ac68847(%rdx), %cl
67 b0 66                            addr32 mov $0x66, %al
4c                                  rex.WR
66 4a                               data16 rex.WX
67 40                               addr32 rex
4c                                  rex.WR
4f                                  rex.WRXB
67 66 c6 47 8a c7                   data16 movb $0xc7, -0x76(%edi)
67 b9 67 67 b3 8a                   addr32 mov $0x8ab36767, %ecx
4f 8a c6                            rex.WRXB mov %r14b, %r8b
67 67 66 66 67 67 b3 66             addr32 addr32 data16 data16 addr32 addr32 mov $0x66, %bl
c7                                  (bad)
b8 c7 66 b3 66                      mov    $0x66b366c7, %eax
4a                                  rex.WX
67 bf 4a 66 8a 8a                   addr32 mov $0x8a8a664a, %edi
c7                                  (bad)
bf 88 c7 4c 66                      mov    $0x664cc788, %edi
67 4f 8a 88 8a bf 66 66             rex.WRXB mov 0x6666bf8a(%r8d), %r9b
67 4f c6                            addr32 rex.WRXB (bad)
bf 67 c6 b8 40                      mov    $0x40b8c667, %edi
b0 66                               mov    $0x66, %al
67 4a b3 66                         addr32 rex.WX mov $0x66, %bl
b0 c7                               mov    $0xc7, %al
bf c7 47 88 4f                      mov    $0x4f8847c7, %edi
67 66 c6                            addr32 data16 (bad)
b0 4f                               mov    $0x4f, %al
88 66 8a                            mov    %ah, -0x76(%rsi)
66 bf b3 88                         mov    $0x88b3, %di
4a b0 40                            rex.WX mov $0x40, %al
c7 c6 67 8a 67 40                   mov    $0x40678a67, %esi
67 c7 c7 67 88 88 b9                addr32 mov $0xb9888867, %edi
c6                                  (bad)
66 b0 47                            data16 mov $0x47, %al
66 b3 4c                            data16 mov $0x4c, %bl
67 66 66 c7                         addr32 data16 data16 (bad)
8a 8a 66 c7 8a 4a                   mov    0x4a8ac766(%rdx), %cl
bf b0 67 bf 66                      mov    $0x66bf67b0, %edi
66 4a                               data16 rex.WX
40 c6                               rex (bad)
b3 b3                               mov    $0xb3, %bl
66 bf 67 67                         mov    $0x6767, %di
c6                                  (bad)
4f                                  rex.WRXB
67 67 67 40                         addr32 addr32 addr32 rex
4c                                  rex.WR
67 c7                               addr32 (bad)
66 88 88 c6 40 67 66                data16 mov %cl, 0x666740c6(%rax)
4f b3 c6                            rex.WRXB mov $0xc6, %r11b
66 b9 bf c7                         mov    $0xc7bf, %cx
8a b3 67 c7 c7 8a                   mov    -0x75383899(%rbx), %dh
4f                                  rex.WRXB
67 40                               addr32 rex
67 4f                               addr32 rex.WRXB
67 88 8a c6 66 bf b9                mov    %cl, -0x4640993a(%edx)
b8 47 8a 8a 8a                      mov    $0x8a8a8a47, %eax
66 66 67 b8 c7 67                   data16 addr32 mov $0x67c7, %ax
b0 4a                               mov    $0x4a, %al
66 c6 40 b9 8a                      data16 movb $0x8a, -0x47(%rax)
88 67 47                            mov    %ah, 0x47(%rdi)
66 4a                               data16 rex.WX
66 88 c6                            data16 mov %al, %dh
47                                  rex.RXB
66 67 67 66 4a                      data16 addr32 addr32 data16 rex.WX
66 bf b9 88                         mov    $0x88b9, %di
66 67 b9 67 67                      addr32 mov $0x6767, %cx
b0 66                               mov    $0x66, %al
b0 bf                               mov    $0xbf, %al
4a                                  rex.WX
4f c6                               rex.WRXB (bad)
66 4c                               data16 rex.WR
40                                  rex
66 4a                               data16 rex.WX
66 c6                               data16 (bad)
88 67 67                            mov    %ah, 0x67(%rdi)
b0 c7                               mov    $0xc7, %al
c6                                  (bad)
4c                                  rex.WR
47 c6                               rex.RXB (bad)
88 c6                               mov    %al, %dh
88 66 40                            mov    %ah, 0x40(%rsi)
66 b3 4f                            data16 mov $0x4f, %bl
8a b3 88 67 c6 bf                   mov    -0x40399878(%rbx), %dh
66 b9 67 c6                         mov    $0xc667, %cx
8a 8a 88 40 66 b0                   mov    -0x4f99bf78(%rdx), %cl
67 66 88 67 67                      data16 mov %ah, 0x67(%edi)
88 c7                               mov    %al, %bh
b0 88                               mov    $0x88, %al
c6 c7 47                            mov    $0x47, %bh
40                                  rex
67 c6                               addr32 (bad)
bf bf 66 88 66                      mov    $0x668866bf, %edi
67 88 67 4c                         mov    %ah, 0x4c(%edi)
c7                                  (bad)
88 8a c6 66 4c b9                   mov    %cl, -0x46b3993a(%rdx)
47 c7 c6 b9 67 88 8a                rex.RXB mov $0x8a8867b9, %r14d
4a                                  rex.WX
67 88 4c 67 66                      mov    %cl, 0x66(%edi, %eiz, 2)
66 b3 8a                            data16 mov $0x8a, %bl
66 b8 4a 66                         mov    $0x664a, %ax
67 c6                               addr32 (bad)
4a 8a 47 b3                         rex.WX mov -0x4d(%rdi), %al
b3 4a                               mov    $0x4a, %bl
66 88 b8 47 67 8a 66                data16 mov %bh, 0x668a6747(%rax)
c6                                  (bad)
66 4c 8a 8a bf 67 4f c6             data16 rex.WR mov -0x39b09841(%rdx), %r9b
88 66 66                            mov    %ah, 0x66(%rsi)
67 8a 67 b9                         mov    -0x47(%edi), %ah
c6                                  (bad)
8a c7                               mov    %bh, %al
c7                                  (bad)
8a b9 67 bf 8a c7                   mov    -0x38754099(%rcx), %bh
88 40 b3                            mov    %al, -0x4d(%rax)
b0 b9                               mov    $0xb9, %al
c6                                  (bad)
8a 4c 4c 88                         mov    -0x78(%rsp, %rcx, 2), %cl
c7 c7 c6 67 4c c7                   mov    $0xc74c67c6, %edi
8a 66 b3                            mov    -0x4d(%rsi), %ah
b0 b8                               mov    $0xb8, %al
4f b9 8a c6 88 b0 8a 47 b3 8a       rex.WRXB movabs $0x8ab3478ab088c68a, %r9
4a c6                               rex.WX (bad)
67 88 c6                            addr32 mov %al, %dh
b3 4c                               mov    $0x4c, %bl
b3 c6                               mov    $0xc6, %bl
66 c7                               data16 (bad)
b0 66                               mov    $0x66, %al
66 4a                               data16 rex.WX
67 8a c6                            addr32 mov %dh, %al
88 8a c7 67 67 c7                   mov    %cl, -0x38989839(%rdx)
66 66 c6                            data16 data16 (bad)
67 c6 c6 67                         addr32 mov $0x67, %dh
67 8a 4c c7 67                      mov    0x67(%edi, %eax, 8), %cl
c6                                  (bad)
67 47 b3 66                         addr32 rex.RXB mov $0x66, %r11b
67 c7 40 b8 67 c7 8a 4c             movl   $0x4c8ac767, -0x48(%eax)
67 88 66 88                         mov    %ah, -0x78(%esi)
67 67 88 8a 40 66 66 bf             addr32 mov %cl, -0x409999c0(%edx)
47 b3 4c                            rex.RXB mov $0x4c, %r11b
4c                                  rex.WR
4c c7                               rex.WR (bad)
bf 67 4a 8a 67                      mov    $0x678a4a67, %edi
c7 40 88 c6 66 66 4c                movl   $0x4c6666c6, -0x78(%rax)
66 47                               data16 rex.RXB
47 c7                               rex.RXB (bad)
66 66 b9 67 66                      data16 mov $0x6667, %cx
66 66 c6 c6 bf                      data16 data16 mov $0xbf, %dh
c6                                  (bad)
66 8a b3 b0 66 88 c6                data16 mov -0x39779950(%rbx), %dh
bf 66 88 66 67                      mov    $0x67668866, %edi
c6                                  (bad)
4c                                  rex.WR
66 66 4f c6 00 00                   data16 data16 rex.WRXB movb $0x0, (%r8)
