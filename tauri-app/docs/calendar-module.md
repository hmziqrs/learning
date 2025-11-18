# Calendar Module (Internal + ICS Export) Implementation

## Overview

Internal calendar with event management and ICS export functionality to integrate with system calendar applications.

## Plugin Setup

### Install Dependencies
```bash
bun add @tauri-apps/plugin-sql
bun add @tauri-apps/plugin-fs
bun add @tauri-apps/plugin-opener
```

### Cargo Dependencies
Add to `src-tauri/Cargo.toml`:
```toml
[dependencies]
tauri-plugin-sql = "2.0"
tauri-plugin-fs = "2.0"
tauri-plugin-opener = "2.0"
```

### Register Plugins
In `src-tauri/src/lib.rs`:
```rust
.plugin(tauri_plugin_sql::Builder::default().build())
.plugin(tauri_plugin_fs::init())
.plugin(tauri_plugin_opener::init())
```

## Permissions Configuration

### Required Permissions
Add to `src-tauri/capabilities/default.json`:
```json
{
  "permissions": [
    "sql:allow-load",
    "sql:allow-execute",
    "fs:allow-write-text-file",
    "fs:allow-create",
    "opener:allow-open"
  ]
}
```

## Core Features

- [ ] Create calendar events (id, title, start, end)
- [ ] View events in list format
- [ ] Mini month view calendar
- [ ] Add event form with validation
- [ ] Edit existing events
- [ ] Delete events
- [ ] Export events as ICS file
- [ ] Open ICS file in system calendar
- [ ] Event persistence with SQLite
- [ ] Event search/filter
- [ ] All-day event support
- [ ] Event descriptions/notes

## Database Schema

### Events Table
```sql
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    is_all_day BOOLEAN DEFAULT 0,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

## ICS Export Format

### ICS File Structure
```
BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//Tauri Calendar//EN
CALSCALE:GREGORIAN
BEGIN:VEVENT
UID:{event-id}@tauri-calendar
DTSTART:{start_time}
DTEND:{end_time}
SUMMARY:{title}
DESCRIPTION:{description}
END:VEVENT
END:VCALENDAR
```

## Rust Backend

### Database Commands
```rust
#[tauri::command]
async fn create_event(
    state: tauri::State<'_, DatabaseState>,
    title: String,
    description: Option<String>,
    start_time: String,
    end_time: String,
    is_all_day: bool,
) -> Result<i64, String> {
    // Insert event into database
    // Return event ID
}

#[tauri::command]
async fn get_events(
    state: tauri::State<'_, DatabaseState>,
) -> Result<Vec<Event>, String> {
    // Fetch all events from database
}

#[tauri::command]
async fn update_event(
    state: tauri::State<'_, DatabaseState>,
    id: i64,
    title: String,
    description: Option<String>,
    start_time: String,
    end_time: String,
    is_all_day: bool,
) -> Result<(), String> {
    // Update event in database
}

#[tauri::command]
async fn delete_event(
    state: tauri::State<'_, DatabaseState>,
    id: i64,
) -> Result<(), String> {
    // Delete event from database
}
```

### ICS Export Command
```rust
use std::fs;

#[tauri::command]
async fn export_events_to_ics(
    state: tauri::State<'_, DatabaseState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    // Fetch all events
    // Generate ICS content
    // Write to file using FS plugin
    // Return file path
}

#[tauri::command]
async fn generate_ics_content(events: Vec<Event>) -> Result<String, String> {
    // Format events into ICS format
    // Return ICS string
}
```

## Frontend Implementation

### API Integration
```typescript
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-opener'
import { writeTextFile } from '@tauri-apps/plugin-fs'

interface Event {
  id: number
  title: string
  description?: string
  start_time: string
  end_time: string
  is_all_day: boolean
  created_at: string
  updated_at: string
}
```

### Create Event
```typescript
const createEvent = async (
  title: string,
  startTime: string,
  endTime: string,
  description?: string,
  isAllDay: boolean = false
) => {
  const eventId = await invoke('create_event', {
    title,
    description,
    startTime,
    endTime,
    isAllDay,
  })
  return eventId
}
```

### Fetch Events
```typescript
const fetchEvents = async (): Promise<Event[]> => {
  return await invoke('get_events')
}
```

### Export to ICS
```typescript
const exportToICS = async () => {
  const filePath = await invoke('export_events_to_ics')
  return filePath
}

