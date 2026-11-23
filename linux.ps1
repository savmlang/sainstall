Copy-Item -Path "../$env:PACKAGE.zip" -Destination "./dist/$env:PACKAGE.zip"

sh ./linux.sh

New-Item -Path tmp -ItemType Directory -ErrorAction SilentlyContinue
Expand-Archive -Path "./dist/$env:PACKAGE.zip" -DestinationPath "./tmp" -Force

# Pre ENV
$nfpmpkgsgnu = "archlinux", "deb", "rpm", "srpm"
$nfpmpkgsmusl = "apk", "ipk"

if ($env:TARGET.Contains("gnu").Equals($true)) {
  $packgers = $nfpmpkgsgnu
}
else {
  $packgers = $nfpmpkgsmusl
}

$rustarch = $env:TARGET.Split("-")[0]

# Set ENV
$env:SAVM_EDITION = $env:PACKAGE.Replace("savm-$target-", "")
$env:SAVM_EXPANDED = Resolve-Path "./tmp/"

switch ($rustarch) {
  "armv7" {
    $env:BUNDLE_ARCH = "arm7"
  }
  "aarch64" {
    $env:BUNDLE_ARCH = "arm64"
  }
  "i686" {
    $env:BUNDLE_ARCH = "386"
  }
  "powerpc64le" {
    $env:BUNDLE_ARCH = "ppc64le"
  }
  "riscv64gc" {
    $env:BUNDLE_ARCH = "riscv64"
  }
  "x86_64" {
    $env:BUNDLE_ARCH = "amd64"
  }
  Default {
    Write-Error "Unable to find a supported architecture for $rustarch"
    exit 1
  }
}

Set-Location nfpm

npm i -g pnpm
pnpm install

node nfpm.setup.js
node nfpm.recipe.js

$packgers | ForEach-Object -Parallel {
  $output = nfpm pkg -p $_ --target ../dist/ 2>&1
  $output

  if ($LASTEXITCODE -ne 0) {
    throw "nfpm failed for package: $_"
  }
}

Set-Location ..
