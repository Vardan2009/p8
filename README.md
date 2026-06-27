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
| opcode (bin) | mnemonic | action | `rwe` | `rwd` | `ainb` | `rstr` | `jmp` |
| ------------ | -------- | ------ | :---: | :---: | :----: | :----: | :---: |
| `0000` | `NOP`  | `<none>`               | L | 0 | 0 | L | L |
| `0001` | `MOV`  | `Ra <- Rb`             | H | 2 | 0 | L | L |
| `0010` | `LDI`  | `Ra <- IMM8`           | H | 0 | 0 | L | L |
| `0011` | `LDR`  | `Ra <- MEM[IA]`        | H | 3 | 0 | L | L |
| `0100` | `STR`  | `MEM[IA] <- Rb`        | L | 0 | 0 | H | L |
| `0101` | `JMP`  | `PC <- IA`             | L | 0 | 0 | L | H |
| `0110` | `JZ`   | `PC <- IA if Z`        | L | 0 | 0 | L | Z |
| `0111` | `JN`   | `PC <- IA if N`        | L | 0 | 0 | L | N |
| `1000` | `ADD`  | `Ra <- Ra + Rb`        | H | 1 | 2 | L | L |
| `1001` | `ADDI` | `Ra <- Ra + IMM8`      | H | 1 | 0 | L | L |
| `1010` | `SUB`  | `Ra <- Ra - Rb`        | H | 1 | 2 | L | L |
| `1011` | `AND`  | `Ra <- Ra & Rb`        | H | 1 | 2 | L | L |
| `1100` | `OR`   | `Ra <- Ra \| Rb`       | H | 1 | 2 | L | L |
| `1101` | `XOR`  | `Ra <- Ra ^ Rb`        | H | 1 | 2 | L | L |
| `1110` | `NOT`  | `Ra <- ~Ra`            | H | 1 | 2 | L | L |
| `1111` | `HLT`  | `<stops clock>`        | L | 0 | 0 | L | L |

_(`Ra`/`Rb` denote whatever registers are encoded in the `reg1`/`reg2` instruction fields — not the specific registers `R1`/`R2`. `jmp: Z`/`N` aren't literal control values; see the note under Control word structure below.)_

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
| `jmp` | `counter: jump to address stored in IA` |

`rwd` selects what's written to the register file when `rwe` is high:
| value | source |
| :---: | ------ |
| `0` | immediate (`IMM8`) |
| `1` | ALU output |
| `2` | other register (`Rb`) |
| `3` | memory (`MEM[IA]`) |

`ainb` selects the ALU's second operand:
| value | source |
| :---: | ------ |
| `0` | immediate (`IMM8`) |
| `2` | other register (`Rb`) |

_(values `1` and `3` are unused by the current instruction set)_

`jmp` isn't a literal stored value for `JZ`/`JN` -- since `Z` and `N` are part of the ROM address rather than the control word, the ROM simply stores `jmp: H` at the address rows where the relevant flag bit is `1`, and `jmp: L` where it's `0`. That's what `jmp: Z` / `jmp: N` denote in the instruction table above.

### Registers

| mnemonic | notes |
| -------- | ----- |
| `R0`-`R5` | general-purpose |
| `IA` | indirect addressing |
| `SP` | stack pointer |

### Addresses

There are two memory devices that a P-8 program can use -- RAM and ROM. 
ROM values are pre-defined for each program and are (obviously) readonly.

Address ranges:
```
0x00-0x7F -- RAM, read/write
0x80-0xFF -- ROM, readonly
```

## What this repository contains

Everything needed to design, assemble, run and debug programs for the P-8.

### Circuit simulation - `/dig/`

The P-8 is modelled and simulated in [Digital](https://github.com/hneemann/digital), a logic circuit designer.

### Assembler - `p8asm`

A C++ assembler for the P-8's custom assembly syntax. Example programs: `/asm/programs/`.

### Emulator - `p8emu`

A Rust emulator for the P-8 with a debugger-like interface for inspecting and controlling execution.

### ROM generator - `romgen.py`

Generates the signal data stored in the control ROM.
