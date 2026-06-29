use crate::opcode::{Opcode, opcode_from_bits};
use crate::reg::{Register, reg_from_bits};

#[derive(Debug)]
pub struct Instruction {
    pub address: u8,
    pub assembled: u16,
    pub opcode: Opcode,
    pub reg1: Register,
    pub reg2: Register,
    pub imm: u8,
}

const OPCODE_SHIFT: u16 = 12;
const REG1_SHIFT: u16 = 9;
const REG2_SHIFT: u16 = 6;

const OPCODE_MASK: u16 = 0b1111 << OPCODE_SHIFT;
const REG1_MASK: u16 = 0b111 << REG1_SHIFT;
const REG2_MASK: u16 = 0b111 << REG2_SHIFT;
const IMM_MASK: u16 = 0b1111_1111;

pub fn decode(address: u8, raw: u16) -> Instruction {
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

pub fn print(inst: &Instruction) {
    print!("{:02x} {: <4} ", inst.address, format!("{:?}", inst.opcode));

    match inst.opcode {
        Opcode::MOV => print!("{}  <-  {}", inst.reg1, inst.reg2),
        Opcode::LDI => print!("{}  <-  {:02x}", inst.reg1, inst.imm),
        Opcode::LDR => print!("{}  <- *IA", inst.reg1),
        Opcode::STR => print!("*IA <-  {}", inst.reg2),
        Opcode::ADD | Opcode::SUB | Opcode::AND | Opcode::OR | Opcode::XOR => {
            print!("{}  <-  {}", inst.reg1, inst.reg2)
        }
        Opcode::NOT => print!("{}        ", inst.reg1),
        Opcode::ADDI => print!("{} <-   {:02x}", inst.reg1, inst.imm),
        _ => print!("          "),
    }

    print!("   {:016b}\n", inst.assembled);
}
