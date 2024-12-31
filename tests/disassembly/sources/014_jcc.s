70 82                       jo     0x3fff84
70 12                       jo     0x400016
71 12                       jno    0x400018
72 12                       jb     0x40001a
73 12                       jae    0x40001c
74 12                       je     0x40001e
75 12                       jne    0x400020
76 12                       jbe    0x400022
77 12                       ja     0x400024
78 12                       js     0x400026
79 12                       jns    0x400028
7a 12                       jp     0x40002a
7b 12                       jnp    0x40002c
7c 12                       jl     0x40002e
7d 12                       jge    0x400030
7e 12                       jle    0x400032
7f 12                       jg     0x400034
e3 12                       jrcxz  0x400036
0f 80 12 34 56 88           jo     0xffffffff8896343c
0f 80 12 34 56 78           jo     0x78963442
0f 81 12 34 56 78           jno    0x78963448
0f 82 12 34 56 78           jb     0x7896344e
0f 83 12 34 56 78           jae    0x78963454
0f 84 12 34 56 78           je     0x7896345a
0f 85 12 34 56 78           jne    0x78963460
0f 86 12 34 56 78           jbe    0x78963466
0f 87 12 34 56 78           ja     0x7896346c
0f 89 12 34 56 78           jns    0x78963472
0f 88 12 34 56 78           js     0x78963478
0f 8a 12 34 56 78           jp     0x7896347e
0f 8b 12 34 56 78           jnp    0x78963484
0f 8c 12 34 56 78           jl     0x7896348a
0f 8d 12 34 56 78           jge    0x78963490
0f 8e 12 34 56 78           jle    0x78963496
0f 8f 12 34 56 78           jg     0x7896349c
66 70 82                    data16 jo 0x40000f
66 70 12                    data16 jo 0x4000a2
66 71 12                    data16 jno 0x4000a5
66 72 12                    data16 jb 0x4000a8
66 73 12                    data16 jae 0x4000ab
66 74 12                    data16 je 0x4000ae
66 75 12                    data16 jne 0x4000b1
66 76 12                    data16 jbe 0x4000b4
66 77 12                    data16 ja 0x4000b7
66 78 12                    data16 js 0x4000ba
66 79 12                    data16 jns 0x4000bd
66 7a 12                    data16 jp 0x4000c0
66 7b 12                    data16 jnp 0x4000c3
66 7c 12                    data16 jl 0x4000c6
66 7d 12                    data16 jge 0x4000c9
66 7e 12                    data16 jle 0x4000cc
66 7f 12                    data16 jg 0x4000cf
66 e3 12                    data16 jrcxz 0x4000d2
66 0f 80 12 34              jo     0x34d7
66 0f 80 12 34              jo     0x34dc
66 0f 81 12 34              jno    0x34e1
66 0f 82 12 34              jb     0x34e6
66 0f 83 12 34              jae    0x34eb
66 0f 84 12 34              je     0x34f0
66 0f 85 12 34              jne    0x34f5
66 0f 86 12 34              jbe    0x34fa
66 0f 87 12 34              ja     0x34ff
66 0f 89 12 34              jns    0x3504
66 0f 88 12 34              js     0x3509
66 0f 8a 12 34              jp     0x350e
66 0f 8b 12 34              jnp    0x3513
66 0f 8c 12 34              jl     0x3518
66 0f 8d 12 34              jge    0x351d
66 0f 8e 12 34              jle    0x3522
66 0f 8f 12 34              jg     0x3527
67 70 82                    addr32 jo 0x40009a
67 70 12                    addr32 jo 0x40012d
67 71 12                    addr32 jno 0x400130
67 72 12                    addr32 jb 0x400133
67 73 12                    addr32 jae 0x400136
67 74 12                    addr32 je 0x400139
67 75 12                    addr32 jne 0x40013c
67 76 12                    addr32 jbe 0x40013f
67 77 12                    addr32 ja 0x400142
67 78 12                    addr32 js 0x400145
67 79 12                    addr32 jns 0x400148
67 7a 12                    addr32 jp 0x40014b
67 7b 12                    addr32 jnp 0x40014e
67 7c 12                    addr32 jl 0x400151
67 7d 12                    addr32 jge 0x400154
67 7e 12                    addr32 jle 0x400157
67 7f 12                    addr32 jg 0x40015a
67 e3 12                    jecxz  0x40015d
67 0f 80 12 34 56 78        addr32 jo 0x78963564
67 0f 80 12 34 56 78        addr32 jo 0x7896356b
67 0f 81 12 34 56 78        addr32 jno 0x78963572
67 0f 82 12 34 56 78        addr32 jb 0x78963579
67 0f 83 12 34 56 78        addr32 jae 0x78963580
67 0f 84 12 34 56 78        addr32 je 0x78963587
67 0f 85 12 34 56 78        addr32 jne 0x7896358e
67 0f 86 12 34 56 78        addr32 jbe 0x78963595
67 0f 87 12 34 56 78        addr32 ja 0x7896359c
67 0f 89 12 34 56 78        addr32 jns 0x789635a3
67 0f 88 12 34 56 78        addr32 js 0x789635aa
67 0f 8a 12 34 56 78        addr32 jp 0x789635b1
67 0f 8b 12 34 56 78        addr32 jnp 0x789635b8
67 0f 8c 12 34 56 78        addr32 jl 0x789635bf
67 0f 8d 12 34 56 78        addr32 jge 0x789635c6
67 0f 8e 12 34 56 78        addr32 jle 0x789635cd
67 0f 8f 12 34 56 78        addr32 jg 0x789635d4
66 67 70 82                 data16 addr32 jo 0x400148
66 67 70 12                 data16 addr32 jo 0x4001dc
66 67 71 12                 data16 addr32 jno 0x4001e0
66 67 72 12                 data16 addr32 jb 0x4001e4
66 67 73 12                 data16 addr32 jae 0x4001e8
66 67 74 12                 data16 addr32 je 0x4001ec
66 67 75 12                 data16 addr32 jne 0x4001f0
66 67 76 12                 data16 addr32 jbe 0x4001f4
66 67 77 12                 data16 addr32 ja 0x4001f8
66 67 78 12                 data16 addr32 js 0x4001fc
66 67 79 12                 data16 addr32 jns 0x400200
66 67 7a 12                 data16 addr32 jp 0x400204
66 67 7b 12                 data16 addr32 jnp 0x400208
66 67 7c 12                 data16 addr32 jl 0x40020c
66 67 7d 12                 data16 addr32 jge 0x400210
66 67 7e 12                 data16 addr32 jle 0x400214
66 67 7f 12                 data16 addr32 jg 0x400218
66 67 e3 12                 data16 jecxz 0x40021c
66 67 0f 80 12 34           addr32 jo 0x3622
66 67 0f 80 12 34           addr32 jo 0x3628
66 67 0f 81 12 34           addr32 jno 0x362e
66 67 0f 82 12 34           addr32 jb 0x3634
66 67 0f 83 12 34           addr32 jae 0x363a
66 67 0f 84 12 34           addr32 je 0x3640
66 67 0f 85 12 34           addr32 jne 0x3646
66 67 0f 86 12 34           addr32 jbe 0x364c
66 67 0f 87 12 34           addr32 ja 0x3652
66 67 0f 89 12 34           addr32 jns 0x3658
66 67 0f 88 12 34           addr32 js 0x365e
66 67 0f 8a 12 34           addr32 jp 0x3664
66 67 0f 8b 12 34           addr32 jnp 0x366a
66 67 0f 8c 12 34           addr32 jl 0x3670
66 67 0f 8d 12 34           addr32 jge 0x3676
66 67 0f 8e 12 34           addr32 jle 0x367c
66 67 0f 8f 12 34           addr32 jg 0x3682
41 70 82                    rex.B jo 0x4001f5
41 e3 12                    rex.B jrcxz 0x400288
48 70 82                    rex.W jo 0x4001fb
48 e3 12                    rex.W jrcxz 0x40028e
49 70 82                    rex.WB jo 0x400201
49 e3 12                    rex.WB jrcxz 0x400294
