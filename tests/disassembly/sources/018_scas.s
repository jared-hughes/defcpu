ae          scas   %es:(%rdi), %al
66 ae       data16 scas %es:(%rdi), %al
67 ae       scas   %es:(%edi), %al
66 67 ae    data16 scas %es:(%edi), %al
41 ae       rex.B scas %es:(%rdi), %al
48 ae       rex.W scas %es:(%rdi), %al
49 ae       rex.WB scas %es:(%rdi), %al
67 49 ae    rex.WB scas %es:(%edi), %al
66 af       scas   %es:(%rdi), %ax
67 af       scas   %es:(%edi), %eax
66 67 af    scas   %es:(%edi), %ax
41 af       rex.B scas %es:(%rdi), %eax
66 41 af    rex.B scas %es:(%rdi), %ax
67 41 af    rex.B scas %es:(%edi), %eax
66 67 41 af rex.B scas %es:(%edi), %ax
48 af       scas   %es:(%rdi), %rax
66 48 af    data16 scas %es:(%rdi), %rax
67 48 af    scas   %es:(%edi), %rax
66 67 48    data16 addr32 rex.W
49 af       rex.WB scas %es:(%rdi), %rax
66 49 af    data16 rex.WB scas %es:(%rdi), %rax
67 49 af    rex.WB scas %es:(%edi), %rax
66 67 49 af data16 rex.WB scas %es:(%edi), %rax
