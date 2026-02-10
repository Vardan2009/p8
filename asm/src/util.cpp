#include "util.h"

#include <cctype>
#include <string>
#include <vector>

std::string trim(std::string s) {
    size_t a = s.find_first_not_of(" \t");
    size_t b = s.find_last_not_of(" \t");
    return (a == std::string::npos) ? "" : s.substr(a, b - a + 1);
}

std::string upper(std::string s) {
    for (char& c : s) c = std::toupper(c);
    return s;
}

std::vector<std::string> split(const std::string& s) {
    std::vector<std::string> tokens;
    std::string current;
    bool inQuotes = false;

    for (char c : s) {
        if (c == '"') {
            inQuotes = !inQuotes;
        } else if (!inQuotes &&
                   (std::isspace(static_cast<unsigned char>(c)) || c == ',')) {
            if (!current.empty()) {
                tokens.push_back(current);
                current.clear();
            }
        } else {
            current.push_back(c);
        }
    }

    if (!current.empty()) {
        tokens.push_back(current);
    }

    return tokens;
}
