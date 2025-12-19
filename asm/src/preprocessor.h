#pragma once

#include <string>
#include <unordered_map>
#include <vector>

class Preprocessor {
   public:
    std::vector<std::string> run(const std::vector<std::string>& input);

   private:
    struct Macro {
        std::vector<std::string> params;
        std::vector<std::string> body;
    };

    std::unordered_map<std::string, std::string> defines;
    std::unordered_map<std::string, Macro> macros;

    std::string applyDefines(std::string line) const;

    void expandMacro(const std::string& line,
                     std::vector<std::string>& out) const;
};
