$arch = switch ($env:TARGET) {
  "aarch64-pc-windows-msvc" { "ARM64" }
  "i686-pc-windows-msvc" { "Win32" }
  default { "x64" }
}

$env:SAVM_TARGET_TRIPLE = $env:TARGET
$target = $env:TARGET

$env:SAVM_EDITION = $env:PACKAGE.Replace("savm-$target-", "")

cmake -B build `
  -S win32 `
  -A $arch `
  -DCMAKE_BUILD_TYPE=Release `
  -DCMAKE_INSTALL_PREFIX="./dist"

cmake --build build --config Release

cmake --install build --config Release

$items = Get-ChildItem -Path "./dist"

foreach ($item in $items) {
  if ($item.Name.EndsWith(".exe")) {
    $item.FullName
    $env:BINARY = $item.FullName
    ./win32_sign.ps1
  }
}
