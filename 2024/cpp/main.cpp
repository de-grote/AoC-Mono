#pragma once
#include <fstream>
#include <iostream>

#include "day01/day01.h"

int main(int argc, char **argv) {
    auto file = std::ifstream("day01/input.txt");

    int day = 0;
    int part = 1;
    if (argc >= 3) {
        day = std::atoi(argv[1]);
        part = std::atoi(argv[2]);
    }

    switch (day) {
        case 1:
            if (part == 1) {
                std::cout << day01::part1(file);
            } else {
                std::cout << day01::part2(file);
            }
            break;
        default:
            day = 1;
            part = 2;
            std::cout << day01::part2(file);
            break;
    }
    std::cout << "\nsolved day " << day << " part " << part << std::endl;

    return 0;
}
