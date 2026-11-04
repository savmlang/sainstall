cmake_minimum_required(VERSION 3.16)
project(SAVM_INSTALLER_WIN32 LANGUAGES CXX C)

# /MT (or, /MTd)
cmake_policy(SET CMP0091 NEW)
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

# C++26
set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(installer_core STATIC
  installer_core/core.c++
  installer_core/miniz.c
)