# Tauri Capability Playground — Implementation Plan

A concise, practical, step-by-step plan to integrate every capability in your Tauri Capability Playground app.

Everything is grouped module-by-module, and each module has its own one-screen implementation checklist.

---

## 📋 Table of Contents

1. [Filesystem Module](#1️⃣-filesystem-module)
2. [Notifications + Scheduling Module](#2️⃣-notifications--scheduling-module)
3. [Deep Linking Module](#3️⃣-deep-linking-module)
4. [Media Module (Audio + Video)](#4️⃣-media-module-audio--video)
5. [Drag & Drop Module](#5️⃣-drag--drop-module)
6. [Alarms (Future Notifications) Module](#6️⃣-alarms-future-notifications-module)
7. [Calendar Module (Internal + ICS Export)](#7️⃣-calendar-module-internal--ics-export)
8. [In-App Purchases Module](#8️⃣-in-app-purchases-module)
9. [SQL + Storage Module](#9️⃣-sql--storage-module)
10. [Network & Realtime Module](#🔟-network--realtime-module)
11. [Contacts Module](#1️⃣1️⃣-contacts-module)
12. [Gallery / Media Library Module](#1️⃣2️⃣-gallery--media-library-module)
13. [Camera Module (Photo + Video Capture)](#1️⃣3️⃣-camera-module-photo--video-capture)

---

## 1️⃣ Filesystem Module

### Purpose
Read + write files, list directories, and test permissions on desktop & mobile.

### Plugins Required
```bash
bun add @tauri-apps/plugin-fs
```

### Integration Steps

1. **Install plugin**
   ```bash
   bun add @tauri-apps/plugin-fs
   ```

2. **Add permission in `src-tauri/tauri.conf.json`**
   ```json
   {
     "permissions": [
       "fs:allow-read",
       "fs:allow-write",
       "fs:allow-create",
       "fs:allow-remove"
     ],
     "fs": {
       "scope": ["$APPDATA/*", "$APPLOCALDATA/*"]
     }
   }
   ```

3. **Implement functions**
   - Create folder
   - Read file
   - Write file
   - Delete file
   - List directory

### UI for This Screen
- **Button**: Create folder
- **Button**: Write sample JSON file
- **Button**: Read file → show output
- **Button**: List directory
- **Output panel**: Shows paths + results

---

## 2️⃣ Notifications + Scheduling Module

### Purpose
Send local notifications + test future scheduling (app alive).

### Plugins Required
```bash
bun add @tauri-apps/plugin-notification
```

### Integration Steps

1. **Install plugin**
   ```bash
   bun add @tauri-apps/plugin-notification
   ```

2. **Request permissions in app start**

3. **Create a Rust command** (`src-tauri/src/lib.rs`):
   ```rust
   #[tauri::command]
   pub async fn schedule_in(seconds: u64, title: String, body: String) {
       tokio::spawn(async move {
           tokio::time::sleep(Duration::from_secs(seconds)).await;
           notification::send(title, body).unwrap();
       });
   }
   ```

4. **Expose it to frontend**

### UI for This Screen
- **Time input**: seconds
- **Button**: "Schedule Notification"
- **Button**: "Send test notification"
- **List**: Scheduled notifications

---

## 3️⃣ Deep Linking Module

### Purpose
Test opening the app via `myapp://route`.

### Plugins Required
```bash
bun add @tauri-apps/plugin-deep-link
bun add @tauri-apps/plugin-single-instance # Optional
```

### Integration Steps

1. **Add plugin + configure custom scheme "myapp"**

2. **On frontend listen**:
   ```typescript
   onOpenUrl((urls) => route(urls[0]))
   ```

3. **Add a router handler in your SPA**

### UI for This Screen
- **Display**: Last received deep link
- **Button**: "Simulate deep link (desktop only)"
- **Log**: Every URL received

---

## 4️⃣ Media Module (Audio + Video)

### Purpose
Play local videos, audio files, and test OS media controls.

### Plugins Required
```bash
bun add tauri-plugin-media
bun add tauri-plugin-videoplayer # Optional
```

### Integration Steps

1. **Install media plugin**

2. **Allow audio/video in webview permissions**

3. **Use `<video>` / `<audio>` with `convertFileSrc(path)`**

4. **For native controls**:
   - Call `media::set_metadata`
   - Call `media::set_state`

### UI for This Screen
- **File picker**: Select audio/video file
- **Play/Pause/Seek controls**
- **Metadata panel**: Artist/title/duration
- **Button**: Open with native video player

---

## 5️⃣ Drag & Drop Module

### Purpose
Test Tauri's native file-drop + HTML5 drag drop.

### Plugins Required
None (built in)

### Integration Steps

1. **Enable in `tauri.conf.json`**:
   ```json
   {
     "windows": [{
       "dragDropEnabled": true
     }]
   }
   ```

2. **Listen**:
   ```typescript
   listen('tauri://file-drop', e => setFiles(e.payload.paths))
   ```

### UI for This Screen
- **Large drop area**
- **List**: Dropped files
- **Toggle**: Use Native vs HTML5 DnD

---

## 6️⃣ Alarms (Future Notifications) Module

### Purpose
Lightweight alarm simulation using scheduled notifications.

### Plugins Required
- Same as notifications module
- SQL plugin if you want persistence

### Integration Steps

1. **Create SQLite event table**

2. **Save alarms** (time + title)

3. **For each alarm**:
   - Compute seconds difference
   - Schedule via background Rust task

### UI for This Screen
- **Time picker**
- **Input**: Alarm title
- **Button**: Add alarm
- **List**: Upcoming alarms

---

## 7️⃣ Calendar Module (Internal + ICS Export)

### Purpose
Internal calendar + exporting events to `.ics` and opening in system calendar.

### Plugins Required
```bash
bun add @tauri-apps/plugin-sql
bun add @tauri-apps/plugin-fs
bun add @tauri-apps/plugin-opener # or sharesheet
```

### Integration Steps

1. **Create event table** (id, title, start, end)

2. **For exporting**:
   - Generate ICS string
   - Write file via FS
   - Use opener/sharesheet to open

### UI for This Screen
- **Mini month view**
- **Add event form**
- **Export as ICS button**
- **Open in system calendar button**

---

## 8️⃣ In-App Purchases Module

### Purpose
Test platform billing: iOS IAP, Android Billing, desktop Stripe.

### Plugins Required
- `tauri-plugin-iap` (iOS)
- `tauri-plugin-in-app-purchase` (iOS + Android + Windows)

### Integration Steps

1. **Wire plugin for each platform** (iOS/Android)

2. **Fetch products**

3. **Trigger purchase**

4. **Validate receipts** (local or backend)

5. **Restore purchases**

### UI for This Screen
- **List of products**
- **Button**: Buy
- **Button**: Restore purchases
- **Panel**: Show receipt JSON

---

## 9️⃣ SQL + Storage Module

### Purpose
Store user state + preferences + event data.

### Plugins Required
```bash
bun add @tauri-apps/plugin-sql
bun add @tauri-apps/plugin-store
```

### Integration Steps

1. **Initialize local SQLite**

2. **Setup settings KV store using store plugin**

### UI for This Screen
- **Toggle**: Dark mode (stored)
- **Input**: Name (persisted)
- **Button**: "Clear storage"

---

## 🔟 Network & Realtime Module

### Purpose
Test WebSockets, uploads, DNS, etc.

### Plugins Required
```bash
bun add @tauri-apps/plugin-websocket
bun add @tauri-apps/plugin-upload
```

### Integration Steps

1. **Connect to WebSocket echo server**

2. **Upload a file to an HTTP endpoint**

3. **Handle stream progress events**

### UI for This Screen
- **Input**: WebSocket URL
- **Button**: Connect
- **Panel**: Messages
- **Upload file button**

---

## 1️⃣1️⃣ Contacts Module

### Purpose
Access device contacts (read-only or read/write depending on platform).

### Plugins Required
📌 **No official Tauri plugin exists yet**

Community approaches:
- `tauri-plugin-contacts` (community, early-stage)
- **OR** build your own custom mobile plugin

### Integration Steps

1. **Create a custom plugin (mobile-only)**

   **Android (Kotlin)**:
   - Request `android.permission.READ_CONTACTS`
   - Query `ContactsContract.Contacts`

   **iOS (Swift)**:
   - Request permission via `CNContactStore`
   - Fetch contacts via `unifiedContacts`

2. **Expose Tauri command**:
   ```rust
   #[tauri::command]
   async fn get_contacts() -> Vec<Contact> {
       // call mobile-side plugin
   }
   ```

3. **Frontend integration**
   ```typescript
   const contacts = await invoke("get_contacts");
   ```

### UI for This Screen
- **Button**: "Load Contacts"
- **Search input**: Filter contacts
- **List**: Name, phone, email
- **Panel**: Permission status (granted/denied)

---

## 1️⃣2️⃣ Gallery / Media Library Module

### Purpose
Allow user to pick photos/videos from their device storage or gallery.

### Plugins Required
Use the View/Opener plugins from Tauri mobile plugin system:
- `tauri-plugin-view` → lets you view/open files
- `tauri-plugin-opener` → open external apps

📌 **For gallery selection, use custom plugin**:
- **Android**: Native intent `ACTION_PICK` / `ACTION_GET_CONTENT`
- **iOS**: `PHPickerViewController`

### Integration Steps

1. **Mobile Plugin functions**
   - `pick_image()` → returns file URI/path
   - `pick_video()`
   - `pick_multiple_media()`

   **Android**:
   ```kotlin
   val intent = Intent(Intent.ACTION_PICK)
   intent.type = "image/*"
   startActivityForResult(intent, REQUEST_CODE)
   ```

   **iOS**:
   ```swift
   let picker = PHPickerViewController(configuration)
   picker.delegate = self
   ```

2. **Convert URI/path to WebView usable URL**
   ```typescript
   import { convertFileSrc } from '@tauri-apps/api/core';
   const webViewUrl = convertFileSrc(path);
   ```

3. **Display preview**
   ```tsx
   <img src={convertFileSrc(path)} />
   <video src={convertFileSrc(path)} />
   ```

### UI for This Screen
- **Button**: Pick Image
- **Button**: Pick Video
- **Button**: Pick Multiple
- **Thumbnail Grid**: Selected media
- **Tap to preview**: Fullscreen view

---

## 1️⃣3️⃣ Camera Module (Photo + Video Capture)

### Purpose
Use device camera to take photos/videos.

### Plugins Required
📌 **No official camera plugin yet**

**Approach A: Use WebView camera via `getUserMedia`** (EASIEST)
- Works on Desktop, Android, iOS (with permissions)
```typescript
navigator.mediaDevices.getUserMedia({
  video: true,
  audio: false
})
```

**Approach B: Native camera** (best UX)
- Via custom plugin

### Integration Steps

1. **For WebView approach**:
   - Enable camera permission in Tauri mobile manifest
   - Use HTML5 `getUserMedia` API
   - Capture frame to canvas
   - Save as blob/file

2. **For Native plugin**:

   **Android**:
   - Use `MediaStore.ACTION_IMAGE_CAPTURE`
   - Use `FileProvider` to return file URI
   - Save JPEG/MP4 file

   **iOS**:
   - `UIImagePickerController`
   - OR better: `AVCaptureSession` for advanced control

3. **Implement plugin commands**:
   ```rust
   #[tauri::command]
   async fn open_camera() -> String {
       // open camera UI, return file path
   }

   #[tauri::command]
   async fn record_video() -> String {
       // open video recorder
   }

   #[tauri::command]
   async fn take_photo() -> String {
       // return stored image path
   }
   ```

4. **Frontend**:
   ```typescript
   const path = await invoke("take_photo");
   const imgUrl = convertFileSrc(path);
   ```

5. **Store captured media** in `AppLocalData`

### UI for This Screen
- **Live Camera Preview**: Via `getUserMedia`
- **Button**: Take Photo
- **Button**: Record Video
- **Panel**: Last 5 captured media thumbnails

---

## 🔥 Final App Structure (All Screens)

```
📁 Tauri Capability Playground
 ├── 📄 Filesystem
 ├── 🔔 Notifications
 ├── ⏰ Alarms
 ├── 🔗 Deep Linking
 ├── 📥 Drag & Drop
 ├── 🎵 Media Player
 ├── 📅 Calendar (ICS Export)
 ├── 💰 In-App Purchases
 ├── 💾 Storage & SQL
 ├── 🌐 Network & Realtime
 ├── 👤 Contacts
 ├── 🖼️ Gallery / Media Library
 └── 📷 Camera (Photo + Video)
```

---

## 📦 All Required Plugins (Complete List)

### Core Functionality
```bash
bun add @tauri-apps/plugin-fs
bun add @tauri-apps/plugin-notification
bun add @tauri-apps/plugin-deep-link
bun add @tauri-apps/plugin-single-instance
bun add @tauri-apps/plugin-sql
bun add @tauri-apps/plugin-store
bun add @tauri-apps/plugin-opener
bun add @tauri-apps/plugin-websocket
bun add @tauri-apps/plugin-upload
```

### Media & Advanced
```bash
bun add tauri-plugin-media
bun add tauri-plugin-videoplayer
bun add tauri-plugin-iap
bun add tauri-plugin-in-app-purchase
```

### Mobile-Specific (Custom Plugins Required)
**Note**: These require custom mobile plugin development
- Contacts (Android: Contacts Provider, iOS: CNContactStore)
- Gallery/Media Library (Android: ACTION_PICK, iOS: PHPickerViewController)
- Camera (Android: MediaStore, iOS: UIImagePickerController/AVCaptureSession)

**WebView Alternative for Camera**:
- Use HTML5 `getUserMedia` API (works on all platforms with permissions)

---

## 🚀 Implementation Order

1. **Start Simple**: Filesystem → Storage → Notifications
2. **Add Interaction**: Drag & Drop → Deep Linking
3. **Rich Features**: Media → Calendar → Alarms
4. **Platform Integration**: In-App Purchases
5. **Advanced**: Network & Realtime
6. **Mobile Features**: Gallery → Camera → Contacts (requires custom plugins)

---

## 📝 Notes

- Everything is designed for **maximum capability coverage**
- Each module is **compact and self-contained**
- All screens are **one-page implementations**
- Perfect for **learning Tauri capabilities**
- Great for **testing cross-platform features**

---

## ✅ Checklist Progress

Track your implementation progress:

### Core Modules
- [ ] Filesystem Module
- [ ] Notifications + Scheduling Module
- [ ] Deep Linking Module
- [ ] Media Module (Audio + Video)
- [ ] Drag & Drop Module
- [ ] Alarms (Future Notifications) Module
- [ ] Calendar Module (Internal + ICS Export)
- [ ] In-App Purchases Module
- [ ] SQL + Storage Module
- [ ] Network & Realtime Module

### Mobile-Specific Modules
- [ ] Contacts Module (custom plugin)
- [ ] Gallery / Media Library Module (custom plugin)
- [ ] Camera Module (WebView or custom plugin)

---

**Last Updated**: November 2025
**Tauri Version**: 2.9.4
**Target Platforms**: Windows, macOS, Linux, iOS, Android
