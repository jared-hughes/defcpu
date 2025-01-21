c0 01 12    rol    BYTE PTR [rcx], 0x12
48 c0 01 12 rex.W rol BYTE PTR [rcx], 0x12
41 c0 01 12 rol    BYTE PTR [r9], 0x12
c0 c1 12    rol    cl, 0x12
c0 09 12    ror    BYTE PTR [rcx], 0x12
48 c0 09 12 rex.W ror BYTE PTR [rcx], 0x12
41 c0 09 12 ror    BYTE PTR [r9], 0x12
c0 c9 12    ror    cl, 0x12
c0 11 12    rcl    BYTE PTR [rcx], 0x12
48 c0 11 12 rex.W rcl BYTE PTR [rcx], 0x12
41 c0 11 12 rcl    BYTE PTR [r9], 0x12
c0 d1 12    rcl    cl, 0x12
c0 19 12    rcr    BYTE PTR [rcx], 0x12
48 c0 19 12 rex.W rcr BYTE PTR [rcx], 0x12
41 c0 19 12 rcr    BYTE PTR [r9], 0x12
c0 d9 12    rcr    cl, 0x12
d0 01       rol    BYTE PTR [rcx], 1
48 d0 01    rex.W rol BYTE PTR [rcx], 1
41 d0 01    rol    BYTE PTR [r9], 1
d0 c1       rol    cl, 1
d0 09       ror    BYTE PTR [rcx], 1
48 d0 09    rex.W ror BYTE PTR [rcx], 1
41 d0 09    ror    BYTE PTR [r9], 1
d0 c9       ror    cl, 1
d0 11       rcl    BYTE PTR [rcx], 1
48 d0 11    rex.W rcl BYTE PTR [rcx], 1
41 d0 11    rcl    BYTE PTR [r9], 1
d0 d1       rcl    cl, 1
d0 19       rcr    BYTE PTR [rcx], 1
48 d0 19    rex.W rcr BYTE PTR [rcx], 1
41 d0 19    rcr    BYTE PTR [r9], 1
d0 d9       rcr    cl, 1
c1 01 12    rol    DWORD PTR [rcx], 0x12
66 c1 01 12 rol    WORD PTR [rcx], 0x12
67 c1 01 12 rol    DWORD PTR [ecx], 0x12
41 c1 01 12 rol    DWORD PTR [r9], 0x12
48 c1 01 12 rol    QWORD PTR [rcx], 0x12
49 c1 01 12 rol    QWORD PTR [r9], 0x12
c1 09 12    ror    DWORD PTR [rcx], 0x12
66 c1 09 12 ror    WORD PTR [rcx], 0x12
67 c1 09 12 ror    DWORD PTR [ecx], 0x12
41 c1 09 12 ror    DWORD PTR [r9], 0x12
48 c1 09 12 ror    QWORD PTR [rcx], 0x12
49 c1 09 12 ror    QWORD PTR [r9], 0x12
c1 11 12    rcl    DWORD PTR [rcx], 0x12
66 c1 11 12 rcl    WORD PTR [rcx], 0x12
67 c1 11 12 rcl    DWORD PTR [ecx], 0x12
41 c1 11 12 rcl    DWORD PTR [r9], 0x12
48 c1 11 12 rcl    QWORD PTR [rcx], 0x12
49 c1 11 12 rcl    QWORD PTR [r9], 0x12
c1 19 12    rcr    DWORD PTR [rcx], 0x12
66 c1 19 12 rcr    WORD PTR [rcx], 0x12
67 c1 19 12 rcr    DWORD PTR [ecx], 0x12
41 c1 19 12 rcr    DWORD PTR [r9], 0x12
48 c1 19 12 rcr    QWORD PTR [rcx], 0x12
49 c1 19 12 rcr    QWORD PTR [r9], 0x12
d1 01       rol    DWORD PTR [rcx], 1
66 d1 01    rol    WORD PTR [rcx], 1
67 d1 01    rol    DWORD PTR [ecx], 1
41 d1 01    rol    DWORD PTR [r9], 1
48 d1 01    rol    QWORD PTR [rcx], 1
49 d1 01    rol    QWORD PTR [r9], 1
d1 09       ror    DWORD PTR [rcx], 1
66 d1 09    ror    WORD PTR [rcx], 1
67 d1 09    ror    DWORD PTR [ecx], 1
41 d1 09    ror    DWORD PTR [r9], 1
48 d1 09    ror    QWORD PTR [rcx], 1
49 d1 09    ror    QWORD PTR [r9], 1
d1 11       rcl    DWORD PTR [rcx], 1
66 d1 11    rcl    WORD PTR [rcx], 1
67 d1 11    rcl    DWORD PTR [ecx], 1
41 d1 11    rcl    DWORD PTR [r9], 1
48 d1 11    rcl    QWORD PTR [rcx], 1
49 d1 11    rcl    QWORD PTR [r9], 1
d1 19       rcr    DWORD PTR [rcx], 1
66 d1 19    rcr    WORD PTR [rcx], 1
67 d1 19    rcr    DWORD PTR [ecx], 1
41 d1 19    rcr    DWORD PTR [r9], 1
48 d1 19    rcr    QWORD PTR [rcx], 1
49 d1 19    rcr    QWORD PTR [r9], 1
d2 01       rol    BYTE PTR [rcx], cl
66 d2 01    data16 rol BYTE PTR [rcx], cl
67 d2 01    rol    BYTE PTR [ecx], cl
41 d2 01    rol    BYTE PTR [r9], cl
48 d2 01    rex.W rol BYTE PTR [rcx], cl
49 d2 01    rex.WB rol BYTE PTR [r9], cl
d2 09       ror    BYTE PTR [rcx], cl
66 d2 09    data16 ror BYTE PTR [rcx], cl
67 d2 09    ror    BYTE PTR [ecx], cl
41 d2 09    ror    BYTE PTR [r9], cl
48 d2 09    rex.W ror BYTE PTR [rcx], cl
49 d2 09    rex.WB ror BYTE PTR [r9], cl
d2 11       rcl    BYTE PTR [rcx], cl
66 d2 11    data16 rcl BYTE PTR [rcx], cl
67 d2 11    rcl    BYTE PTR [ecx], cl
41 d2 11    rcl    BYTE PTR [r9], cl
48 d2 11    rex.W rcl BYTE PTR [rcx], cl
49 d2 11    rex.WB rcl BYTE PTR [r9], cl
d2 19       rcr    BYTE PTR [rcx], cl
66 d2 19    data16 rcr BYTE PTR [rcx], cl
67 d2 19    rcr    BYTE PTR [ecx], cl
41 d2 19    rcr    BYTE PTR [r9], cl
48 d2 19    rex.W rcr BYTE PTR [rcx], cl
49 d2 19    rex.WB rcr BYTE PTR [r9], cl
d3 01       rol    DWORD PTR [rcx], cl
66 d3 01    rol    WORD PTR [rcx], cl
67 d3 01    rol    DWORD PTR [ecx], cl
41 d3 01    rol    DWORD PTR [r9], cl
48 d3 01    rol    QWORD PTR [rcx], cl
49 d3 01    rol    QWORD PTR [r9], cl
d3 09       ror    DWORD PTR [rcx], cl
66 d3 09    ror    WORD PTR [rcx], cl
67 d3 09    ror    DWORD PTR [ecx], cl
41 d3 09    ror    DWORD PTR [r9], cl
48 d3 09    ror    QWORD PTR [rcx], cl
49 d3 09    ror    QWORD PTR [r9], cl
d3 11       rcl    DWORD PTR [rcx], cl
66 d3 11    rcl    WORD PTR [rcx], cl
67 d3 11    rcl    DWORD PTR [ecx], cl
41 d3 11    rcl    DWORD PTR [r9], cl
48 d3 11    rcl    QWORD PTR [rcx], cl
49 d3 11    rcl    QWORD PTR [r9], cl
d3 19       rcr    DWORD PTR [rcx], cl
66 d3 19    rcr    WORD PTR [rcx], cl
67 d3 19    rcr    DWORD PTR [ecx], cl
41 d3 19    rcr    DWORD PTR [r9], cl
48 d3 19    rcr    QWORD PTR [rcx], cl
49 d3 19    rcr    QWORD PTR [r9], cl
