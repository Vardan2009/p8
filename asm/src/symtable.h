#pragma once

#include <string>
#include <unordered_map>

class SymbolTable {
   public:
    void define(const std::string& name, int value);

    int resolve(const std::string& name) const;

    bool contains(const std::string& name) const;

   private:
    std::unordered_map<std::string, int> table;
};
