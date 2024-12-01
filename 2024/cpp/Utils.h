#ifndef UTILS_H
#define UTILS_H
#include <vector>
#include <string>
#include <sstream>


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
};


#endif //UTILS_H
