use std::io::{self, Write, BufRead, BufReader};
use std::fs::File;
use std::fmt;

const OPCODE_SHIFT: u16 = 12;
const REG1_SHIFT: u16 = 9;
const REG2_SHIFT: u16 = 6;
 
const OPCODE_MASK: u16 = 0b1111 << OPCODE_SHIFT;
const REG1_MASK: u16 = 0b111 << REG1_SHIFT;
const REG2_MASK: u16 = 0b111 << REG2_SHIFT;
const IMM_MASK: u16 = 0b1111_1111;
 
fn read_hex_file(path: &str, data: &mut [u16; 256]) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut index = 0;
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        if line_num == 0 && line.starts_with("v2.0") {
            continue;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if index >= data.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "file has more entries than the array can hold",
            ));
        }

        let value = u16::from_str_radix(line, 16).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("line {}: {}", line_num + 1, e))
        })?;

        data[index] = value;
        index += 1;
    }

    Ok(())
}

fn read_hex_file_u8(path: &str, data: &mut [u8; 128]) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut index = 0;
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        if line_num == 0 && line.starts_with("v2.0") {
            continue;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if index >= data.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "file has more entries than the array can hold",
            ));
        }

        let value = u8::from_str_radix(line, 16).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("line {}: {}", line_num + 1, e))
        })?;

        data[index] = value;
        index += 1;
    }

    Ok(())
}

struct CPUState {
    r0 : u8, // general registers
    r1 : u8,
    r2 : u8,
    r3 : u8,
    r4 : u8,
    r5 : u8,
    ia : u8, // indirect addressing
    sp : u8, // stack pointer
    pc : u8, // program counter,
    
    zflag : bool,
    nflag : bool,

    progmem : [u16; 256],
    ram     : [u8;  128],
    rom     : [u8;  128],
}

