# App Lifecycle & OS Integration Module

## Overview
Monitor app and window lifecycle events, detect system theme and platform info, manage window states, and integrate with OS dialogs.

---

## Plugin Setup

### Install Dependencies
```bash
bun add @tauri-apps/api
```

### Cargo Dependencies
Add to `src-tauri/Cargo.toml`:
```toml
[dependencies]
tauri = { version = "2.0", features = ["macos-private-api"] }
tauri-plugin-dialog = "2.0"
```

### Register Plugins
In `src-tauri/src/lib.rs`:
```rust
.plugin(tauri_plugin_dialog::init())
```

---

## Permissions Configuration

### Required Permissions
Add to `src-tauri/capabilities/default.json`:
```json
{
  "permissions": [
    "dialog:allow-message",
    "dialog:allow-ask",
    "dialog:allow-confirm",
    "core:window:allow-set-title",
    "core:window:allow-set-size",
    "core:window:allow-set-position",
    "core:window:allow-center",
    "core:window:allow-minimize",
    "core:window:allow-maximize",
    "core:window:allow-unmaximize",
    "core:window:allow-show",
    "core:window:allow-hide",
    "core:window:allow-close",
    "core:window:allow-set-decorations",
    "core:window:allow-set-fullscreen",
    "core:window:allow-is-fullscreen",
    "core:window:allow-is-minimized",
    "core:window:allow-is-maximized",
    "core:window:allow-is-focused",
    "core:window:allow-is-visible",
    "core:window:allow-theme",
    "core:event:allow-listen",
    "core:event:allow-unlisten"
  ]
}
```

---

## Core Features

### 1. Window Lifecycle Events
- [ ] Listen to window focus events
- [ ] Listen to window blur events
- [ ] Listen to window resize events
- [ ] Listen to window move events
- [ ] Listen to window close requested events
- [ ] Display event logs in UI
- [ ] Track event count

### 2. Window State Management
- [ ] Get current window state (focused, minimized, maximized, visible)
- [ ] Minimize window
- [ ] Maximize/Unmaximize window
- [ ] Show/Hide window
- [ ] Close window
- [ ] Center window
- [ ] Toggle fullscreen mode
- [ ] Display current state

### 3. Window Property Control
- [ ] Set window title
- [ ] Set window size
- [ ] Set window position
- [ ] Toggle window decorations
- [ ] Get current theme (light/dark)
- [ ] Display window properties

### 4. System Information
- [ ] Detect operating system (Windows, macOS, Linux, iOS, Android)
- [ ] Get OS version
- [ ] Get architecture (x86_64, aarch64)
- [ ] Get app version
- [ ] Display system info

### 5. System Dialogs
- [ ] Show message dialog
- [ ] Show confirmation dialog
- [ ] Show ask dialog (with input)
- [ ] Handle dialog responses
- [ ] Display dialog results

### 6. Process Information
- [ ] Get process ID
- [ ] Get app uptime
- [ ] Monitor memory usage (if available)
- [ ] Display process info

---

## Rust Backend

### System Info Command
Create in `src-tauri/src/lib.rs`:

```rust
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(serde::Serialize)]
struct SystemInfo {
    os: String,
    version: String,
    arch: String,
    app_version: String,
    process_id: u32,
}

#[tauri::command]
fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        version: std::env::consts::VERSION.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        process_id: std::process::id(),
    }
}

#[tauri::command]
fn get_app_uptime(start_time: u64) -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    now - start_time
}
```

Register commands:
```rust
.invoke_handler(tauri::generate_handler![
    get_system_info,
    get_app_uptime
])
```

---

## Frontend Implementation

### API Integration
```typescript
import { getCurrent } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { message, ask, confirm } from '@tauri-apps/plugin-dialog';
```

### Window Event Listeners
```typescript
// Listen to window events
const unlisten = await listen('tauri://focus', () => {
  console.log('Window focused');
});

// Cleanup
unlisten();
```

### Window State Management
```typescript
const window = getCurrent();

// Check states
const isFocused = await window.isFocused();
const isMinimized = await window.isMinimized();
const isMaximized = await window.isMaximized();

// Control window
await window.minimize();
await window.maximize();
await window.show();
await window.hide();
```

