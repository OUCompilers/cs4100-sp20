setframe 0
push Lmain
call
halt
Lmain:
_Lmain:
push 1
push 5
push Ltailcallfact
setframe 3
swap
call
ret
Ltailcallfact:
_Ltailcallfact:
push 1
var 1
binary <
push _L1
branch
var 0
var 1
binary *
store 0
push 1
var 1
binary -
store 1
push true
push _Ltailcallfact
branch
push true
push _L2
branch
_L1:
var 0
_L2:
ret
