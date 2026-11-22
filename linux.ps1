Copy-Item -Path "../$env:PACKAGE.zip" -Destination "./dist/$env:PACKAGE.zip"

Set-Location nfpm

npm i -g pnpm
pnpm install

Set-Location ..
