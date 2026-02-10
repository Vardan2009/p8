; define single byte
.ROM BYTE   NUMBER 123
.ROM ASCII  MSG    "Hello, "
.ROM ASCII  MSG1   "World"
.ROM ASCIIZ MSG2   "!"

LDI IA, NUMBER
LDR R0

HLT
