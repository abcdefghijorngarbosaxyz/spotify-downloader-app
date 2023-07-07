#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app;
mod config;
mod constants;
mod utils;

use app::{menu, setup};
use config::AppConf;
use tauri_plugin_log::LogTarget;

#[tokio::main]
async fn main() {
  let app_config: AppConf = AppConf::read().write();

  let mut context: tauri::Context<tauri::utils::assets::EmbeddedAssets> =
    tauri::generate_context!();

  let mut log: tauri_plugin_log::Builder = tauri_plugin_log::Builder::default()
    .targets([LogTarget::Stdout, LogTarget::Stderr])
    .level(log::LevelFilter::Debug);

  let mut builder = tauri::Builder::default()
    .plugin(log.build())
    .setup(setup::init)
    .menu(menu::init(&mut context));

  builder
    .on_menu_event(menu::handle_menu)
    .run(context)
    .expect("Error while running spotDL GUI application");
}
