use std::path::{Path, PathBuf};

use crate::{
  config::AppConf,
  constants::{DISCORD_URL, DOCS_URL, ISSUES_URL},
  utils::{exists, shell_open},
};

use log::info;
use tauri::{
  api::dialog, Context, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu,
  WindowMenuEvent, Wry,
};

pub fn init(context: &mut Context<tauri::utils::assets::EmbeddedAssets>) -> Menu {
  let name: String = context.package_info().name.clone();

  let app_menu: Menu = Menu::with_items([
    #[cfg(target_os = "macos")]
    MenuEntry::Submenu(Submenu::new(
      name,
      Menu::with_items([
        MenuItem::About(name, AboutMetadata::default()).into(),
        MenuItem::Separator.into(),
        MenuItem::Services.into(),
        MenuItem::Separator.into(),
        MenuItem::Hide.into(),
        MenuItem::HideOthers.into(),
        MenuItem::ShowAll.into(),
        MenuItem::Separator.into(),
        MenuItem::Quit.into(),
      ]),
    )),
    MenuEntry::Submenu(Submenu::new(
      "File",
      Menu::with_items([
        CustomMenuItem::new("open_download_folder", "Open Download Folder")
          .accelerator("CmdOrCtrl+O")
          .into(),
        CustomMenuItem::new("select_download_folder", "Select Download Folder")
          .accelerator("CmdOrCtrl+Shift+O")
          .into(),
        MenuItem::Separator.into(),
        #[cfg(not(target_os = "macos"))]
        CustomMenuItem::new("options", "Options...")
          .accelerator("CmdOrCtrl+,")
          .into(),
        #[cfg(not(target_os = "macos"))]
        MenuItem::Separator.into(),
        #[cfg(not(target_os = "macos"))]
        MenuItem::CloseWindow.into(),
      ]),
    )),
    MenuEntry::Submenu(Submenu::new(
      "Edit",
      Menu::with_items([
        #[cfg(target_os = "macos")]
        MenuItem::Undo.into(),
        #[cfg(target_os = "macos")]
        MenuItem::Redo.into(),
        #[cfg(target_os = "macos")]
        MenuItem::Separator.into(),
        MenuItem::Cut.into(),
        MenuItem::Copy.into(),
        MenuItem::Paste.into(),
      ]),
    )),
    MenuEntry::Submenu(Submenu::new(
      "View",
      Menu::with_items([
        CustomMenuItem::new("downloads", "Downloads")
          .accelerator("CmdOrCtrl+K")
          .into(),
        CustomMenuItem::new("show_status_bar", "Show Status Bar")
          .accelerator("CmdOrCtrl+J")
          .into(),
        #[cfg(target_os = "macos")]
        MenuItem::Separator.into(),
        #[cfg(target_os = "macos")]
        MenuItem::Zoom.into(),
        #[cfg(target_os = "macos")]
        MenuItem::Separator.into(),
        #[cfg(target_os = "macos")]
        MenuItem::EnterFullScreen.into(),
        MenuItem::Separator.into(),
        CustomMenuItem::new("devtools", "Toggle Dev Tools")
          .accelerator("CmdOrCtrl+Shift+I")
          .into(),
      ]),
    )),
    MenuEntry::Submenu(Submenu::new(
      "Window",
      Menu::with_items([
        MenuItem::Minimize.into(),
        MenuItem::Separator.into(),
        CustomMenuItem::new("always_on_top", "Always on Top")
          .accelerator("CmdOrCtrl+T")
          .into(),
      ]),
    )),
    MenuEntry::Submenu(Submenu::new(
      "Help",
      Menu::with_items([
        CustomMenuItem::new("docs", "Documentation").into(),
        CustomMenuItem::new("show_release_notes", "Show Release Notes").into(),
        MenuItem::Separator.into(),
        CustomMenuItem::new("report_issue", "Report Issue").into(),
        CustomMenuItem::new("join_us_on_discord", "Join Us on Discord").into(),
        MenuItem::Separator.into(),
        CustomMenuItem::new("check_for_updates", "Check for Updates...").into(),
        #[cfg(not(target_os = "macos"))]
        MenuItem::Separator.into(),
        #[cfg(not(target_os = "macos"))]
        CustomMenuItem::new("about", "About ".to_string() + &name).into(),
      ]),
    )),
  ]);
  app_menu
}

pub fn handle_menu(event: WindowMenuEvent<Wry>) {
  let window: &tauri::Window = Some(event.window()).unwrap();
  let app_handle: tauri::AppHandle = window.app_handle();
  let menu_id: &str = event.menu_item_id();
  let menu_handle: tauri::window::MenuHandle = window.menu_handle();
  let conf: AppConf = AppConf::read();

  match menu_id {
    "docs" => shell_open(&app_handle, DOCS_URL),
    "report_issue" => shell_open(&app_handle, ISSUES_URL),
    "join_us_on_discord" => shell_open(&app_handle, DISCORD_URL),
    "devtools" => {
      window.open_devtools();
      window.close_devtools();
    }
    "always_on_top" => {
      let always_on_top: bool = !conf.always_on_top;

      menu_handle
        .get_item(menu_id)
        .set_selected(always_on_top)
        .unwrap();
      window.set_always_on_top(always_on_top).unwrap();
      conf
        .patch(serde_json::json!({ "always_on_top": always_on_top }))
        .write();
    }
    "open_download_folder" => {
      if exists(PathBuf::from(&conf.download_folder).as_path()) {
        shell_open(&app_handle, &conf.download_folder)
      }
    }
    "select_download_folder" => {
      let dialog: dialog::FileDialogBuilder = dialog::FileDialogBuilder::new();
      dialog.pick_folder(|folder_path: Option<std::path::PathBuf>| {
        if folder_path.is_some() {
          let folder_path_string: String = folder_path.unwrap().to_str().unwrap().to_string();
          info!("Download folder changed: {}", folder_path_string);
          conf
            .patch(serde_json::json!({ "download_folder": folder_path_string }))
            .write();
        }
      })
    }
    _ => {}
  }
}
