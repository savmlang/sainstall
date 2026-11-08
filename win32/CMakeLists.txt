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

if(DEFINED ENV{SAVM_EDITION} AND NOT "$ENV{SAVM_EDITION}" STREQUAL "")
  set(EDITION "$ENV{SAVM_EDITION}")
else()
  set(EDITION "nano")
  message(WARNING "SAVM_EDITION was not specified. Defaulting to nano")
endif()

set(CLI "savm-${TRIPLE}-${EDITION}-installer_cli.exe")
set(GUI "savm-${TRIPLE}-${EDITION}-installer_gui.exe")
set(ZIP "savm-${TRIPLE}-${EDITION}.zip")

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
  set(CMAKE_CXX_STANDARD 26)
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

# #embed macros, LICENSE
set(PAYLOAD_BIN_LICENSE "${CMAKE_CURRENT_SOURCE_DIR}/../../LICENSE")
set(PAYLOAD_BIN_BUILD "${CMAKE_CURRENT_SOURCE_DIR}/../../build.zip")
set(PAYLOAD_HDR "${CMAKE_CURRENT_BINARY_DIR}/generated/embedded_payload.h")

add_custom_command(
  OUTPUT "${PAYLOAD_HDR}"
  COMMAND ${CMAKE_COMMAND}
    -P "${CMAKE_CURRENT_SOURCE_DIR}/generate_embed.cmake"
  DEPENDS "${PAYLOAD_BIN_LICENSE}"
  DEPENDS "${PAYLOAD_BIN_BUILD}"
  COMMENT "Embedding ${PAYLOAD_BIN} -> ${PAYLOAD_HDR}"
  VERBATIM
)


# SaVM CLI Installer
add_executable(savm_cli
  installer/cli.c++
  resource.rc
)

target_link_libraries(savm_cli
  PRIVATE CoreInstaller::Static
)
target_link_options(savm_cli PRIVATE 
  "/MANIFEST:NO"
)
target_compile_options(savm_cli PRIVATE /std:c++latest)

target_sources(savm_cli PRIVATE "${PAYLOAD_HDR}")
target_include_directories(savm_cli PRIVATE "${CMAKE_CURRENT_BINARY_DIR}/generated")

# SaVM GUI Installer
file(MAKE_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/generated")
set(SLINT_UI_FILE "${CMAKE_CURRENT_SOURCE_DIR}/ui/index.slint")
set(SLINT_GENERATED_HEADER "${CMAKE_CURRENT_BINARY_DIR}/generated/main_slint.h")

add_custom_command(
  OUTPUT "${SLINT_GENERATED_HEADER}"
  COMMAND Slint::Compiler
    "${SLINT_UI_FILE}"
    -o "${SLINT_GENERATED_HEADER}"
    -I "${CMAKE_CURRENT_SOURCE_DIR}/ui"
    --style "fluent"
  DEPENDS "${SLINT_UI_FILE}" Slint::Compiler
  COMMENT "Compiling Slint UI markup: ${SLINT_UI_FILE}"
  VERBATIM
)

add_executable(savm_gui
  "${SLINT_GENERATED_HEADER}"
  installer/gui.c++
  installer/gui/osver.c++
  installer/gui/mica.c++
  resource.rc
)

target_compile_options(savm_gui PRIVATE /std:c++latest)
target_include_directories(savm_gui 
  PRIVATE "${CMAKE_CURRENT_BINARY_DIR}/generated"
  PRIVATE "${slint_SOURCE_DIR}/include/slint"
)
target_link_libraries(savm_gui 
  PRIVATE CoreInstaller::Static
  PRIVATE Slint::Static
  ws2_32
  ntdll
  imm32
  uxtheme
  dwmapi
  userenv
)

target_sources(savm_gui PRIVATE "${PAYLOAD_HDR}")

target_link_options(savm_gui PRIVATE 
  "/SUBSYSTEM:WINDOWS" 
  "/ENTRY:mainCRTStartup"
  "/MANIFEST:NO"
)

# CMAKE Install
install(CODE [[
  file(INSTALL 
    "${CMAKE_CURRENT_BINARY_DIR}/savm_cli.exe"
    DESTINATION "${CMAKE_INSTALL_PREFIX}/"
    RENAME "${CLI}"
  )
  file(INSTALL 
    "${CMAKE_CURRENT_BINARY_DIR}/savm_gui.exe"
    DESTINATION "${CMAKE_INSTALL_PREFIX}/"
    RENAME "${GUI}"
  )
  file(INSTALL 
    "${PAYLOAD_BIN_BUILD}"
    DESTINATION "${CMAKE_INSTALL_PREFIX}"
    RENAME "${ZIP}"
  )
]])