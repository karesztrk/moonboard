mod commands;

use kontroll::Kontroll;
use tauri::{async_runtime, Builder, Manager};

struct AppState {
    api: Kontroll,
}

pub fn run() {
    Builder::default()
        .setup(|app| {
            let handler_clone = app.handle().clone();
            async_runtime::spawn(async move {
                let api = Kontroll::new(None).await.unwrap();
                handler_clone.manage(AppState { api });
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::animation::wipe_animation,
            commands::animation::sequence_animation,
            commands::animation::light_on_key,
            commands::animation::clear,
            commands::animation::reset,
            commands::animation::torpedo_on_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
