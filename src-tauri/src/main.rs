// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu};

fn main() {
  let context = tauri::generate_context!();

  let app = tauri::Builder::default()
    .menu(Menu::with_items([
      // Ready for macos cfg but no support for now
      #[cfg(target_os = "macos")]
      MenuEntry::Submenu(Submenu::new(
        &context.package_info().name,
        Menu::with_items([
          MenuItem::About(
            context.package_info().name.clone(),
            AboutMetadata::default(),
          )
          .into(),
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
          CustomMenuItem::new("Open Download Folder", "Open Download Folder")
            .accelerator("CmdOrCtrl+O")
            .into(),
          CustomMenuItem::new("Select Download Folder", "Select Download Folder")
            .accelerator("CmdOrCtrl+Shift+O")
            .into(),
          MenuItem::Separator.into(),
          #[cfg(not(target_os = "macos"))]
          CustomMenuItem::new("Options...", "Options...")
            .accelerator("CmdOrCtrl+,")
            .into(),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
          MenuItem::CloseWindow.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Edit",
        Menu::with_items([
          MenuItem::Cut.into(),
          MenuItem::Copy.into(),
          MenuItem::Paste.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Help",
        Menu::with_items([
          CustomMenuItem::new("Report Issue...", "Report Issue...").into(),
          CustomMenuItem::new("Documentation", "Documentation").into(),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
          CustomMenuItem::new("About", "About").into(),
        ]),
      )),
    ]))
    .on_menu_event(|event| {
      let event_name = event.menu_item_id();
      match event_name {
        "Report Issue..." => {
          shell::open(
            &event.window().shell_scope(),
            "https://github.com".to_string(),
            None,
          )
          .unwrap();
        }
        "Documentation" => {
          shell::open(
            &event.window().shell_scope(),
            "https://github.com".to_string(),
            None,
          )
          .unwrap();
        }
        _ => {}
      }
    })
    .build(context)
    .expect("error while running tauri application");

  app.run(|_app_handle, run_event| match run_event {
    // TODO populate with events
    _ => {}
  });
}
