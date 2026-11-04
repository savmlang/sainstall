cmake_minimum_required(VERSION 3.24)
cmake_policy(SET CMP0135 NEW)

project(SAVM_INSTALLER_WIN32 LANGUAGES CXX C)

include(FetchContent)

# Get the Rust Target Triple
if(DEFINED ENV{SAVM_TARGET_TRIPLE} AND NOT "$ENV{SAVM_TARGET_TRIPLE}" STREQUAL "")
  set(TRIPLE "$ENV{SAVM_TARGET_TRIPLE}")
else()
  set(TRIPLE "x86_64-pc-windows-msvc")
  message(WARNING "SAVM_TARGET_TRIPLE was not specified. Defaulting to x64")
endif()

set(CLI "savm_installer_$ENV{SAVM_TARGET_TRIPLE}_cli")
set(GUI "savm_installer_$ENV{SAVM_TARGET_TRIPLE}_gui")

# coreinstaller
FetchContent_Declare(
  coreinstaller
  URL "https://github.com/savmlang/sainstall/releases/latest/download/coreinstaller_${TRIPLE}.zip"
)

# slint
FetchContent_Declare(
  slint
  URL "https://github.com/savmlang/sainstall/releases/latest/download/${TRIPLE}.zip"
)

# /MT (or, /MTd)
cmake_policy(SET CMP0091 NEW)
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

# C++ Latest
if (MSVC)
  unset(CMAKE_CXX_STANDARD)
  add_compile_options(/std:c++latest)
else()
  message(FATAL_ERROR "SaVM Win32 installer is only configured for MSVC")
endif()

# Make coreinstaller available
FetchContent_MakeAvailable(coreinstaller)

# Import coreinstaller
add_library(CoreInstaller::Static STATIC IMPORTED GLOBAL)
set_target_properties(CoreInstaller::Static PROPERTIES
  IMPORTED_LOCATION "${coreinstaller_SOURCE_DIR}/coreinstaller.lib"
)

# Make slint available
FetchContent_MakeAvailable(slint)

# Import slint
add_library(Slint::Static STATIC IMPORTED GLOBAL)
set_target_properties(Slint::Static PROPERTIES
  IMPORTED_LOCATION "${slint_SOURCE_DIR}/lib/slint_cpp.lib"
  INTERFACE_INCLUDE_DIRECTORIES "${slint_SOURCE_DIR}/include"
)

set(SLINT_COMPILER_BIN "${slint_SOURCE_DIR}/bin/slint-compiler.exe")

add_executable(Slint::Compiler IMPORTED GLOBAL)
set_target_properties(Slint::Compiler PROPERTIES
  IMPORTED_LOCATION "${SLINT_COMPILER_BIN}"
)

# SaVM CLI Installer
add_executable(savm_cli
  installer/cli.c++
)

target_link_libraries(savm_cli
  PRIVATE CoreInstaller::Static
)
set_target_properties(savm_cli PROPERTIES OUTPUT_NAME ${CLI})

# SaVM GUI Installer
file(MAKE_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/generated")
set(SLINT_UI_FILE "${CMAKE_CURRENT_SOURCE_DIR}/ui/index.slint")
set(SLINT_GENERATED_HEADER "${CMAKE_CURRENT_BINARY_DIR}/generated/main_slint.h")

add_custom_command(
  OUTPUT "${SLINT_GENERATED_HEADER}"
  COMMAND Slint::Compiler
    "${SLINT_UI_FILE}"
    --output "${SLINT_GENERATED_HEADER}"
    -I "${CMAKE_CURRENT_SOURCE_DIR}/ui"
    --style "fluent"
  DEPENDS "${SLINT_UI_FILE}" Slint::Compiler
  COMMENT "Compiling Slint UI markup: ${SLINT_UI_FILE}"
  VERBATIM
)

add_executable(savm_gui
  installer/gui.c++
  "${SLINT_GENERATED_HEADER}"
)

target_include_directories(savm_gui PRIVATE "${CMAKE_CURRENT_BINARY_DIR}/generated")
target_link_libraries(savm_gui 
  PRIVATE CoreInstaller::Static
  PRIVATE Slint::Static
)
