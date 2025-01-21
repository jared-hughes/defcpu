b0 12                       mov    al, 0x12
66 b0 12                    data16 mov al, 0x12
67 b0 12                    addr32 mov al, 0x12
66 67 b0 12                 data16 addr32 mov al, 0x12
66 66 67 67 b0 12           data16 data16 addr32 addr32 mov al, 0x12
67 67 66 66 b0 12           addr32 addr32 data16 data16 mov al, 0x12
67 66 67 66 b0 12           addr32 data16 addr32 data16 mov al, 0x12
88 23                       mov    BYTE PTR [rbx], ah
66 88 23                    data16 mov BYTE PTR [rbx], ah
67 88 23                    mov    BYTE PTR [ebx], ah
66 67 88 23                 data16 mov BYTE PTR [ebx], ah
66 66 67 67 88 23           data16 data16 addr32 mov BYTE PTR [ebx], ah
67 67 66 66 88 23           addr32 data16 data16 mov BYTE PTR [ebx], ah
67 66 67 66 88 23           addr32 data16 data16 mov BYTE PTR [ebx], ah
40 88 23                    mov    BYTE PTR [rbx], spl
40                          rex
66 88 23                    data16 mov BYTE PTR [rbx], ah
40                          rex
67 88 23                    mov    BYTE PTR [ebx], ah
40                          rex
66 67 88 23                 data16 mov BYTE PTR [ebx], ah
40                          rex
66 66 67 67 88 23           data16 data16 addr32 mov BYTE PTR [ebx], ah
40                          rex
67 67 66 66 88 23           addr32 data16 data16 mov BYTE PTR [ebx], ah
40                          rex
67 66 67 66 88 23           addr32 data16 data16 mov BYTE PTR [ebx], ah
40 88 23                    mov    BYTE PTR [rbx], spl
66 40 88 23                 data16 mov BYTE PTR [rbx], spl
67 40 88 23                 mov    BYTE PTR [ebx], spl
66 67 40 88 23              data16 mov BYTE PTR [ebx], spl
66 66 67 67 40 88 23        data16 data16 addr32 mov BYTE PTR [ebx], spl
67 67 66 66 40 88 23        addr32 data16 data16 mov BYTE PTR [ebx], spl
67 66 67 66 40 88 23        addr32 data16 data16 mov BYTE PTR [ebx], spl
48 88 23                    rex.W mov BYTE PTR [rbx], spl
66 48 88 23                 data16 rex.W mov BYTE PTR [rbx], spl
67 48 88 23                 rex.W mov BYTE PTR [ebx], spl
66 67 48 88 23              data16 rex.W mov BYTE PTR [ebx], spl
66 66 67 67 48 88 23        data16 data16 addr32 rex.W mov BYTE PTR [ebx], spl
67 67 66 66 48 88 23        addr32 data16 data16 rex.W mov BYTE PTR [ebx], spl
67 66 67 66 48 88 23        addr32 data16 data16 rex.W mov BYTE PTR [ebx], spl
41 88 23                    mov    BYTE PTR [r11], spl
66 41 88 23                 data16 mov BYTE PTR [r11], spl
67 41 88 23                 mov    BYTE PTR [r11d], spl
66 67 41 88 23              data16 mov BYTE PTR [r11d], spl
66 66 67 67 41 88 23        data16 data16 addr32 mov BYTE PTR [r11d], spl
67 67 66 66 41 88 23        addr32 data16 data16 mov BYTE PTR [r11d], spl
67 66 67 66 41 88 23        addr32 data16 data16 mov BYTE PTR [r11d], spl
49 88 23                    rex.WB mov BYTE PTR [r11], spl
66 49 88 23                 data16 rex.WB mov BYTE PTR [r11], spl
67 49 88 23                 rex.WB mov BYTE PTR [r11d], spl
66 67 49 88 23              data16 rex.WB mov BYTE PTR [r11d], spl
66 66 67 67 49 88 23        data16 data16 addr32 rex.WB mov BYTE PTR [r11d], spl
67 67 66 66 49 88 23        addr32 data16 data16 rex.WB mov BYTE PTR [r11d], spl
67 66 67 66 49 88 23        addr32 data16 data16 rex.WB mov BYTE PTR [r11d], spl
