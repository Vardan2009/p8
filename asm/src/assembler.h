#pragma once

#include <cstdint>
#include <string>
#include <vector>

#include "def.h"
#include "symtable.h"
#include "util.h"

class Assembler {
   public:
    std::vector<uint16_t> assemble(const std::vector<std::string>& src);

   private:
    SymbolTable symbols;

    void firstPass(const std::vector<std::string>& src);

    std::vector<uint16_t> secondPass(const std::vector<std::string>& src);

    Instruction parse(const std::string& line);

    static std::string stripComment(std::string s) {
        return trim(s.substr(0, s.find(';')));
    }
};
