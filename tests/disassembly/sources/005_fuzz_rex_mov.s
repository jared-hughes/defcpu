bf 4f b0 b9 4a                      mov    edi, 0x4ab9b04f
b8 47 4a 4c 40                      mov    eax, 0x404c4a47
b0 b8                               mov    al, 0xb8
47 b3 b9                            rex.RXB mov r11b, 0xb9
40 bf 47 4a 47 b3                   rex mov edi, 0xb3474a47
4f                                  rex.WRXB
4c b3 4c                            rex.WR mov bl, 0x4c
40                                  rex
4c bf bf b8 4c 4c 4a 4a 47 47       rex.WR movabs rdi, 0x47474a4a4c4cb8bf
4c                                  rex.WR
4c                                  rex.WR
4f b0 47                            rex.WRXB mov r8b, 0x47
b9 47 4c 47 b3                      mov    ecx, 0xb3474c47
4f                                  rex.WRXB
4a b0 b3                            rex.WX mov al, 0xb3
4c                                  rex.WR
4c                                  rex.WR
4f                                  rex.WRXB
40 b0 4f                            rex mov al, 0x4f
bf bf 4a bf b0                      mov    edi, 0xb0bf4abf
40                                  rex
4f b0 4f                            rex.WRXB mov r8b, 0x4f
b8 b0 4c b8 b8                      mov    eax, 0xb8b84cb0
4c                                  rex.WR
4a                                  rex.WX
4f                                  rex.WRXB
4a b0 b3                            rex.WX mov al, 0xb3
4a b9 b3 b0 b3 bf 4a b8 4a 4c       rex.WX movabs rcx, 0x4c4ab84abfb3b0b3
bf 47 40 47 b8                      mov    edi, 0xb8474047
b0 b9                               mov    al, 0xb9
b0 b9                               mov    al, 0xb9
4f b8 40 bf b0 4f 4a b3 40 47       rex.WRXB movabs r8, 0x4740b34a4fb0bf40
b0 b9                               mov    al, 0xb9
bf b0 4c b0 4f                      mov    edi, 0x4fb04cb0
b9 40 4f b0 4f                      mov    ecx, 0x4fb04f40
4c                                  rex.WR
40                                  rex
4c                                  rex.WR
4f b8 4c 4a 40 bf b9 b3 4a 47       rex.WRXB movabs r8, 0x474ab3b9bf404a4c
bf b0 4f b8 b3                      mov    edi, 0xb3b84fb0
4c                                  rex.WR
4a                                  rex.WX
4a b8 b0 47 4c bf 4c b3 40 4c       rex.WX movabs rax, 0x4c40b34cbf4c47b0
b3 b0                               mov    bl, 0xb0
4c                                  rex.WR
4a b3 4f                            rex.WX mov bl, 0x4f
4c b8 bf 4c b9 b8 b8 b0 b8 4f       rex.WR movabs rax, 0x4fb8b0b8b8b94cbf
4f b8 b3 4c 40 b3 b9 4c b8 b0       rex.WRXB movabs r8, 0xb0b84cb9b3404cb3
4c                                  rex.WR
40 b8 4f b9 b9 b9                   rex mov eax, 0xb9b9b94f
b0 40                               mov    al, 0x40
b0 bf                               mov    al, 0xbf
4a                                  rex.WX
4f b0 b9                            rex.WRXB mov r8b, 0xb9
4f b8 4f 4f b0 4c bf 4a b8 b9       rex.WRXB movabs r8, 0xb9b84abf4cb04f4f
4f b0 4f                            rex.WRXB mov r8b, 0x4f
b8 4f b9 b0 b0                      mov    eax, 0xb0b0b94f
4f b0 b3                            rex.WRXB mov r8b, 0xb3
b0 4c                               mov    al, 0x4c
b8 b0 b0 b9 4c                      mov    eax, 0x4cb9b0b0
b9 4c 47 b0 b8                      mov    ecx, 0xb8b0474c
4f                                  rex.WRXB
40 b3 4c                            rex mov bl, 0x4c
bf bf b9 b3 4f                      mov    edi, 0x4fb3b9bf
bf b9 4f 4a 47                      mov    edi, 0x474a4fb9
4c bf b8 bf 4c 47 4c 4a b8 47       rex.WR movabs rdi, 0x47b84a4c474cbfb8
4c                                  rex.WR
4a                                  rex.WX
4c                                  rex.WR
40 b0 4c                            rex mov al, 0x4c
b8 47 b9 4a b3                      mov    eax, 0xb34ab947
b8 b0 b3 bf 40                      mov    eax, 0x40bfb3b0
bf 47 47 b0 4a                      mov    edi, 0x4ab04747
b0 b3                               mov    al, 0xb3
4f b3 40                            rex.WRXB mov r11b, 0x40
b9 b0 4a b9 bf                      mov    ecx, 0xbfb94ab0
4f                                  rex.WRXB
40                                  rex
4f b9 b9 b0 bf 4f b0 4c b3 b3       rex.WRXB movabs r9, 0xb3b34cb04fbfb0b9
bf b9 b0 b8 4c                      mov    edi, 0x4cb8b0b9
4c bf b3 bf b8 47 4c bf 40 b0       rex.WR movabs rdi, 0xb040bf4c47b8bfb3
4a b3 40                            rex.WX mov bl, 0x40
b0 bf                               mov    al, 0xbf
bf b9 4c b0 bf                      mov    edi, 0xbfb04cb9
b3 b3                               mov    bl, 0xb3
b3 47                               mov    bl, 0x47
b8 4f b8 b3 b3                      mov    eax, 0xb3b3b84f
4c bf bf 4f 4f b8 4f b3 4a b0       rex.WR movabs rdi, 0xb04ab34fb84f4fbf
4f b8 4f 4c b8 4a 40 bf b8 47       rex.WRXB movabs r8, 0x47b8bf404ab84c4f
4f                                  rex.WRXB
4a                                  rex.WX
4c b3 4a                            rex.WR mov bl, 0x4a
b8 b9 b9 4f 47                      mov    eax, 0x474fb9b9
b8 4a 47 b8 4f                      mov    eax, 0x4fb8474a
4c                                  rex.WR
4f                                  rex.WRXB
4f b8 bf b8 b9 bf 40 4a 4c 4f       rex.WRXB movabs r8, 0x4f4c4a40bfb9b8bf
4f b9 b0 bf 4f b9 4a b8 b0 b0       rex.WRXB movabs r9, 0xb0b0b84ab94fbfb0
bf 4c 4a 4a 4f                      mov    edi, 0x4f4a4a4c
b9 b8 40 b9 47                      mov    ecx, 0x47b940b8
4a b3 b3                            rex.WX mov bl, 0xb3
bf bf b8 b3 b8                      mov    edi, 0xb8b3b8bf
b3 47                               mov    bl, 0x47
4f b8 40 4f 4a b3 bf 4f b8 4f       rex.WRXB movabs r8, 0x4fb84fbfb34a4f40
4c                                  rex.WR
4c b8 b9 b8 b0 b9 4c b3 bf b9       rex.WR movabs rax, 0xb9bfb34cb9b0b8b9
4a                                  rex.WX
40                                  rex
47                                  rex.RXB
4f                                  rex.WRXB
40                                  rex
40                                  rex
4a b0 b3                            rex.WX mov al, 0xb3
40 b3 b8                            rex mov bl, 0xb8
4a                                  rex.WX
40                                  rex
40                                  rex
47 bf 40 4c 4f b8                   rex.RXB mov r15d, 0xb84f4c40
4c                                  rex.WR
4c bf 4c b3 4c 40 bf 47 4a b9       rex.WR movabs rdi, 0xb94a47bf404cb34c
40 b3 40                            rex mov bl, 0x40
4f                                  rex.WRXB
4a bf bf 40 b3 bf 4c 4a b3 40       rex.WX movabs rdi, 0x40b34a4cbfb340bf
b0 bf                               mov    al, 0xbf
b8 b8 b0 b0 4a                      mov    eax, 0x4ab0b0b8
4c                                  rex.WR
4f                                  rex.WRXB
4c bf b9 4f b9 b9 bf 47 4f 40       rex.WR movabs rdi, 0x404f47bfb9b94fb9
b9 47 4f 4f b9                      mov    ecx, 0xb94f4f47
4c                                  rex.WR
47 b0 b8                            rex.RXB mov r8b, 0xb8
b3 4c                               mov    bl, 0x4c
4c                                  rex.WR
4a b0 40                            rex.WX mov al, 0x40
bf 4a 40 40 b9                      mov    edi, 0xb940404a
bf b8 47 b3 b8                      mov    edi, 0xb8b347b8
b8 b0 40 b9 4a                      mov    eax, 0x4ab940b0
b9 b3 4a 4c b3                      mov    ecx, 0xb34c4ab3
47                                  rex.RXB
40                                  rex
40 b8 47 4c bf b3                   rex mov eax, 0xb3bf4c47
b0 47                               mov    al, 0x47
4f b0 bf                            rex.WRXB mov r8b, 0xbf
b0 b3                               mov    al, 0xb3
b3 4c                               mov    bl, 0x4c
b0 4f                               mov    al, 0x4f
b3 b0                               mov    bl, 0xb0
b9 4a bf bf 47                      mov    ecx, 0x47bfbf4a
4c b3 40                            rex.WR mov bl, 0x40
40                                  rex
4a                                  rex.WX
4a                                  rex.WX
4c                                  rex.WR
47                                  rex.RXB
47                                  rex.RXB
4a b8 b8 b0 4a b3 b8 4f bf b3       rex.WX movabs rax, 0xb3bf4fb8b34ab0b8
b0 b8                               mov    al, 0xb8
b8 40 47 4c 4c                      mov    eax, 0x4c4c4740
40 b0 b3                            rex mov al, 0xb3
b8 4c b9 4f 40                      mov    eax, 0x404fb94c
4f                                  rex.WRXB
4c                                  rex.WR
4f                                  rex.WRXB
47                                  rex.RXB
4a b8 4a b0 b0 b0 4a 4a b8 b8       rex.WX movabs rax, 0xb8b84a4ab0b0b04a
4c                                  rex.WR
40 b0 4f                            rex mov al, 0x4f
b8 47 4c 4a 47                      mov    eax, 0x474a4c47
4a bf 47 40 bf b3 b0 4f 4c b8       rex.WX movabs rdi, 0xb84c4fb0b3bf4047
b0 4f                               mov    al, 0x4f
40 b8 4c 47 4c 47                   rex mov eax, 0x474c474c
4a bf b3 4f 4a b8 4a b8 b3 4c       rex.WX movabs rdi, 0x4cb3b84ab84a4fb3
4a                                  rex.WX
4a b9 b9 bf b0 40 40 47 b8 40       rex.WX movabs rcx, 0x40b8474040b0bfb9
b8 4a 47 4a b8                      mov    eax, 0xb84a474a
b3 4a                               mov    bl, 0x4a
4a                                  rex.WX
4f b3 b8                            rex.WRXB mov r11b, 0xb8
4f                                  rex.WRXB
4a                                  rex.WX
4a                                  rex.WX
47                                  rex.RXB
4c b9 b3 47 b9 47 40 b0 40 40       rex.WR movabs rcx, 0x4040b04047b947b3
b9 4c bf 40 47                      mov    ecx, 0x4740bf4c
bf 4a b8 b9 b0                      mov    edi, 0xb0b9b84a
b8 b3 bf b3 b9                      mov    eax, 0xb9b3bfb3
4c                                  rex.WR
4a                                  rex.WX
4f b8 47 b9 40 4c 4f b8 40 4f       rex.WRXB movabs r8, 0x4f40b84f4c40b947
b3 b0                               mov    bl, 0xb0
4c                                  rex.WR
4c b9 40 b8 47 40 b3 bf 47 4f       rex.WR movabs rcx, 0x4f47bfb34047b840
4a b8 47 4a 47 4f b0 40 40 b3       rex.WX movabs rax, 0xb34040b04f474a47
b0 b3                               mov    al, 0xb3
b9 4a 4f 4c bf                      mov    ecx, 0xbf4c4f4a
4a b8 4c 4a 4a b3 4f 40 b3 b9       rex.WX movabs rax, 0xb9b3404fb34a4a4c
4c b0 40                            rex.WR mov al, 0x40
4f                                  rex.WRXB
4c                                  rex.WR
4c                                  rex.WR
4f b8 b0 4f 4a 40 b9 b9 4a 4a       rex.WRXB movabs r8, 0x4a4ab9b9404a4fb0
40                                  rex
4a                                  rex.WX
4c b0 b3                            rex.WR mov al, 0xb3
4f bf 4a 47 b9 b3 4c b0 47 b0       rex.WRXB movabs r15, 0xb047b04cb3b9474a
b0 4a                               mov    al, 0x4a
4c bf bf 4c 40 4f b9 bf 4c 4c       rex.WR movabs rdi, 0x4c4cbfb94f404cbf
b8 b3 b3 40 b0                      mov    eax, 0xb040b3b3
b0 b8                               mov    al, 0xb8
47 b8 bf b3 4c b8                   rex.RXB mov r8d, 0xb84cb3bf
4a                                  rex.WX
4c b9 b8 b8 bf 4a 40 4c b3 4a       rex.WR movabs rcx, 0x4ab34c404abfb8b8
47                                  rex.RXB
4a                                  rex.WX
4a bf 47 b8 b0 b3 b8 47 47 4f       rex.WX movabs rdi, 0x4f4747b8b3b0b847
47                                  rex.RXB
4f b9 b9 b0 47 b0 b0 47 4a bf       rex.WRXB movabs r9, 0xbf4a47b0b047b0b9
b8 b3 b0 4c 4c                      mov    eax, 0x4c4cb0b3
47 b8 b3 4a 40 4c                   rex.RXB mov r8d, 0x4c404ab3
b3 40                               mov    bl, 0x40
4f                                  rex.WRXB
47 b0 b8                            rex.RXB mov r8b, 0xb8
b8 b9 bf 40 47                      mov    eax, 0x4740bfb9
b3 4a                               mov    bl, 0x4a
40 b9 bf bf bf 4c                   rex mov ecx, 0x4cbfbfbf
b3 4a                               mov    bl, 0x4a
b0 4a                               mov    al, 0x4a
bf 4f b9 b3 b9                      mov    edi, 0xb9b3b94f
b8 b9 b3 b8 bf                      mov    eax, 0xbfb8b3b9
b9 4a b9 4c b3                      mov    ecx, 0xb34cb94a
b0 bf                               mov    al, 0xbf
4a b9 47 4f 4f 40 4a 40 b0 bf       rex.WX movabs rcx, 0xbfb0404a404f4f47
bf b3 4f 4a 4f                      mov    edi, 0x4f4a4fb3
47                                  rex.RXB
40                                  rex
4a                                  rex.WX
40                                  rex
4f b0 bf                            rex.WRXB mov r8b, 0xbf
bf b9 47 b8 b9                      mov    edi, 0xb9b847b9
40                                  rex
40                                  rex
47 b0 b9                            rex.RXB mov r8b, 0xb9
b3 b3                               mov    bl, 0xb3
4a b3 b3                            rex.WX mov bl, 0xb3
4c b9 47 bf b0 b3 b0 40 b9 b0       rex.WR movabs rcx, 0xb0b940b0b3b0bf47
47                                  rex.RXB
40                                  rex
47                                  rex.RXB
4f                                  rex.WRXB
4f                                  rex.WRXB
4f bf 40 4c 40 b3 4c 47 40 bf       rex.WRXB movabs r15, 0xbf40474cb3404c40
bf b9 b3 b3 47                      mov    edi, 0x47b3b3b9
b3 b8                               mov    bl, 0xb8
b9 bf 47 4f b3                      mov    ecx, 0xb34f47bf
4f                                  rex.WRXB
40 bf b0 4c b3 4a                   rex mov edi, 0x4ab34cb0
4c                                  rex.WR
40                                  rex
4f bf b9 bf bf 4c b3 47 b0 bf       rex.WRXB movabs r15, 0xbfb047b34cbfbfb9
40 b0 40                            rex mov al, 0x40
4a b8 4c b9 40 b0 47 b8 b3 47       rex.WX movabs rax, 0x47b3b847b040b94c
bf b8 40 4a 47                      mov    edi, 0x474a40b8
4c                                  rex.WR
40                                  rex
40 b0 4c                            rex mov al, 0x4c
b0 bf                               mov    al, 0xbf
40 bf b8 b0 4c b9                   rex mov edi, 0xb94cb0b8
