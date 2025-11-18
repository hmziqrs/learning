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

- [x] LocalStorage for alarm persistence
- [x] Time picker for setting alarms
- [x] Date picker for setting alarms
- [x] Alarm title/label input
- [x] Add new alarm
- [x] Quick preset buttons (30s, 1min, 2min, 5min, 10min, 30min)
- [x] Quick date buttons (Today, Tomorrow)
- [x] List of upcoming alarms
- [x] Delete individual alarms
- [x] Toggle alarm on/off (pause/resume)
- [x] Alarm status (active/inactive/fired)
- [x] Scheduled notification delivery
- [x] Real-time countdown display
- [x] Alarm history
- [x] Clear all alarms

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
- [x] Permission status display
- [x] "Request Permission" button
- [x] Visual indicator (granted/denied)

### Quick Alarms Section
- [x] 6 preset buttons for common durations (30s, 1min, 2min, 5min, 10min, 30min)
- [x] Auto-fills date, time, and title when clicked

### Add Alarm Section (Custom)
- [x] Alarm title input field
- [x] Quick date buttons (Today, Tomorrow)
- [x] Date picker input
- [x] Time picker input
- [x] "Add Alarm" button

### Active Alarms List Section
- [x] Display upcoming alarms
- [x] Show alarm title and time
- [x] Real-time countdown timer (updates every second)
- [x] Pause/Resume toggle button
- [x] Delete alarm button
- [x] Auto-sorted by scheduled time

### Alarm History Section
- [x] Display fired alarms
- [x] Show fire timestamp
- [x] Delete individual history item
- [x] Clear all history button

### Output Panel
- [x] Display operation results with timestamps
- [x] Show success/error messages
- [x] Log alarm events
- [x] Clear output button

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
- [x] Install notification plugin dependencies
- [x] Configure permissions in capabilities/default.json
- [x] Register notification plugin in Rust
- [x] Use localStorage for persistence (instead of SQLite)

### Development Phase
- [x] Implement alarm scheduling logic
- [x] Create alarm form UI with quick presets
- [x] Build alarms list component
- [x] Add time picker component
- [x] Add date picker component
- [x] Implement permission check
- [x] Add error handling
- [x] Add loading states
- [x] Implement real-time countdown timer
- [x] Add pause/resume functionality

### Testing Phase
- [x] Test basic alarm creation
- [x] Test alarm persistence (localStorage)
- [x] Test alarm firing
- [x] Test quick preset buttons
- [ ] Test on desktop platforms (macOS pending)
- [ ] Test on mobile platforms
- [ ] Test edge cases

### Polish Phase
- [x] Improve UI/UX with quick presets
- [x] Add better error messages
- [x] Add success feedback with timestamps
- [x] Add visual sections for organization

## Implementation Status

**Status**: ✅ Implemented and Working

### Backend Configuration
- [x] Route: Active at `/alarms`
- [x] Component: Fully implemented in `src/routes/alarms.tsx`
- [x] Permissions: Configured in `src-tauri/capabilities/default.json`
- [x] Notification Plugin: Registered in `src-tauri/src/lib.rs`
- [x] Persistence: Using localStorage (no SQLite needed)

### Frontend Implementation
- [x] Alarm creation: Fully implemented with validation
- [x] Quick preset buttons: 6 time presets (30s - 30mins)
- [x] Quick date buttons: Today and Tomorrow
- [x] Alarms list: Active alarms with real-time countdown
- [x] Time/Date pickers: HTML5 native inputs
- [x] Persistence: localStorage save/load on mount
- [x] Alarm firing: Automatic detection and notification
- [x] Alarm history: Fired alarms tracking
- [x] Toggle functionality: Pause/Resume alarms

### Features Implemented
- ✅ Permission check and request
- ✅ Add alarm with date/time validation
- ✅ Quick preset buttons for common durations
- ✅ Quick date selection (Today/Tomorrow)
- ✅ Real-time countdown timer (updates every second)
- ✅ Active alarms list sorted by time
- ✅ Pause/Resume individual alarms
- ✅ Delete individual alarms
- ✅ Automatic alarm firing at scheduled time
- ✅ Alarm history with timestamps
- ✅ Clear history functionality
- ✅ Output panel with timestamped logs
- ✅ LocalStorage persistence
- ✅ Auto-load alarms on app startup

### Testing Results
- [x] Basic functionality: Working
- [x] Quick presets: Working
- [x] Alarm firing: Working
- [x] Persistence: Working
- [ ] Desktop (macOS): Pending manual testing
- [ ] Desktop (Windows): Pending testing
- [ ] Desktop (Linux): Pending testing
- [ ] Mobile: Pending testing

## Implementation Notes

### Time Handling
- Uses JavaScript Date objects for time operations
- Stores times in ISO format in localStorage
- Displays times in local timezone
- Real-time countdown updates every second

### Persistence Strategy
- Store all alarms in localStorage as JSON
- Auto-load alarms on app startup
- Auto-save on any alarm change
- Mark alarms as "fired" after delivery
- Keep alarm history for reference

### Scheduling Approach
- Uses Rust `schedule_notification` command with tokio
- Frontend checks alarms every second
- Marks alarms as fired when time arrives
- Automatically moves fired alarms to history

### Scheduling Limitations
- Alarms work only while app is running
- Notifications scheduled via Rust backend
- No true background alarms (app must be alive)
- For persistent alarms, would need Background Tasks module

### Best Practices Implemented
- ✅ Always check notification permission before operations
- ✅ Validate alarm times (prevents past times)
- ✅ Provide clear feedback with timestamps
- ✅ Visual indicators for alarm state
- ✅ Quick presets for better UX

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
