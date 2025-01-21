66 b8 ef be                         mov    ax, 0xbeef
66 b8 12 34                         mov    ax, 0x3412
66 b9 12 34                         mov    cx, 0x3412
66 ba 12 34                         mov    dx, 0x3412
66 bb 12 34                         mov    bx, 0x3412
66 bc 12 34                         mov    sp, 0x3412
66 bd 12 34                         mov    bp, 0x3412
66 be 12 34                         mov    si, 0x3412
66 bf 12 34                         mov    di, 0x3412
66 41 b8 12 34                      mov    r8w, 0x3412
66 41 b9 12 34                      mov    r9w, 0x3412
66 41 ba 12 34                      mov    r10w, 0x3412
66 41 bb 12 34                      mov    r11w, 0x3412
66 41 bc 12 34                      mov    r12w, 0x3412
66 41 bd 12 34                      mov    r13w, 0x3412
66 41 be 12 34                      mov    r14w, 0x3412
66 41 bf 12 34                      mov    r15w, 0x3412
66 40 b8 12 34                      rex mov ax, 0x3412
66 41 b8 12 34                      mov    r8w, 0x3412
66 42 b8 12 34                      rex.X mov ax, 0x3412
66 43 b8 12 34                      rex.XB mov r8w, 0x3412
66 44 b8 12 34                      rex.R mov ax, 0x3412
66 45 b8 12 34                      rex.RB mov r8w, 0x3412
66 46 b8 12 34                      rex.RX mov ax, 0x3412
66 47 b8 12 34                      rex.RXB mov r8w, 0x3412
66 48 b8 12 34 56 78 9a bc de ff    data16 movabs rax, 0xffdebc9a78563412
66 49 b8 12 34 56 78 9a bc de ff    data16 movabs r8, 0xffdebc9a78563412
66 4a b8 12 34 56 78 9a bc de ff    data16 rex.WX movabs rax, 0xffdebc9a78563412
66 4b b8 12 34 56 78 9a bc de ff    data16 rex.WXB movabs r8, 0xffdebc9a78563412
66 4c b8 12 34 56 78 9a bc de ff    data16 rex.WR movabs rax, 0xffdebc9a78563412
66 4d b8 12 34 56 78 9a bc de ff    data16 rex.WRB movabs r8, 0xffdebc9a78563412
66 4e b8 12 34 56 78 9a bc de ff    data16 rex.WRX movabs rax, 0xffdebc9a78563412
66 4f b8 12 34 56 78 9a bc de ff    data16 rex.WRXB movabs r8, 0xffdebc9a78563412
