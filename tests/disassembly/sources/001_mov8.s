b0 11       mov    $0x11, %al
b1 12       mov    $0x12, %cl
b2 13       mov    $0x13, %dl
b3 14       mov    $0x14, %bl
b4 21       mov    $0x21, %ah
b5 22       mov    $0x22, %ch
b6 23       mov    $0x23, %dh
b7 24       mov    $0x24, %bh
40 b6 31    mov    $0x31, %sil
40 b7 32    mov    $0x32, %dil
40 b4 33    mov    $0x33, %spl
40 b5 34    mov    $0x34, %bpl
41 b0 41    mov    $0x41, %r8b
41 b1 42    mov    $0x42, %r9b
41 b2 43    mov    $0x43, %r10b
41 b3 44    mov    $0x44, %r11b
41 b4 45    mov    $0x45, %r12b
41 b5 46    mov    $0x46, %r13b
41 b6 47    mov    $0x47, %r14b
41 b7 48    mov    $0x48, %r15b
42 b6 52    rex.X mov $0x52, %sil
43 b6 53    rex.XB mov $0x53, %r14b
44 b6 54    rex.R mov $0x54, %sil
45 b6 55    rex.RB mov $0x55, %r14b
46 b6 56    rex.RX mov $0x56, %sil
47 b6 57    rex.RXB mov $0x57, %r14b
48 b6 58    rex.W mov $0x58, %sil
49 b6 59    rex.WB mov $0x59, %r14b
4a b6 5a    rex.WX mov $0x5a, %sil
4b b6 5b    rex.WXB mov $0x5b, %r14b
4c b6 5c    rex.WR mov $0x5c, %sil
4d b6 5d    rex.WRB mov $0x5d, %r14b
4e b6 5e    rex.WRX mov $0x5e, %sil
4f b6 5f    rex.WRXB mov $0x5f, %r14b
