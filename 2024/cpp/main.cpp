#include <fstream>
#include <iostream>

#include "day01/day01.h"
#include "day02/day02.h"
#include "day03/day03.h"
#include "day04/day04.h"

int main(const int argc, char **argv) {
    int day = 4;
    int part = 2;
    if (argc >= 3) {
        day = std::atoi(argv[1]);
        part = std::atoi(argv[2]);
    }

    std::string filename = "day";
    if (day <= 9) filename.push_back('0');
    filename.append(std::to_string(day));
    filename.append("/input.txt");
    auto file = std::ifstream(filename);

    switch (day) {
        case 1:
            if (part == 1) {
                std::cout << day01::part1(file);
            } else {
                std::cout << day01::part2(file);
            }
            break;
        case 2:
            if (part == 1) {
                std::cout << day02::part1(file);
            } else {
                std::cout << day02::part2(file);
            }
            break;
        case 3:
            if (part == 1) {
                std::cout << day03::part1(file);
            } else {
                std::cout << day03::part2(file);
            }
            break;
        case 4:
            if (part == 1) {
                std::cout << day04::part1(file);
            } else {
                std::cout << day04::part2(file);
            }
            break;
        default:
            day = 2;
            part = 1;
            std::cout << day02::part1(file);
            break;
    }
    std::cout << "\nsolved day " << day << " part " << part << std::endl;

    return 0;
}
