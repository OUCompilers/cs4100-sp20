setframe 0
push Lmain
call
halt
Lmain:
_Lmain:
push 1000
push Lf
setframe 2
swap
call
ret
Lf:
_Lf:
push 1
var 0
binary <
push _L1
branch
push 1
var 0
binary -
store 0
push true
push _Lf
branch
push true
push _L2
branch
_L1:
push 0
_L2:
ret
