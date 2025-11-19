# Haptics / Vibrations Module Implementation

## Overview

The Haptics Module provides tactile feedback capabilities for mobile devices, allowing the app to trigger various vibration patterns and haptic effects. This enhances user experience by providing physical feedback for user interactions, notifications, and events.

## Current Implementation Status

⚠️ **Status**: Planned

This module is currently in the planning phase. Implementation requires custom mobile plugin development.

## Plugin Setup

### Dependencies

**Custom Mobile Plugin Required**
- No existing Tauri plugin available
- Requires native platform APIs:
  - **Android**: Vibrator API with VibrationEffect
  - **iOS**: UIFeedbackGenerator

### Cargo Dependencies

```toml
[dependencies]
# Mobile platform dependencies will be added during implementation
```

### Plugin Registration

```rust
// Plugin registration will be added in src-tauri/src/lib.rs
```

## Permissions Configuration

### Android Manifest

```xml
<uses-permission android:name="android.permission.VIBRATE" />
```

### iOS Info.plist

No special permissions required for haptic feedback on iOS.

### Tauri Capabilities

Custom capability file will be created for haptics commands.

## Core Features

- [ ] Light haptic feedback
- [ ] Medium haptic feedback
- [ ] Heavy haptic feedback
- [ ] Success haptic pattern
- [ ] Error/failure haptic pattern
- [ ] Warning haptic pattern
- [ ] Custom vibration duration
- [ ] Custom vibration pattern
- [ ] Haptic feedback availability check

## Data Structures

### TypeScript Interfaces

```typescript
// Haptic feedback types
type HapticImpactStyle = 'light' | 'medium' | 'heavy';
type HapticNotificationType = 'success' | 'warning' | 'error';

interface HapticOptions {
  duration?: number; // milliseconds
  intensity?: number; // 0.0 to 1.0
}

interface VibrationPattern {
  pattern: number[]; // alternating durations of on/off
  repeat?: boolean;
}
```

## Rust Backend

### Commands

```rust
#[tauri::command]
async fn haptic_impact(style: String) -> Result<(), String> {
    // Trigger impact haptic feedback
}

#[tauri::command]
async fn haptic_notification(notification_type: String) -> Result<(), String> {
    // Trigger notification haptic feedback
}

#[tauri::command]
async fn vibrate(duration: u64) -> Result<(), String> {
    // Trigger basic vibration
}

#[tauri::command]
async fn vibrate_pattern(pattern: Vec<u64>) -> Result<(), String> {
    // Trigger pattern vibration
}

#[tauri::command]
async fn cancel_vibration() -> Result<(), String> {
    // Stop ongoing vibration
}

#[tauri::command]
async fn has_vibrator() -> Result<bool, String> {
    // Check if device supports vibration
}
```

### Android Implementation

```kotlin
import android.os.VibrationEffect
import android.os.Vibrator
import android.content.Context

// Light Impact
vibrator.vibrate(VibrationEffect.createOneShot(10, 50))

// Medium Impact
vibrator.vibrate(VibrationEffect.createOneShot(20, 100))

// Heavy Impact
vibrator.vibrate(VibrationEffect.createOneShot(30, 200))

// Pattern Vibration
val pattern = longArrayOf(0, 100, 50, 100)
vibrator.vibrate(VibrationEffect.createWaveform(pattern, -1))
```

### iOS Implementation

```swift
import UIKit

// Impact Feedback
let generator = UIImpactFeedbackGenerator(style: .medium)
generator.prepare()
generator.impactOccurred()

// Notification Feedback
let notificationGenerator = UINotificationFeedbackGenerator()
notificationGenerator.prepare()
notificationGenerator.notificationOccurred(.success)

// Selection Feedback
let selectionGenerator = UISelectionFeedbackGenerator()
selectionGenerator.prepare()
selectionGenerator.selectionChanged()
```

## Frontend Implementation

### React Hook

```typescript
import { invoke } from '@tauri-apps/api/core';

export function useHaptics() {
  const impact = async (style: HapticImpactStyle) => {
    try {
      await invoke('haptic_impact', { style });
    } catch (error) {
      console.error('Haptic impact failed:', error);
    }
  };

  const notification = async (type: HapticNotificationType) => {
    try {
      await invoke('haptic_notification', { notificationType: type });
    } catch (error) {
      console.error('Haptic notification failed:', error);
    }
  };

  const vibrate = async (duration: number) => {
    try {
      await invoke('vibrate', { duration });
    } catch (error) {
      console.error('Vibration failed:', error);
    }
  };

  const vibratePattern = async (pattern: number[]) => {
    try {
      await invoke('vibrate_pattern', { pattern });
    } catch (error) {
      console.error('Pattern vibration failed:', error);
    }
  };

  const cancel = async () => {
    try {
      await invoke('cancel_vibration');
    } catch (error) {
      console.error('Cancel vibration failed:', error);
    }
  };

  const checkAvailability = async () => {
    try {
      return await invoke<boolean>('has_vibrator');
    } catch (error) {
      console.error('Check vibrator failed:', error);
      return false;
    }
  };

  return {
    impact,
    notification,
    vibrate,
    vibratePattern,
    cancel,
    checkAvailability,
  };
}
```

### Component Usage

