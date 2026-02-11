; adds numbers in rom until 0
.ROM BYTE START 1
.ROM BYTE _____ 2
.ROM BYTE _____ 3
.ROM BYTE _____ 4
.ROM BYTE _____ 5
.ROM BYTE _____ 6
.ROM BYTE _____ 7
.ROM BYTE _____ 8
.ROM BYTE _____ 9
.ROM BYTE _____ 10
.ROM BYTE _____ 9
.ROM BYTE _____ 8
.ROM BYTE _____ 7
.ROM BYTE _____ 6
.ROM BYTE _____ 5
.ROM BYTE _____ 4
.ROM BYTE _____ 3
.ROM BYTE _____ 2
.ROM BYTE _____ 1
.ROM BYTE _____ 0

.DEFINE PTR R2
.DEFINE CUR R0
.DEFINE SUM R1

LDI CUR, 0
LDI SUM, 0
LDI PTR, START

ADDLOOP:
  MOV IA, PTR
  LDR CUR

  ADD SUM, CUR

  ADDI PTR, 1

  LDI IA, END

  ADDI CUR, 0
  JZ

  LDI IA, ADDLOOP
  JMP

END:
  HLT
