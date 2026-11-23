#pragma once

#include "resourceinfodata.h"

#include <iostream>
#include <windows.h>
#include <span>

std::span<const char> GetAsset(WORD resourceId)
{
  // Current exe
  HMODULE hModule = nullptr;

  HRSRC hRes = FindResourceW(hModule, MAKEINTRESOURCEW(resourceId), MAKEINTRESOURCEW(RT_RCDATA));

  if (!hRes)
  {
    DWORD err = GetLastError();
    std::cout << err << std::endl;
    std::cout.flush();
    return {};
  };

  DWORD size = SizeofResource(hModule, hRes);
  HGLOBAL hData = LoadResource(hModule, hRes);
  if (!hData || size == 0)
  {
    DWORD err = GetLastError();
    std::cout << err << std::endl;
    std::cout.flush();
    return {};
  };

  const void *ptr = LockResource(hData);
  if (!ptr)
  {
    DWORD err = GetLastError();
    std::cout << err << std::endl;
    std::cout.flush();
    return {};
  };

  return {static_cast<const char *>(ptr), size};
}