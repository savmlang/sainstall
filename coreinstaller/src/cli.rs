use windows::Win32::System::Com::{CoInitialize, CoUninitialize};

use crate::{
  Config, SDKConfig, Slice, StringSlice, ToolConfig, WinConfig, install_info, uninstall,
};
use core::str;
use std::{
  env::current_exe,
  ffi::{CStr, OsStr, c_char},
  hint::unreachable_unchecked,
  io::{BufRead, Write},
  slice,
  sync::atomic::{AtomicBool, Ordering},
  thread,
  time::Duration,
};

#[derive(PartialEq, Eq)]
enum Page {
  None,
  License,
  Install,
  Uninstall,
  Help,
}

static NOINPUT: AtomicBool = AtomicBool::new(false);

#[unsafe(no_mangle)]
pub extern "C" fn cli(args: Slice<*const c_char>, license: StringSlice, zip: Slice<u8>) {
  let guard = unsafe { ComGuard(CoInitialize(None).is_ok()) };

  let argv = unsafe {
    core::slice::from_raw_parts(args.pt, args.len)
      .iter()
      .map(|&x| CStr::from_ptr(x))
  };

  let mut page = Page::None;

  let mut noinput = false;
  let mut repair = false;
  let mut config = Config {
    sdk: SDKConfig {
      headers: true,
      linklibs: true,
      staticarchives: true,
    },
    tools: ToolConfig {
      saapprt: true,
      satest: true,
    },
    w32: WinConfig {
      path: true,
      start: true,
    },
  };

  let mut err = false;
  let mut argparse = |err: &mut bool, noinput: &mut bool, arg: &[u8]| match arg {
    b"--license" if page == Page::None => page = Page::License,
    b"--no-input" => *noinput = true,
    b"help" if page == Page::None => page = Page::Help,
    b"install" | b"repair" if page == Page::None => {
      page = Page::Install;
      repair = arg == b"repair";
    }
    b"uninstall" if page == Page::None => {
      page = Page::Uninstall;
    }

    // --<name> = true/false
    _ if page == Page::Install
      && arg.starts_with(b"--")
      && (arg.ends_with(b"=true") || arg.ends_with(b"=false"))
      && str::from_utf8(arg).is_ok() =>
    {
      let (key, val) = unsafe { str::from_utf8_unchecked(arg) }
        .strip_prefix("--")
        .unwrap()
        .rsplit_once("=")
        .unwrap();

      let val = match val.trim().to_ascii_lowercase().as_str() {
        "true" => true,
        "false" => false,
        _ => unsafe { unreachable_unchecked() },
      };

      *match key {
        "headers" => &mut config.sdk.headers,
        "linklibs" => &mut config.sdk.linklibs,
        "staticarchives" => &mut config.sdk.staticarchives,

        "satest" => &mut config.tools.satest,
        "saapprt" => &mut config.tools.saapprt,

        "path" => &mut config.w32.path,
        "start" => &mut config.w32.start,

        _ => {
          *err = true;
          let mut stdout = std::io::stdout().lock();
          _ = stdout.write_all(b"Unexpected Argument: `");
          _ = stdout.write_all(arg);
          _ = stdout.write_all(b"`\r\n");
          _ = stdout.flush();
          return;
        }
      } = val;
    }

    arg => {
      *err = true;
      let mut stdout = std::io::stdout().lock();
      _ = stdout.write_all(b"Unexpected Argument: `");
      _ = stdout.write_all(arg);
      _ = stdout.write_all(b"`\r\n");
      _ = stdout.flush();
    }
  };

  for arg in argv {
    argparse(&mut err, &mut noinput, arg.to_bytes());
  }

  if err {
    eprintln!("Exiting due to invalid arguments");
    std::process::exit(1);
  }

  // Take Input from user
  if !noinput {
    print_help();

    let mut stdout = std::io::stdout().lock();
    let mut stdin = std::io::stdin().lock();

    _ = stdout.write_all(b"Arguments : ");
    _ = stdout.flush();

    let mut args = String::with_capacity(32);
    if let Err(_) = stdin.read_line(&mut args) {
      _ = stdout.write_all(b"\r\nExiting as user cancelled");
      std::process::exit(0);
    }

    args
      .split_whitespace()
      .for_each(|x| argparse(&mut err, &mut noinput, x.as_bytes()));

    if err {
      eprintln!("Exiting due to invalid arguments");
      std::process::exit(1);
    }
  }

  NOINPUT.store(noinput, Ordering::Release);

  match page {
    Page::None | Page::Help => print_help(),
    Page::License => println!("{}", unsafe {
      str::from_utf8_unchecked(slice::from_raw_parts(license.pt, license.len))
    }),
    Page::Install => {
      install_info(cb, installed, repair, false, config, zip);
    }
    Page::Uninstall => {
      uninstall(done, false);
    }
  }

  drop(guard);
}

extern "C" fn cb(s: StringSlice<'static>, prog: f64) -> bool {
  println!("[{:.2}%] {}", prog * 100.0, unsafe {
    str::from_utf8_unchecked(slice::from_raw_parts(s.pt, s.len))
  });
  true
}

extern "C" fn installed() {
  println!("SaVM has been installed!");
  if !NOINPUT.load(Ordering::Acquire) {
    println!();
    loop {
      thread::sleep(Duration::from_secs(60 * 10));
    }
  }

  println!(" You can close the window or run Ctrl+C");
}

extern "C" fn done() {
  print!("SaVM has been uninstalled!");
  if !NOINPUT.load(Ordering::Acquire) {
    println!();
    loop {
      thread::sleep(Duration::from_secs(60 * 10));
    }
  }

  println!(" You can close the window or run Ctrl+C");
}

fn print_help() {
  println!(
    "{} [...Args]
Args:
  --license: Prints the SaVM License
  --no-input: The setup would not ask to enter arguments interactively

  help: Shows this help screen
  install [..args]: Install SaVM
  repair [..args]: Repairs SaVM
  uninstall: Uninstall SaVM

Install/Repair Args:
  --no-input
  --headers=true/false
  --linklibs=true/false
  --staticarchives=true/false
  --satest=true/false
  --saapprt=true/false
  --path=true/false
  --start=true/false",
    current_exe()
      .unwrap_or_default()
      .file_name()
      .unwrap_or(OsStr::new("savminstaller.exe"))
      .display()
  );
}

struct ComGuard(bool);
impl Drop for ComGuard {
  fn drop(&mut self) {
    if self.0 {
      unsafe { CoUninitialize() };
    }
  }
}
