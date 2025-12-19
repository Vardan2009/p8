#pragma once

#include <cstdint>

#include "def.h"

class Encoder {
   public:
    static uint16_t encode(const Instruction& i) {
        uint16_t word = static_cast<uint16_t>(i.opcode) << 12;
        if (i.r1) word |= static_cast<uint16_t>(*i.r1) << 9;
        if (i.r2) word |= static_cast<uint16_t>(*i.r2) << 6;
        if (i.imm) word |= *i.imm;
        return word;
    }
};
