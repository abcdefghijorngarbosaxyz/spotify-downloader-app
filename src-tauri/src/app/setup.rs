use crate::app;

use log::{error, info};
use tauri::{App, AppHandle, Context, Manager, WindowBuilder};

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
  info!("[INFO] Setup");
  let app: AppHandle = app.handle();
  let name: String = app.package_info().name.clone();

  tauri::async_runtime::spawn(async move {
    let main_url: &str = "/";

    let mut main_window: WindowBuilder<'_> =
      WindowBuilder::new(&app, "main", tauri::WindowUrl::App(main_url.into()))
        .title(name)
        .resizable(false)
        .fullscreen(false)
        .center()
        .maximizable(false)
        .inner_size(800.0, 600.0);

    #[cfg(target_os = "macos")]
    {
      main_window = main_window.hidden_title(true);
    }

    if let Err(err) = main_window.build() {
      error!("Build main_window: {}", err);
    }
  });
  Ok(())
}
