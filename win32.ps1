$arch = switch ($env:TARGET) {
  "aarch64-pc-windows-msvc" { "ARM64" }
  "i686-pc-windows-msvc" { "Win32" }
  default { "x64" }
}

$env:SAVM_TARGET_TRIPLE = $env:TARGET
$target = $env:TARGET

$env:SAVM_EDITION = $env:PACKAGE.Split("-")[-1]

cmake -B build `
  -S win32 `
  -A $arch `
  -DCMAKE_BUILD_TYPE=Release `
  -DCMAKE_INSTALL_PREFIX="./dist"

cmake --build build --config Release

cmake --install build --config Release

$items = Get-ChildItem -Path "./dist/" -Filter "*.exe" -File | Select-Object -ExpandProperty FullName
./win32_loadsign.ps1
./win32_sign.ps1 @items