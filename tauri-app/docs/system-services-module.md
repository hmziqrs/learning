# System Services Module Implementation

## Overview

Access clipboard operations, battery/power information, and system audio device management across desktop and mobile platforms.

## Current Implementation Status

⚠️ **Planned** - Clipboard support available via official plugin. Battery and audio require custom plugin development for full cross-platform support.

## Plugin Setup

### Clipboard Manager (Official Plugin)

The official Tauri clipboard plugin provides cross-platform clipboard access:

```bash
bun add @tauri-apps/plugin-clipboard-manager
```

**Rust Setup:**
```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Frontend Usage:**
```typescript
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager'

// Write to clipboard
await writeText('Hello clipboard!')

// Read from clipboard
const text = await readText()
```

**Permissions Required:**
- Desktop: No special permissions needed
- Mobile: Clipboard access is generally permitted

### Battery API

**Web API (Basic Support):**
```typescript
// Browser Battery API
const battery = await navigator.getBattery()
console.log(battery.level * 100) // percentage
console.log(battery.charging) // boolean
console.log(battery.chargingTime) // seconds until full
console.log(battery.dischargingTime) // seconds until empty
```

**Note:** Web Battery API is deprecated in some browsers and not available on iOS/Safari.

**Custom Plugin Required for Full Support:**
- Android: BatteryManager API
- iOS: UIDevice battery monitoring
- Desktop: Platform-specific battery APIs

### Audio Devices (Custom Plugin Required)

For audio device enumeration and management, custom native plugins are required:

**Web Audio API (Limited):**
```typescript
// List audio input/output devices
const devices = await navigator.mediaDevices.enumerateDevices()
const audioOutputs = devices.filter(d => d.kind === 'audiooutput')
const audioInputs = devices.filter(d => d.kind === 'audioinput')
```

**Desktop:**
- Windows: Core Audio API
- macOS: Core Audio / AudioUnit
- Linux: PulseAudio / ALSA

**Mobile:**
- Android: AudioManager
- iOS: AVAudioSession

## Permissions Configuration

### Android Manifest
```xml
<!-- Battery access (no permission required for basic info) -->
<!-- For detailed battery stats -->
<uses-permission android:name="android.permission.BATTERY_STATS" />

<!-- Audio device access (no permission required for enumeration) -->
<!-- For recording audio -->
<uses-permission android:name="android.permission.RECORD_AUDIO" />
```

### iOS Info.plist
```xml
<!-- No special permissions for clipboard -->
<!-- For microphone access (if needed) -->
<key>NSMicrophoneUsageDescription</key>
<string>We need microphone access to manage audio devices</string>
```

### Tauri Capabilities
```json
{
  "permissions": [
    "clipboard-manager:allow-read-text",
    "clipboard-manager:allow-write-text"
  ]
}
```

## Core Features

### Clipboard Operations
- [ ] Write text to clipboard
- [ ] Read text from clipboard
- [ ] Clear clipboard
- [ ] Clipboard history (app-level tracking)
- [ ] Copy/paste confirmation feedback

### Battery & Power
- [ ] Get current battery level (percentage)
- [ ] Check charging status
- [ ] Get battery temperature
- [ ] Monitor battery state changes
- [ ] Low battery warnings
- [ ] Power source detection (AC/battery)
- [ ] Estimated time remaining

### Audio Devices
- [ ] List available audio output devices
- [ ] List available audio input devices
- [ ] Detect device connections/disconnections
- [ ] Get default audio device
- [ ] Display device properties (name, type)
- [ ] Bluetooth audio device detection

## Data Structures

### Battery Info Schema
```typescript
interface BatteryInfo {
  level: number // 0-100 percentage
  charging: boolean
  chargingTime: number | null // seconds until full, null if not charging
  dischargingTime: number | null // seconds until empty, null if charging
  temperature: number | null // celsius (mobile only)
  powerSource: 'battery' | 'ac' | 'usb' | 'wireless' | 'unknown'
  batteryState: 'full' | 'charging' | 'discharging' | 'not_charging' | 'unknown'
}
```

### Audio Device Schema
```typescript
interface AudioDevice {
  id: string
  name: string
  kind: 'audioinput' | 'audiooutput'
  isDefault: boolean
  isConnected: boolean
  type: 'speaker' | 'headphones' | 'bluetooth' | 'usb' | 'built-in' | 'unknown'
}
```

### Clipboard Entry Schema
```typescript
interface ClipboardEntry {
  id: string
  text: string
  timestamp: number
  source: 'app' | 'system'
}
```

## Rust Backend

### Clipboard Commands

```rust
use tauri_plugin_clipboard_manager::{ClipboardExt, ClipKind};

