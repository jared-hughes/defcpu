# hole: fibonacci
mov $0x00007FFEA10A6BE0, %rsp
mov $mem, %ebp
mov (%ebp), %al
mov (%ebp), %bh
mov (%ebp), %cx
mov (%ebp), %edx
mov (%ebp), %rsi
hlt

.data
mem: .byte 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0
