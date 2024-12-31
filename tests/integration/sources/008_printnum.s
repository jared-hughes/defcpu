# hole: fibonacci
.comm x, 0xf78000000

mov $0, %rsp # just so test files line up

mov $123456789, %eax

# SUB: print decimal followed by newline
# Requires .comm directive above to let
# (-1_i32 as u32) be accessible memory locations
mov $10, %bl
mov $'\n'-'0', %dl

for_digit_in_num:
add $'0', %dl
dec %esi
# first run through the loop: esi = -1 = FFFF_FFFF.
# following two can be replaced with xchg %dl, (%rsi)
mov %dl, (%rsi)
mov $0, %dl
cmp %eax, %edx
div %ebx
jc for_digit_in_num

# Actually print the result
# sample code to print it
mov $1, %al # or pop %eax if the hole is argumentless
# %eax = 1.
mov %eax, %edi # fd = %rdi
# %rdx = 0
# buf = %rsi
sub %esi, %edx # cnt = rdx
syscall

hlt
