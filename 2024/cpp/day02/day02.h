#ifndef DAY02_H
#define DAY02_H
#include <string>
#include <fstream>
#include <vector>
#include <ranges>

#include "../Utils.h"

class day02 {
public:
    static std::string part1(std::ifstream &input) {
        std::string line;
        int res = 0;
        auto parseInt = [](const std::string &x) { return std::stoi(x); };
        while (std::getline(input, line) && !line.empty()) {
            auto nums = Utils::split(line, ' ');
            // I miss map
            std::vector<int> nums2;
            std::ranges::copy(std::ranges::views::transform(nums, parseInt),
                              std::back_inserter(nums2));

            if (isValid(nums2)) {
                res += 1;
            }
        }
        return std::to_string(res);
    }

    static bool isValid(const std::vector<int> &vec) {
        auto valid1 = true;
        for (int i = 0; i < vec.size() - 1; i++) {
            const int diff = vec[i] - vec[i + 1];
            if (!(diff >= 1 && diff <= 3)) {
                valid1 = false;
                break;
            }
        }
        if (valid1) return true;
        auto valid2 = true;
        for (int i = 0; i < vec.size() - 1; i++) {
            const int diff = vec[i + 1] - vec[i];
            if (!(diff >= 1 && diff <= 3)) {
                valid2 = false;
                break;
            }
        }
        return valid2;
    }

    static std::string part2(std::ifstream &input) {
        std::string line;
        int res = 0;
        auto parseInt = [](const std::string &x) { return std::stoi(x); };
        while (std::getline(input, line) && !line.empty()) {
            auto nums = Utils::split(line, ' ');
            std::vector<int> nums2;
            std::ranges::copy(std::ranges::views::transform(nums, parseInt),
                              std::back_inserter(nums2));

            if (isValid(nums2)) {
                res += 1;
            } else {
                bool found = false;
                for (int i = 0; i < nums2.size(); i++) {
                    auto nums3 = nums2;
                    nums3.erase(nums3.begin() + i);
                    if (isValid(nums3)) {
                        found = true;
                        break;
                    }
                }
                if (found) {
                    res += 1;
                }
            }
        }
        return std::to_string(res);
    }
};


#endif //DAY02_H
