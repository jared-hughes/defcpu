# hole: fibonacci
SYS_WRITE = 1
SYS_EXIT = 60
STDOUT_FILENO = 1

# Printing
.data
buffer: .string "Hello, World!\n"
bufferLen = . - buffer

.text
mov $SYS_WRITE, %eax
mov $STDOUT_FILENO, %edi
mov $buffer, %esi
mov $bufferLen, %edx
syscall

mov $SYS_EXIT, %eax
mov $0, %edi
syscall
