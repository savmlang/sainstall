$arch = switch ($env:TARGET) {
  "aarch64-pc-windows-msvc" { "arm64" }
  "i686-pc-windows-msvc" { "x86" }
  default { "x64" }
}

$sdkPath = (Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows Kits\Installed Roots").KitsRoot10
$signtool = Get-ChildItem -Path "${sdkPath}bin\*\$arch\signtool.exe" | Select-Object -ExpandProperty FullName -First 1

$thumbprint = "DD1E95E5E75F7BDF1AC4D60B9DFB687A70F2FFF5"

& $signtool sign `
  /s "My" /sha1 "$thumbprint" `
  /fd SHA512 /tr "http://timestamp.digicert.com" `
  /td SHA512 /v `
  "$env:BINARY"