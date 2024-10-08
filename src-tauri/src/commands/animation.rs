use moonsweeper::{
    model::{
        Keyboard,
        KeyboardLayout::{self, Norman},
        KeyboardModel::Moonlander,
    },
    service::{Animation, Clear, Sequence, SingleKey, Wipe},
};
use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn wipe_animation(app: State<'_, AppState>) -> Result<(), String> {
    let moonlander = Keyboard::new(Moonlander, Norman);
    let animation = Wipe::new(moonlander);
    let api = &app.api;
    animation.play(api).await;
    Ok(())
}

#[tauri::command]
pub async fn sequence_animation(app: State<'_, AppState>) -> Result<(), String> {
    let moonlander = Keyboard::new(Moonlander, Norman);
    let animation = Sequence::new(moonlander);
    let api = &app.api;
    animation.play(api).await;
    Ok(())
}

#[tauri::command]
pub async fn light_on_key(
    app: State<'_, AppState>,
    key: char,
    layout: KeyboardLayout,
) -> Result<(), String> {
    let moonlander = Keyboard::new(Moonlander, layout);
    let animation = SingleKey::new(moonlander, key);
    let api = &app.api;
    animation.run(api).await;
    Ok(())
}

#[tauri::command]
pub async fn clear(app: State<'_, AppState>) -> Result<(), String> {
    let animation = Clear::new();
    let api = &app.api;
    animation.run(api).await;
    Ok(())
}

#[tauri::command]
pub async fn reset(app: State<'_, AppState>) -> Result<(), String> {
    let api = &app.api;
    let _ = api.restore_rgb_leds().await;
    Ok(())
}
