cmake_minimum_required(VERSION 3.5)
project(cpp)

set(CMAKE_CXX_STANDARD 20)

add_executable(cpp main.cpp
        day01/day01.h
        Utils.h
        day02/day02.h
        day03/day03.h
        day04/day04.h
)
