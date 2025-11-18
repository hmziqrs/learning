// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_notification::NotificationExt;
use std::time::Duration;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn schedule_notification(
    app: tauri::AppHandle,
    seconds: u64,
    title: String,
    body: String,
) -> Result<(), String> {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(seconds)).await;

        let _ = app.notification()
            .builder()
            .title(&title)
            .body(&body)
            .show();
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_deep_link::init())
        .invoke_handler(tauri::generate_handler![greet, schedule_notification])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
