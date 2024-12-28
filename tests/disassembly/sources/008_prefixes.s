b0 12                       mov    $0x12, %al
66 b0 12                    data16 mov $0x12, %al
67 b0 12                    addr32 mov $0x12, %al
66 67 b0 12                 data16 addr32 mov $0x12, %al
66 66 67 67 b0 12           data16 data16 addr32 addr32 mov $0x12, %al
67 67 66 66 b0 12           addr32 addr32 data16 data16 mov $0x12, %al
67 66 67 66 b0 12           addr32 data16 addr32 data16 mov $0x12, %al
88 23                       mov    %ah, (%rbx)
66 88 23                    data16 mov %ah, (%rbx)
67 88 23                    mov    %ah, (%ebx)
66 67 88 23                 data16 mov %ah, (%ebx)
66 66 67 67 88 23           data16 data16 addr32 mov %ah, (%ebx)
67 67 66 66 88 23           addr32 data16 data16 mov %ah, (%ebx)
67 66 67 66 88 23           addr32 data16 data16 mov %ah, (%ebx)
40 88 23                    mov    %spl, (%rbx)
40                          rex
66 88 23                    data16 mov %ah, (%rbx)
40                          rex
67 88 23                    mov    %ah, (%ebx)
40                          rex
66 67 88 23                 data16 mov %ah, (%ebx)
40                          rex
66 66 67 67 88 23           data16 data16 addr32 mov %ah, (%ebx)
40                          rex
67 67 66 66 88 23           addr32 data16 data16 mov %ah, (%ebx)
40                          rex
67 66 67 66 88 23           addr32 data16 data16 mov %ah, (%ebx)
40 88 23                    mov    %spl, (%rbx)
66 40 88 23                 data16 mov %spl, (%rbx)
67 40 88 23                 mov    %spl, (%ebx)
66 67 40 88 23              data16 mov %spl, (%ebx)
66 66 67 67 40 88 23        data16 data16 addr32 mov %spl, (%ebx)
67 67 66 66 40 88 23        addr32 data16 data16 mov %spl, (%ebx)
67 66 67 66 40 88 23        addr32 data16 data16 mov %spl, (%ebx)
48 88 23                    rex.W mov %spl, (%rbx)
66 48 88 23                 data16 rex.W mov %spl, (%rbx)
67 48 88 23                 rex.W mov %spl, (%ebx)
66 67 48 88 23              data16 rex.W mov %spl, (%ebx)
66 66 67 67 48 88 23        data16 data16 addr32 rex.W mov %spl, (%ebx)
67 67 66 66 48 88 23        addr32 data16 data16 rex.W mov %spl, (%ebx)
67 66 67 66 48 88 23        addr32 data16 data16 rex.W mov %spl, (%ebx)
41 88 23                    mov    %spl, (%r11)
66 41 88 23                 data16 mov %spl, (%r11)
67 41 88 23                 mov    %spl, (%r11d)
66 67 41 88 23              data16 mov %spl, (%r11d)
66 66 67 67 41 88 23        data16 data16 addr32 mov %spl, (%r11d)
67 67 66 66 41 88 23        addr32 data16 data16 mov %spl, (%r11d)
67 66 67 66 41 88 23        addr32 data16 data16 mov %spl, (%r11d)
49 88 23                    rex.WB mov %spl, (%r11)
66 49 88 23                 data16 rex.WB mov %spl, (%r11)
67 49 88 23                 rex.WB mov %spl, (%r11d)
66 67 49 88 23              data16 rex.WB mov %spl, (%r11d)
66 66 67 67 49 88 23        data16 data16 addr32 rex.WB mov %spl, (%r11d)
67 67 66 66 49 88 23        addr32 data16 data16 rex.WB mov %spl, (%r11d)
67 66 67 66 49 88 23        addr32 data16 data16 rex.WB mov %spl, (%r11d)
