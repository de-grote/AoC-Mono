#ifndef UTILS_H
#define UTILS_H
#include <vector>
#include <string>
#include <sstream>
#include <iostream>


class Utils {
public:
    static std::vector<std::string> split(const std::string &s, const char delimiter) {
        std::vector<std::string> tokens;
        std::string token;
        std::istringstream tokenStream(s);
        while (std::getline(tokenStream, token, delimiter)) {
            if (!token.empty())
                tokens.push_back(token);
        }
        return tokens;
    }

    template<typename T>
    static void printVec(const std::vector<T> &vec) {
        std::cout << '[';
        if (!vec.empty()) {
            std::cout << vec[0];
        }
        for (int i = 1; i < vec.size(); i++) {
            std::cout << ',';
            std::cout << vec[i];
        }
        std::cout << "]\n";
    }
};


#endif //UTILS_H
