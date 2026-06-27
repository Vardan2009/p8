use std::fmt;

#[derive(Debug)]
#[repr(u8)]
pub enum Opcode {
    NOP, MOV, LDI, LDR, STR,
    JMP, JZ,  JN,  ADD, ADDI,
    SUB, AND, OR,  XOR, NOT,
    HLT
}

pub fn opcode_from_bits(bits: u8) -> Opcode {
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

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
