cmake_minimum_required(VERSION 3.22)
project(r21_thread_cpp)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS -pthread)

add_executable(r21_thread_cpp main.cpp)
