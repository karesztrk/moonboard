use moonsweeper::{
    model::{Keyboard, KeyboardModel::Moonlander},
    service::{Animation, SingleKey, Wipe},
};
use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn animate(app: State<'_, AppState>) -> Result<(), String> {
    let moonlander = Keyboard::new(Moonlander);
    let animation = Wipe::new(moonlander);
    let api = &app.api;
    animation.play(api).await;
    Ok(())
}

#[tauri::command]
pub async fn light_on_key(app: State<'_, AppState>, key: char) -> Result<(), String> {
    let moonlander = Keyboard::new(Moonlander);
    let animation = SingleKey::new(moonlander, key);
    let api = &app.api;
    animation.run(api).await;
    Ok(())
}
