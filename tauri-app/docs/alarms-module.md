# Alarms (Future Notifications) Module Implementation

## Overview

Lightweight alarm simulation using scheduled notifications with persistence.

## Plugin Setup

### Install Dependencies
```bash
bun add @tauri-apps/plugin-notification
bun add @tauri-apps/plugin-sql
```

### Cargo Dependencies
Add to `src-tauri/Cargo.toml`:
```toml
[dependencies]
tauri-plugin-notification = "2.0"
tauri-plugin-sql = "2.0"
tokio = { version = "1", features = ["full", "time"] }
```

### Register Plugins
In `src-tauri/src/lib.rs`:
```rust
.plugin(tauri_plugin_notification::init())
.plugin(tauri_plugin_sql::Builder::default().build())
```

## Permissions Configuration

### Required Permissions
Add to `src-tauri/capabilities/default.json`:
```json
{
  "permissions": [
    "notification:allow-is-permission-granted",
    "notification:allow-request-permission",
    "notification:allow-notify",
    "notification:allow-register-listener",
    "sql:allow-load",
    "sql:allow-execute"
  ]
}
```

## Core Features

- [ ] SQLite database for alarm persistence
- [ ] Time picker for setting alarms
- [ ] Alarm title/label input
- [ ] Add new alarm
- [ ] List of upcoming alarms
- [ ] Delete individual alarms
- [ ] Toggle alarm on/off
- [ ] Alarm status (active/inactive/fired)
- [ ] Scheduled notification delivery
- [ ] Alarm history
- [ ] Clear all alarms

## Database Schema

### Alarms Table
```sql
CREATE TABLE IF NOT EXISTS alarms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    scheduled_time TEXT NOT NULL,
    is_active BOOLEAN DEFAULT 1,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    fired_at TEXT
);
```

## Rust Backend

### Schedule Alarm Command
Create in `src-tauri/src/lib.rs`:

```rust
use tauri_plugin_notification::NotificationExt;
use std::time::Duration;
use chrono::{DateTime, Utc, Local};

#[tauri::command]
async fn schedule_alarm(
    app: tauri::AppHandle,
    alarm_id: i64,
    title: String,
    scheduled_time: String,
) -> Result<(), String> {
    let scheduled: DateTime<Local> = scheduled_time
        .parse()
        .map_err(|e| format!("Invalid time format: {}", e))?;

    let now = Local::now();
    let duration = (scheduled - now).to_std()
        .map_err(|e| format!("Time calculation error: {}", e))?;

    tokio::spawn(async move {
        tokio::time::sleep(duration).await;

        let _ = app.notification()
            .builder()
            .title(&title)
            .body("Alarm")
            .show();
    });

    Ok(())
}
```

### Database Commands
```rust
#[tauri::command]
async fn create_alarm(
    state: tauri::State<'_, DatabaseState>,
    title: String,
    scheduled_time: String,
) -> Result<i64, String> {
    // Insert alarm into database
    // Return alarm ID
}

#[tauri::command]
async fn get_alarms(
    state: tauri::State<'_, DatabaseState>,
) -> Result<Vec<Alarm>, String> {
    // Fetch all alarms from database
}

#[tauri::command]
async fn delete_alarm(
    state: tauri::State<'_, DatabaseState>,
    id: i64,
) -> Result<(), String> {
    // Delete alarm from database
}

#[tauri::command]
async fn toggle_alarm(
    state: tauri::State<'_, DatabaseState>,
    id: i64,
    is_active: bool,
) -> Result<(), String> {
    // Toggle alarm active status
}
```

## Frontend Implementation

### API Integration
```typescript
import { invoke } from '@tauri-apps/api/core'
import {
  isPermissionGranted,
  requestPermission,
} from '@tauri-apps/plugin-notification'

interface Alarm {
  id: number
  title: string
  scheduled_time: string
  is_active: boolean
  created_at: string
  fired_at?: string
}
```

### Create Alarm
```typescript
const createAlarm = async (title: string, scheduledTime: string) => {
  const alarmId = await invoke('create_alarm', {
    title,
    scheduledTime,
  })

  await invoke('schedule_alarm', {
    alarmId,
    title,
    scheduledTime,
  })
}
```

