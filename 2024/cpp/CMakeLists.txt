cmake_minimum_required(VERSION 3.30)
project(cpp)

set(CMAKE_CXX_STANDARD 20)

add_executable(cpp main.cpp
        day01/day01.h
        Utils.h
)