```tsx
function HapticsDemo() {
  const { impact, notification, vibrate } = useHaptics();

  return (
    <div>
      <Button onClick={() => impact('light')}>Light Tap</Button>
      <Button onClick={() => impact('medium')}>Medium Impact</Button>
      <Button onClick={() => impact('heavy')}>Heavy Impact</Button>
      <Button onClick={() => notification('success')}>Success</Button>
      <Button onClick={() => notification('error')}>Error</Button>
      <Button onClick={() => vibrate(100)}>Vibrate 100ms</Button>
    </div>
  );
}
```

## UI Components

- **Impact Feedback Section**: Buttons for light, medium, heavy haptic impacts
- **Notification Feedback Section**: Buttons for success, warning, error haptics
- **Custom Vibration Section**: Input for duration and trigger button
- **Pattern Vibration Section**: Pattern input and trigger button
- **Status Display**: Shows device vibration capability
- **Output Log**: Real-time feedback on haptic operations

## Testing Checklist

### Android Testing
- [ ] Test on physical Android device (haptics don't work in emulator)
- [ ] Verify VIBRATE permission is granted
- [ ] Test each impact style (light, medium, heavy)
- [ ] Test notification haptics
- [ ] Test custom duration vibrations
- [ ] Test pattern vibrations
- [ ] Test vibration cancellation
- [ ] Test on devices with different Android versions

### iOS Testing
- [ ] Test on physical iOS device (haptics don't work in simulator)
- [ ] Test each UIImpactFeedbackGenerator style
- [ ] Test UINotificationFeedbackGenerator types
- [ ] Test UISelectionFeedbackGenerator
- [ ] Verify feedback on devices with Taptic Engine
- [ ] Test on older devices without Taptic Engine

### Desktop Testing
- [ ] Verify graceful degradation on desktop platforms
- [ ] Display appropriate message that haptics are unavailable
- [ ] Ensure no crashes when calling haptic commands on desktop

## Troubleshooting

### Common Issues

**Haptics Not Working**
- Ensure testing on physical device (not emulator/simulator)
- Check device haptic settings are enabled
- Verify VIBRATE permission on Android
- Confirm device supports haptic feedback

**Weak or Missing Feedback**
- Check device battery saver mode (may disable haptics)
- Verify device haptic intensity settings
- Some Android devices have poor vibration motors

**iOS Haptics Silent**
- Check device mute switch position
- Verify haptic settings in iOS Settings > Sounds & Haptics
- Confirm device has Taptic Engine (iPhone 6s and later)

## Resources

### Android
- [Vibrator API Documentation](https://developer.android.com/reference/android/os/Vibrator)
- [VibrationEffect Documentation](https://developer.android.com/reference/android/os/VibrationEffect)
- [Haptics Design Guidelines](https://developer.android.com/develop/ui/views/haptics)

### iOS
- [UIFeedbackGenerator Documentation](https://developer.apple.com/documentation/uikit/uifeedbackgenerator)
- [Haptic Feedback Guidelines](https://developer.apple.com/design/human-interface-guidelines/playing-haptics)
- [Core Haptics Framework](https://developer.apple.com/documentation/corehaptics)

### General
- [Mobile Haptics Best Practices](https://www.nngroup.com/articles/haptic-feedback/)
- [Haptic Design Patterns](https://www.interaction-design.org/literature/article/haptic-feedback-design-patterns)

## Platform Support

| Feature | Android | iOS | Windows | macOS | Linux |
|---------|---------|-----|---------|-------|-------|
| Light Impact | ✅ | ✅ | ❌ | ❌ | ❌ |
| Medium Impact | ✅ | ✅ | ❌ | ❌ | ❌ |
| Heavy Impact | ✅ | ✅ | ❌ | ❌ | ❌ |
| Success Haptic | ✅ | ✅ | ❌ | ❌ | ❌ |
| Warning Haptic | ✅ | ✅ | ❌ | ❌ | ❌ |
| Error Haptic | ✅ | ✅ | ❌ | ❌ | ❌ |
| Custom Duration | ✅ | ⚠️ | ❌ | ❌ | ❌ |
| Pattern Vibration | ✅ | ⚠️ | ❌ | ❌ | ❌ |
| Selection Haptic | ⚠️ | ✅ | ❌ | ❌ | ❌ |

**Legend:**
- ✅ Fully Supported
- ⚠️ Limited Support
- ❌ Not Supported

## Implementation Status

### Phase 1: Core Setup
- [ ] Create custom mobile plugin structure
- [ ] Add Android Vibrator API integration
- [ ] Add iOS UIFeedbackGenerator integration
- [ ] Register Tauri commands
- [ ] Add platform permissions

### Phase 2: Basic Haptics
- [ ] Implement impact feedback (light, medium, heavy)
- [ ] Implement notification feedback (success, warning, error)
- [ ] Implement device capability check
- [ ] Add error handling and fallbacks

### Phase 3: Advanced Features
- [ ] Implement custom duration vibration
- [ ] Implement pattern vibration
- [ ] Implement vibration cancellation
- [ ] Add iOS selection feedback

### Phase 4: Frontend Integration
- [ ] Create React hooks for haptics
- [ ] Build UI demo page
- [ ] Add output logging
- [ ] Implement desktop fallback behavior

### Phase 5: Testing & Polish
- [ ] Test on Android devices
- [ ] Test on iOS devices
- [ ] Add user documentation
- [ ] Performance optimization