#[derive(Debug)]
#[repr(u8)]
enum Opcode {
    NOP, MOV, LDI, LDR, STR,
    JMP, JZ,  JN,  ADD, ADDI,
    SUB, AND, OR,  XOR, NOT,
    HLT
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Register {
    R0, R1, R2, R3, R4, R5, IA, SP,
}

fn opcode_from_bits(bits: u8) -> Opcode {
    match bits & 0xF {
        0b0000 => Opcode::NOP,
        0b0001 => Opcode::MOV,
        0b0010 => Opcode::LDI,
        0b0011 => Opcode::LDR,
        0b0100 => Opcode::STR,
        0b0101 => Opcode::JMP,
        0b0110 => Opcode::JZ,
        0b0111 => Opcode::JN,
        0b1000 => Opcode::ADD,
        0b1001 => Opcode::ADDI,
        0b1010 => Opcode::SUB,
        0b1011 => Opcode::AND,
        0b1100 => Opcode::OR,
        0b1101 => Opcode::XOR,
        0b1110 => Opcode::NOT,
        0b1111 => Opcode::HLT,
        _ => Opcode::HLT, 
    }
}

fn reg_from_bits(bits: u8) -> Register {
    match bits & 0x7 {
        0b000 => Register::R0,
        0b001 => Register::R1,
        0b010 => Register::R2,
        0b011 => Register::R3,
        0b100 => Register::R4,
        0b101 => Register::R5,
        0b110 => Register::IA,
        _ => Register::SP,
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Instruction {
    address: u8,
    assembled: u16,
    opcode: Opcode,
    reg1: Register,
    reg2: Register,
    imm: u8,
}

fn inst_decode(address : u8, raw: u16) -> Instruction {
    let opcode_bits = ((raw & OPCODE_MASK) >> OPCODE_SHIFT) as u8;
    let reg1_bits = ((raw & REG1_MASK) >> REG1_SHIFT) as u8;
    let reg2_bits = ((raw & REG2_MASK) >> REG2_SHIFT) as u8;
    let imm = (raw & IMM_MASK) as u8;
 
    Instruction {
        address: address,
        assembled: raw,
        opcode: opcode_from_bits(opcode_bits),
        reg1: reg_from_bits(reg1_bits),
        reg2: reg_from_bits(reg2_bits),
        imm,
    }
}



fn print_instruction(inst : &Instruction) {
    print!("{:02x}\t{} ", inst.address, inst.opcode);

    match inst.opcode {
        Opcode::MOV => print!("{}  <- {}", inst.reg1, inst.reg2),
        Opcode::LDI => print!("{}  <- {:02x}", inst.reg1, inst.imm),
        Opcode::LDR => print!("{}  <- *IA", inst.reg1),
        Opcode::STR => print!("*IA <- {}", inst.reg2),
        Opcode::ADD |
        Opcode::SUB |
        Opcode::AND |
        Opcode::OR  |
        Opcode::XOR => print!("{}  <- {}", inst.reg1, inst.reg2),
        Opcode::NOT => print!("{}", inst.reg1),
        Opcode::ADDI => print!("{} <- {:02x}", inst.reg1, inst.imm),
        _ => {}
    }

    print!("\t\t\t\t{:016b}\n", inst.assembled);
}

fn print_cpu_state(cpu_state: &CPUState) {
    print!("======= CPU STATE =======\n\n");
    print!("======= REGISTERS =======\n");
    print!(" - R0: {:02x}\n", cpu_state.r0);
    print!(" - R1: {:02x}\n", cpu_state.r1);
    print!(" - R2: {:02x}\n", cpu_state.r2);
    print!(" - R3: {:02x}\n", cpu_state.r3);
    print!(" - R4: {:02x}\n", cpu_state.r4);
    print!(" - R5: {:02x}\n", cpu_state.r5);
    print!(" - IA: {:02x}\n", cpu_state.ia);
    print!(" - SP: {:02x}\n", cpu_state.sp);
    print!(" - PC: {:02x}\n", cpu_state.pc);

    print!("\n====== PROGRAM STATE ======\n");

    cpu_view_disasm_nearby(cpu_state);

    print!("\n=========================\n");
}

fn cpu_reg_to_pointer(reg: Register, cpu_state: &mut CPUState) -> &mut u8 {
    match reg {
        Register::R0 => &mut cpu_state.r0,
        Register::R1 => &mut cpu_state.r1,
        Register::R2 => &mut cpu_state.r2,
        Register::R3 => &mut cpu_state.r3,
        Register::R4 => &mut cpu_state.r4,
        Register::R5 => &mut cpu_state.r5,
        Register::IA => &mut cpu_state.ia,
        Register::SP => &mut cpu_state.sp,
    }
}

fn cpu_mem_read(address : u8, cpu_state: &CPUState) -> u8 {
    if address >= 0x80 {
        return cpu_state.rom[(address - 0x80) as usize];
    }
    return cpu_state.ram[address as usize];
}

fn cpu_mem_write(address : u8, value : u8, cpu_state: &mut CPUState) {
   if address >= 0x80 {
       print!("ERR: attempted to write above 0x80, which is mapped to ROM\n");
       return;
   }

   cpu_state.ram[address as usize] = value;
}

fn cpu_step(cpu_state: &mut CPUState) {
    let inst = inst_decode(cpu_state.pc, cpu_state.progmem[cpu_state.pc as usize]);

    print!("executing ");
    print_instruction(&inst);

    let mut jflag = false;

    match inst.opcode {
        Opcode::MOV  => *cpu_reg_to_pointer(inst.reg1, cpu_state) = *cpu_reg_to_pointer(inst.reg2, cpu_state),
        Opcode::LDI  => *cpu_reg_to_pointer(inst.reg1, cpu_state) = inst.imm,
        Opcode::LDR  => {
            let value = cpu_mem_read(cpu_state.ia, cpu_state);
            *cpu_reg_to_pointer(inst.reg1, cpu_state) = value; 
        },
        Opcode::STR => cpu_mem_write(cpu_state.ia, *cpu_reg_to_pointer(inst.reg2, cpu_state), cpu_state),
        Opcode::JMP  => { cpu_state.pc = cpu_state.ia; jflag = true; },
        Opcode::JZ   => if cpu_state.zflag { cpu_state.pc = cpu_state.ia; jflag = true; },
        Opcode::JN   => if cpu_state.nflag { cpu_state.pc = cpu_state.ia; jflag = true; },

        Opcode::ADD => {
            let rhs = *cpu_reg_to_pointer(inst.reg2, cpu_state);
            let lhs = cpu_reg_to_pointer(inst.reg1, cpu_state);
            *lhs = lhs.wrapping_add(rhs);
        },
        Opcode::ADDI => {
            let lhs = cpu_reg_to_pointer(inst.reg1, cpu_state);
            *lhs = lhs.wrapping_add(inst.imm);
        },
        Opcode::SUB => {
            let rhs = *cpu_reg_to_pointer(inst.reg2, cpu_state);
            let lhs = cpu_reg_to_pointer(inst.reg1, cpu_state);
            *lhs = lhs.wrapping_sub(rhs);
        },
        Opcode::AND  => *cpu_reg_to_pointer(inst.reg1, cpu_state) &= *cpu_reg_to_pointer(inst.reg2, cpu_state),
        Opcode::OR   => *cpu_reg_to_pointer(inst.reg1, cpu_state) |= *cpu_reg_to_pointer(inst.reg2, cpu_state),
        Opcode::XOR  => *cpu_reg_to_pointer(inst.reg1, cpu_state) ^= *cpu_reg_to_pointer(inst.reg2, cpu_state),
        Opcode::NOT => {
            let ptr = cpu_reg_to_pointer(inst.reg1, cpu_state);
            *ptr = !*ptr;
        },
        _ => {}
    }

    match inst.opcode {
        Opcode::ADD | Opcode::ADDI | Opcode::SUB | Opcode::AND |
        Opcode::OR  | Opcode::XOR  | Opcode::NOT => {
            let val = *cpu_reg_to_pointer(inst.reg1, cpu_state);

            cpu_state.zflag = val == 0;
            cpu_state.nflag = (val as i8) < 0;
        },
        _ => {}
    }

    if !jflag {
        cpu_state.pc += 1;
    }
}

fn cpu_run(cpu_state: &mut CPUState) {
    while (cpu_state.progmem[cpu_state.pc as usize] & 0xF000) != 0xF000 {
        cpu_step(cpu_state);
    }
}

fn cpu_view_disasm_nearby(cpu_state: &CPUState) {
    let pc = cpu_state.pc as usize;
    let start = pc.saturating_sub(3);
    let end = pc.saturating_add(3);

    for (index, inst) in cpu_state.progmem.iter().enumerate() {
        if index < start || index > end {
            continue;
        }

        let decoded = inst_decode(index as u8, *inst);

        if index as u8 == cpu_state.pc {
            print!("> ");
        }

        print_instruction(&decoded);
    }
}

fn cpu_view_disasm(cpu_state: &CPUState) {
    for (index, inst) in cpu_state.progmem.iter().enumerate() {
        let decoded = inst_decode(index as u8, *inst);
        
        if index as u8 == cpu_state.pc {
            print!("> ");
        }
        
        print_instruction(&decoded);
    }
}

fn unknown_command(cmd: &Vec<&str>) {
    print!("Unknown command `{}`\n", cmd[0]);
}

fn load_file(cmd: &Vec<&str>, cpu_state: &mut CPUState) {
    let _ = read_hex_file(cmd[1], &mut cpu_state.progmem);
}

fn load_rom(cmd: &Vec<&str>, cpu_state: &mut CPUState) {
    let _ = read_hex_file_u8(cmd[1], &mut cpu_state.rom);
}

fn cpu_memdump(cpu_state: &CPUState) {
    for (addr, val) in cpu_state.ram.iter().enumerate() {
        print!("{:02x}: {:02x}\n", addr as u8, val);
    }
    print!("=== ROM AREA\n");
    for (addr, val) in cpu_state.rom.iter().enumerate() {
        print!("{:02x}: {:02x}\n", (addr + 0x80) as u8, val);
    }
}

fn main() {
    let mut cpu_state = CPUState {
        r0: 0,
        r1: 0,
        r2: 0,
        r3: 0,
        r4: 0,
        r5: 0,
        ia: 0,
        sp: 0,
        pc: 0,
        zflag: false,
        nflag: false,
        progmem: [0; 256],
        ram:     [0; 128],
        rom:     [0; 128]
    };

    loop {
        print!("p8emu [{:02x}]> ", cpu_state.pc);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() { continue; }

        let cmd: Vec<&str> = trimmed_input.split_whitespace().collect();

        match cmd[0] {
            "status"  | "st" => print_cpu_state(&cpu_state),
            "step"    | "s"  => cpu_step(&mut cpu_state),
            "run"     | "r"  => cpu_run(&mut cpu_state),
            "disasm"  | "d"  => cpu_view_disasm(&cpu_state),
            "load"    | "l"  => load_file(&cmd, &mut cpu_state),
            "loadrom" | "lr" => load_rom(&cmd, &mut cpu_state),
            "memdump" | "m"  => cpu_memdump(&cpu_state),
            _               => unknown_command(&cmd)
        }
    }
}
