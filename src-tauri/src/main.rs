// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod constants;

#[tokio::main]
async fn main() {
  let context: tauri::Context<tauri::utils::assets::EmbeddedAssets> = tauri::generate_context!();

  let logger: tauri_plugin_log::Builder = tauri_plugin_log::Builder::default().targets([
    tauri_plugin_log::LogTarget::LogDir,
    tauri_plugin_log::LogTarget::Stdout,
    tauri_plugin_log::LogTarget::Stderr,
    tauri_plugin_log::LogTarget::Webview,
  ]);

  let app_builder = tauri::Builder::default()
    .plugin(logger.build())
    .menu(app::menu::init())
    .setup(app::setup::init);

  if let Err(err) = app_builder.run(context) {
    log::error!("Error while running generating context: {}", err);
  } else {
    log::info!("Context generated.");
  }
}