### Fetch Alarms
```typescript
const fetchAlarms = async (): Promise<Alarm[]> => {
  return await invoke('get_alarms')
}
```

## UI Components

### Permission Section
- [ ] Permission status display
- [ ] "Request Permission" button
- [ ] Visual indicator (granted/denied)

### Add Alarm Section
- [ ] Time picker input
- [ ] Date picker input
- [ ] Alarm title input field
- [ ] "Add Alarm" button

### Alarms List Section
- [ ] Display upcoming alarms
- [ ] Show alarm title and time
- [ ] Time until alarm fires
- [ ] Active/inactive toggle switch
- [ ] Delete alarm button
- [ ] Sort by scheduled time

### Output Panel
- [ ] Display operation results
- [ ] Show success/error messages
- [ ] Log alarm events

## Testing Checklist

### Desktop Testing
- [ ] Test on Windows
- [ ] Test on macOS
- [ ] Test on Linux
- [ ] Test notification permission
- [ ] Test alarm scheduling
- [ ] Test alarm firing
- [ ] Test database persistence

### Mobile Testing
- [ ] Test on Android
- [ ] Test on iOS
- [ ] Verify alarm delivery
- [ ] Test app in background

### Edge Cases
- [ ] Handle permission denial
- [ ] Test with past times
- [ ] Test with multiple alarms
- [ ] Test alarm persistence after app restart
- [ ] Handle app kill/restart
- [ ] Test database errors

## Progress Tracking

### Setup Phase
- [ ] Install notification plugin dependencies
- [ ] Install SQL plugin dependencies
- [ ] Configure permissions in capabilities/default.json
- [ ] Register plugins in Rust
- [ ] Create database schema

### Development Phase
- [ ] Implement database commands
- [ ] Implement alarm scheduling logic
- [ ] Create alarm form UI
- [ ] Build alarms list component
- [ ] Add time picker component
- [ ] Implement permission check
- [ ] Add error handling
- [ ] Add loading states

### Testing Phase
- [ ] Test on desktop platforms
- [ ] Test on mobile platforms
- [ ] Test alarm persistence
- [ ] Test edge cases
- [ ] Fix bugs

### Polish Phase
- [ ] Improve UI/UX
- [ ] Add better error messages
- [ ] Add success feedback
- [ ] Code cleanup and documentation

## Implementation Status

**Status**: Not Started

### Backend Configuration
- [ ] Route: Not created
- [ ] Component: Not created
- [ ] Permissions: Not configured
- [ ] Plugins: Not installed
- [ ] Database: Not created

### Frontend Implementation
- [ ] Alarm creation: Not implemented
- [ ] Alarms list: Not implemented
- [ ] Time picker: Not implemented
- [ ] Database integration: Not implemented

### Testing Results
- [ ] Desktop: Not tested
- [ ] Mobile: Not tested

## Implementation Notes

### Time Handling
- Use `chrono` crate for Rust time operations
- Store times in UTC format
- Display times in local timezone
- Calculate duration until alarm fires

### Persistence Strategy
- Store all alarms in SQLite database
- Re-schedule alarms on app startup
- Mark alarms as "fired" after delivery
- Keep alarm history for reference

### Scheduling Limitations
- Alarms work only while app is running
- For true background alarms, use Background Tasks module
- Consider adding "snooze" functionality
- Add alarm repeat patterns (daily, weekly, etc.)

### Best Practices
- Always check notification permission
- Validate alarm times (no past times)
- Handle timezone changes
- Provide clear feedback to user

## Known Limitations

- Alarms only work while app is alive
- No system-level alarm integration
- Limited to notification-based alarms
- Requires app to be running for alarm to fire
- Background task support needed for true persistent alarms

## Resources

- [Tauri Notification Plugin](https://v2.tauri.app/plugin/notification/)
- [Tauri SQL Plugin](https://v2.tauri.app/plugin/sql/)
- [Tokio Time Module](https://docs.rs/tokio/latest/tokio/time/)
- [Chrono Documentation](https://docs.rs/chrono/latest/chrono/)
