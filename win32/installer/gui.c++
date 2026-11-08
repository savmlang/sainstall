#include "embedded_payload.h"
#include "main_slint.h"

#include "gui.h++"
#include "installer.h++"

#include <slint.h>
#include <iostream>

using namespace slint;

ComponentHandle<MainWindow> *wnd = nullptr;

void setup(std::span<char *> args, ComponentHandle<MainWindow> window);

int main(int argc, char *argv[])
{
  std::span<char *> args = std::span(argv, (size_t)argc);
  if (AttachConsole(ATTACH_PARENT_PROCESS))
  {
    FILE *fp;
    freopen_s(&fp, "CONOUT$", "w", stdout);
    freopen_s(&fp, "CONOUT$", "w", stderr);
    freopen_s(&fp, "CONIN$", "r", stdin);

    std::cout << std::endl;
  }

  ComponentHandle<MainWindow> window = MainWindow::create();
  wnd = &window;

  window->hide();
  setup(args, window);

  window->on_startinstall([&window](bool repair)
                          {
    auto conf = window->get_configs();

    CXXConfiguration config = {
      .sdk = CXXSDKConfig {
        .keepStaticArchives = conf.sdkConfig.keepStaticArchives,
        .keepLibraryStubs = conf.sdkConfig.keepLibraryStubs,
        .keepCHeaders = conf.sdkConfig.keepCHeaders,
      },
      .tools = CXXToolConfig {
        .saTestBinary = conf.toolConfig.saTestBinary,
        .saAppRuntime = conf.toolConfig.saAppRuntime,
      },
      .w32 = CXXWinConfig {
        .addToPath = conf.windowsConfig.addToPath,
        .startMenuEntries = conf.windowsConfig.startMenuEntries,
      }
    };

    std::jthread([repair, config] {
    Cbk status = [](Slice txt, double progress) {
      auto view = std::string_view(txt.pt, txt.len);

      std::cout << view << std::endl;
      std::cout.flush();

      slint::blocking_invoke_from_event_loop([txt, progress, view = SharedString(view)]() {
        auto windl = ComponentWeakHandle<MainWindow>(*wnd);

        auto window = windl.lock().value();
        window->set_status(
          view
        );
        window->set_prog(progress);
      });
      return true;
    };

    install_info(
      status,
      []() {
        slint::blocking_invoke_from_event_loop([] {
        auto window = ComponentWeakHandle<MainWindow>(*wnd);

        auto wnd = window.lock().value();
        wnd->set_prog(true);
        });
      },
      repair,
      true,
      config,
      Slice {
        .pt = (char*)ZipFile,
        .len = ZipFile_SIZE
      }
    ); 
  }).detach(); });
  window->on_startuninstall([]()
                            {
    // thread logic
    std::jthread([]() {
        
        auto status_callback = []() {
            slint::blocking_invoke_from_event_loop([]() {
                if (auto windl = ComponentWeakHandle<MainWindow>(*wnd).lock()) {
                    windl.value()->set_uninstalled(true);
                }
            });
        };

        uninstall(status_callback, true);
    }).detach(); });

  window->run();
  return 0;
}

std::shared_ptr<VectorModel<slint::SharedString>> split_to_slint_vec(const std::string &input)
{
  std::istringstream stream(input);
  std::string line;
  std::vector<SharedString> slint_lines;

  while (std::getline(stream, line))
  {
    slint_lines.push_back(SharedString(line));
  }

  return std::make_shared<VectorModel<SharedString>>(std::move(slint_lines));
}

enum class TaskId
{
  None,
  Uninstall,
  Repair
};

void setup(std::span<char *> args, ComponentHandle<MainWindow> window)
{
  HWND hwnd = window->window().win32_hwnd();
  if (!hwnd)
  {
    Timer::single_shot(std::chrono::milliseconds(5), [args, window]()
                       { setup(args, window); });
    return;
  }
  bool modern = applyMicaReturnModern(hwnd);

  if (modern)
  {
    window->set_win10(false);
    window->set_backg(Brush(
        Color::from_argb_uint8(0, 0, 0, 0)));
  }
  else
  {
    window->set_win10(true);

    if (window->get_dark())
    {
      window->set_backg(Brush(Color::from_rgb_uint8(25, 30, 31)));
    };
  }

  std::string_view license = "--water";
  if (!std::ranges::contains(args, license, [](char *s)
                             { return std::string_view(s); }))
  {
    window->set_licensegpl(split_to_slint_vec(
        std::string((char *)LicenseText, LicenseText_SIZE)));
  }

  TaskId taskid = TaskId::None;
  for (auto it : args.subspan(1))
  {
    std::string_view arg = std::string_view(it);

    if (arg == "--water")
      continue;

    if (taskid == TaskId::None)
    {
      if (arg == "uninstall")
      {
        taskid = TaskId::Uninstall;
        continue;
      }
      else if (arg == "repair")
      {
        taskid = TaskId::Repair;
        continue;
      }
    }

    std::cout << "Unexpected Argument: " << arg << std::endl;
  }

  switch (taskid)
  {
  case TaskId::Uninstall:
    window->set_uninstall(true);
    break;
  case TaskId::Repair:
    window->set_curpage(Page::Repairconf);
    break;
  default:
    break;
  }

  window->show();
}