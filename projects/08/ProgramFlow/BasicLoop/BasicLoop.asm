@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@0
D=A
@LCL
A=D+M
D=A
@SP
A=M
D=D+M
@SP
A=M
A=D-M
M=D-A
(LOOP_START)
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=M+D
@SP
M=M-1
@0
D=A
@LCL
A=D+M
D=A
@SP
A=M
D=D+M
@SP
A=M
A=D-M
M=D-A
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=M-D
@SP
M=M-1
@0
D=A
@ARG
A=D+M
D=A
@SP
A=M
D=D+M
@SP
A=M
A=D-M
M=D-A
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@SP
A=M
D=M
@LOOP_START
D;JNE
@0
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1