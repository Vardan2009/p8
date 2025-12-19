#pragma once

#include <cstdint>
#include <optional>
#include <string>
#include <unordered_map>

enum class Opcode : uint8_t {
    MOV = 0x1,
    LDI = 0x2,
    LDR = 0x3,
    STR = 0x4,
    JMP = 0x5,
    JZ = 0x6,
    JN = 0x7,
    ADD = 0x8,
    ADDI = 0x9,
    SUB = 0xA,
    AND = 0xB,
    OR = 0xC,
    XOR = 0xD,
    NOT = 0xE,
    HLT = 0xF
};

enum class Register : uint8_t { R0 = 0, R1, R2, R3, R4, R5, IA, SP };

struct Instruction {
    Opcode opcode;
    std::optional<Register> r1;
    std::optional<Register> r2;
    std::optional<uint8_t> imm;
};

extern const std::unordered_map<std::string, Opcode> OPCODE_MAP;
extern const std::unordered_map<std::string, Register> REG_MAP;
