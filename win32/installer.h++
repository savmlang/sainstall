#pragma once

struct CXXConfiguration
{
  CXXSDKConfig sdk;
  CXXToolConfig tools;
  CXXInstallConfig windows;
};

struct CXXSDKConfig
{
  bool keepCHeaders;
  bool keepLibraryStubs;
  bool keepStaticArchives;
};

struct CXXToolConfig
{
  bool saTestBinary;
  bool saAppRuntime;
};

struct CXXInstallConfig
{
  bool addToPath;
  bool startMenuEntries;
};