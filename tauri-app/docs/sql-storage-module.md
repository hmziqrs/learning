# SQL + Storage Module Implementation

## Overview

Store user state, preferences, and application data using SQLite database and key-value store for persistent storage across desktop and mobile platforms.

## Plugin Setup

### Install Dependencies
```bash
bun add @tauri-apps/plugin-sql
bun add @tauri-apps/plugin-store
```

### Cargo Dependencies
Add to `src-tauri/Cargo.toml`:
```toml
[dependencies]
tauri-plugin-sql = { version = "2.0", features = ["sqlite"] }
tauri-plugin-store = "2.0"
```

### Register Plugins
In `src-tauri/src/lib.rs`:
```rust
.plugin(tauri_plugin_sql::Builder::default().build())
.plugin(tauri_plugin_store::Builder::default().build())
```

## Permissions Configuration

### Required Permissions
Add to `src-tauri/capabilities/default.json`:
```json
{
  "permissions": [
    "sql:allow-load",
    "sql:allow-execute",
    "sql:allow-select",
    "sql:allow-close",
    "store:allow-get",
    "store:allow-set",
    "store:allow-delete",
    "store:allow-clear",
    "store:allow-has",
    "store:allow-save"
  ]
}
```

## Core Features

### SQLite Database
- [ ] Initialize SQLite database
- [ ] Create tables for application data
- [ ] Insert records
- [ ] Query records
- [ ] Update records
- [ ] Delete records
- [ ] Transaction support
- [ ] Error handling

### Key-Value Store
- [ ] Store user preferences
- [ ] Retrieve stored values
- [ ] Update preferences
- [ ] Delete specific keys
- [ ] Clear all stored data
- [ ] Check if key exists
- [ ] Persist to disk

### Data Management
- [x] User profile storage (name, settings)
- [x] Dark mode preference persistence
- [x] Application state management
- [ ] Settings synchronization
- [x] Data export (JSON, CSV)
- [x] Complete data backup
- [ ] Data import
- [x] Clear all data functionality

## Database Schema

### User Preferences Table
```sql
CREATE TABLE IF NOT EXISTS user_preferences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

### Application Data Table
```sql
CREATE TABLE IF NOT EXISTS app_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category TEXT NOT NULL,
    data_key TEXT NOT NULL,
    data_value TEXT NOT NULL,
    metadata TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(category, data_key)
);
```

### Notes Table (Example)
```sql
CREATE TABLE IF NOT EXISTS notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT,
    tags TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

## Rust Backend

### Database Commands
```rust
#[tauri::command]
async fn init_database(
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Initialize SQLite database
    // Create tables
    // Return success/error
}

#[tauri::command]
async fn save_preference(
    key: String,
    value: String,
) -> Result<(), String> {
    // Save preference to database
}

#[tauri::command]
async fn get_preference(
    key: String,
) -> Result<Option<String>, String> {
    // Get preference from database
}

#[tauri::command]
async fn delete_preference(
    key: String,
) -> Result<(), String> {
    // Delete preference from database
}

#[tauri::command]
async fn clear_all_data() -> Result<(), String> {
    // Clear all tables
    // Reset store
}
```

### Store Commands
```rust
use tauri_plugin_store::StoreExt;

#[tauri::command]
async fn store_set(
    app: tauri::AppHandle,
    key: String,
    value: serde_json::Value,
) -> Result<(), String> {
    // Use store plugin to set value
}

#[tauri::command]
async fn store_get(
    app: tauri::AppHandle,
    key: String,
) -> Result<Option<serde_json::Value>, String> {
    // Use store plugin to get value
}
```

## Frontend Implementation

### API Integration
```typescript
import Database from '@tauri-apps/plugin-sql'
import { Store } from '@tauri-apps/plugin-store'

// Initialize database
const db = await Database.load('sqlite:app.db')

// Initialize store
const store = await Store.load('settings.json')
```

### SQLite Operations
```typescript
// Create table
await db.execute(`
  CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
  )
`)

// Insert data
await db.execute(
  'INSERT INTO users (name) VALUES (?)',
  [userName]
)

// Query data
const users = await db.select(
  'SELECT * FROM users'
)

// Update data
await db.execute(
  'UPDATE users SET name = ? WHERE id = ?',
  [newName, userId]
)

// Delete data
await db.execute(
  'DELETE FROM users WHERE id = ?',
  [userId]
)
```