### System Dialogs
```typescript
// Message dialog
await message('This is a message', 'App Lifecycle');

// Confirmation dialog
const confirmed = await confirm('Are you sure?', 'Confirm');

// Ask dialog
const answer = await ask('Enter your name:', 'Input');
```

---

## UI Components

### Window Events Section
- [ ] Event log display
- [ ] "Start Listening" button
- [ ] "Stop Listening" button
- [ ] Event counter
- [ ] Clear events button

### Window State Section
- [ ] Current state display (focused, minimized, maximized, visible)
- [ ] "Refresh State" button
- [ ] State indicators (green/red)

### Window Controls Section
- [ ] "Minimize" button
- [ ] "Maximize" button
- [ ] "Show" button
- [ ] "Hide" button
- [ ] "Center" button
- [ ] "Toggle Fullscreen" button

### Window Properties Section
- [ ] Title input field
- [ ] "Set Title" button
- [ ] Width input field
- [ ] Height input field
- [ ] "Set Size" button
- [ ] "Toggle Decorations" button
- [ ] "Get Theme" button

### System Info Section
- [ ] OS name display
- [ ] OS version display
- [ ] Architecture display
- [ ] App version display
- [ ] Process ID display
- [ ] Uptime display
- [ ] "Refresh Info" button

### System Dialogs Section
- [ ] Message input field
- [ ] "Show Message" button
- [ ] "Show Confirm" button
- [ ] "Show Ask" button
- [ ] Dialog result display

### Output Panel
- [ ] Display operation results
- [ ] Show success/error messages
- [ ] Log all actions

---

## Testing Checklist

### Desktop Testing
- [ ] Test window events on Windows
- [ ] Test window events on macOS
- [ ] Test window events on Linux
- [ ] Test window controls on all desktop platforms
- [ ] Test system dialogs on all desktop platforms
- [ ] Test theme detection on all desktop platforms

### Mobile Testing
- [ ] Test app lifecycle on Android
- [ ] Test app lifecycle on iOS
- [ ] Test system dialogs on mobile
- [ ] Test system info on mobile

### Edge Cases
- [ ] Handle rapid event firing
- [ ] Test window controls when already in state (e.g., minimize when minimized)
- [ ] Test theme changes
- [ ] Handle dialog dismissal
- [ ] Test fullscreen on different screen sizes

---

## Implementation Notes

### Platform Differences
- **Desktop**: Full window control and event support
- **Mobile**: Limited window controls, focus on app lifecycle
- **macOS**: May require `macos-private-api` feature for some events
- **Linux**: Window manager dependent behavior

### Event Listeners
- Always clean up event listeners to prevent memory leaks
- Use `unlisten()` function returned by `listen()`
- Consider using React's `useEffect` cleanup for automatic management

### Window State
- Some operations may be no-ops on certain platforms
- Window visibility behaves differently on mobile vs desktop
- Fullscreen support varies by platform

### Best Practices
- Check window state before performing operations
- Provide user feedback for all actions
- Handle platform-specific limitations gracefully
- Use async operations with proper error handling

---

## Progress Tracking

### Setup Phase
- [ ] Install plugin dependencies
- [ ] Configure permissions
- [ ] Register plugins in Rust
- [ ] Add custom commands

### Development Phase
- [ ] Implement window event listeners
- [ ] Implement window state checks
- [ ] Implement window controls
- [ ] Implement window property setters
- [ ] Implement system info retrieval
- [ ] Implement system dialogs
- [ ] Build UI components
- [ ] Add error handling
- [ ] Add loading states

### Testing Phase
- [ ] Test on desktop platforms
- [ ] Test on mobile platforms
- [ ] Test edge cases
- [ ] Fix bugs

### Polish Phase
- [ ] Improve UI/UX
- [ ] Add better error messages
- [ ] Add success feedback
- [ ] Code cleanup and documentation

---

## Implementation Status

### ⏳ Planning Phase

This module is currently in the planning phase. Implementation has not yet started.

### Next Steps
1. Set up dependencies and permissions
2. Implement Rust commands for system info
3. Create frontend component with event listeners
4. Build UI for window controls and state management
5. Implement system dialogs integration
6. Test on multiple platforms
