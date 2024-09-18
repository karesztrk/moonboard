// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kontroll::Kontroll;
use moonsweeper::{
    model::{Keyboard, KeyboardModel::Moonlander},
    service::{Animation, Wipe},
};
use tauri::{async_runtime, Builder, Manager, State};

struct AppState {
    api: Kontroll,
}

#[tauri::command]
async fn animate(app: State<'_, AppState>) -> Result<(), String> {
    let moonlander = Keyboard::new(Moonlander);
    let animation = Wipe::new(moonlander);
    let api = &app.api;
    animation.play(api).await;
    Ok(())
}

fn main() {
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
        .invoke_handler(tauri::generate_handler![animate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
