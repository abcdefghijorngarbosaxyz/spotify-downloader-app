// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod utils;

use tauri::api::{dialog, shell};
use tauri::{CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, WindowBuilder};

fn main() {
  let context = tauri::generate_context!();

  let app = tauri::Builder::default()
    .setup(move |app| {
      let main_window = WindowBuilder::new(app, "main", tauri::WindowUrl::default())
        .title("spotDL GUI")
        .theme(tauri::Theme::Light.into())
        .center()
        .fullscreen(false)
        .inner_size(800.0, 600.0)
        .maximizable(false)
        .resizable(false);

      let _ = main_window.build().expect("unable to build main window");
      Ok(())
    })
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
          CustomMenuItem::new("Downloads", "Downloads")
            .accelerator("CmdOrCtrl+K")
            .into(),
          CustomMenuItem::new("Show Status Bar", "Show Status Bar")
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
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Window",
        Menu::with_items([MenuItem::Minimize.into()]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Help",
        Menu::with_items([
          CustomMenuItem::new("Documentation", "Documentation").into(),
          CustomMenuItem::new("Show Release Notes", "Show Release Notes").into(),
          MenuItem::Separator.into(),
          CustomMenuItem::new("Report Issue", "Report Issue").into(),
          CustomMenuItem::new("Join Us on Discord", "Join Us on Discord").into(),
          MenuItem::Separator.into(),
          CustomMenuItem::new("Check for Updates...", "Check for Updates...").into(),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
          #[cfg(not(target_os = "macos"))]
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
            "https://github.com/abcdefghijorngarbosaxyz/spotify-downloader-app/issues/new/choose"
              .to_string(),
            None,
          )
          .unwrap();
        }
        "Documentation" => {
          shell::open(
            &event.window().shell_scope(),
            "https://github.com/abcdefghijorngarbosaxyz/spotify-downloader-app/blob/main/README.md"
              .to_string(),
            None,
          )
          .unwrap();
        }
        "Join Us on Discord" => {
          shell::open(
            &event.window().shell_scope(),
            "https://discord.gg/xCa23pwJWY",
            None,
          )
          .unwrap();
        }
        _ => {}
      }
    })
    .build(context)
    .expect("error while running tauri application");

  app.run(|app_handle, run_event| match run_event {
    tauri::RunEvent::WindowEvent { event, .. } => match event {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        api.prevent_close();
        dialog::confirm(
          Some(&app_handle.get_window("main").unwrap()),
          "Close spotDL GUI",
          "Are you sure?",
          |answer| {
            if answer {
              std::process::exit(0);
            }
          },
        )
      }
      _ => {}
    },
    _ => {}
  })
}
