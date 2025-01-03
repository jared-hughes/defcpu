f3 ae               repz scas %es:(%rdi), %al
f2 ae               repnz scas %es:(%rdi), %al
f2 03 00            repnz add (%rax), %eax
03 00               add    (%rax), %eax
66 f2 67 f3 00 00   data16 repnz repz add %al, (%eax)
67 f2 ae            repnz scas %es:(%edi), %al
f2 ae               repnz scas %es:(%rdi), %al
66 f2 49            data16 repnz rex.WB
f2 67 ae            repnz scas %es:(%edi), %al
66 f2 49            data16 repnz rex.WB
f2 ae               repnz scas %es:(%rdi), %al
66 f2 67 49 ae      data16 repnz rex.WB scas %es:(%edi), %al
