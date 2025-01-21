f6 28                       imul   BYTE PTR [rax]
f6 e8                       imul   al
66 f7 28                    imul   WORD PTR [rax]
66 f7 e8                    imul   ax
f7 28                       imul   DWORD PTR [rax]
f7 e8                       imul   eax
48 f7 28                    imul   QWORD PTR [rax]
48 f7 e8                    imul   rax
6b 23 12                    imul   esp, DWORD PTR [rbx], 0x12
66 0f af 23                 imul   sp, WORD PTR [rbx]
67 0f af 23                 imul   esp, DWORD PTR [ebx]
66 67 0f af 23              imul   sp, WORD PTR [ebx]
40 0f af 23                 rex imul esp, DWORD PTR [rbx]
67 40 0f af 23              rex imul esp, DWORD PTR [ebx]
66 40 0f af 23              rex imul sp, WORD PTR [rbx]
66 67 40 0f af 23           rex imul sp, WORD PTR [ebx]
41 0f af 23                 imul   esp, DWORD PTR [r11]
67 41 0f af 23              imul   esp, DWORD PTR [r11d]
66 41 0f af 23              imul   sp, WORD PTR [r11]
66 67 41 0f af 23           imul   sp, WORD PTR [r11d]
48 0f af 23                 imul   rsp, QWORD PTR [rbx]
67 48 0f af 23              imul   rsp, QWORD PTR [ebx]
66 48 0f af 23              data16 imul rsp, QWORD PTR [rbx]
66 67 48 0f af 23           data16 imul rsp, QWORD PTR [ebx]
6b 23 12                    imul   esp, DWORD PTR [rbx], 0x12
66 6b 23 12                 imul   sp, WORD PTR [rbx], 0x12
67 6b 23 12                 imul   esp, DWORD PTR [ebx], 0x12
66 67 6b 23 12              imul   sp, WORD PTR [ebx], 0x12
40 6b 23 12                 rex imul esp, DWORD PTR [rbx], 0x12
67 40 6b 23 12              rex imul esp, DWORD PTR [ebx], 0x12
66 40 6b 23 12              rex imul sp, WORD PTR [rbx], 0x12
66 67 40 6b 23 12           rex imul sp, WORD PTR [ebx], 0x12
41 6b 23 12                 imul   esp, DWORD PTR [r11], 0x12
67 41 6b 23 12              imul   esp, DWORD PTR [r11d], 0x12
66 41 6b 23 12              imul   sp, WORD PTR [r11], 0x12
66 67 41 6b 23 12           imul   sp, WORD PTR [r11d], 0x12
48 6b 23 12                 imul   rsp, QWORD PTR [rbx], 0x12
67 48 6b 23 12              imul   rsp, QWORD PTR [ebx], 0x12
66 48 6b 23 12              data16 imul rsp, QWORD PTR [rbx], 0x12
66 67 48 6b 23 12           data16 imul rsp, QWORD PTR [ebx], 0x12
69 23 12 34 56 78           imul   esp, DWORD PTR [rbx], 0x78563412
66 69 23 12 34              imul   sp, WORD PTR [rbx], 0x3412
67 69 23 12 34 56 78        imul   esp, DWORD PTR [ebx], 0x78563412
66 67 69 23 12 34           imul   sp, WORD PTR [ebx], 0x3412
40 69 23 12 34 56 78        rex imul esp, DWORD PTR [rbx], 0x78563412
67 40 69 23 12 34 56 78     rex imul esp, DWORD PTR [ebx], 0x78563412
66 40 69 23 12 34           rex imul sp, WORD PTR [rbx], 0x3412
66 67 40 69 23 12 34        rex imul sp, WORD PTR [ebx], 0x3412
41 69 23 12 34 56 78        imul   esp, DWORD PTR [r11], 0x78563412
67 41 69 23 12 34 56 78     imul   esp, DWORD PTR [r11d], 0x78563412
66 41 69 23 12 34           imul   sp, WORD PTR [r11], 0x3412
66 67 41 69 23 12 34        imul   sp, WORD PTR [r11d], 0x3412
48 69 23 12 34 56 78        imul   rsp, QWORD PTR [rbx], 0x78563412
67 48 69 23 12 34 56 78     imul   rsp, QWORD PTR [ebx], 0x78563412
66 48 69 23 12 34 56 78     data16 imul rsp, QWORD PTR [rbx], 0x78563412
66 67 48 69 23 12 34 56 78  data16 imul rsp, QWORD PTR [ebx], 0x78563412
6b 23 82                    imul   esp, DWORD PTR [rbx], 0xffffff82
66 6b 23 82                 imul   sp, WORD PTR [rbx], 0xff82
67 6b 23 82                 imul   esp, DWORD PTR [ebx], 0xffffff82
66 67 6b 23 82              imul   sp, WORD PTR [ebx], 0xff82
40 6b 23 82                 rex imul esp, DWORD PTR [rbx], 0xffffff82
67 40 6b 23 82              rex imul esp, DWORD PTR [ebx], 0xffffff82
66 40 6b 23 82              rex imul sp, WORD PTR [rbx], 0xff82
66 67 40 6b 23 82           rex imul sp, WORD PTR [ebx], 0xff82
41 6b 23 82                 imul   esp, DWORD PTR [r11], 0xffffff82
67 41 6b 23 82              imul   esp, DWORD PTR [r11d], 0xffffff82
66 41 6b 23 82              imul   sp, WORD PTR [r11], 0xff82
66 67 41 6b 23 82           imul   sp, WORD PTR [r11d], 0xff82
48 6b 23 82                 imul   rsp, QWORD PTR [rbx], 0xffffffffffffff82
67 48 6b 23 82              imul   rsp, QWORD PTR [ebx], 0xffffffffffffff82
66 48 6b 23 82              data16 imul rsp, QWORD PTR [rbx], 0xffffffffffffff82
66 67 48 6b 23 82           data16 imul rsp, QWORD PTR [ebx], 0xffffffffffffff82
69 23 12 34 56 88           imul   esp, DWORD PTR [rbx], 0x88563412
66 69 23 12 84              imul   sp, WORD PTR [rbx], 0x8412
67 69 23 12 34 56 88        imul   esp, DWORD PTR [ebx], 0x88563412
66 67 69 23 12 84           imul   sp, WORD PTR [ebx], 0x8412
40 69 23 12 34 56 88        rex imul esp, DWORD PTR [rbx], 0x88563412
67 40 69 23 12 34 56 88     rex imul esp, DWORD PTR [ebx], 0x88563412
66 40 69 23 12 84           rex imul sp, WORD PTR [rbx], 0x8412
66 67 40 69 23 12 84        rex imul sp, WORD PTR [ebx], 0x8412
41 69 23 12 34 56 88        imul   esp, DWORD PTR [r11], 0x88563412
67 41 69 23 12 34 56 88     imul   esp, DWORD PTR [r11d], 0x88563412
66 41 69 23 12 84           imul   sp, WORD PTR [r11], 0x8412
66 67 41 69 23 12 84        imul   sp, WORD PTR [r11d], 0x8412
48 69 23 12 34 56 88        imul   rsp, QWORD PTR [rbx], 0xffffffff88563412
67 48 69 23 12 34 56 88     imul   rsp, QWORD PTR [ebx], 0xffffffff88563412
66 48 69 23 12 34 56 88     data16 imul rsp, QWORD PTR [rbx], 0xffffffff88563412
66 67 48 69 23 12 34 56 88  data16 imul rsp, QWORD PTR [ebx], 0xffffffff88563412
