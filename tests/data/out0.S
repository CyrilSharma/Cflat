.global __start
.align 16

__start: 
mov X0, #1000
bl l1
mov X0, #0
mov X16, #1
svc #128

l1: 
str X29, [SP, #-16]
sub SP, SP, #-16
mov X2, X0
mov X0, X2
mov X1, #0
cmp X0, X1
cset X0, EQ
cbnz X0, l3

l4: 
mov X0, X2
mov X1, #10
sdiv X0, X0, X1
bl l1
mov X0, #10
sdiv X1, X2, X0
smull X1, W1, W0
sub X1, X2, X1
mov X0, X1
bl l2

l2: 
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

l3: 
ret