#include "def.h"

const std::unordered_map<std::string, Opcode> OPCODE_MAP{
    {"MOV", Opcode::MOV}, {"LDI", Opcode::LDI}, {"LDR", Opcode::LDR},
    {"STR", Opcode::STR}, {"JMP", Opcode::JMP}, {"JZ", Opcode::JZ},
    {"JN", Opcode::JN},   {"ADD", Opcode::ADD}, {"ADDI", Opcode::ADDI},
    {"SUB", Opcode::SUB}, {"AND", Opcode::AND}, {"OR", Opcode::OR},
    {"XOR", Opcode::XOR}, {"NOT", Opcode::NOT}, {"HLT", Opcode::HLT}};

const std::unordered_map<std::string, Register> REG_MAP{
    {"R0", Register::R0}, {"R1", Register::R1}, {"R2", Register::R2},
    {"R3", Register::R3}, {"R4", Register::R4}, {"R5", Register::R5},
    {"IA", Register::IA}, {"SP", Register::SP}};
