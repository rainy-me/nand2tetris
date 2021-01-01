// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

@SCREEN
D=A

@addr
M=D

@black
M=0

(LOOP)
    @KBD
    D=M
    
    @SET_W
    D;JEQ

    @SET_B
    D;JNE

(SET_W)
    @black
    M=0
    
    @FILL
    0;JMP

(SET_B)
    @black
    M=-1
    
    @FILL
    0;JMP


(FILL)
    @SCREEN
    D=A

    @8191
    D=D+A

    @addr
    D=D-M

    @OK
    D;JEQ

    @black
    D=M

    @addr
    A=M
    M=D

    @addr
    M=M+1

    @FILL
    0;JMP

(OK)
    @SCREEN
    D=A

    @addr
    M=D

    @LOOP
    0;JMP