const openInSystemCalendar = async (filePath: string) => {
  await open(filePath)
}
```

## UI Components

### Add Event Section
- [ ] Event title input field
- [ ] Event description textarea
- [ ] Start date picker
- [ ] Start time picker
- [ ] End date picker
- [ ] End time picker
- [ ] All-day event checkbox
- [ ] "Add Event" button
- [ ] Form validation

### Events List Section
- [ ] Display all events sorted by date
- [ ] Show event title and time range
- [ ] Visual distinction for all-day events
- [ ] Edit event button
- [ ] Delete event button
- [ ] Event count display

### Mini Calendar View Section
- [ ] Month/Year header with navigation
- [ ] Calendar grid (7 columns × 5-6 rows)
- [ ] Current day highlight
- [ ] Event indicators on dates
- [ ] Click date to filter events

### Export Section
- [ ] "Export to ICS" button
- [ ] "Open in System Calendar" button
- [ ] Export success notification
- [ ] File path display

### Output Panel
- [ ] Display operation results
- [ ] Show success/error messages
- [ ] Log event operations
- [ ] Clear output button

## Testing Checklist

### Desktop Testing
- [ ] Test on Windows
- [ ] Test on macOS
- [ ] Test on Linux
- [ ] Test event creation
- [ ] Test event editing
- [ ] Test event deletion
- [ ] Test ICS export
- [ ] Test opening in system calendar
- [ ] Test database persistence

### Mobile Testing
- [ ] Test on Android
- [ ] Test on iOS
- [ ] Test calendar UI responsiveness
- [ ] Test ICS export on mobile
- [ ] Test opening in mobile calendar apps

### Edge Cases
- [ ] Handle invalid date ranges (end before start)
- [ ] Test with empty events list
- [ ] Test with multiple events on same day
- [ ] Test all-day events
- [ ] Test multi-day events
- [ ] Test ICS file generation errors
- [ ] Test calendar opener fallback
- [ ] Test database errors
- [ ] Test very long event titles/descriptions

## Progress Tracking

### Setup Phase
- [ ] Install SQL plugin dependencies
- [ ] Install FS plugin dependencies
- [ ] Install opener plugin dependencies
- [ ] Configure permissions in capabilities/default.json
- [ ] Register plugins in Rust
- [ ] Create events database table

### Development Phase
- [ ] Implement event CRUD operations
- [ ] Create add event form UI
- [ ] Build events list component
- [ ] Implement mini calendar view
- [ ] Add date/time pickers
- [ ] Implement ICS export logic
- [ ] Add system calendar opener integration
- [ ] Add error handling
- [ ] Add loading states
- [ ] Add form validation

### Testing Phase
- [ ] Test basic event creation
- [ ] Test event persistence
- [ ] Test ICS export format
- [ ] Test system calendar integration
- [ ] Test on desktop platforms
- [ ] Test on mobile platforms
- [ ] Test edge cases

### Polish Phase
- [ ] Improve UI/UX
- [ ] Add better error messages
- [ ] Add success feedback
- [ ] Add visual calendar enhancements
- [ ] Add event color coding (optional)

## Implementation Status

**Status**: 🚧 Not Started

### Backend Configuration
- [ ] Route: Create `/calendar` route
- [ ] Component: Create `src/routes/calendar.tsx`
- [ ] Permissions: Configure in `src-tauri/capabilities/default.json`
- [ ] Plugins: Register SQL, FS, and opener plugins
- [ ] Database: Create events table schema

### Frontend Implementation
- [ ] Event creation: Not started
- [ ] Event list: Not started
- [ ] Mini calendar view: Not started
- [ ] ICS export: Not started
- [ ] System calendar opener: Not started
- [ ] Database integration: Not started

### Features Implemented
- ⏳ Event CRUD operations
- ⏳ Mini calendar view
- ⏳ ICS export
- ⏳ System calendar integration
- ⏳ Database persistence

### Testing Results
- [ ] Basic functionality: Not tested
- [ ] ICS export: Not tested
- [ ] Desktop: Not tested
- [ ] Mobile: Not tested

## Implementation Notes

### ICS File Format
- Use RFC 5545 standard for ICS files
- Handle timezone conversions properly
- Support all-day events with DATE format
- Include UID for each event

### Time Handling
- Store times in ISO 8601 format
- Handle timezone conversions
- Support all-day events
- Validate date ranges

### File Storage
- Store ICS files in app data directory
- Use temporary files for export
- Handle file permissions properly

### Calendar Integration
- Use opener plugin for cross-platform support
- Fallback to file explorer if no calendar app
- Handle mobile-specific calendar apps

## Known Limitations

- ICS export is one-way (no import from system calendar)
- No recurring event support initially
- No event reminders (use Alarms module instead)
- Limited calendar view (month only)
- No event categories/tags initially

## Resources

- [Tauri SQL Plugin](https://v2.tauri.app/plugin/sql/)
- [Tauri FS Plugin](https://v2.tauri.app/plugin/fs/)
- [Tauri Opener Plugin](https://v2.tauri.app/plugin/opener/)
- [ICS/iCalendar RFC 5545](https://www.rfc-editor.org/rfc/rfc5545)
- [iCalendar Format Guide](https://icalendar.org/)
