#pragma once

#include <stdint.h>

extern "C"
{
  struct CXXSDKConfig
  {
    bool keepStaticArchives;
    bool keepLibraryStubs;
    bool keepCHeaders;
  };

  struct CXXToolConfig
  {
    bool saTestBinary;
    bool saAppRuntime;
  };

  struct CXXWinConfig
  {
    bool addToPath;
    bool startMenuEntries;
  };

  struct CXXConfiguration
  {
    CXXSDKConfig sdk;
    CXXToolConfig tools;
    CXXWinConfig w32;
  };

  struct Slice
  {
    char *pt;
    uintptr_t len;
  };

  struct SliceAny
  {
    void *pt;
    uintptr_t len;
  };

  using Cbk = bool (*)(Slice, double);
  using Done = void (*)();

  void install_info(
      Cbk cb,
      Done installed,
      bool repair,
      bool autoexit,
      CXXConfiguration config,
      Slice zip);

  void uninstall(Done done, bool autoexit);

  void cli(
      SliceAny args,
      Slice license,
      Slice zip);
}