#include "assembler.h"

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <string>
#include <vector>

#include "def.h"
#include "encoder.h"
#include "expr.h"
#include "symtable.h"
#include "util.h"

std::vector<uint16_t> Assembler::assemble(const std::vector<std::string>& src) {
    firstPass(src);
    return secondPass(src);
}

void Assembler::firstPass(const std::vector<std::string>& src) {
    int pc = 0;
    for (auto line : src) {
        line = stripComment(line);
        if (line.empty()) continue;

        if (line.back() == ':')
            symbols.define(upper(line.substr(0, line.size() - 1)), pc);
        else
            pc++;
    }
}

std::vector<uint16_t> Assembler::secondPass(
    const std::vector<std::string>& src) {
    std::vector<uint16_t> out;
    int pc = 0;

    for (auto line : src) {
        line = stripComment(line);
        if (line.empty()) continue;
        if (line.back() == ':') continue;

        symbols.define("$", pc);
        Instruction inst = parse(line);
        out.push_back(Encoder::encode(inst));
        pc++;
    }
    return out;
}

Instruction Assembler::parse(const std::string& line) {
    auto parts = split(line);
    auto opstr = upper(parts[0]);

    auto it = OPCODE_MAP.find(opstr);
    if (it == OPCODE_MAP.end()) {
        fprintf(stderr, "p8asm: unknown opcode: %s\n", opstr.c_str());
        exit(1);
    }

    Instruction inst{it->second};

    auto reg = [&](size_t i) { return REG_MAP.at(upper(parts.at(i))); };

    if (opstr == "MOV" || opstr == "ADD" || opstr == "SUB" || opstr == "AND" ||
        opstr == "OR" || opstr == "XOR") {
        inst.r1 = reg(1);
        inst.r2 = reg(2);
    } else if (opstr == "LDI" || opstr == "ADDI") {
        inst.r1 = reg(1);
        uint8_t imm = Expression::eval(parts[2], symbols);
        inst.r2 = static_cast<Register>((imm >> 6) & 0x3);
        inst.imm = imm & 0x3F;
    } else if (opstr == "LDR" || opstr == "NOT") {
        inst.r1 = reg(1);
    } else if (opstr == "STR") {
        inst.r2 = reg(1);
    }
    return inst;
}

static std::string stripComment(std::string s) {
    return trim(s.substr(0, s.find(';')));
}
