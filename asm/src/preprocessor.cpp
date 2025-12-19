#include "preprocessor.h"

#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

#include "util.h"

std::vector<std::string> Preprocessor::run(
    const std::vector<std::string>& input) {
    std::vector<std::string> output;

    for (size_t i = 0; i < input.size(); ++i) {
        std::string line = trim(input[i]);
        if (line.empty() || line[0] == ';') continue;

        auto tokens = split(line);

        if (tokens.size() < 1) continue;

        if (upper(tokens[0]) == ".DEFINE") {
            if (tokens.size() != 3) {
                fprintf(stderr, "p8asm: invalid .define\n");
                exit(1);
            }
            defines[tokens[1]] = tokens[2];
            continue;
        }

        if (upper(tokens[0]) == ".MACRO") {
            if (tokens.size() < 2) {
                fprintf(stderr, "p8asm: invalid .macro\n");
                exit(1);
            }

            Macro m;
            std::string name = tokens[1];
            m.params.assign(tokens.begin() + 2, tokens.end());

            while (++i < input.size()) {
                auto body = trim(input[i]);
                if (upper(body) == ".ENDMACRO") break;
                m.body.push_back(body);
            }
            macros[name] = m;
            continue;
        }

        line = applyDefines(line);
        expandMacro(line, output);
    }
    return output;
}

std::string Preprocessor::applyDefines(std::string line) const {
    auto tokens = split(line);
    for (auto& t : tokens) {
        auto it = defines.find(t);
        if (it != defines.end()) t = it->second;
    }

    std::ostringstream oss;
    for (size_t i = 0; i < tokens.size(); ++i) {
        if (i) oss << ' ';
        oss << tokens[i];
    }
    return oss.str();
}

void Preprocessor::expandMacro(const std::string& line,
                               std::vector<std::string>& out) const {
    auto tokens = split(line);
    auto it = macros.find(tokens[0]);
    if (it == macros.end()) {
        out.push_back(line);
        return;
    }

    const Macro& m = it->second;
    if (tokens.size() - 1 != m.params.size()) {
        fprintf(stderr, "p8asm: macro arg mismatch\n");
        exit(1);
    }

    std::unordered_map<std::string, std::string> subst;
    for (size_t i = 0; i < m.params.size(); ++i)
        subst[m.params[i]] = tokens[i + 1];

    for (auto bodyLine : m.body) {
        auto bodyTokens = split(bodyLine);
        for (auto& t : bodyTokens) {
            auto s = subst.find(t);
            if (s != subst.end()) t = s->second;
        }

        std::ostringstream oss;
        for (size_t i = 0; i < bodyTokens.size(); ++i) {
            if (i) oss << ' ';
            oss << bodyTokens[i];
        }
        out.push_back(oss.str());
    }
}
