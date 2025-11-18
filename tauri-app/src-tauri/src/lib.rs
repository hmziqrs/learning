// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_notification::NotificationExt;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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

// Calendar Module Structs
#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: i64,
    title: String,
    description: Option<String>,
    start_time: String,
    end_time: String,
    is_all_day: bool,
    created_at: String,
    updated_at: String,
}

// Calendar Module Commands
#[tauri::command]
async fn init_calendar_db(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_sql::Builder;

    let db = app.state::<tauri_plugin_sql::Db>();

    db.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            is_all_day BOOLEAN DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        vec![],
    )
    .await
    .map_err(|e| format!("Failed to create events table: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn create_event(
    app: tauri::AppHandle,
    title: String,
    description: Option<String>,
    start_time: String,
    end_time: String,
    is_all_day: bool,
) -> Result<i64, String> {
    let db = app.state::<tauri_plugin_sql::Db>();

    let result = db
        .execute(
            "INSERT INTO events (title, description, start_time, end_time, is_all_day, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
            vec![
                serde_json::Value::String(title),
                description.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null),
                serde_json::Value::String(start_time),
                serde_json::Value::String(end_time),
                serde_json::Value::Bool(is_all_day),
            ],
        )
        .await
        .map_err(|e| format!("Failed to create event: {}", e))?;

    Ok(result.last_insert_id)
}

#[tauri::command]
async fn get_events(app: tauri::AppHandle) -> Result<Vec<Event>, String> {
    let db = app.state::<tauri_plugin_sql::Db>();

    let events: Vec<Event> = db
        .select("SELECT * FROM events ORDER BY start_time ASC")
        .await
        .map_err(|e| format!("Failed to get events: {}", e))?;

    Ok(events)
}

#[tauri::command]
async fn update_event(
    app: tauri::AppHandle,
    id: i64,
    title: String,
    description: Option<String>,
    start_time: String,
    end_time: String,
    is_all_day: bool,
) -> Result<(), String> {
    let db = app.state::<tauri_plugin_sql::Db>();

    db.execute(
        "UPDATE events SET title = ?, description = ?, start_time = ?, end_time = ?, is_all_day = ?, updated_at = datetime('now')
         WHERE id = ?",
        vec![
            serde_json::Value::String(title),
            description.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null),
            serde_json::Value::String(start_time),
            serde_json::Value::String(end_time),
            serde_json::Value::Bool(is_all_day),
            serde_json::Value::Number(id.into()),
        ],
    )
    .await
    .map_err(|e| format!("Failed to update event: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn delete_event(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let db = app.state::<tauri_plugin_sql::Db>();

    db.execute(
        "DELETE FROM events WHERE id = ?",
        vec![serde_json::Value::Number(id.into())],
    )
    .await
    .map_err(|e| format!("Failed to delete event: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn export_events_to_ics(app: tauri::AppHandle) -> Result<String, String> {
    let db = app.state::<tauri_plugin_sql::Db>();

    // Get all events
    let events: Vec<Event> = db
        .select("SELECT * FROM events ORDER BY start_time ASC")
        .await
        .map_err(|e| format!("Failed to get events: {}", e))?;

    // Generate ICS content
    let ics_content = generate_ics_content(events)?;

    // Write to file using FS plugin
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    let ics_path = app_data_dir.join("calendar_export.ics");

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    std::fs::write(&ics_path, ics_content)
        .map_err(|e| format!("Failed to write ICS file: {}", e))?;

    Ok(ics_path.to_string_lossy().to_string())
}

fn generate_ics_content(events: Vec<Event>) -> Result<String, String> {
    let mut ics = String::from("BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//Tauri Calendar//EN\r\nCALSCALE:GREGORIAN\r\n");

    for event in events {
        let start = format_ics_date(&event.start_time, event.is_all_day)?;
        let end = format_ics_date(&event.end_time, event.is_all_day)?;

        ics.push_str("BEGIN:VEVENT\r\n");
        ics.push_str(&format!("UID:{}@tauri-calendar\r\n", event.id));
        ics.push_str(&format!("DTSTART:{}\r\n", start));
        ics.push_str(&format!("DTEND:{}\r\n", end));
        ics.push_str(&format!("SUMMARY:{}\r\n", event.title));

        if let Some(desc) = event.description {
            ics.push_str(&format!("DESCRIPTION:{}\r\n", desc));
        }

        ics.push_str("END:VEVENT\r\n");
    }

    ics.push_str("END:VCALENDAR\r\n");
    Ok(ics)
}

fn format_ics_date(date_str: &str, is_all_day: bool) -> Result<String, String> {
    if is_all_day {
        // For all-day events, use DATE format (YYYYMMDD)
        let date = date_str.split('T').next().unwrap_or(date_str);
        Ok(date.replace("-", ""))
    } else {
        // For timed events, use DATE-TIME format (YYYYMMDDTHHMMSSZ)
        Ok(date_str.replace(&['-', ':'][..], "").replace(".000Z", "Z"))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:calendar.db", vec![])
                .build()
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            schedule_notification,
            init_calendar_db,
            create_event,
            get_events,
            update_event,
            delete_event,
            export_events_to_ics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
