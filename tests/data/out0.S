.global __start
.align 16

__start: 
mov X2, #1
mov X0, X2
mov X2, X0
mov X0, #7
add X2, X2, X0
mov X0, X2
mov X2, X0
mov X0, #4
sdiv X1, X2, X0
smull X1, W1, W0
sub X1, X2, X1
mov X0, X1
mov X3, #5
b l2

l4: 
mov X0, #0
mov X16, #1
svc #128

l1: 
str X29, [SP, #-16]
sub SP, SP, #-16
mov X1, X0
add X3, X0, #48
mov X0, #1
mov X1, SP
sub SP, SP, #-16
str X3, [X1]
mov X2, #1
mov X16, #4
svc #128
ret

l2: 
mov X1, X3
mov X2, #0
cmp X1, X2
cset X1, LT
cbnz X1, l4

l3: 
mov X2, X0
mov X0, X2
bl l1
mov X0, #1
sub X3, X3, X0
b l2