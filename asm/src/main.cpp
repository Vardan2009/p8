#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <vector>

#include "assembler.h"
#include "preprocessor.h"

int main(int argc, char** argv) {
    if (argc != 2) {
        fprintf(stderr, "p8asm: usage: p8asm <file>\n");
        return 1;
    }

    const char* path = argv[1];

    std::ifstream in(path);
    if (!in) {
        fprintf(stderr, "p8asm: cannot open input file: %s\n", path);
        exit(1);
    }

    std::vector<std::string> src;
    std::string line;
    while (std::getline(in, line)) src.push_back(line);

    Preprocessor pp;
    auto expanded = pp.run(src);

    Assembler asmblr;
    auto code = asmblr.assemble(expanded);

    std::ofstream out(std::string(path) + ".hex");
    out << "v2.0 raw\n";
    for (auto w : code)
        out << std::uppercase << std::hex << std::setw(4) << std::setfill('0')
            << w << "\n";
}
