88 23               mov    %ah, (%rbx)
66 88 23            data16 mov %ah, (%rbx)
67 88 23            mov    %ah, (%ebx)
66 67 88 23         data16 mov %ah, (%ebx)
40 88 23            mov    %spl, (%rbx)
67 40 88 23         mov    %spl, (%ebx)
66 40 88 23         data16 mov %spl, (%rbx)
66 67 40 88 23      data16 mov %spl, (%ebx)
41 88 23            mov    %spl, (%r11)
67 41 88 23         mov    %spl, (%r11d)
66 41 88 23         data16 mov %spl, (%r11)
66 67 41 88 23      data16 mov %spl, (%r11d)
48 88 23            rex.W mov %spl, (%rbx)
67 48 88 23         rex.W mov %spl, (%ebx)
66 48 88 23         data16 rex.W mov %spl, (%rbx)
66 67 48 88 23      data16 rex.W mov %spl, (%ebx)
49 88 23            rex.WB mov %spl, (%r11)
67 49 88 23         rex.WB mov %spl, (%r11d)
66 49 88 23         data16 rex.WB mov %spl, (%r11)
66 67 49 88 23      data16 rex.WB mov %spl, (%r11d)
88 c0               mov    %al, %al
88 c1               mov    %al, %cl
88 c2               mov    %al, %dl
88 c3               mov    %al, %bl
88 c4               mov    %al, %ah
88 c5               mov    %al, %ch
88 c6               mov    %al, %dh
88 c7               mov    %al, %bh
66 88 c1            data16 mov %al, %cl
67 88 c1            addr32 mov %al, %cl
66 67 88 c1         data16 addr32 mov %al, %cl
40 88 c1            rex mov %al, %cl
67 40 88 c1         addr32 rex mov %al, %cl
66 40 88 c1         data16 rex mov %al, %cl
66 67 40 88 c1      data16 addr32 rex mov %al, %cl
41 88 c1            mov    %al, %r9b
67 41 88 c1         addr32 mov %al, %r9b
66 41 88 c1         data16 mov %al, %r9b
66 67 41 88 c1      data16 addr32 mov %al, %r9b
48 88 c1            rex.W mov %al, %cl
67 48 88 c1         addr32 rex.W mov %al, %cl
66 48 88 c1         data16 rex.W mov %al, %cl
66 67 48 88 c1      data16 addr32 rex.W mov %al, %cl
49 88 c1            rex.WB mov %al, %r9b
67 49 88 c1         addr32 rex.WB mov %al, %r9b
66 49 88 c1         data16 rex.WB mov %al, %r9b
66 67 49 88 c1      data16 addr32 rex.WB mov %al, %r9b
8a 23               mov    (%rbx), %ah
66 8a 23            data16 mov (%rbx), %ah
67 8a 23            mov    (%ebx), %ah
66 67 8a 23         data16 mov (%ebx), %ah
40 8a 23            mov    (%rbx), %spl
67 40 8a 23         mov    (%ebx), %spl
66 40 8a 23         data16 mov (%rbx), %spl
66 67 40 8a 23      data16 mov (%ebx), %spl
41 8a 23            mov    (%r11), %spl
67 41 8a 23         mov    (%r11d), %spl
66 41 8a 23         data16 mov (%r11), %spl
66 67 41 8a 23      data16 mov (%r11d), %spl
48 8a 23            rex.W mov (%rbx), %spl
67 48 8a 23         rex.W mov (%ebx), %spl
66 48 8a 23         data16 rex.W mov (%rbx), %spl
66 67 48 8a 23      data16 rex.W mov (%ebx), %spl
49 8a 23            rex.WB mov (%r11), %spl
67 49 8a 23         rex.WB mov (%r11d), %spl
66 49 8a 23         data16 rex.WB mov (%r11), %spl
66 67 49 8a 23      data16 rex.WB mov (%r11d), %spl
