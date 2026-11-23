$arch = switch ($env:TARGET) {
  "aarch64-pc-windows-msvc" { "arm64" }
  "i686-pc-windows-msvc" { "x86" }
  default { "x64" }
}

$sdkPath = (Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows Kits\Installed Roots").KitsRoot10
$signtool = Get-ChildItem -Path "${sdkPath}bin\*\$arch\signtool.exe" | Select-Object -ExpandProperty FullName -First 1

$thumbprint = Get-Content "$PSScriptRoot/thumb.txt"

& $signtool sign `
  /s "My" /sha1 "$thumbprint" `
  /fd SHA512 /tr "http://timestamp.digicert.com" `
  /td SHA512 /v `
  "$env:BINARY"