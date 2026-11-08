#include "embedded_payload.h"

#include "installer.h++"

#include <iostream>
#include <span>

int main(int argc, char *argv[])
{
  std::span<char *> ar(argv, argc);
  auto args = ar.subspan(1);

  std::cout << "SaVM Runtime Installer" << std::endl
            << std::endl
            << "SaVM Runtime is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**." << std::endl
            << "Run with `--license` to view the full license terms." << std::endl
            << std::endl;

  cli(
      SliceAny{
          (void *)(args.data()),
          args.size()},
      Slice{
          (char *)LicenseText,
          LicenseText_SIZE},
      Slice{
          (char *)ZipFile,
          ZipFile_SIZE});

  return 0;
}