bf 4f b0 b9 4a                      mov    $0x4ab9b04f, %edi
b8 47 4a 4c 40                      mov    $0x404c4a47, %eax
b0 b8                               mov    $0xb8, %al
47 b3 b9                            rex.RXB mov $0xb9, %r11b
40 bf 47 4a 47 b3                   rex mov $0xb3474a47, %edi
4f                                  rex.WRXB
4c b3 4c                            rex.WR mov $0x4c, %bl
40                                  rex
4c bf bf b8 4c 4c 4a 4a 47 47       rex.WR movabs $0x47474a4a4c4cb8bf, %rdi
4c                                  rex.WR
4c                                  rex.WR
4f b0 47                            rex.WRXB mov $0x47, %r8b
b9 47 4c 47 b3                      mov    $0xb3474c47, %ecx
4f                                  rex.WRXB
4a b0 b3                            rex.WX mov $0xb3, %al
4c                                  rex.WR
4c                                  rex.WR
4f                                  rex.WRXB
40 b0 4f                            rex mov $0x4f, %al
bf bf 4a bf b0                      mov    $0xb0bf4abf, %edi
40                                  rex
4f b0 4f                            rex.WRXB mov $0x4f, %r8b
b8 b0 4c b8 b8                      mov    $0xb8b84cb0, %eax
4c                                  rex.WR
4a                                  rex.WX
4f                                  rex.WRXB
4a b0 b3                            rex.WX mov $0xb3, %al
4a b9 b3 b0 b3 bf 4a b8 4a 4c       rex.WX movabs $0x4c4ab84abfb3b0b3, %rcx
bf 47 40 47 b8                      mov    $0xb8474047, %edi
b0 b9                               mov    $0xb9, %al
b0 b9                               mov    $0xb9, %al
4f b8 40 bf b0 4f 4a b3 40 47       rex.WRXB movabs $0x4740b34a4fb0bf40, %r8
b0 b9                               mov    $0xb9, %al
bf b0 4c b0 4f                      mov    $0x4fb04cb0, %edi
b9 40 4f b0 4f                      mov    $0x4fb04f40, %ecx
4c                                  rex.WR
40                                  rex
4c                                  rex.WR
4f b8 4c 4a 40 bf b9 b3 4a 47       rex.WRXB movabs $0x474ab3b9bf404a4c, %r8
bf b0 4f b8 b3                      mov    $0xb3b84fb0, %edi
4c                                  rex.WR
4a                                  rex.WX
4a b8 b0 47 4c bf 4c b3 40 4c       rex.WX movabs $0x4c40b34cbf4c47b0, %rax
b3 b0                               mov    $0xb0, %bl
4c                                  rex.WR
4a b3 4f                            rex.WX mov $0x4f, %bl
4c b8 bf 4c b9 b8 b8 b0 b8 4f       rex.WR movabs $0x4fb8b0b8b8b94cbf, %rax
4f b8 b3 4c 40 b3 b9 4c b8 b0       rex.WRXB movabs $0xb0b84cb9b3404cb3, %r8
4c                                  rex.WR
40 b8 4f b9 b9 b9                   rex mov $0xb9b9b94f, %eax
b0 40                               mov    $0x40, %al
b0 bf                               mov    $0xbf, %al
4a                                  rex.WX
4f b0 b9                            rex.WRXB mov $0xb9, %r8b
4f b8 4f 4f b0 4c bf 4a b8 b9       rex.WRXB movabs $0xb9b84abf4cb04f4f, %r8
4f b0 4f                            rex.WRXB mov $0x4f, %r8b
b8 4f b9 b0 b0                      mov    $0xb0b0b94f, %eax
4f b0 b3                            rex.WRXB mov $0xb3, %r8b
b0 4c                               mov    $0x4c, %al
b8 b0 b0 b9 4c                      mov    $0x4cb9b0b0, %eax
b9 4c 47 b0 b8                      mov    $0xb8b0474c, %ecx
4f                                  rex.WRXB
40 b3 4c                            rex mov $0x4c, %bl
bf bf b9 b3 4f                      mov    $0x4fb3b9bf, %edi
bf b9 4f 4a 47                      mov    $0x474a4fb9, %edi
4c bf b8 bf 4c 47 4c 4a b8 47       rex.WR movabs $0x47b84a4c474cbfb8, %rdi
4c                                  rex.WR
4a                                  rex.WX
4c                                  rex.WR
40 b0 4c                            rex mov $0x4c, %al
b8 47 b9 4a b3                      mov    $0xb34ab947, %eax
b8 b0 b3 bf 40                      mov    $0x40bfb3b0, %eax
bf 47 47 b0 4a                      mov    $0x4ab04747, %edi
b0 b3                               mov    $0xb3, %al
4f b3 40                            rex.WRXB mov $0x40, %r11b
b9 b0 4a b9 bf                      mov    $0xbfb94ab0, %ecx
4f                                  rex.WRXB
40                                  rex
4f b9 b9 b0 bf 4f b0 4c b3 b3       rex.WRXB movabs $0xb3b34cb04fbfb0b9, %r9
bf b9 b0 b8 4c                      mov    $0x4cb8b0b9, %edi
4c bf b3 bf b8 47 4c bf 40 b0       rex.WR movabs $0xb040bf4c47b8bfb3, %rdi
4a b3 40                            rex.WX mov $0x40, %bl
b0 bf                               mov    $0xbf, %al
bf b9 4c b0 bf                      mov    $0xbfb04cb9, %edi
b3 b3                               mov    $0xb3, %bl
b3 47                               mov    $0x47, %bl
b8 4f b8 b3 b3                      mov    $0xb3b3b84f, %eax
4c bf bf 4f 4f b8 4f b3 4a b0       rex.WR movabs $0xb04ab34fb84f4fbf, %rdi
4f b8 4f 4c b8 4a 40 bf b8 47       rex.WRXB movabs $0x47b8bf404ab84c4f, %r8
4f                                  rex.WRXB
4a                                  rex.WX
4c b3 4a                            rex.WR mov $0x4a, %bl
b8 b9 b9 4f 47                      mov    $0x474fb9b9, %eax
b8 4a 47 b8 4f                      mov    $0x4fb8474a, %eax
4c                                  rex.WR
4f                                  rex.WRXB
4f b8 bf b8 b9 bf 40 4a 4c 4f       rex.WRXB movabs $0x4f4c4a40bfb9b8bf, %r8
4f b9 b0 bf 4f b9 4a b8 b0 b0       rex.WRXB movabs $0xb0b0b84ab94fbfb0, %r9
bf 4c 4a 4a 4f                      mov    $0x4f4a4a4c, %edi
b9 b8 40 b9 47                      mov    $0x47b940b8, %ecx
4a b3 b3                            rex.WX mov $0xb3, %bl
bf bf b8 b3 b8                      mov    $0xb8b3b8bf, %edi
b3 47                               mov    $0x47, %bl
4f b8 40 4f 4a b3 bf 4f b8 4f       rex.WRXB movabs $0x4fb84fbfb34a4f40, %r8
4c                                  rex.WR
4c b8 b9 b8 b0 b9 4c b3 bf b9       rex.WR movabs $0xb9bfb34cb9b0b8b9, %rax
4a                                  rex.WX
40                                  rex
47                                  rex.RXB
4f                                  rex.WRXB
40                                  rex
40                                  rex
4a b0 b3                            rex.WX mov $0xb3, %al
40 b3 b8                            rex mov $0xb8, %bl
4a                                  rex.WX
40                                  rex
40                                  rex
47 bf 40 4c 4f b8                   rex.RXB mov $0xb84f4c40, %r15d
4c                                  rex.WR
4c bf 4c b3 4c 40 bf 47 4a b9       rex.WR movabs $0xb94a47bf404cb34c, %rdi
40 b3 40                            rex mov $0x40, %bl
4f                                  rex.WRXB
4a bf bf 40 b3 bf 4c 4a b3 40       rex.WX movabs $0x40b34a4cbfb340bf, %rdi
b0 bf                               mov    $0xbf, %al
b8 b8 b0 b0 4a                      mov    $0x4ab0b0b8, %eax
4c                                  rex.WR
4f                                  rex.WRXB
4c bf b9 4f b9 b9 bf 47 4f 40       rex.WR movabs $0x404f47bfb9b94fb9, %rdi
b9 47 4f 4f b9                      mov    $0xb94f4f47, %ecx
4c                                  rex.WR
47 b0 b8                            rex.RXB mov $0xb8, %r8b
b3 4c                               mov    $0x4c, %bl
4c                                  rex.WR
4a b0 40                            rex.WX mov $0x40, %al
bf 4a 40 40 b9                      mov    $0xb940404a, %edi
bf b8 47 b3 b8                      mov    $0xb8b347b8, %edi
b8 b0 40 b9 4a                      mov    $0x4ab940b0, %eax
b9 b3 4a 4c b3                      mov    $0xb34c4ab3, %ecx
47                                  rex.RXB
40                                  rex
40 b8 47 4c bf b3                   rex mov $0xb3bf4c47, %eax
b0 47                               mov    $0x47, %al
4f b0 bf                            rex.WRXB mov $0xbf, %r8b
b0 b3                               mov    $0xb3, %al
b3 4c                               mov    $0x4c, %bl
b0 4f                               mov    $0x4f, %al
b3 b0                               mov    $0xb0, %bl
b9 4a bf bf 47                      mov    $0x47bfbf4a, %ecx
4c b3 40                            rex.WR mov $0x40, %bl
40                                  rex
4a                                  rex.WX
4a                                  rex.WX
4c                                  rex.WR
47                                  rex.RXB
47                                  rex.RXB
4a b8 b8 b0 4a b3 b8 4f bf b3       rex.WX movabs $0xb3bf4fb8b34ab0b8, %rax
b0 b8                               mov    $0xb8, %al
b8 40 47 4c 4c                      mov    $0x4c4c4740, %eax
40 b0 b3                            rex mov $0xb3, %al
b8 4c b9 4f 40                      mov    $0x404fb94c, %eax
4f                                  rex.WRXB
4c                                  rex.WR
4f                                  rex.WRXB
47                                  rex.RXB
4a b8 4a b0 b0 b0 4a 4a b8 b8       rex.WX movabs $0xb8b84a4ab0b0b04a, %rax
4c                                  rex.WR
40 b0 4f                            rex mov $0x4f, %al
b8 47 4c 4a 47                      mov    $0x474a4c47, %eax
4a bf 47 40 bf b3 b0 4f 4c b8       rex.WX movabs $0xb84c4fb0b3bf4047, %rdi
b0 4f                               mov    $0x4f, %al
40 b8 4c 47 4c 47                   rex mov $0x474c474c, %eax
4a bf b3 4f 4a b8 4a b8 b3 4c       rex.WX movabs $0x4cb3b84ab84a4fb3, %rdi
4a                                  rex.WX
4a b9 b9 bf b0 40 40 47 b8 40       rex.WX movabs $0x40b8474040b0bfb9, %rcx
b8 4a 47 4a b8                      mov    $0xb84a474a, %eax
b3 4a                               mov    $0x4a, %bl
4a                                  rex.WX
4f b3 b8                            rex.WRXB mov $0xb8, %r11b
4f                                  rex.WRXB
4a                                  rex.WX
4a                                  rex.WX
47                                  rex.RXB
4c b9 b3 47 b9 47 40 b0 40 40       rex.WR movabs $0x4040b04047b947b3, %rcx
b9 4c bf 40 47                      mov    $0x4740bf4c, %ecx
bf 4a b8 b9 b0                      mov    $0xb0b9b84a, %edi
b8 b3 bf b3 b9                      mov    $0xb9b3bfb3, %eax
4c                                  rex.WR
4a                                  rex.WX
4f b8 47 b9 40 4c 4f b8 40 4f       rex.WRXB movabs $0x4f40b84f4c40b947, %r8
b3 b0                               mov    $0xb0, %bl
4c                                  rex.WR
4c b9 40 b8 47 40 b3 bf 47 4f       rex.WR movabs $0x4f47bfb34047b840, %rcx
4a b8 47 4a 47 4f b0 40 40 b3       rex.WX movabs $0xb34040b04f474a47, %rax
b0 b3                               mov    $0xb3, %al
b9 4a 4f 4c bf                      mov    $0xbf4c4f4a, %ecx
4a b8 4c 4a 4a b3 4f 40 b3 b9       rex.WX movabs $0xb9b3404fb34a4a4c, %rax
4c b0 40                            rex.WR mov $0x40, %al
4f                                  rex.WRXB
4c                                  rex.WR
4c                                  rex.WR
4f b8 b0 4f 4a 40 b9 b9 4a 4a       rex.WRXB movabs $0x4a4ab9b9404a4fb0, %r8
40                                  rex
4a                                  rex.WX
4c b0 b3                            rex.WR mov $0xb3, %al
4f bf 4a 47 b9 b3 4c b0 47 b0       rex.WRXB movabs $0xb047b04cb3b9474a, %r15
b0 4a                               mov    $0x4a, %al
4c bf bf 4c 40 4f b9 bf 4c 4c       rex.WR movabs $0x4c4cbfb94f404cbf, %rdi
b8 b3 b3 40 b0                      mov    $0xb040b3b3, %eax
b0 b8                               mov    $0xb8, %al
47 b8 bf b3 4c b8                   rex.RXB mov $0xb84cb3bf, %r8d
4a                                  rex.WX
4c b9 b8 b8 bf 4a 40 4c b3 4a       rex.WR movabs $0x4ab34c404abfb8b8, %rcx
47                                  rex.RXB
4a                                  rex.WX
4a bf 47 b8 b0 b3 b8 47 47 4f       rex.WX movabs $0x4f4747b8b3b0b847, %rdi
47                                  rex.RXB
4f b9 b9 b0 47 b0 b0 47 4a bf       rex.WRXB movabs $0xbf4a47b0b047b0b9, %r9
b8 b3 b0 4c 4c                      mov    $0x4c4cb0b3, %eax
47 b8 b3 4a 40 4c                   rex.RXB mov $0x4c404ab3, %r8d
b3 40                               mov    $0x40, %bl
4f                                  rex.WRXB
47 b0 b8                            rex.RXB mov $0xb8, %r8b
b8 b9 bf 40 47                      mov    $0x4740bfb9, %eax
b3 4a                               mov    $0x4a, %bl
40 b9 bf bf bf 4c                   rex mov $0x4cbfbfbf, %ecx
b3 4a                               mov    $0x4a, %bl
b0 4a                               mov    $0x4a, %al
bf 4f b9 b3 b9                      mov    $0xb9b3b94f, %edi
b8 b9 b3 b8 bf                      mov    $0xbfb8b3b9, %eax
b9 4a b9 4c b3                      mov    $0xb34cb94a, %ecx
b0 bf                               mov    $0xbf, %al
4a b9 47 4f 4f 40 4a 40 b0 bf       rex.WX movabs $0xbfb0404a404f4f47, %rcx
bf b3 4f 4a 4f                      mov    $0x4f4a4fb3, %edi
47                                  rex.RXB
40                                  rex
4a                                  rex.WX
40                                  rex
4f b0 bf                            rex.WRXB mov $0xbf, %r8b
bf b9 47 b8 b9                      mov    $0xb9b847b9, %edi
40                                  rex
40                                  rex
47 b0 b9                            rex.RXB mov $0xb9, %r8b
b3 b3                               mov    $0xb3, %bl
4a b3 b3                            rex.WX mov $0xb3, %bl
4c b9 47 bf b0 b3 b0 40 b9 b0       rex.WR movabs $0xb0b940b0b3b0bf47, %rcx
47                                  rex.RXB
40                                  rex
47                                  rex.RXB
4f                                  rex.WRXB
4f                                  rex.WRXB
4f bf 40 4c 40 b3 4c 47 40 bf       rex.WRXB movabs $0xbf40474cb3404c40, %r15
bf b9 b3 b3 47                      mov    $0x47b3b3b9, %edi
b3 b8                               mov    $0xb8, %bl
b9 bf 47 4f b3                      mov    $0xb34f47bf, %ecx
4f                                  rex.WRXB
40 bf b0 4c b3 4a                   rex mov $0x4ab34cb0, %edi
4c                                  rex.WR
40                                  rex
4f bf b9 bf bf 4c b3 47 b0 bf       rex.WRXB movabs $0xbfb047b34cbfbfb9, %r15
40 b0 40                            rex mov $0x40, %al
4a b8 4c b9 40 b0 47 b8 b3 47       rex.WX movabs $0x47b3b847b040b94c, %rax
bf b8 40 4a 47                      mov    $0x474a40b8, %edi
4c                                  rex.WR
40                                  rex
40 b0 4c                            rex mov $0x4c, %al
b0 bf                               mov    $0xbf, %al
40 bf b8 b0 4c b9                   rex mov $0xb94cb0b8, %edi
