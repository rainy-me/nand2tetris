(SimpleFunction.test)
@0
                     D=A
@SP
A=M
M=D
@SP
M=M+1
@0
                     D=A
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
@1
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
A=M-1
M=!M
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
A=M
D=M
@SP
A=M-1
M=M+D
@1
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
A=M
D=M
@SP
A=M-1
M=M-D
@SP
M=M-1
@LCL
D=M
@5
A=D-A
D=M
@R13
M=D
@ARG
D=M
@SP
A=M
D=D+M
@SP
A=M
A=D-M
MD=D-A
@ARG
D=M
@SP
M=D+1
@LCL //that
A=M-1
D=M
@THAT
M=D
@2  //this
D=A
@LCL
A=M-D
D=M
@THIS
M=D
@3  //arg
D=A
@LCL
A=M-D
D=M
@ARG
M=D
@4  //lcl
D=A
@LCL
A=M-D
D=M
@LCL
M=D
@R13
A=M
0;JMP
