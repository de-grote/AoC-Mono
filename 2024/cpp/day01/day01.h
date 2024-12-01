#ifndef DAY01_H
#define DAY01_H
#include <algorithm>
#include <string>
#include <fstream>
#include <vector>
#include <sstream>
#include <map>
#include "Utils.h"


class day01 {
public:
    static std::string part1(std::ifstream &input) {
        std::string line;
        std::vector<int> as;
        std::vector<int> bs;
        while (std::getline(input, line) && !line.empty()) {
            auto nums = Utils::split(line, ' ');
            as.emplace_back(std::stoi(nums[0]));
            bs.emplace_back(std::stoi(nums[1]));
        }
        std::ranges::sort(as);
        std::ranges::sort(bs);
        int res = 0;
        for (int i = 0; i < as.size(); i++) {
            res += std::abs(as[i] - bs[i]);
        }
        return std::to_string(res);
    }

    static std::string part2(std::ifstream &input) {
        std::string line;
        std::map<int, int> as;
        std::map<int, int> bs;
        while (std::getline(input, line) && !line.empty()) {
            auto nums = Utils::split(line, ' ');
            as[std::stoi(nums[0])] += 1;
            bs[std::stoi(nums[1])] += 1;
        }
        int res = 0;
        for (auto [key, value]: as) {
            res += key * value * bs[key];
        }
        return std::to_string(res);
    }
};

#endif //DAY01_H