### Store Operations
```typescript
// Save preference
await store.set('theme', 'dark')
await store.set('userName', 'John Doe')
await store.save()

// Get preference
const theme = await store.get('theme')
const userName = await store.get('userName')

// Check if key exists
const hasTheme = await store.has('theme')

// Delete preference
await store.delete('theme')
await store.save()

// Clear all
await store.clear()
await store.save()
```

## UI Components

### User Settings Section
- [ ] Input field for user name
- [ ] Save button
- [ ] Display current name

### Theme Preferences Section
- [ ] Dark mode toggle switch
- [ ] Theme persisted to store
- [ ] Auto-apply on load

### Data Management Section
- [ ] Display stored preferences count
- [ ] Button to clear all data
- [ ] Confirmation dialog before clearing
- [ ] Success/error feedback

### Database Demo Section
- [ ] Add note form (title + content)
- [ ] Notes list display
- [ ] Delete note button
- [ ] Edit note functionality
- [ ] Search/filter notes

### Output Panel
- [ ] Display operation results
- [ ] Show SQL queries executed
- [ ] Show store operations
- [ ] Clear output button
- [ ] Success/error indicators

## Testing Checklist

### Desktop Testing
- [ ] Test on Windows
- [ ] Test on macOS
- [ ] Test on Linux
- [ ] Test database persistence across restarts
- [ ] Test store persistence across restarts

### Mobile Testing
- [ ] Test on Android
- [ ] Test on iOS
- [ ] Test data persistence
- [ ] Test storage permissions
- [ ] Test app background/foreground data retention

### Edge Cases
- [ ] Handle database initialization errors
- [ ] Test with invalid SQL queries
- [ ] Test with large data sets
- [ ] Handle storage quota limits
- [ ] Test concurrent read/write operations
- [ ] Test store file corruption
- [ ] Handle missing store file
- [ ] Test special characters in data
- [ ] Test unicode/emoji support
- [ ] Handle database migration scenarios

## Progress Tracking

### Setup Phase
- [ ] Install SQL plugin dependencies
- [ ] Install Store plugin dependencies
- [ ] Configure permissions
- [ ] Register plugins in Rust
- [ ] Create database schema

### Development Phase
- [ ] Implement database initialization
- [ ] Create CRUD operations for SQLite
- [ ] Implement store operations
- [ ] Build user settings UI
- [ ] Build theme toggle with persistence
- [ ] Build notes demo feature
- [ ] Add data management controls
- [ ] Add error handling
- [ ] Add loading states
- [ ] Add form validation

### Testing Phase
- [ ] Test database operations
- [ ] Test store operations
- [ ] Test persistence across restarts
- [ ] Test on desktop platforms
- [ ] Test on mobile platforms
- [ ] Test edge cases
- [ ] Fix bugs

### Polish Phase
- [ ] Improve UI/UX
- [ ] Add better error messages
- [ ] Add success notifications
- [ ] Add loading indicators
- [ ] Code cleanup and documentation

## Implementation Status

**Status**: ✅ Implemented - Pending Full Testing

### Backend Configuration
- [x] Installed `@tauri-apps/plugin-sql` v2.3.1
- [x] Installed `@tauri-apps/plugin-store` v2.4.1
- [x] Added `tauri-plugin-store` to Cargo.toml
- [x] Registered store plugin in lib.rs
- [x] Configured SQL and Store permissions in capabilities/default.json

### Frontend Implementation
All core features have been implemented in `src/routes/sql-storage.tsx`:

#### Store Plugin (Key-Value Storage)
- ✅ Initialize Store plugin with `settings.json`
- ✅ Save user name preference
- ✅ Toggle and persist dark mode preference
- ✅ Load preferences on app start
- ✅ Check if keys exist with `.has()`
- ✅ Clear all preferences with `.clear()`

#### SQLite Database (Relational Storage)
- ✅ Initialize SQLite database `storage.db`
- ✅ Create notes table with auto-increment ID
- ✅ Insert notes with title and content
- ✅ Query all notes with `SELECT` statement
- ✅ Delete notes by ID with prepared statements
- ✅ Clear all notes with `DELETE` statement
- ✅ Database cleanup on component unmount

