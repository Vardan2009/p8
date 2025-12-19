.DEFINE N 10

LDI R0, 0
LDI R1, 1
LDI R3, 0
LDI R4, N

LOOP_START:
    MOV IA, R3
    STR R0

    LDI R2, 0

    ADD R2, R0
    ADD R2, R1
    MOV R0, R1
    MOV R1, R2

    ADDI R3, 1


    MOV R5, R4
    SUB R5, R3
    LDI IA, HALT
    JZ

    LDI IA, LOOP_START
    JMP

HALT:
    HLT
