#pragma once

#include <cstdint>
#include <string>

#include "symtable.h"

class Expression {
   public:
    static uint8_t eval(const std::string& expr, const SymbolTable& symbols);

   private:
    static int parseToken(const std::string& t, const SymbolTable& symbols);
};
