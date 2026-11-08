#include "osver.h++"
#include <iostream>
#include <dwmapi.h>

#define WINDOWS_MICA (ULONG)22523

bool applyMicaReturnModern(HWND hwnd)
{
  bool win11 = IsWindowsBuildOrGreater(WINDOWS_MICA);

  if (win11)
  {
    auto backdrop = DWMSBT_MAINWINDOW;
    auto hr = DwmSetWindowAttribute(
        hwnd,
        DWMWA_SYSTEMBACKDROP_TYPE, &backdrop,
        sizeof(backdrop));

    if (FAILED(hr))
    {
      LPWSTR errorText = nullptr;

      // Ask Windows to look up the message for the given HRESULT
      DWORD result = FormatMessageW(
          FORMAT_MESSAGE_ALLOCATE_BUFFER |
              FORMAT_MESSAGE_FROM_SYSTEM |
              FORMAT_MESSAGE_IGNORE_INSERTS,
          NULL,
          hr,
          MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
          (LPWSTR)&errorText,
          0,
          NULL);

      if (result != 0 && errorText != nullptr)
      {
        std::wcout << L"Error 0x" << std::hex << hr << L": " << errorText << std::endl;
        LocalFree(errorText); // Free the buffer allocated by FormatMessage
      }
      else
      {
        std::wcout << L"Error 0x" << std::hex << hr << L" (Unknown error code)" << std::endl;
      }
    }
  }

  return win11;
}