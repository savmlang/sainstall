cmake -B build `
  -S win32 `
  -DCMAKE_BUILD_TYPE=Release `
  -DCMAKE_INSTALL_PREFIX="./dist"

cmake --build build --config Release

cmake --install build --config Release