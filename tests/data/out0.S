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
mov X4, #1
mov X3, X0
b l3

l8: 
ret

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
mov X1, X3
mov X2, #0
cmp X1, X2
cset X1, GT
mvn X1, X1
cbnz X1, l5

l4: 
mov X0, #10
sdiv X3, X3, X0
mov X0, #10
smull X4, W4, W0
b l3

l5: 

l6: 
mov X1, X4
mov X2, #0
cmp X1, X2
cset X1, LE
cbnz X1, l8

l7: 
mov X1, X4
sdiv X0, X0, X1
bl l2
mov X0, #10
sdiv X4, X4, X0
b l6