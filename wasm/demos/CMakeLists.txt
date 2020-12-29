cmake_minimum_required(VERSION 3.17)
project(wasm C)

set(CMAKE_C_STANDARD 11)

add_executable(wasm
        demos/hello.c)
