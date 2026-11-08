#include <windows.h>
#include <iostream>

#pragma comment(lib, "ntdll.lib")

#ifndef STATUS_SUCCESS
#define STATUS_SUCCESS ((LONG)0x00000000L)
#endif

extern "C" NTSTATUS NTAPI RtlGetVersion(OSVERSIONINFOEXW *lpVersionInformation);

bool IsWindowsBuildOrGreater(ULONG dwBuildNumber) noexcept
{
  OSVERSIONINFOEXW version = {};
  version.dwOSVersionInfoSize = sizeof(version);

  if (RtlGetVersion(&version) != STATUS_SUCCESS)
  {
    return false;
  };

  return version.dwBuildNumber >= dwBuildNumber;
}