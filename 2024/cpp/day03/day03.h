#ifndef DAY03_H
#define DAY03_H

#include <string>
#include <fstream>
#include <regex>

class day03 {
public:
    static std::string part1(std::ifstream &input) {
        std::string line;
        std::getline(input, line, '\0');
        const std::regex mul(R"(mul\((\d{1,3}),(\d{1,3})\))");
        int res = 0;
        // I have no clue how or why these next 4 lines work
        auto it = std::sregex_iterator(line.begin(), line.end(), mul);
        auto end = std::sregex_iterator();
        for (std::sregex_iterator i = it; i != end; ++i) {
            const std::smatch &match = *i;

            auto fst = std::stoi(match[1].str());
            auto snd = std::stoi(match[2].str());
            res += fst * snd;
        }

        return std::to_string(res);
    }

    static std::string part2(std::ifstream &input) {
        std::string line;
        std::getline(input, line, '\0');
        const std::regex mul(R"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))");
        int res = 0;
        bool enable = true;

        auto it = std::sregex_iterator(line.begin(), line.end(), mul);
        auto end = std::sregex_iterator();
        for (std::sregex_iterator i = it; i != end; ++i) {
            const std::smatch &match = *i;

            if (match.str() == "do()") {
                enable = true;
            } else if (match.str() == "don't()") {
                enable = false;
            } else if (enable) {
                auto fst = std::stoi(match[1].str());
                auto snd = std::stoi(match[2].str());
                res += fst * snd;
            }
        }

        return std::to_string(res);
    }
};


#endif //DAY03_H
