#include "symtable.h"

#include <cstdio>
#include <cstdlib>

#include "util.h"

void SymbolTable::define(const std::string& name, int value) {
    table[name] = value;
}

int SymbolTable::resolve(const std::string& name) const {
    auto it = table.find(upper(name));
    if (it == table.end()) {
        fprintf(stderr, "p8asm: undefined symbol: %s\n", upper(name).c_str());
        exit(1);
    }
    return it->second;
}

bool SymbolTable::contains(const std::string& name) const {
    return table.count(name) != 0;
}
