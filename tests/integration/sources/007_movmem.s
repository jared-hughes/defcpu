# hole: fibonacci
mov $0x00007FFEA10A6BE0, %rsp
mov $mem, %bx
mov (%rbx), %ah
hlt

.data
mem: .byte 0x12, 0x34, 0x56, 0x78
