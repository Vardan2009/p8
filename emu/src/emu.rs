use crate::opcode::{Opcode};
use crate::reg::{Register};
use crate::inst;

pub struct CPUState {
    pub r0 : u8, // general registers
    pub r1 : u8,
    pub r2 : u8,
    pub r3 : u8,
    pub r4 : u8,
    pub r5 : u8,
    pub ia : u8, // indirect addressing
    pub sp : u8, // stack pointer
    pub pc : u8, // program counter,
    
    pub zflag : bool,
    pub nflag : bool,

    pub progmem : [u16; 256],
    pub ram     : [u8;  128],
    pub rom     : [u8;  128],
}

pub fn reg_to_pointer(reg: Register, cpu_state: &mut CPUState) -> &mut u8 {
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

pub fn mem_read(address : u8, cpu_state: &CPUState) -> u8 {
    if address >= 0x80 {
        return cpu_state.rom[(address - 0x80) as usize];
    }
    return cpu_state.ram[address as usize];
}

pub fn mem_write(address : u8, value : u8, cpu_state: &mut CPUState) {
   if address >= 0x80 {
       print!("ERR: attempted to write above 0x80, which is mapped to ROM\n");
       return;
   }

   cpu_state.ram[address as usize] = value;
}

pub fn step(cpu_state: &mut CPUState) {
    let inst = inst::decode(cpu_state.pc, cpu_state.progmem[cpu_state.pc as usize]);

    print!("executing ");
    inst::print(&inst);

    let mut jflag = false;

    match inst.opcode {
        Opcode::MOV  => *reg_to_pointer(inst.reg1, cpu_state) = *reg_to_pointer(inst.reg2, cpu_state),
        Opcode::LDI  => *reg_to_pointer(inst.reg1, cpu_state) = inst.imm,
        Opcode::LDR  => {
            let value = mem_read(cpu_state.ia, cpu_state);
            *reg_to_pointer(inst.reg1, cpu_state) = value; 
        },
        Opcode::STR => mem_write(cpu_state.ia, *reg_to_pointer(inst.reg2, cpu_state), cpu_state),
        Opcode::JMP  => { cpu_state.pc = cpu_state.ia; jflag = true; },
        Opcode::JZ   => if cpu_state.zflag { cpu_state.pc = cpu_state.ia; jflag = true; },
        Opcode::JN   => if cpu_state.nflag { cpu_state.pc = cpu_state.ia; jflag = true; },

        Opcode::ADD => {
            let rhs = *reg_to_pointer(inst.reg2, cpu_state);
            let lhs = reg_to_pointer(inst.reg1, cpu_state);
            *lhs = lhs.wrapping_add(rhs);
        },
        Opcode::ADDI => {
            let lhs = reg_to_pointer(inst.reg1, cpu_state);
            *lhs = lhs.wrapping_add(inst.imm);
        },
        Opcode::SUB => {
            let rhs = *reg_to_pointer(inst.reg2, cpu_state);
            let lhs = reg_to_pointer(inst.reg1, cpu_state);
            *lhs = lhs.wrapping_sub(rhs);
        },
        Opcode::AND  => *reg_to_pointer(inst.reg1, cpu_state) &= *reg_to_pointer(inst.reg2, cpu_state),
        Opcode::OR   => *reg_to_pointer(inst.reg1, cpu_state) |= *reg_to_pointer(inst.reg2, cpu_state),
        Opcode::XOR  => *reg_to_pointer(inst.reg1, cpu_state) ^= *reg_to_pointer(inst.reg2, cpu_state),
        Opcode::NOT => {
            let ptr = reg_to_pointer(inst.reg1, cpu_state);
            *ptr = !*ptr;
        },
        _ => {}
    }

    match inst.opcode {
        Opcode::ADD | Opcode::ADDI | Opcode::SUB | Opcode::AND |
        Opcode::OR  | Opcode::XOR  | Opcode::NOT => {
            let val = *reg_to_pointer(inst.reg1, cpu_state);

            cpu_state.zflag = val == 0;
            cpu_state.nflag = (val as i8) < 0;
        },
        _ => {}
    }

    if !jflag {
        cpu_state.pc += 1;
    }
}
