use std::{
  env::current_exe, fs, marker::PhantomData, path::PathBuf, thread::sleep, time::Duration,
};
use windows::Win32::{
  Storage::FileSystem::{MOVEFILE_DELAY_UNTIL_REBOOT, MoveFileExW},
  System::Threading::ExitProcess,
};
use windows_registry::HSTRING;

use crate::{
  dir::get_savm_startmenu,
  regedit::{cleanup_registry, setup_registry},
};

mod dir;
mod regedit;

mod zip {
  use std::{io::Cursor, path::Path};
  use zip::ZipArchive;

  pub fn extract<T: AsRef<Path>>(directory: T, inner: &[u8]) {
    let mut archive = ZipArchive::new(Cursor::new(inner)).unwrap();

    archive.extract(directory).unwrap();
  }
}

#[repr(C)]
pub struct Config {
  pub sdk: SDKConfig,
  pub tools: ToolConfig,
  pub w32: WinConfig,
}

#[derive(Debug)]
#[repr(C)]
pub struct ToolConfig {
  pub satest: bool,
  pub saapprt: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct WinConfig {
  pub path: bool,
  pub start: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct SDKConfig {
  pub staticarchives: bool,
  pub linklibs: bool,
  pub headers: bool,
}

#[repr(C)]
pub struct Slice<'a> {
  pub pt: *const u8,
  pub len: usize,

  pub _life: PhantomData<&'a ()>,
}

pub type StringSlice<'a> = Slice<'a>;

#[unsafe(no_mangle)]
pub extern "C" fn install_info(
  cb: extern "C" fn(StringSlice<'static>, f64) -> bool,
  installed: extern "C" fn(),

  repair: bool,
  autoexit: bool,

  config: Config,
  zip: Slice,
) {
  let path;
  if repair {
    cb(s("Clearing old SaVM"), 0.0);

    path = unsafe { dir::get_savmdir() };

    for entry in fs::read_dir(&path).unwrap().map(Result::unwrap) {
      let path = entry.path();

      if entry.file_name() != "savmsetupconfigureuninstallrepair.exe" {
        fs::remove_file(&path)
          .or_else(|_| fs::remove_dir_all(&path))
          .expect("SaVM has another install currently running!");
      }
    }
  } else {
    cb(s("Creating Directories..."), 0.0);

    path = unsafe { dir::get_savmdir() };
    _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("This shouldn't error, if it does we crash");
  }

  let steps = 5.0;

  {
    cb(s("Copying Core Runtime..."), 1.0 / steps);

    zip::extract(&path, unsafe {
      core::slice::from_raw_parts(zip.pt, zip.len)
    });
  }

  if !repair {
    cb(s("Copying important files..."), 2.0 / steps);

    let cexe = current_exe().unwrap();
    _ = fs::copy(
      cexe,
      format!("{}/savmsetupconfigureuninstallrepair.exe", &path),
    );
  }

  {
    cb(s("Configuring SaVM..."), 3.0 / steps);
    dir::configure(&path, &config);
  }

  {
    cb(s("Setting Up Registry..."), 4.0 / steps);

    setup_registry(&path, config.w32.path);
  }

  {
    cb(s("Successful"), 1.0);
    installed();
  }

  if autoexit {
    sleep(Duration::from_secs(10));
    unsafe { ExitProcess(0) };
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn uninstall(done: extern "C" fn(), autoexit: bool) {
  let path = unsafe { dir::get_savmdir() };

  // Autodelete as much as we can
  if let Ok(d) = fs::read_dir(&path) {
    d.map(Result::ok)
      .filter(Option::is_some)
      .map(Option::unwrap)
      .for_each(|x| {
        let path = x.path();

        _ = fs::remove_dir_all(&path);
        _ = fs::remove_file(&path);
      });
  }
  let mut pbuf = PathBuf::from(path);

  {
    unsafe {
      let start = get_savm_startmenu();
      fs::remove_dir_all(start).unwrap();
    }
  }

  unsafe {
    let dir = HSTRING::from(pbuf.to_str().unwrap());
    pbuf.push("savmsetupconfigureuninstallrepair.exe");
    let uninstaller = HSTRING::from(pbuf.to_str().unwrap());

    _ = MoveFileExW(&uninstaller, None, MOVEFILE_DELAY_UNTIL_REBOOT);
    _ = MoveFileExW(&dir, None, MOVEFILE_DELAY_UNTIL_REBOOT);
  }

  cleanup_registry();

  done();

  if autoexit {
    sleep(Duration::from_secs(3));
    unsafe { ExitProcess(0) };
  }
}

fn s<'a>(st: &'a str) -> StringSlice<'a> {
  StringSlice {
    pt: st.as_ptr(),
    len: st.len(),

    _life: PhantomData,
  }
}
