use std::{
  fs,
  path::{Path, PathBuf},
};

use tauri::{AppHandle, Manager};

pub fn convert_path(path: &str) -> String {
  if cfg!(target_os = "windows") {
    path.replace("/", "\\")
  } else {
    String::from(path)
  }
}

pub fn get_app_dir() -> PathBuf {
  tauri::api::path::home_dir().unwrap().join(".spotdl")
}

pub fn exists(path: &Path) -> bool {
  Path::new(path).exists()
}

pub fn create_file<P: AsRef<Path>>(filename: P) -> anyhow::Result<()> {
  let filename: &Path = filename.as_ref();
  if let Some(parent) = filename.parent() {
    if !parent.exists() {
      fs::create_dir_all(parent)?;
    }
  }
  fs::File::create(filename)?;
  Ok(())
}

pub fn shell_open(app_handle: &AppHandle, url: &str) {
  tauri::api::shell::open(&app_handle.shell_scope(), url, None).unwrap();
}
