(Sys.init)
@4000
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@THIS
D=A
@SP
A=M
D=D+M
@SP
A=M
A=D-M
M=D-A
@5000
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@THAT
D=A
@SP
A=M
D=D+M
@SP
A=M
A=D-M
M=D-A
@Sys.main$ret.1
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@5
D=A
@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.main
0;JMP
(Sys.main$ret.1)
@SP
M=M-1
@5
D=A
@1
A=D+A
D=A
@SP
A=M
D=D+M
@SP
A=M
A=D-M
M=D-A
(LOOP)
@LOOP
0;JMP
(Sys.main)
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
@SP
A=M
M=D
@SP
M=M+1
@4001
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@THIS
D=A
@SP
A=M
D=D+M
@SP
A=M
A=D-M
M=D-A
@5001
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@THAT
D=A
@SP
A=M
D=D+M
@SP
A=M
A=D-M
M=D-A
@200
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@1
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
@40
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@2
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
@6
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@3
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
@123
D=A
@SP
A=M
M=D
@SP
M=M+1
@Sys.add12$ret.2
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@6
D=A
@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.add12
0;JMP
(Sys.add12$ret.2)
@SP
M=M-1
@5
D=A
@0
A=D+A
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
@2
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@3
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@4
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
A=M
D=M
@SP
A=M-1
M=M+D
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=M+D
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=M+D
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
(Sys.add12)
@4002
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@THIS
D=A
@SP
A=M
D=D+M
@SP
A=M
A=D-M
M=D-A
@5002
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@THAT
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
@12
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
M=M+D
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