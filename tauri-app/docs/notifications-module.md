# Notifications + Scheduling Module Implementation

## Overview
Send local notifications + test future scheduling (app alive).

---

## Plugin Setup

### Install Dependencies
```bash
bun add @tauri-apps/plugin-notification
```

### Cargo Dependencies
Add to `src-tauri/Cargo.toml`:
```toml
[dependencies]
tauri-plugin-notification = "2.0"
```

### Register Plugin
In `src-tauri/src/lib.rs`:
```rust
.plugin(tauri_plugin_notification::init())
```

---

## Permissions Configuration

### Required Permissions
Add to `src-tauri/capabilities/default.json`:
```json
{
  "permissions": [
    "notification:allow-is-permission-granted",
    "notification:allow-request-permission",
    "notification:allow-notify",
    "notification:allow-register-listener"
  ]
}
```

---

## Core Features

### 1. Request Permission
- [ ] Check if notification permission is granted
- [ ] Request permission if not granted
- [ ] Display permission status
- [ ] Handle permission denial

### 2. Send Instant Notification
- [ ] Send notification immediately
- [ ] Set title and body
- [ ] Support custom icon
- [ ] Handle notification click events

### 3. Schedule Notification
- [ ] Schedule notification for future time
- [ ] Input delay in seconds
- [ ] Store scheduled notifications list
- [ ] Cancel scheduled notifications

### 4. Notification Actions
- [ ] Listen for notification clicks
- [ ] Handle notification actions
- [ ] Track notification status

---

## Rust Backend

### Custom Scheduling Command
Create in `src-tauri/src/lib.rs`:

```rust
use tauri_plugin_notification::NotificationExt;
use std::time::Duration;

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
```

Register command:
```rust
.invoke_handler(tauri::generate_handler![schedule_notification])
```

---

## Frontend Implementation

### API Integration
```typescript
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';
import { invoke } from '@tauri-apps/api/core';
```

### Permission Check Flow
```typescript
// Check permission
let permissionGranted = await isPermissionGranted();

// Request if not granted
if (!permissionGranted) {
  const permission = await requestPermission();
  permissionGranted = permission === 'granted';
}
```

### Send Notification
```typescript
if (permissionGranted) {
  sendNotification({
    title: 'Test Notification',
    body: 'This is a test message'
  });
}
```

### Schedule Notification
```typescript
await invoke('schedule_notification', {
  seconds: 5,
  title: 'Scheduled',
  body: 'This was scheduled 5 seconds ago'
});
```

---

## UI Components

### Permission Section
- [ ] Permission status display
- [ ] "Request Permission" button
- [ ] Visual indicator (granted/denied)

### Instant Notification Section
- [ ] Title input field
- [ ] Body input field
- [ ] "Send Notification" button

### Scheduling Section
- [ ] Delay input (seconds)
- [ ] Title input field
- [ ] Body input field
- [ ] "Schedule Notification" button
- [ ] List of scheduled notifications

### Output Panel
- [ ] Display operation results
- [ ] Show success/error messages
- [ ] Log notification events

---

## Testing Checklist

### Desktop Testing
- [ ] Send notification on Windows
- [ ] Send notification on macOS
- [ ] Send notification on Linux
- [ ] Test permission request
- [ ] Test scheduled notifications
- [ ] Test notification click handling

### Mobile Testing
- [ ] Test on Android
- [ ] Test on iOS
- [ ] Verify permission prompt
- [ ] Test background notifications
- [ ] Test notification icons

### Edge Cases
- [ ] Handle permission denial
- [ ] Test with empty title/body
- [ ] Test with long text
- [ ] Test multiple scheduled notifications
- [ ] Test canceling scheduled notifications
- [ ] Handle notification while app in background

---

## Implementation Notes

### Platform Differences
- **Desktop**: Notifications appear in system notification center
- **Mobile**: Follows platform-specific notification styles
- **Permissions**: Mobile requires explicit permission, desktop may vary

### Scheduling Limitations
- Scheduled notifications work only while app is alive
- For persistent scheduling, use Background Tasks module
- Store scheduled notifications in state for tracking

### Best Practices
- Always check permission before sending
- Provide clear permission request context
- Keep notification text concise
- Handle notification actions appropriately

---

## Progress Tracking

### Setup Phase
- [ ] Install plugin dependencies
- [ ] Configure permissions
- [ ] Register plugin in Rust
- [ ] Add custom scheduling command

### Development Phase
- [ ] Implement permission check
- [ ] Implement permission request
- [ ] Implement instant notification
- [ ] Implement scheduled notification
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

### 🚧 Module In Development

Current status: Planning and documentation complete. Ready for implementation.
