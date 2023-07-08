use crate::constants;
use tauri::Manager;

pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
  let app: tauri::AppHandle = app.app_handle();

  tauri::async_runtime::spawn(async move {
    let main_window: tauri::WindowBuilder<'_> =
      tauri::WindowBuilder::new(&app, "main", tauri::WindowUrl::App("/".into()))
        .title(constants::APP_NAME)
        .center()
        .fullscreen(false)
        .inner_size(constants::MAIN_WIDTH, constants::MAIN_HEIGHT)
        .maximizable(false)
        .resizable(false)
        .visible(true);

    if let Err(error) = main_window.build() {
      log::error!("Error while building application: {}", error);
    } else {
      log::info!("Application build successful.");
    }

    #[cfg(debug_assertions)]
    app.get_window("main").unwrap().open_devtools();
  });

  Ok(())
}
