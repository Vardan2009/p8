# P-8
![screenshot](/images/p8.png)
P-8 is an 8-bit RISC with a single-cycle CPU.

## Design

### Instruction structure
```
[(4b opcode)(3b reg1)(3b reg2)(6b imm/addr)]
```
_(the lower 2 bits of reg2 are used for imm8, since all instructions either use 2 registers or a register and immediate)_

### Instruction table
| opcode (bin) | mnemonic | explanation | control signals |
| ------------ |  ------------ |  ------------ |  ------------ | 
| `0000` | `NOP` | `<none>` | `rwe: L rwd: 0 ainb: 0 rstr: L jmp: L` |
| `0001` | `MOV` | `R1 <- R2` | `rwe: H rwd: 2 ainb: 0 rstr: L jmp: L` |
| `0010` | `LDI` | `R1 <- IMM8` | `rwe: H rwd: 0 ainb: 0 rstr: L jmp: L` |
| `0011` | `LDR` | `R1 <- RAM[IA]` | `rwe: H rwd: 3 ainb: 0 rstr: L jmp: L` |
| `0100` | `STR` | `RAM[IA] <- R2` | `rwe: L rwd: 0 ainb: 0 rstr: H jmp: L` |
| `0101` | `JMP` | `PC <- IA` | `rwe: L rwd: 0 ainb: 0 rstr: L jmp: H` |
| `0110` | `JZ` | `PC <- IA if Z` | `rwe: L rwd: 0 ainb: 0 rstr: L jmp: Z` |
| `0111` | `JN` | `PC <- IA if N` | `rwe: L rwd: 0 ainb: 0 rstr: L jmp: N` |
| `1000` | `ADD` | `R1 <- R1 + R2` | `rwe: H rwd: 1 ainb: 2 rstr: L jmp: L` |
| `1001` | `ADDI` | `R1 <- R1 + IMM8` | `rwe: H rwd: 1 ainb: 0 rstr: L jmp: L` |
| `1010` | `SUB` | `R1 <- R1 - R2` | `rwe: H rwd: 1 ainb: 2 rstr: L jmp: L` |
| `1011` | `AND` | `R1 <- R1 - R2` | `rwe: H rwd: 1 ainb: 2 rstr: L jmp: L` |
| `1100` | `OR` | `R1 <- R1 - R2` | `rwe: H rwd: 1 ainb: 2 rstr: L jmp: L` |
| `1101` | `XOR` | `R1 <- R1 - R2` | `rwe: H rwd: 1 ainb: 2 rstr: L jmp: L` |
| `1110` | `NOT` | `R1 <- R1 - R2` | `rwe: H rwd: 1 ainb: 2 rstr: L jmp: L` |
| `1111` | `HLT` | `<stops clock>` | `rwe: L rwd: 0 ainb: 0 rstr: L jmp: L` |

### Control ROM structure

The control ROM stores the control signals based on the CPU's state (opcode, zero flag, negative flag)

#### Address structure
```
[(2b unused)(4b opcode)(1b Z)(1b N)]
```

#### Control word structure
```
[(1b rwe)(2b rwd) (2b ainb) (1b rstr) (1b jmp) (1b unused)]
```
| name | explanation |
| ---- | ----------- |
| `rwe` | `regs: write enable` |
| `rwd` | `regs: write data selector` |
| `ainb` | `alu: b input selector` |
| `rstr` | `ram: store flag` |
| `jmp` | `counter: jump to IA flag` |

### Registers

| mnemonic | notes |
| -------- | ----- |
| `R0` | general-purpose |
| `R1` | 
| `R2` | 
| `R3` | 
| `R4` | 
| `R5` | 
| `IA` | indirect addressing |
| `SP` | stack pointer |

## Assembler

This repository also contains an assembler written in C++ that can be used to compile programs for the P-8.

## `romgen.py`

This can be used to generate the control signals stored in the control ROM
