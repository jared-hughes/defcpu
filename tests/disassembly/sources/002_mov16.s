66 b8 ef be                         mov    $0xbeef, %ax
66 b8 12 34                         mov    $0x3412, %ax
66 b9 12 34                         mov    $0x3412, %cx
66 ba 12 34                         mov    $0x3412, %dx
66 bb 12 34                         mov    $0x3412, %bx
66 bc 12 34                         mov    $0x3412, %sp
66 bd 12 34                         mov    $0x3412, %bp
66 be 12 34                         mov    $0x3412, %si
66 bf 12 34                         mov    $0x3412, %di
66 41 b8 12 34                      mov    $0x3412, %r8w
66 41 b9 12 34                      mov    $0x3412, %r9w
66 41 ba 12 34                      mov    $0x3412, %r10w
66 41 bb 12 34                      mov    $0x3412, %r11w
66 41 bc 12 34                      mov    $0x3412, %r12w
66 41 bd 12 34                      mov    $0x3412, %r13w
66 41 be 12 34                      mov    $0x3412, %r14w
66 41 bf 12 34                      mov    $0x3412, %r15w
66 40 b8 12 34                      rex mov $0x3412, %ax
66 41 b8 12 34                      mov    $0x3412, %r8w
66 42 b8 12 34                      rex.X mov $0x3412, %ax
66 43 b8 12 34                      rex.XB mov $0x3412, %r8w
66 44 b8 12 34                      rex.R mov $0x3412, %ax
66 45 b8 12 34                      rex.RB mov $0x3412, %r8w
66 46 b8 12 34                      rex.RX mov $0x3412, %ax
66 47 b8 12 34                      rex.RXB mov $0x3412, %r8w
66 48 b8 12 34 56 78 9a bc de ff    data16 movabs $0xffdebc9a78563412, %rax
66 49 b8 12 34 56 78 9a bc de ff    data16 movabs $0xffdebc9a78563412, %r8
66 4a b8 12 34 56 78 9a bc de ff    data16 rex.WX movabs $0xffdebc9a78563412, %rax
66 4b b8 12 34 56 78 9a bc de ff    data16 rex.WXB movabs $0xffdebc9a78563412, %r8
66 4c b8 12 34 56 78 9a bc de ff    data16 rex.WR movabs $0xffdebc9a78563412, %rax
66 4d b8 12 34 56 78 9a bc de ff    data16 rex.WRB movabs $0xffdebc9a78563412, %r8
66 4e b8 12 34 56 78 9a bc de ff    data16 rex.WRX movabs $0xffdebc9a78563412, %rax
66 4f b8 12 34 56 78 9a bc de ff    data16 rex.WRXB movabs $0xffdebc9a78563412, %r8
