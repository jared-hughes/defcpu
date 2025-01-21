f3 ae               repz scas al, BYTE PTR es:[rdi]
f2 ae               repnz scas al, BYTE PTR es:[rdi]
f2 03 00            repnz add eax, DWORD PTR [rax]
03 00               add    eax, DWORD PTR [rax]
66 f2 67 f3 00 00   data16 repnz repz add BYTE PTR [eax], al
67 f2 ae            repnz scas al, BYTE PTR es:[edi]
f2 ae               repnz scas al, BYTE PTR es:[rdi]
66 f2 49            data16 repnz rex.WB
f2 67 ae            repnz scas al, BYTE PTR es:[edi]
66 f2 49            data16 repnz rex.WB
f2 ae               repnz scas al, BYTE PTR es:[rdi]
66 f2 67 49 ae      data16 repnz rex.WB scas al, BYTE PTR es:[edi]
