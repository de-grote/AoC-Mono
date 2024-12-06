#ifndef DAY04_H
#define DAY04_H

#include <string>
#include <fstream>
#include <vector>


class day04 {
public:
    static std::string part1(std::ifstream &input) {
        std::string line;
        std::vector<std::vector<char> > grid;

        while (std::getline(input, line) && !line.empty()) {
            grid.emplace_back(line.cbegin(), line.cend());
        }
        int res = 0;

        const auto height = grid.size();
        for (int y = 0; y < height; y++) {
            const auto width = grid[y].size();
            for (int x = 0; x < width; x++) {
                for (int dir = 0; dir < 8; dir++) {
                    // 0 1 2
                    // 3 _ 4
                    // 5 6 7
                    const int dx = dir == 0 || dir == 3 || dir == 5 ? -1 : dir == 1 || dir == 6 ? 0 : 1;
                    const int dy = dir <= 2 ? -1 : dir <= 4 ? 0 : 1;
                    int px = x;
                    int py = y;
                    bool ok = true;
                    for (const char letter: "XMAS") {
                        if (letter == '\0') break;
                        if (px >= 0 && px < width && py >= 0 && py < height) {
                            if (grid[py][px] != letter) {
                                ok = false;
                                break;
                            }
                            px += dx;
                            py += dy;
                        } else {
                            ok = false;
                            break;
                        }
                    }
                    if (ok) res++;
                }
            }
        }

        return std::to_string(res);
    }

    static std::string part2(std::ifstream &input) {
        std::string line;
        std::vector<std::vector<char> > grid;

        while (std::getline(input, line) && !line.empty()) {
            grid.emplace_back(line.cbegin(), line.cend());
        }
        int res = 0;

        const auto height = grid.size();
        for (int y = 1; y + 1 < height; y++) {
            const auto width = grid[y].size();
            for (int x = 1; x + 1 < width; x++) {
                if (grid[y][x] != 'A') continue;
                const int xl = x - 1;
                const int xr = x + 1;
                const int yu = y - 1;
                const int yd = y + 1;
                const char lu = grid[yu][xl];
                const char rd = grid[yd][xr];
                const char ru = grid[yu][xr];
                const char ld = grid[yd][xl];
                if ((lu == 'M' && rd == 'S' || lu == 'S' && rd == 'M') &&
                    (ru == 'M' && ld == 'S' || ru == 'S' && ld == 'M'))
                    res++;
            }
        }

        return std::to_string(res);
    }
};


#endif //DAY04_H
