$b64 = $env:WIN32_PFX_BASE64
$bytes = [System.Convert]::FromBase64String($b64)

"" > sign.pfx
$pfxPath = Resolve-Path "./sign.pfx"

[System.IO.File]::WriteAllBytes($pfxPath, $bytes)

$flags = [System.Security.Cryptography.X509Certificates.X509KeyStorageFlags]::DefaultKeySet
$pfx = [System.Security.Cryptography.X509Certificates.X509Certificate2]::new(
  $pfxPath,
  $env:WIN32_SIGN_PASSWORD,
  $flags
)

$store = [System.Security.Cryptography.X509Certificates.X509Store]::new(
  [System.Security.Cryptography.X509Certificates.StoreName]::My,
  [System.Security.Cryptography.X509Certificates.StoreLocation]::CurrentUser
)
$store.Open([System.Security.Cryptography.X509Certificates.OpenFlags]::ReadWrite)
$store.Add($pfx)

$pfx.Thumbprint > "$PSScriptRoot/thumb.txt"

Remove-Item "./sign.pfx"