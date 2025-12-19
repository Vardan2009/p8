#include "expr.h"

#include <cctype>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <string>

#include "symtable.h"

uint8_t Expression::eval(const std::string& expr, const SymbolTable& symbols) {
    int value = 0;
    int sign = 1;
    size_t i = 0;

    while (i < expr.size()) {
        if (expr[i] == '+') {
            sign = +1;
            i++;
        } else if (expr[i] == '-') {
            sign = -1;
            i++;
        }

        size_t start = i;
        while (i < expr.size() && (std::isalnum(expr[i]) || expr[i] == '_'))
            i++;

        std::string tok = expr.substr(start, i - start);
        int v = parseToken(tok, symbols);
        value += sign * v;

        if (value < 0 || value > 0xFF) {
            fprintf(stderr, "p8asm: immediate out of 8-bit range: %s\n",
                    expr.c_str());
            exit(1);
        }
    }
    return static_cast<uint8_t>(value);
}

int Expression::parseToken(const std::string& t, const SymbolTable& symbols) {
    if (t.empty()) {
        fprintf(stderr, "p8asm: empty expression\n");
        exit(1);
    }

    if (t == "$") return symbols.resolve("$");

    if (t.size() > 2 && t[0] == '0' && t[1] == 'x')
        return std::stoi(t, nullptr, 16);

    if (std::isdigit(t[0])) return std::stoi(t);

    return symbols.resolve(t);
}