#[tauri::command]
async fn read_clipboard_text(app: tauri::AppHandle) -> Result<String, String> {
    app.clipboard()
        .read_text()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn write_clipboard_text(app: tauri::AppHandle, text: String) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn clear_clipboard(app: tauri::AppHandle) -> Result<(), String> {
    app.clipboard()
        .clear()
        .map_err(|e| e.to_string())
}
```

### Battery Commands (Custom Plugin Required)

#### Android: Battery Manager

```kotlin
import android.content.Context
import android.os.BatteryManager
import android.content.Intent
import android.content.IntentFilter

class SystemServicesPlugin(private val activity: Activity) {

    @Command
    fun getBatteryInfo(invoke: Invoke) {
        val batteryManager = activity.getSystemService(Context.BATTERY_SERVICE) as BatteryManager
        val batteryStatus: Intent? = activity.registerReceiver(
            null,
            IntentFilter(Intent.ACTION_BATTERY_CHANGED)
        )

        val level = batteryManager.getIntProperty(BatteryManager.BATTERY_PROPERTY_CAPACITY)
        val status = batteryStatus?.getIntExtra(BatteryManager.EXTRA_STATUS, -1) ?: -1
        val charging = status == BatteryManager.BATTERY_STATUS_CHARGING ||
                      status == BatteryManager.BATTERY_STATUS_FULL

        val temperature = batteryStatus?.getIntExtra(BatteryManager.EXTRA_TEMPERATURE, -1)?.div(10) ?: -1

        val plugged = batteryStatus?.getIntExtra(BatteryManager.EXTRA_PLUGGED, -1) ?: -1
        val powerSource = when (plugged) {
            BatteryManager.BATTERY_PLUGGED_AC -> "ac"
            BatteryManager.BATTERY_PLUGGED_USB -> "usb"
            BatteryManager.BATTERY_PLUGGED_WIRELESS -> "wireless"
            else -> "battery"
        }

        val result = mapOf(
            "level" to level,
            "charging" to charging,
            "temperature" to temperature,
            "powerSource" to powerSource,
            "batteryState" to when (status) {
                BatteryManager.BATTERY_STATUS_FULL -> "full"
                BatteryManager.BATTERY_STATUS_CHARGING -> "charging"
                BatteryManager.BATTERY_STATUS_DISCHARGING -> "discharging"
                BatteryManager.BATTERY_STATUS_NOT_CHARGING -> "not_charging"
                else -> "unknown"
            }
        )

        invoke.resolve(result)
    }

    @Command
    fun startBatteryMonitoring(invoke: Invoke) {
        val filter = IntentFilter(Intent.ACTION_BATTERY_CHANGED)
        val receiver = object : BroadcastReceiver() {
            override fun onReceive(context: Context?, intent: Intent?) {
                // Emit battery update event
                emitEvent("battery-changed", getBatteryData(intent))
            }
        }
        activity.registerReceiver(receiver, filter)
        invoke.resolve(mapOf("success" to true))
    }
}
```

#### iOS: UIDevice Battery

```swift
import UIKit

class SystemServicesPlugin: NSObject {

    @objc func getBatteryInfo(_ invoke: Invoke) {
        UIDevice.current.isBatteryMonitoringEnabled = true

        let level = Int(UIDevice.current.batteryLevel * 100)
        let state = UIDevice.current.batteryState

        let charging = state == .charging || state == .full
        let batteryState: String

        switch state {
        case .full:
            batteryState = "full"
        case .charging:
            batteryState = "charging"
        case .unplugged:
            batteryState = "discharging"
        default:
            batteryState = "unknown"
        }

        let result: [String: Any] = [
            "level": level,
            "charging": charging,
            "batteryState": batteryState,
            "powerSource": charging ? "ac" : "battery",
            "temperature": NSNull() // iOS doesn't expose battery temperature
        ]

        invoke.resolve(result)
    }

    @objc func startBatteryMonitoring(_ invoke: Invoke) {
        UIDevice.current.isBatteryMonitoringEnabled = true

        NotificationCenter.default.addObserver(
            self,
            selector: #selector(batteryLevelChanged),
            name: UIDevice.batteryLevelDidChangeNotification,
            object: nil
        )

        NotificationCenter.default.addObserver(
            self,
            selector: #selector(batteryStateChanged),
            name: UIDevice.batteryStateDidChangeNotification,
            object: nil
        )

        invoke.resolve(["success": true])
    }

    @objc private func batteryLevelChanged() {
        emitEvent("battery-changed", data: getBatteryData())
    }

    @objc private func batteryStateChanged() {
        emitEvent("battery-changed", data: getBatteryData())
    }
}
```

### Audio Devices Commands (Custom Plugin Required)

#### Android: AudioManager

```kotlin
import android.media.AudioManager
import android.media.AudioDeviceInfo
import android.content.Context

@Command
fun getAudioDevices(invoke: Invoke) {
    val audioManager = activity.getSystemService(Context.AUDIO_SERVICE) as AudioManager
    val devices = audioManager.getDevices(AudioManager.GET_DEVICES_ALL)

    val deviceList = devices.map { device ->
        mapOf(
            "id" to device.id.toString(),
            "name" to device.productName.toString(),
            "kind" to if (device.isSink) "audiooutput" else "audioinput",
            "type" to getDeviceType(device.type),
            "isConnected" to true
        )
    }

    invoke.resolve(mapOf("devices" to deviceList))
}

private fun getDeviceType(type: Int): String {
    return when (type) {
        AudioDeviceInfo.TYPE_BUILTIN_SPEAKER -> "built-in"
        AudioDeviceInfo.TYPE_WIRED_HEADPHONES -> "headphones"
        AudioDeviceInfo.TYPE_BLUETOOTH_A2DP -> "bluetooth"
        AudioDeviceInfo.TYPE_USB_HEADSET -> "usb"
        else -> "unknown"
    }
}
```

#### iOS: AVAudioSession

```swift
import AVFoundation

@objc func getAudioDevices(_ invoke: Invoke) {
    let session = AVAudioSession.sharedInstance()

    var devices: [[String: Any]] = []

    // Current route
    let currentRoute = session.currentRoute

    for output in currentRoute.outputs {
        let device: [String: Any] = [
            "id": output.uid,
            "name": output.portName,
            "kind": "audiooutput",
            "type": getPortType(output.portType),
            "isConnected": true,
            "isDefault": true
        ]
        devices.append(device)
    }

    for input in currentRoute.inputs {
        let device: [String: Any] = [
            "id": input.uid,
            "name": input.portName,
            "kind": "audioinput",
            "type": getPortType(input.portType),
            "isConnected": true,
            "isDefault": true
        ]
        devices.append(device)
    }

    invoke.resolve(["devices": devices])
}

private func getPortType(_ portType: AVAudioSession.Port) -> String {
    switch portType {
    case .builtInSpeaker:
        return "built-in"
    case .headphones:
        return "headphones"
    case .bluetoothA2DP, .bluetoothHFP, .bluetoothLE:
        return "bluetooth"
    case .usbAudio:
        return "usb"
    default:
        return "unknown"
    }
}
```

#### Rust Bridge Commands

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BatteryInfo {
    level: i32,
    charging: bool,
    temperature: Option<i32>,
    power_source: String,
    battery_state: String,
    charging_time: Option<i64>,
    discharging_time: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct AudioDevice {
    id: String,
    name: String,
    kind: String,
    is_default: bool,
    is_connected: bool,
    device_type: String,
}

#[tauri::command]
async fn get_battery_info() -> Result<BatteryInfo, String> {
    #[cfg(mobile)]
    {
        mobile::system_services::get_battery_info()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        // Fallback to Web Battery API or custom implementation
        Err("Battery info not available on desktop".to_string())
    }
}

#[tauri::command]
async fn start_battery_monitoring() -> Result<bool, String> {
    #[cfg(mobile)]
    {
        mobile::system_services::start_battery_monitoring()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Ok(false)
    }
}

#[tauri::command]
async fn get_audio_devices() -> Result<Vec<AudioDevice>, String> {
    #[cfg(mobile)]
    {
        mobile::system_services::get_audio_devices()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        // Fallback to Web Audio API
        Err("Audio device enumeration not available on desktop".to_string())
    }
}
```

## Frontend Implementation

### TypeScript Interfaces

```typescript
interface BatteryInfo {
  level: number
  charging: boolean
  chargingTime: number | null
  dischargingTime: number | null
  temperature: number | null
  powerSource: 'battery' | 'ac' | 'usb' | 'wireless' | 'unknown'
  batteryState: 'full' | 'charging' | 'discharging' | 'not_charging' | 'unknown'
}

interface AudioDevice {
  id: string
  name: string
  kind: 'audioinput' | 'audiooutput'
  isDefault: boolean
  isConnected: boolean
  type: 'speaker' | 'headphones' | 'bluetooth' | 'usb' | 'built-in' | 'unknown'
}

interface ClipboardEntry {
  id: string
  text: string
  timestamp: number
}
```

### React Component Example

```typescript
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager'

function SystemServicesPage() {
  const [clipboardText, setClipboardText] = useState('')
  const [clipboardHistory, setClipboardHistory] = useState<ClipboardEntry[]>([])
  const [batteryInfo, setBatteryInfo] = useState<BatteryInfo | null>(null)
  const [audioDevices, setAudioDevices] = useState<AudioDevice[]>([])

  useEffect(() => {
    // Initialize
    loadBatteryInfo()
    loadAudioDevices()

    // Listen for battery changes
    const unlistenBattery = listen<BatteryInfo>('battery-changed', (event) => {
      setBatteryInfo(event.payload)
    })

    // Start battery monitoring
    invoke('start_battery_monitoring').catch(console.error)

    return () => {
      unlistenBattery.then((fn) => fn())
    }
  }, [])

  const loadBatteryInfo = async () => {
    try {
      const info = await invoke<BatteryInfo>('get_battery_info')
      setBatteryInfo(info)
    } catch (error) {
      // Try Web Battery API
      try {
        const battery = await (navigator as any).getBattery()
        setBatteryInfo({
          level: Math.round(battery.level * 100),
          charging: battery.charging,
          chargingTime: battery.chargingTime,
          dischargingTime: battery.dischargingTime,
          temperature: null,
          powerSource: battery.charging ? 'ac' : 'battery',
          batteryState: battery.charging ? 'charging' : 'discharging',
        })

        // Listen for battery events
        battery.addEventListener('levelchange', () => {
          setBatteryInfo((prev) => ({
            ...prev!,
            level: Math.round(battery.level * 100),
          }))
        })
      } catch {
        console.error('Battery API not available')
      }
    }
  }

  const loadAudioDevices = async () => {
    try {
      const devices = await invoke<AudioDevice[]>('get_audio_devices')
      setAudioDevices(devices)
    } catch (error) {
      // Try Web Audio API
      try {
        const devices = await navigator.mediaDevices.enumerateDevices()
        const audioDeviceList: AudioDevice[] = devices
          .filter((d) => d.kind === 'audioinput' || d.kind === 'audiooutput')
          .map((d) => ({
            id: d.deviceId,
            name: d.label || 'Unnamed Device',
            kind: d.kind as 'audioinput' | 'audiooutput',
            isDefault: d.deviceId === 'default',
            isConnected: true,
            type: 'unknown',
          }))
        setAudioDevices(audioDeviceList)
      } catch {
        console.error('Audio device enumeration not available')
      }
    }
  }

  const handleCopyToClipboard = async () => {
    try {
      await writeText(clipboardText)

      // Add to history
      const entry: ClipboardEntry = {
        id: Date.now().toString(),
        text: clipboardText,
        timestamp: Date.now(),
      }
      setClipboardHistory((prev) => [entry, ...prev].slice(0, 5))

      console.log('Copied to clipboard!')
    } catch (error) {
      console.error('Failed to copy:', error)
    }
  }

  const handlePasteFromClipboard = async () => {
    try {
      const text = await readText()
      setClipboardText(text || '')
    } catch (error) {
      console.error('Failed to paste:', error)
    }
  }

  return (
    <div className="space-y-6">
      {/* Clipboard Section */}
      <div className="card">
        <h3>Clipboard Manager</h3>
        <div className="space-y-3">
          <textarea
            className="w-full p-2 border rounded"
            rows={4}
            value={clipboardText}
            onChange={(e) => setClipboardText(e.target.value)}
            placeholder="Enter text to copy..."
          />
          <div className="flex gap-2">
            <button onClick={handleCopyToClipboard}>Copy to Clipboard</button>
            <button onClick={handlePasteFromClipboard}>Paste from Clipboard</button>
          </div>
        </div>

        {/* Clipboard History */}
        {clipboardHistory.length > 0 && (
          <div className="mt-4">
            <h4 className="text-sm font-medium mb-2">Recent Copies:</h4>
            <div className="space-y-2">
              {clipboardHistory.map((entry) => (
                <div
                  key={entry.id}
                  className="text-sm p-2 bg-muted rounded border cursor-pointer"
                  onClick={() => setClipboardText(entry.text)}
                >
                  <div className="truncate">{entry.text}</div>
                  <div className="text-xs text-muted-foreground">
                    {new Date(entry.timestamp).toLocaleTimeString()}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Battery Info Section */}
      <div className="card">
        <h3>Battery & Power</h3>
        {batteryInfo ? (
          <div className="grid grid-cols-2 gap-4">
            <div>
              <div className="text-sm text-muted-foreground">Level</div>
              <div className="text-2xl font-bold">{batteryInfo.level}%</div>
            </div>
            <div>
              <div className="text-sm text-muted-foreground">Status</div>
              <div className="text-lg">{batteryInfo.batteryState}</div>
            </div>
            <div>
              <div className="text-sm text-muted-foreground">Charging</div>
              <div className="text-lg">{batteryInfo.charging ? '✓ Yes' : '✗ No'}</div>
            </div>
            <div>
              <div className="text-sm text-muted-foreground">Power Source</div>
              <div className="text-lg">{batteryInfo.powerSource}</div>
            </div>
            {batteryInfo.temperature && (
              <div>
                <div className="text-sm text-muted-foreground">Temperature</div>
                <div className="text-lg">{batteryInfo.temperature}°C</div>
              </div>
            )}
          </div>
        ) : (
          <p className="text-muted-foreground">Battery info not available</p>
        )}
      </div>

      {/* Audio Devices Section */}
      <div className="card">
        <h3>Audio Devices</h3>
        <button onClick={loadAudioDevices} className="mb-4">
          Refresh Devices
        </button>

        <div className="space-y-4">
          {/* Output Devices */}
          <div>
            <h4 className="text-sm font-medium mb-2">Output Devices:</h4>
            <div className="space-y-2">
              {audioDevices
                .filter((d) => d.kind === 'audiooutput')
                .map((device) => (
                  <div
                    key={device.id}
                    className="p-3 bg-muted rounded border"
                  >
                    <div className="font-medium">{device.name}</div>
                    <div className="text-xs text-muted-foreground">
                      {device.type} {device.isDefault && '(Default)'}
                    </div>
                  </div>
                ))}
            </div>
          </div>

          {/* Input Devices */}
          <div>
            <h4 className="text-sm font-medium mb-2">Input Devices:</h4>
            <div className="space-y-2">
              {audioDevices
                .filter((d) => d.kind === 'audioinput')
                .map((device) => (
                  <div
                    key={device.id}
                    className="p-3 bg-muted rounded border"
                  >
                    <div className="font-medium">{device.name}</div>
                    <div className="text-xs text-muted-foreground">
                      {device.type} {device.isDefault && '(Default)'}
                    </div>
                  </div>
                ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
```

## Testing Checklist

### Desktop Testing
- [ ] Clipboard copy/paste works
- [ ] Clipboard history tracking
- [ ] Battery API works (if supported)
- [ ] Audio device enumeration
- [ ] Default device detection

### Android Testing
- [ ] Clipboard operations work
- [ ] Battery level accurate
- [ ] Charging state detection
- [ ] Battery temperature reading
- [ ] Power source detection
- [ ] Audio device enumeration
- [ ] Bluetooth device detection
- [ ] Battery state change events

### iOS Testing
- [ ] Clipboard operations work
- [ ] Battery level accurate
- [ ] Charging state detection
- [ ] Audio session management
- [ ] Device route changes
- [ ] Battery monitoring events

### Edge Cases
- [ ] Empty clipboard handling
- [ ] Very long clipboard text
- [ ] Battery API unavailable
- [ ] No audio devices connected
- [ ] Permission denied scenarios
- [ ] Low battery scenarios

## Implementation Status

### Backend
- [ ] Clipboard commands (via official plugin)
- [ ] Android battery plugin
- [ ] iOS battery plugin
- [ ] Android audio device plugin
- [ ] iOS audio device plugin
- [ ] Desktop battery implementation
- [ ] Desktop audio device implementation
- [ ] Event emission system

### Frontend
- [ ] Clipboard UI with history
- [ ] Battery info display
- [ ] Audio device list UI
- [ ] Real-time battery monitoring
- [ ] Device change notifications
- [ ] Error handling
- [ ] Loading states

### Features Implemented
- [ ] Text clipboard operations
- [ ] Clipboard history (app-level)
- [ ] Battery level display
- [ ] Charging status
- [ ] Power source detection
- [ ] Audio device enumeration
- [ ] Default device detection
- [ ] Real-time battery updates

### Testing
- [ ] Desktop clipboard tested
- [ ] Mobile clipboard tested
- [ ] Battery monitoring tested
- [ ] Audio device listing tested
- [ ] Event listeners tested
- [ ] Error scenarios tested

## Troubleshooting

### Clipboard Not Working
**Issue**: Cannot read/write clipboard

**Solutions**:
- Verify plugin is properly initialized
- Check permissions in capabilities config
- Ensure clipboard has content before reading
- Try with plain text first

### Battery Info Not Available
**Issue**: Battery API returns null or error

**Solutions**:
- Web Battery API is deprecated/unavailable in some browsers
- Use custom plugin for reliable mobile support
- Desktop may not expose battery info on some devices
- Check device battery monitoring is enabled (iOS)

### Audio Devices Not Listed
**Issue**: No audio devices returned

**Solutions**:
- Request microphone permissions (required for device enumeration)
- Verify MediaDevices API is available
- Check devices are properly connected
- Some platforms restrict device info access

### Battery Temperature Not Showing
**Issue**: Temperature is null

**Solutions**:
- iOS doesn't expose battery temperature via public APIs
- Android requires additional permissions for detailed stats
- Desktop platforms generally don't provide temperature data

## Resources

### Official Documentation
- [Tauri Clipboard Plugin](https://v2.tauri.app/plugin/clipboard-manager/)
- [Web Battery Status API](https://developer.mozilla.org/en-US/docs/Web/API/Battery_Status_API)
- [Web Audio API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)
- [Android BatteryManager](https://developer.android.com/reference/android/os/BatteryManager)
- [Android AudioManager](https://developer.android.com/reference/android/media/AudioManager)
- [iOS UIDevice Battery](https://developer.apple.com/documentation/uikit/uidevice)
- [iOS AVAudioSession](https://developer.apple.com/documentation/avfoundation/avaudiosession)

### Tauri Resources
- [Tauri Mobile Plugin Development](https://tauri.app/develop/plugins/)
- [Tauri Event System](https://tauri.app/develop/calling-frontend/)
- [Tauri Permissions](https://tauri.app/security/capabilities/)

## Platform Support

| Feature | Windows | macOS | Linux | iOS | Android |
|---------|---------|-------|-------|-----|---------|
| **Clipboard** |
| Text Copy/Paste | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Battery & Power** |
| Battery Level | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| Charging Status | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| Battery Temperature | ❌ | ❌ | ❌ | ❌ | ✅ |
| Power Source Detection | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| Battery Monitoring | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| **Audio Devices** |
| Device Enumeration | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| Default Device Detection | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| Device Type Detection | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| Connection Events | 🔶* | 🔶* | 🔶* | ✅ | ✅ |

**Notes:**
- ✅ = Fully supported
- 🔶 = Partial support (Web API) or custom plugin required
- ❌ = Not available via public APIs
- \* Desktop battery/audio features require platform-specific implementations or rely on limited Web APIs

---

Last Updated: November 2025
Module Version: 1.0.0
Status: Planned
