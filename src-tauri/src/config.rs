use std::{collections::BTreeMap, path::PathBuf};

use log::{error, info};

use serde_json::Value;

use crate::utils::{create_file, exists, get_app_dir};

const APP_CONFIG_FILE: &str = "gui.config.json";

macro_rules! pub_struct {
  ($name:ident {$($field:ident: $t:ty,)*}) => {
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct $name {
      $(pub $field: $t),*
    }
  }
}

pub_struct!(AppConf {
  always_on_top: bool,
  global_shortcut: Option<String>,
  download_folder: String,
});

impl AppConf {
  pub fn new() -> Self {
    Self {
      always_on_top: false,
      global_shortcut: None,
      download_folder: tauri::api::path::download_dir()
        .unwrap()
        .to_string_lossy()
        .to_string(),
    }
  }

  pub fn config_path() -> PathBuf {
    get_app_dir().join(APP_CONFIG_FILE)
  }

  pub fn read() -> Self {
    match std::fs::read_to_string(Self::config_path()) {
      Ok(file_content) => {
        if let Ok(parsed_config) = serde_json::from_str::<AppConf>(&file_content) {
          parsed_config
        } else {
          error!("Config file parse error");
          Self::default()
        }
      }
      Err(err) => {
        error!("Config file read error: {}", err);
        Self::default()
      }
    }
  }

  pub fn write(self) -> Self {
    let path: &PathBuf = &Self::config_path();

    if !exists(path) {
      create_file(path).unwrap();
      info!("Config file created");
    }
    if let Ok(parsed_config) = serde_json::to_string_pretty(&self) {
      std::fs::write(path, parsed_config).unwrap_or_else(|err| {
        error!("Config write error: {}", err);
        Self::default().write();
      })
    } else {
      error!("Config parse error");
    }
    self
  }

  pub fn patch(self, json: Value) -> Self {
    let val = serde_json::to_value(&self).unwrap();
    let mut config: BTreeMap<String, Value> = serde_json::from_value(val).unwrap();
    let new_json: BTreeMap<String, Value> = serde_json::from_value(json).unwrap();

    for (k, v) in new_json {
      config.insert(k, v);
    }

    match serde_json::to_string_pretty(&config) {
      Ok(v) => match serde_json::from_str::<AppConf>(&v) {
        Ok(v) => v,
        Err(err) => {
          error!("conf_amend_parse: {}", err);
          self
        }
      },
      Err(err) => {
        error!("conf_amend_str: {}", err);
        self
      }
    }
  }
}

impl Default for AppConf {
  fn default() -> Self {
    Self::new()
  }
}

pub mod handlers {
  use super::AppConf;

  #[tauri::command]
  pub fn get_app_config() -> AppConf {
    AppConf::read()
  }

  #[tauri::command]
  pub fn reset_app_config() -> AppConf {
    AppConf::default().write()
  }
}
