use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    IA,
    SP,
}

pub fn reg_from_bits(bits: u8) -> Register {
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

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
