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
14. [Sensors & Device Hardware Module](#1️⃣4️⃣-sensors--device-hardware-module)
15. [Networking & Radio Access Module](#1️⃣5️⃣-networking--radio-access-module)
16. [Background Tasks Module](#1️⃣6️⃣-background-tasks-module)
17. [System Services Module](#1️⃣7️⃣-system-services-module)
18. [App Lifecycle & OS Integration Module](#1️⃣8️⃣-app-lifecycle--os-integration-module)
19. [Haptics / Vibrations Module](#1️⃣9️⃣-haptics--vibrations-module)
20. [Speech & Media Intelligence Module](#2️⃣0️⃣-speech--media-intelligence-module)
21. [File Sharing & Social Integration Module](#2️⃣1️⃣-file-sharing--social-integration-module)
22. [System Info & Device Profiling Module](#2️⃣2️⃣-system-info--device-profiling-module)
23. [Security & Biometrics Module](#2️⃣3️⃣-security--biometrics-module)
24. [Maps & Navigation Module](#2️⃣4️⃣-maps--navigation-module)
25. [Printing & PDF / Document Handling Module](#2️⃣5️⃣-printing--pdf--document-handling-module)
26. [Local Web Server Module](#2️⃣6️⃣-local-web-server-module)

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

## 1️⃣4️⃣ Sensors & Device Hardware Module

### Purpose
Access device sensors for motion-controlled UI, compass apps, AR apps, and GPS/location tracking.

### Plugins Required
📌 **Custom plugins required for most sensors**

**A. Gyroscope / Accelerometer / Magnetometer**
- Android: `SensorManager`
- iOS: `CoreMotion`

**B. GPS / Geolocation**
- Web API: `navigator.geolocation` (simplest)
- Native precision:
  - Android: Location Services
  - iOS: CoreLocation

### Integration Steps

1. **For Web API (Geolocation)**:
   ```typescript
   navigator.geolocation.getCurrentPosition(
     (position) => {
       const { latitude, longitude } = position.coords;
     }
   );
   ```

2. **For Native Sensors (Custom Plugin)**:

   **Android**:
   ```kotlin
   val sensorManager = getSystemService(Context.SENSOR_SERVICE) as SensorManager
   val accelerometer = sensorManager.getDefaultSensor(Sensor.TYPE_ACCELEROMETER)
   ```

   **iOS**:
   ```swift
   let motionManager = CMMotionManager()
   motionManager.startAccelerometerUpdates()
   ```

3. **Rust commands**:
   ```rust
   #[tauri::command]
   async fn get_sensor_data() -> SensorData {
       // call mobile plugin
   }
   ```

### UI for This Screen
- **Live 3-axis graph**: X, Y, Z acceleration
- **Button**: "Shake to trigger action"
- **Compass**: Heading display
- **GPS Panel**: Coordinates, accuracy
- **Button**: "Track movement"
- **Map preview**: Leaflet.js + OpenStreetMap

---

## 1️⃣5️⃣ Networking & Radio Access Module

### Purpose
Access Bluetooth Low Energy (BLE) devices and scan Wi-Fi networks.

### Plugins Required
📌 **Custom plugins required**

**A. Bluetooth (BLE)**
- Android: `BluetoothGatt`
- iOS: `CoreBluetooth`

**B. Wi-Fi Scanning**
- Android: `WifiManager`
- Desktop: OS tools via CLI

### Integration Steps

1. **Bluetooth Plugin**:

   **Android**:
   ```kotlin
   val bluetoothManager = getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
   val bluetoothAdapter = bluetoothManager.adapter
   bluetoothLeScanner.startScan(scanCallback)
   ```

   **iOS**:
   ```swift
   centralManager = CBCentralManager(delegate: self, queue: nil)
   centralManager.scanForPeripherals(withServices: nil)
   ```

2. **Rust commands**:
   ```rust
   #[tauri::command]
   async fn scan_bluetooth_devices() -> Vec<BluetoothDevice> {}

   #[tauri::command]
   async fn connect_device(id: String) -> Result<(), String> {}
   ```

### UI for This Screen
- **Button**: Scan BLE devices
- **List**: Available devices with RSSI
- **Button**: Connect/Disconnect
- **Panel**: Read/write characteristics
- **Wi-Fi List**: Nearby SSIDs
- **RSSI Graph**: Signal strength
- **Panel**: Frequency band info

---

## 1️⃣6️⃣ Background Tasks Module

### Purpose
Real background processing even when app is killed or in background.

### Plugins Required
📌 **Custom plugin required**

- **Android**: WorkManager
- **iOS**: Background Fetch / BGTaskScheduler

### Integration Steps

1. **Android WorkManager**:
   ```kotlin
   val workRequest = PeriodicWorkRequestBuilder<SyncWorker>(15, TimeUnit.MINUTES)
       .build()
   WorkManager.getInstance(context).enqueue(workRequest)
   ```

2. **iOS Background Tasks**:
   ```swift
   BGTaskScheduler.shared.register(
       forTaskWithIdentifier: "com.app.refresh",
       using: nil
   ) { task in
       handleBackgroundTask(task: task as! BGAppRefreshTask)
   }
   ```

3. **Use cases**:
   - Sync notes
   - Refresh location
   - Background uploads
   - Periodic API calls

### UI for This Screen
- **Button**: Schedule background task
- **Input**: Interval (minutes)
- **Log panel**: Last 10 background executions
- **Toggle**: Enable/disable background sync
- **Button**: Trigger notification on completion

---

## 1️⃣7️⃣ System Services Module

### Purpose
Access clipboard, system audio devices, and battery/power information.

### Plugins Required
- **Clipboard**: `@tauri-apps/plugin-clipboard-manager`
- **Battery**: Web API or custom plugin
- **Audio devices**: Custom plugin

### Integration Steps

1. **Clipboard**:
   ```bash
   bun add @tauri-apps/plugin-clipboard-manager
   ```

   ```typescript
   import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';
   ```

2. **Battery API (Web)**:
   ```typescript
   const battery = await navigator.getBattery();
   console.log(battery.level * 100); // percentage
   console.log(battery.charging); // boolean
   ```

3. **Native Battery (Custom Plugin)**:

   **Android**:
   ```kotlin
   val batteryManager = getSystemService(Context.BATTERY_SERVICE) as BatteryManager
   val batteryLevel = batteryManager.getIntProperty(BATTERY_PROPERTY_CAPACITY)
   ```

### UI for This Screen
- **Input**: Text to copy
- **Button**: Copy to clipboard
- **Button**: Paste from clipboard
- **Panel**: Clipboard history (last 5)
- **Audio Devices List**: Speakers, headphones, Bluetooth
- **Battery Panel**: Percentage, charging state, temperature

---

## 1️⃣8️⃣ App Lifecycle & OS Integration Module

### Purpose
Monitor app lifecycle events, create system tray, and manage multiple windows.

### Plugins Required
- System tray: Built-in Tauri
- Multiple windows: Built-in Tauri
- Lifecycle: Custom mobile plugin

### Integration Steps

1. **App Lifecycle Hooks**:
   ```rust
   #[tauri::command]
   fn on_app_foreground() {
       println!("App moved to foreground");
   }
   ```

2. **System Tray (Desktop)**:
   ```rust
   use tauri::SystemTray;

   let tray = SystemTray::new()
       .with_menu(menu);
   ```

3. **Multiple Windows**:
   ```rust
   use tauri::WindowBuilder;

   WindowBuilder::new(
       &app,
       "secondary",
       tauri::WindowUrl::App("index.html".into())
   ).build()?;
   ```

### UI for This Screen
- **Log panel**: Lifecycle events (foreground, background, pause, resume)
- **Button**: Create system tray icon
- **Button**: Open new window
- **Input**: Window message
- **Button**: Send message to other window
- **Toggle**: Background mode

---

## 1️⃣9️⃣ Haptics / Vibrations Module

### Purpose
Provide tactile feedback on mobile devices.

### Plugins Required
📌 **Custom mobile plugin**

- Android: Vibrator API
- iOS: UIFeedbackGenerator

### Integration Steps

1. **Android**:
   ```kotlin
   val vibrator = getSystemService(Context.VIBRATOR_SERVICE) as Vibrator
   vibrator.vibrate(VibrationEffect.createOneShot(100, VibrationEffect.DEFAULT_AMPLITUDE))
   ```

2. **iOS**:
   ```swift
   let generator = UIImpactFeedbackGenerator(style: .medium)
   generator.impactOccurred()
   ```

3. **Rust command**:
   ```rust
   #[tauri::command]
   async fn vibrate(intensity: String) {
       // call mobile plugin
   }
   ```

### UI for This Screen
- **Button**: Light tap
- **Button**: Medium impact
- **Button**: Heavy impact
- **Button**: Success vibration
- **Button**: Error vibration
- **Button**: Warning vibration

---

## 2️⃣0️⃣ Speech & Media Intelligence Module

### Purpose
Speech-to-text transcription and text-to-speech synthesis.

### Plugins Required
**Approach A: Web Speech API** (works on most platforms)
**Approach B: Native APIs** (better quality)

### Integration Steps

1. **Speech-to-Text (Web)**:
   ```typescript
   const recognition = new webkitSpeechRecognition();
   recognition.onresult = (event) => {
       const transcript = event.results[0][0].transcript;
   };
   recognition.start();
   ```

2. **Text-to-Speech (Web)**:
   ```typescript
   const utterance = new SpeechSynthesisUtterance('Hello world');
   speechSynthesis.speak(utterance);
   ```

3. **Native STT/TTS (Custom Plugin)**:

   **Android**:
   ```kotlin
   val speechRecognizer = SpeechRecognizer.createSpeechRecognizer(context)
   val textToSpeech = TextToSpeech(context) { status -> }
   ```

   **iOS**:
   ```swift
   let recognizer = SFSpeechRecognizer()
   let synthesizer = AVSpeechSynthesizer()
   ```

### UI for This Screen
- **Button**: Press to speak (STT)
- **Panel**: Live transcript
- **Button**: Save transcript
- **Input**: Text to speak (TTS)
- **Button**: Play voice
- **Dropdown**: Select voice
- **Slider**: Speech rate

---

## 2️⃣1️⃣ File Sharing & Social Integration Module

### Purpose
Share files via native share sheet and receive files from other apps.

### Plugins Required
📌 **Custom mobile plugin**

### Integration Steps

1. **Share Sheet (Android)**:
   ```kotlin
   val shareIntent = Intent().apply {
       action = Intent.ACTION_SEND
       putExtra(Intent.EXTRA_TEXT, "Share content")
       type = "text/plain"
   }
   startActivity(Intent.createChooser(shareIntent, null))
   ```

2. **Share Sheet (iOS)**:
   ```swift
   let activityController = UIActivityViewController(
       activityItems: ["Share content"],
       applicationActivities: nil
   )
   present(activityController, animated: true)
   ```

3. **Receiving Shared Files**:
   - Android: Intent filter in manifest
   - iOS: Share extension

### UI for This Screen
- **Button**: Share text
- **Button**: Share image
- **Button**: Share file
- **Panel**: Files received from other apps
- **Thumbnail preview**: Shared media

---

## 2️⃣2️⃣ System Info & Device Profiling Module

### Purpose
Display hardware information and performance benchmarks.

### Plugins Required
- `@tauri-apps/plugin-os`
- Custom plugin for extended info

### Integration Steps

1. **Install plugin**:
   ```bash
   bun add @tauri-apps/plugin-os
   ```

2. **Get system info**:
   ```typescript
   import { platform, version, arch } from '@tauri-apps/plugin-os';
   ```

3. **Performance benchmarks**:
   ```rust
   #[tauri::command]
   async fn benchmark_performance() -> BenchmarkResults {
       // Test JS loop, command latency, file I/O, etc.
   }
   ```

### UI for This Screen
- **Panel**: Hardware info (CPU, GPU, RAM, storage)
- **Panel**: OS info (platform, version, arch)
- **Panel**: Device model
- **Button**: Run benchmark
- **Graph**: Performance results
- **Metrics**: Command latency, file I/O speed, DB operations

---

## 2️⃣3️⃣ Security & Biometrics Module

### Purpose
Biometric authentication (fingerprint, face) and secure crypto operations.

### Plugins Required
📌 **Custom mobile plugin**

- Android: BiometricPrompt
- iOS: LocalAuthentication

### Integration Steps

1. **Android Biometrics**:
   ```kotlin
   val biometricPrompt = BiometricPrompt(this, executor, callback)
   val promptInfo = BiometricPrompt.PromptInfo.Builder()
       .setTitle("Authenticate")
       .setNegativeButtonText("Cancel")
       .build()
   biometricPrompt.authenticate(promptInfo)
   ```

2. **iOS Biometrics**:
   ```swift
   let context = LAContext()
   context.evaluatePolicy(
       .deviceOwnerAuthenticationWithBiometrics,
       localizedReason: "Authenticate"
   ) { success, error in }
   ```

3. **Secure storage**:
   - Android: Keystore
   - iOS: Keychain

### UI for This Screen
- **Button**: "Authenticate with biometrics"
- **Panel**: Authentication result
- **Panel**: Available biometric methods
- **Button**: Generate encryption key
- **Button**: Encrypt data
- **Button**: Decrypt data

---

## 2️⃣4️⃣ Maps & Navigation Module

### Purpose
Display interactive maps with user location, markers, and routing.

### Plugins Required
```bash
bun add leaflet
bun add @types/leaflet -D
```

### Integration Steps

1. **Setup Leaflet.js**:
   ```typescript
   import L from 'leaflet';

   const map = L.map('map').setView([51.505, -0.09], 13);
   L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png').addTo(map);
   ```

2. **Add markers**:
   ```typescript
   L.marker([lat, lng]).addTo(map)
       .bindPopup('Marker content');
   ```

3. **Get user location** (from Sensors module):
   ```typescript
   navigator.geolocation.getCurrentPosition((pos) => {
       map.setView([pos.coords.latitude, pos.coords.longitude], 13);
   });
   ```

### UI for This Screen
- **Map view**: Interactive Leaflet map
- **Button**: Show my location
- **Button**: Drop marker
- **Input**: Search location
- **Panel**: Route calculator (optional)
- **List**: Saved markers

---

## 2️⃣5️⃣ Printing & PDF / Document Handling Module

### Purpose
System print dialog and PDF viewing/annotation.

### Plugins Required
- Printing: Built-in browser API
- PDF: `react-pdf` or `pdf.js`

### Integration Steps

1. **Print HTML (Desktop)**:
   ```typescript
   window.print();
   ```

2. **Generate PDF** (use library like `jsPDF`):
   ```bash
   bun add jspdf
   ```

   ```typescript
   import jsPDF from 'jspdf';
   const doc = new jsPDF();
   doc.text('Hello world', 10, 10);
   doc.save('document.pdf');
   ```

3. **View PDF**:
   ```bash
   bun add react-pdf
   ```

### UI for This Screen
- **Button**: Print current page
- **Button**: Generate PDF
- **PDF Viewer**: Display PDF with zoom/pan
- **Button**: Download PDF
- **Button**: Share PDF

---

## 2️⃣6️⃣ Local Web Server Module

### Purpose
Embed a Rust HTTP server inside Tauri for local APIs, webhooks, and multi-process communication.

### Plugins Required
Add to `Cargo.toml`:
```toml
axum = "0.7"
tokio = { version = "1", features = ["full"] }
```

### Integration Steps

1. **Create Axum server**:
   ```rust
   use axum::{Router, routing::get};

   #[tauri::command]
   async fn start_local_server() {
       let app = Router::new()
           .route("/", get(|| async { "Hello from local server!" }));

       tokio::spawn(async {
           axum::Server::bind(&"127.0.0.1:3030".parse().unwrap())
               .serve(app.into_make_service())
               .await
               .unwrap();
       });
   }
   ```

2. **Test from frontend**:
   ```typescript
   const response = await fetch('http://localhost:3030');
   const data = await response.text();
   ```

### UI for This Screen
- **Button**: Start local server
- **Button**: Stop server
- **Panel**: Server status (running/stopped)
- **Input**: API endpoint
- **Button**: Test endpoint
- **Panel**: Response output
- **Log**: Request history

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
 ├── 📷 Camera (Photo + Video)
 ├── 🧭 Sensors & Motion
 ├── 📡 Bluetooth & Wi-Fi
 ├── ⚙️ Background Tasks
 ├── 📋 Clipboard & Battery
 ├── 🔄 App Lifecycle & System Tray
 ├── 📳 Haptics & Vibrations
 ├── 🎤 Speech (TTS & STT)
 ├── 📤 File Sharing & Share Sheet
 ├── 📊 System Info & Device Profile
 ├── 🔐 Biometrics & Secure Storage
 ├── 🗺️ Maps & Geolocation
 ├── 🖨️ Printing & PDF
 └── 🌐 Local Web Server
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
bun add @tauri-apps/plugin-clipboard-manager
bun add @tauri-apps/plugin-os
bun add @tauri-apps/plugin-process
bun add @tauri-apps/plugin-shell
bun add @tauri-apps/plugin-global-shortcut
```

### Media & Advanced
```bash
bun add tauri-plugin-media
bun add tauri-plugin-videoplayer
bun add tauri-plugin-iap
bun add tauri-plugin-in-app-purchase
```

### Geolocation & Maps
```bash
bun add @tauri-apps/plugin-geolocation
# Leaflet.js for maps (frontend)
bun add leaflet
bun add @types/leaflet -D
```

### HTTP & Networking
```bash
# For embedded web server (add to Cargo.toml)
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["fs", "cors"] }
```

### Custom Plugins Required
**Note**: These require custom mobile/desktop plugin development

#### Mobile-Specific
- **Contacts** (Android: Contacts Provider, iOS: CNContactStore)
- **Gallery/Media Library** (Android: ACTION_PICK, iOS: PHPickerViewController)
- **Camera** (Android: MediaStore, iOS: UIImagePickerController/AVCaptureSession)
  - *Alternative*: Use HTML5 `getUserMedia` API (works on all platforms with permissions)

#### Sensors & Hardware
- **Sensors** (Android: SensorManager, iOS: CoreMotion)
- **Bluetooth LE** (Android: BluetoothGatt, iOS: CoreBluetooth)
- **Wi-Fi Management** (Android: WifiManager, iOS: NEHotspotConfiguration)

#### System Integration
- **Background Tasks** (Android: WorkManager, iOS: BGTaskScheduler)
- **Battery Info** (Android: BatteryManager, iOS: UIDevice)
- **Audio Devices** (Desktop: cpal, Mobile: AudioManager/AVAudioSession)
- **Haptics** (Android: Vibrator API, iOS: UIFeedbackGenerator)
- **System Tray** (Desktop: built-in Tauri, Mobile: N/A)
- **Multi-Window** (Desktop: built-in Tauri, Mobile: N/A)

#### Security & Biometrics
- **Biometric Auth** (Android: BiometricPrompt, iOS: LocalAuthentication)
- **Secure Storage** (Use @tauri-apps/plugin-store with encryption)

#### Document & Media
- **Speech-to-Text** (WebView: Web Speech API, Native: Android SpeechRecognizer, iOS: SFSpeechRecognizer)
- **Text-to-Speech** (WebView: Web Speech API, Native: Android TextToSpeech, iOS: AVSpeechSynthesizer)
- **Share Sheet** (Android: ACTION_SEND, iOS: UIActivityViewController)
- **PDF Generation** (jsPDF on frontend, or rust-pdf backend)
- **Printing** (Desktop: native dialogs, Mobile: PrintManager/UIPrintInteractionController)

---

## 🚀 Implementation Order

### Phase 1: Foundation (Modules 1-5)
1. **Filesystem** - Basic file operations
2. **Storage & SQL** - Data persistence
3. **Notifications** - Simple alerts
4. **Drag & Drop** - File interaction
5. **Deep Linking** - URL handling

### Phase 2: Media & Content (Modules 6-8)
6. **Media Player** - Audio/video playback
7. **Calendar** - Event management
8. **Alarms** - Scheduled notifications

### Phase 3: Monetization & Networking (Modules 9-10)
9. **In-App Purchases** - Payment integration
10. **Network & Realtime** - HTTP, WebSocket, SSE

### Phase 4: Mobile Integration (Modules 11-13)
11. **Gallery/Media Library** - Access photo library
12. **Camera** - Photo/video capture
13. **Contacts** - Contact management

### Phase 5: Device Hardware (Modules 14-15)
14. **Sensors & Motion** - Accelerometer, gyroscope, proximity
15. **Bluetooth & Wi-Fi** - BLE scanning, Wi-Fi management

### Phase 6: System Services (Modules 16-19)
16. **Background Tasks** - Long-running operations
17. **Clipboard & Battery** - System utilities
18. **App Lifecycle & System Tray** - Window management
19. **Haptics & Vibrations** - Tactile feedback

### Phase 7: Intelligence & Sharing (Modules 20-21)
20. **Speech (TTS & STT)** - Voice interaction
21. **File Sharing & Share Sheet** - Cross-app sharing

### Phase 8: Advanced Features (Modules 22-26)
22. **System Info & Device Profile** - Hardware/OS details
23. **Biometrics & Secure Storage** - Authentication
24. **Maps & Geolocation** - Location services
25. **Printing & PDF** - Document generation
26. **Local Web Server** - Embedded HTTP server

**Recommended**: Start with Phase 1-3, then choose modules based on your app's requirements.

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

### Phase 1: Foundation (Modules 1-5)
- [x] 1. Filesystem Module (See: [filesystem-module.md](filesystem-module.md))
- [x] 2. Notifications + Scheduling Module (See: [notifications-module.md](notifications-module.md))
- [x] 3. Deep Linking Module (See: [deep-linking-module.md](deep-linking-module.md))
- [x] 4. Media Module (Audio + Video) (See: [media-module.md](media-module.md))
- [x] 5. Drag & Drop Module (See: [drag-drop-module.md](drag-drop-module.md))

### Phase 2: Media & Content (Modules 6-8)
- [ ] 6. Alarms (Future Notifications) Module (See: [alarms-module.md](alarms-module.md))
- [ ] 7. Calendar Module (Internal + ICS Export)
- [ ] 8. In-App Purchases Module

### Phase 3: Monetization & Networking (Modules 9-10)
- [ ] 9. SQL + Storage Module
- [ ] 10. Network & Realtime Module

### Phase 4: Mobile Integration (Modules 11-13)
- [ ] 11. Contacts Module (custom plugin)
- [ ] 12. Gallery / Media Library Module (custom plugin)
- [ ] 13. Camera Module (WebView or custom plugin)

### Phase 5: Device Hardware (Modules 14-15)
- [ ] 14. Sensors & Motion Module (custom plugin)
- [ ] 15. Bluetooth & Wi-Fi Module (custom plugin)

### Phase 6: System Services (Modules 16-19)
- [ ] 16. Background Tasks Module (custom plugin)
- [ ] 17. Clipboard & Battery Module
- [ ] 18. App Lifecycle & System Tray Module
- [ ] 19. Haptics & Vibrations Module (custom plugin)

### Phase 7: Intelligence & Sharing (Modules 20-21)
- [ ] 20. Speech (TTS & STT) Module
- [ ] 21. File Sharing & Share Sheet Module (custom plugin)

### Phase 8: Advanced Features (Modules 22-26)
- [ ] 22. System Info & Device Profile Module
- [ ] 23. Biometrics & Secure Storage Module (custom plugin)
- [ ] 24. Maps & Geolocation Module
- [ ] 25. Printing & PDF Module (custom plugin)
- [ ] 26. Local Web Server Module

---

**Last Updated**: November 2025
**Tauri Version**: 2.9.4
**Target Platforms**: Windows, macOS, Linux, iOS, Android
