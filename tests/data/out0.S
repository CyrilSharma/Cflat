.global __start
.align 4

__start: 
mov X0, #1000
mov X0, X1
bl l1
mov X16, #0
svc #0

l1: 
str X29, [SP, #-16]
sub SP, SP, #-16
mov X2, X0
mov X0, X2
mov X0, #0
cmp X0, X0
cset X0, EQ
cbnz X0, l3

l4: 
mov X0, X2
mov X1, #10
sdiv X0, X0, X1
bl l1
mov X0, #10
sdiv X1, X2, X0
smsubl X0, W2, W1, X0
bl l2

l2: 
str X29, [SP, #-16]
sub SP, SP, #-16
mov X1, X0
add X1, X0, #41
mov X0, #1
mov X1, X0
mov X2, #1
mov X16, #4
svc #128
ret

l3: 
ret