#### UI Features
- ✅ Storage statistics dashboard (preferences count, notes count)
- ✅ User preferences section with input and save button
- ✅ Dark mode toggle with persistent state
- ✅ Notes CRUD interface (add, view, delete)
- ✅ Data export functionality (JSON, CSV, complete backup)
- ✅ Data management section with clear all functionality
- ✅ Real-time output panel with operation logging
- ✅ Loading states on all buttons
- ✅ Error handling for all operations

#### Advanced Export Features
- ✅ Export notes to JSON format with metadata
- ✅ Export notes to CSV format with proper escaping
- ✅ Export complete data (notes + preferences) to JSON
- ✅ File save dialog integration
- ✅ Timestamp-based default filenames
- ✅ Disabled state when no data to export

### Testing Status
- [ ] Desktop (Windows) - Pending
- [ ] Desktop (macOS) - Pending
- [ ] Desktop (Linux) - Pending (requires GTK dependencies)
- [ ] Mobile (Android) - Pending
- [ ] Mobile (iOS) - Pending

### Notes
- Implementation uses Tauri SQL and Store plugins as documented
- All localStorage placeholders replaced with actual plugin calls
- Database uses prepared statements for SQL injection prevention
- Store requires explicit `.save()` call to persist changes
- Both plugins persist data across app restarts
- Database file: `storage.db` in app data directory
- Store file: `settings.json` in app data directory

### Export Features Detail

The module includes comprehensive data export functionality:

**1. Export Notes to JSON**
- Exports all notes with metadata (export date, total count)
- Pretty-printed JSON format (2-space indentation)
- File saved via native dialog with `.json` filter
- Default filename: `notes-export-{timestamp}.json`

**2. Export Notes to CSV**
- Exports notes in CSV format for spreadsheet applications
- Includes headers: ID, Title, Content, Created At
- Proper CSV escaping (quotes doubled)
- File saved via native dialog with `.csv` filter
- Default filename: `notes-export-{timestamp}.csv`

**3. Export Complete Data**
- Exports both notes and preferences in single JSON file
- Includes version number for future compatibility
- Structured format with separate sections for each data type
- File saved via native dialog with `.json` filter
- Default filename: `complete-export-{timestamp}.json`

**Example Export Format (Complete Data)**:
```json
{
  "exportDate": "2025-01-15T10:30:00.000Z",
  "version": "1.0",
  "data": {
    "preferences": {
      "userName": "John Doe",
      "isDarkMode": true
    },
    "notes": {
      "count": 3,
      "items": [
        {
          "id": 1,
          "title": "My Note",
          "content": "Note content",
          "created_at": "2025-01-15T10:00:00.000Z"
        }
      ]
    }
  }
}
```

## Implementation Notes

### Database Location
- Desktop: `~/.local/share/{app-name}/` or app data directory
- Mobile: App-specific storage directory
- Database file: `app.db` (or custom name)
- Store file: `settings.json` (or custom name)

### Store Plugin
- JSON-based key-value storage
- Automatically persists to disk
- Type-safe with TypeScript
- Supports nested objects
- Must call `.save()` to persist changes

### SQLite Plugin
- Full SQLite support
- Prepared statements for security
- Async operations
- Transaction support
- Custom SQL queries

### Data Persistence
- Both plugins persist data automatically
- Store requires explicit `.save()` call
- Database commits are automatic (unless in transaction)
- Data survives app restarts

### Performance Considerations
- Use prepared statements for repeated queries
- Batch operations when possible
- Index frequently queried columns
- Avoid storing large blobs in SQLite
- Use store for simple key-value data
- Use SQLite for relational data

### Security
- SQL injection prevention via prepared statements
- Store file permissions handled by plugin
- No sensitive data in plain text
- Consider encryption for sensitive data

## Resources

- [Tauri SQL Plugin](https://v2.tauri.app/plugin/sql/)
- [Tauri Store Plugin](https://v2.tauri.app/plugin/store/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)
- [SQL Tutorial](https://www.w3schools.com/sql/)
