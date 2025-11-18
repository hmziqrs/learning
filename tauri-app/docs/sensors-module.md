# Sensors & Device Hardware Module Implementation

## Overview

Access device sensors for motion-controlled UI, compass applications, AR experiences, and GPS/location tracking. Provides real-time access to accelerometer, gyroscope, magnetometer, and geolocation data across desktop and mobile platforms.

## Current Implementation Status

⚠️ **Planned** - Requires custom plugin development for mobile sensors

## Plugin Setup

### Geolocation (Web API)

The simplest approach for location tracking is using the Web Geolocation API:

```typescript
navigator.geolocation.getCurrentPosition(
  (position) => {
    const { latitude, longitude, accuracy } = position.coords
  }
)
```

**Permissions Required:**
- **Android**: `ACCESS_FINE_LOCATION`, `ACCESS_COARSE_LOCATION`
- **iOS**: `NSLocationWhenInUseUsageDescription`
- **Desktop**: Browser permissions

### Motion Sensors (Custom Plugin Required)

For accelerometer, gyroscope, and magnetometer access, custom native plugins are required:

**Android:**
- SensorManager API
- Sensors: `TYPE_ACCELEROMETER`, `TYPE_GYROSCOPE`, `TYPE_MAGNETIC_FIELD`
- Permissions: None required for sensors

**iOS:**
- CoreMotion framework
- CMMotionManager for device motion
- Permissions: None required for sensors

**Desktop:**
- Limited or no native sensor access
- Some laptops have accelerometers (via platform-specific APIs)

### Dependencies

```bash
# For maps integration
bun add leaflet
bun add @types/leaflet -D
```

## Permissions Configuration

### Android Manifest
```xml
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
<uses-feature android:name="android.hardware.sensor.accelerometer" android:required="false" />
<uses-feature android:name="android.hardware.sensor.gyroscope" android:required="false" />
<uses-feature android:name="android.hardware.sensor.compass" android:required="false" />
```

### iOS Info.plist
```xml
<key>NSLocationWhenInUseUsageDescription</key>
<string>We need your location to provide location-based features</string>
<key>NSLocationAlwaysUsageDescription</key>
<string>We need your location for continuous tracking</string>
<key>NSMotionUsageDescription</key>
<string>We need motion data for activity tracking</string>
```

### Tauri Capabilities
```json
{
  "permissions": [
    "geolocation:allow-current-position",
    "geolocation:allow-watch-position"
  ]
}
```

## Core Features

- [ ] Get current GPS location
- [ ] Watch location (continuous tracking)
- [ ] Display location on map
- [ ] Read accelerometer data (X, Y, Z axes)
- [ ] Read gyroscope data (rotation rate)
- [ ] Read magnetometer data (compass heading)
- [ ] Live 3-axis acceleration graph
- [ ] Shake detection
- [ ] Step counter (pedometer)
- [ ] Device orientation tracking
- [ ] Calculate distance traveled
- [ ] Sensor availability detection

## Data Structures

### Location Data Schema
```typescript
interface LocationData {
  latitude: number
  longitude: number
  accuracy: number
  altitude: number | null
  altitudeAccuracy: number | null
  heading: number | null
  speed: number | null
  timestamp: number
}
```

### Sensor Data Schema
```typescript
interface SensorData {
  x: number
  y: number
  z: number
  timestamp: number
}

interface AccelerometerData extends SensorData {
  type: 'accelerometer'
}

interface GyroscopeData extends SensorData {
  type: 'gyroscope'
  unit: 'rad/s'
}

interface MagnetometerData extends SensorData {
  type: 'magnetometer'
  unit: 'μT' // microtesla
}
```

### Compass Data Schema
```typescript
interface CompassData {
  heading: number // 0-360 degrees
  accuracy: number
  timestamp: number
}
```

### Motion Event Schema
```typescript
interface MotionEvent {
  type: 'shake' | 'tilt' | 'rotation'
  intensity: number
  timestamp: number
}
```

## Rust Backend

### Geolocation Commands (Web API Wrapper)

#### 1. Location Status
```rust
#[tauri::command]
async fn check_location_permission() -> Result<bool, String> {
    // This would be handled by the frontend using navigator.permissions
    // Backend can provide fallback for native implementations
    Ok(true)
}
```

### Mobile Sensor Implementation (Custom Plugin Required)

#### Android: Sensor Manager

```kotlin
// Custom Tauri plugin for Android sensors
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager
import android.content.Context

class SensorPlugin(private val activity: Activity) : SensorEventListener {
    private val sensorManager: SensorManager =
        activity.getSystemService(Context.SENSOR_SERVICE) as SensorManager

    private var accelerometer: Sensor? = null
    private var gyroscope: Sensor? = null
    private var magnetometer: Sensor? = null

    init {
        accelerometer = sensorManager.getDefaultSensor(Sensor.TYPE_ACCELEROMETER)
        gyroscope = sensorManager.getDefaultSensor(Sensor.TYPE_GYROSCOPE)
        magnetometer = sensorManager.getDefaultSensor(Sensor.TYPE_MAGNETIC_FIELD)
    }

    @Command
    fun startAccelerometer(invoke: Invoke) {
        accelerometer?.let {
            sensorManager.registerListener(
                this,
                it,
                SensorManager.SENSOR_DELAY_UI
            )
            invoke.resolve(mapOf("success" to true))
        } ?: invoke.reject("Accelerometer not available")
    }

    @Command
    fun startGyroscope(invoke: Invoke) {
        gyroscope?.let {
            sensorManager.registerListener(
                this,
                it,
                SensorManager.SENSOR_DELAY_UI
            )
            invoke.resolve(mapOf("success" to true))
        } ?: invoke.reject("Gyroscope not available")
    }

    @Command
    fun startMagnetometer(invoke: Invoke) {
        magnetometer?.let {
            sensorManager.registerListener(
                this,
                it,
                SensorManager.SENSOR_DELAY_UI
            )
            invoke.resolve(mapOf("success" to true))
        } ?: invoke.reject("Magnetometer not available")
    }

    @Command
    fun stopAllSensors(invoke: Invoke) {
        sensorManager.unregisterListener(this)
        invoke.resolve(mapOf("success" to true))
    }

    @Command
    fun getSensorAvailability(invoke: Invoke) {
        val availability = mapOf(
            "accelerometer" to (accelerometer != null),
            "gyroscope" to (gyroscope != null),
            "magnetometer" to (magnetometer != null)
        )
        invoke.resolve(availability)
    }

    override fun onSensorChanged(event: SensorEvent) {
        val data = mapOf(
            "x" to event.values[0],
            "y" to event.values[1],
            "z" to event.values[2],
            "timestamp" to event.timestamp,
            "accuracy" to event.accuracy
        )

        when (event.sensor.type) {
            Sensor.TYPE_ACCELEROMETER -> {
                // Emit event to frontend
                emitEvent("accelerometer-data", data)
            }
            Sensor.TYPE_GYROSCOPE -> {
                emitEvent("gyroscope-data", data)
            }
            Sensor.TYPE_MAGNETIC_FIELD -> {
                emitEvent("magnetometer-data", data)
            }
        }
    }

    override fun onAccuracyChanged(sensor: Sensor, accuracy: Int) {
        // Handle accuracy changes
    }

    private fun emitEvent(eventName: String, data: Map<String, Any>) {
        // Send to Tauri frontend via plugin channel
    }
}
```

#### iOS: CoreMotion

```swift
// Custom Tauri plugin for iOS sensors
import CoreMotion
import CoreLocation

class SensorPlugin: NSObject {
    private let motionManager = CMMotionManager()
    private let locationManager = CLLocationManager()

    override init() {
        super.init()
        setupLocationManager()
    }

    private func setupLocationManager() {
        locationManager.delegate = self
        locationManager.desiredAccuracy = kCLLocationAccuracyBest
    }

    @objc func startAccelerometer(_ invoke: Invoke) {
        guard motionManager.isAccelerometerAvailable else {
            invoke.reject("Accelerometer not available")
            return
        }

        motionManager.accelerometerUpdateInterval = 0.1
        motionManager.startAccelerometerUpdates(to: .main) { [weak self] (data, error) in
            guard let data = data else { return }

            let payload: [String: Any] = [
                "x": data.acceleration.x,
                "y": data.acceleration.y,
                "z": data.acceleration.z,
                "timestamp": Date().timeIntervalSince1970
            ]

            self?.emitEvent("accelerometer-data", data: payload)
        }

        invoke.resolve(["success": true])
    }

    @objc func startGyroscope(_ invoke: Invoke) {
        guard motionManager.isGyroAvailable else {
            invoke.reject("Gyroscope not available")
            return
        }

        motionManager.gyroUpdateInterval = 0.1
        motionManager.startGyroUpdates(to: .main) { [weak self] (data, error) in
            guard let data = data else { return }

            let payload: [String: Any] = [
                "x": data.rotationRate.x,
                "y": data.rotationRate.y,
                "z": data.rotationRate.z,
                "timestamp": Date().timeIntervalSince1970
            ]

            self?.emitEvent("gyroscope-data", data: payload)
        }

        invoke.resolve(["success": true])
    }

    @objc func startMagnetometer(_ invoke: Invoke) {
        guard motionManager.isMagnetometerAvailable else {
            invoke.reject("Magnetometer not available")
            return
        }

        motionManager.magnetometerUpdateInterval = 0.1
        motionManager.startMagnetometerUpdates(to: .main) { [weak self] (data, error) in
            guard let data = data else { return }

            let payload: [String: Any] = [
                "x": data.magneticField.x,
                "y": data.magneticField.y,
                "z": data.magneticField.z,
                "timestamp": Date().timeIntervalSince1970
            ]

            self?.emitEvent("magnetometer-data", data: payload)
        }

        invoke.resolve(["success": true])
    }

    @objc func stopAllSensors(_ invoke: Invoke) {
        motionManager.stopAccelerometerUpdates()
        motionManager.stopGyroUpdates()
        motionManager.stopMagnetometerUpdates()
        motionManager.stopDeviceMotionUpdates()

        invoke.resolve(["success": true])
    }

    @objc func getSensorAvailability(_ invoke: Invoke) {
        let availability: [String: Any] = [
            "accelerometer": motionManager.isAccelerometerAvailable,
            "gyroscope": motionManager.isGyroAvailable,
            "magnetometer": motionManager.isMagnetometerAvailable,
            "deviceMotion": motionManager.isDeviceMotionAvailable
        ]

        invoke.resolve(availability)
    }

    @objc func requestLocationPermission(_ invoke: Invoke) {
        locationManager.requestWhenInUseAuthorization()
        invoke.resolve(["success": true])
    }

    private func emitEvent(_ eventName: String, data: [String: Any]) {
        // Send to Tauri frontend via plugin channel
    }
}

extension SensorPlugin: CLLocationManagerDelegate {
    func locationManager(_ manager: CLLocationManager, didUpdateLocations locations: [CLLocation]) {
        guard let location = locations.last else { return }

        let payload: [String: Any] = [
            "latitude": location.coordinate.latitude,
            "longitude": location.coordinate.longitude,
            "accuracy": location.horizontalAccuracy,
            "altitude": location.altitude,
            "heading": location.course,
            "speed": location.speed,
            "timestamp": location.timestamp.timeIntervalSince1970
        ]

        emitEvent("location-update", data: payload)
    }
}
```

#### Rust Bridge Commands

```rust
use tauri::Emitter;

#[tauri::command]
async fn start_accelerometer() -> Result<bool, String> {
    // Call mobile plugin via Tauri Mobile API
    #[cfg(mobile)]
    {
        // Mobile-specific implementation
        mobile::sensors::start_accelerometer()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Err("Accelerometer not available on desktop".to_string())
    }
}

#[tauri::command]
async fn start_gyroscope() -> Result<bool, String> {
    #[cfg(mobile)]
    {
        mobile::sensors::start_gyroscope()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Err("Gyroscope not available on desktop".to_string())
    }
}

#[tauri::command]
async fn start_magnetometer() -> Result<bool, String> {
    #[cfg(mobile)]
    {
        mobile::sensors::start_magnetometer()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Err("Magnetometer not available on desktop".to_string())
    }
}

#[tauri::command]
async fn stop_all_sensors() -> Result<bool, String> {
    #[cfg(mobile)]
    {
        mobile::sensors::stop_all()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Ok(true)
    }
}

#[tauri::command]
async fn get_sensor_availability() -> Result<SensorAvailability, String> {
    #[cfg(mobile)]
    {
        mobile::sensors::get_availability()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Ok(SensorAvailability {
            accelerometer: false,
            gyroscope: false,
            magnetometer: false,
            geolocation: true, // Web API available
        })
    }
}

#[derive(serde::Serialize)]
struct SensorAvailability {
    accelerometer: bool,
    gyroscope: bool,
    magnetometer: bool,
    geolocation: bool,
}
```

## Frontend Implementation

### TypeScript Interfaces

```typescript
// Sensor data types
interface SensorData {
  x: number
  y: number
  z: number
  timestamp: number
}

interface LocationData {
  latitude: number
  longitude: number
  accuracy: number
  altitude: number | null
  heading: number | null
  speed: number | null
  timestamp: number
}

interface SensorAvailability {
  accelerometer: boolean
  gyroscope: boolean
  magnetometer: boolean
  geolocation: boolean
}
```

### React Component Example

```typescript
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

function SensorsPage() {
  const [availability, setAvailability] = useState<SensorAvailability | null>(null)
  const [accelerometerData, setAccelerometerData] = useState<SensorData | null>(null)
  const [gyroscopeData, setGyroscopeData] = useState<SensorData | null>(null)
  const [magnetometerData, setMagnetometerData] = useState<SensorData | null>(null)
  const [location, setLocation] = useState<LocationData | null>(null)
  const [isTracking, setIsTracking] = useState(false)

  useEffect(() => {
    // Check sensor availability
    checkAvailability()

    // Listen for sensor events
    const unlistenAccel = listen<SensorData>('accelerometer-data', (event) => {
      setAccelerometerData(event.payload)
    })

    const unlistenGyro = listen<SensorData>('gyroscope-data', (event) => {
      setGyroscopeData(event.payload)
    })

    const unlistenMag = listen<SensorData>('magnetometer-data', (event) => {
      setMagnetometerData(event.payload)
    })

    const unlistenLoc = listen<LocationData>('location-update', (event) => {
      setLocation(event.payload)
    })

    return () => {
      Promise.all([unlistenAccel, unlistenGyro, unlistenMag, unlistenLoc]).then(
        (cleanups) => cleanups.forEach((cleanup) => cleanup())
      )
    }
  }, [])

  const checkAvailability = async () => {
    try {
      const result = await invoke<SensorAvailability>('get_sensor_availability')
      setAvailability(result)
    } catch (error) {
      console.error('Failed to check sensor availability:', error)
    }
  }

  const startAccelerometer = async () => {
    try {
      await invoke('start_accelerometer')
      setIsTracking(true)
    } catch (error) {
      console.error('Failed to start accelerometer:', error)
    }
  }

  const stopAllSensors = async () => {
    try {
      await invoke('stop_all_sensors')
      setIsTracking(false)
    } catch (error) {
      console.error('Failed to stop sensors:', error)
    }
  }

  const getCurrentLocation = () => {
    if ('geolocation' in navigator) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          setLocation({
            latitude: position.coords.latitude,
            longitude: position.coords.longitude,
            accuracy: position.coords.accuracy,
            altitude: position.coords.altitude,
            heading: position.coords.heading,
            speed: position.coords.speed,
            timestamp: position.timestamp,
          })
        },
        (error) => {
          console.error('Geolocation error:', error)
        }
      )
    }
  }

  return (
    <div className="space-y-6">
      {/* Sensor Availability */}
      <div className="card">
        <h3>Sensor Availability</h3>
        {availability && (
          <div className="grid grid-cols-2 gap-4">
            <div>Accelerometer: {availability.accelerometer ? '✓' : '✗'}</div>
            <div>Gyroscope: {availability.gyroscope ? '✓' : '✗'}</div>
            <div>Magnetometer: {availability.magnetometer ? '✓' : '✗'}</div>
            <div>Geolocation: {availability.geolocation ? '✓' : '✗'}</div>
          </div>
        )}
      </div>

      {/* Controls */}
      <div className="flex gap-4">
        <button onClick={startAccelerometer} disabled={isTracking}>
          Start Sensors
        </button>
        <button onClick={stopAllSensors} disabled={!isTracking}>
          Stop Sensors
        </button>
        <button onClick={getCurrentLocation}>Get Location</button>
      </div>

      {/* Sensor Data Display */}
      <div className="grid grid-cols-3 gap-4">
        <div className="card">
          <h4>Accelerometer</h4>
          {accelerometerData && (
            <div>
              <div>X: {accelerometerData.x.toFixed(2)}</div>
              <div>Y: {accelerometerData.y.toFixed(2)}</div>
              <div>Z: {accelerometerData.z.toFixed(2)}</div>
            </div>
          )}
        </div>

        <div className="card">
          <h4>Gyroscope</h4>
          {gyroscopeData && (
            <div>
              <div>X: {gyroscopeData.x.toFixed(2)}</div>
              <div>Y: {gyroscopeData.y.toFixed(2)}</div>
              <div>Z: {gyroscopeData.z.toFixed(2)}</div>
            </div>
          )}
        </div>

        <div className="card">
          <h4>Magnetometer</h4>
          {magnetometerData && (
            <div>
              <div>X: {magnetometerData.x.toFixed(2)}</div>
              <div>Y: {magnetometerData.y.toFixed(2)}</div>
              <div>Z: {magnetometerData.z.toFixed(2)}</div>
            </div>
          )}
        </div>
      </div>

      {/* Location Data */}
      {location && (
        <div className="card">
          <h4>Location</h4>
          <div className="grid grid-cols-2 gap-2">
            <div>Latitude: {location.latitude.toFixed(6)}</div>
            <div>Longitude: {location.longitude.toFixed(6)}</div>
            <div>Accuracy: {location.accuracy.toFixed(2)}m</div>
            <div>Speed: {location.speed ? `${location.speed.toFixed(2)}m/s` : 'N/A'}</div>
          </div>
        </div>
      )}
    </div>
  )
}
```

### Web Geolocation API Integration

```typescript
// Helper function for geolocation
const useGeolocation = () => {
  const [location, setLocation] = useState<LocationData | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [watchId, setWatchId] = useState<number | null>(null)

  const getCurrentPosition = () => {
    if (!('geolocation' in navigator)) {
      setError('Geolocation not supported')
      return
    }

    navigator.geolocation.getCurrentPosition(
      (position) => {
        setLocation({
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
          accuracy: position.coords.accuracy,
          altitude: position.coords.altitude,
          heading: position.coords.heading,
          speed: position.coords.speed,
          timestamp: position.timestamp,
        })
        setError(null)
      },
      (err) => {
        setError(err.message)
      },
      {
        enableHighAccuracy: true,
        timeout: 5000,
        maximumAge: 0,
      }
    )
  }

  const watchPosition = () => {
    if (!('geolocation' in navigator)) {
      setError('Geolocation not supported')
      return
    }

    const id = navigator.geolocation.watchPosition(
      (position) => {
        setLocation({
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
          accuracy: position.coords.accuracy,
          altitude: position.coords.altitude,
          heading: position.coords.heading,
          speed: position.coords.speed,
          timestamp: position.timestamp,
        })
        setError(null)
      },
      (err) => {
        setError(err.message)
      },
      {
        enableHighAccuracy: true,
        timeout: 5000,
        maximumAge: 0,
      }
    )

    setWatchId(id)
  }

  const clearWatch = () => {
    if (watchId !== null) {
      navigator.geolocation.clearWatch(watchId)
      setWatchId(null)
    }
  }

  return { location, error, getCurrentPosition, watchPosition, clearWatch }
}
```

## UI Components

### Sensor Dashboard
```typescript
import { Activity, Compass, MapPin, Radio } from 'lucide-react'

<div className="grid grid-cols-1 md:grid-cols-2 gap-6">
  {/* Motion Sensors */}
  <div className="card">
    <div className="flex items-center gap-2 mb-4">
      <Activity className="w-5 h-5" />
      <h3>Motion Sensors</h3>
    </div>

    {/* 3-axis visualization */}
    <canvas ref={accelerometerCanvasRef} width={400} height={300} />

    <div className="mt-4 space-y-2">
      <button onClick={startAccelerometer}>Start Accelerometer</button>
      <button onClick={startGyroscope}>Start Gyroscope</button>
    </div>
  </div>

  {/* Compass */}
  <div className="card">
    <div className="flex items-center gap-2 mb-4">
      <Compass className="w-5 h-5" />
      <h3>Compass</h3>
    </div>

    {/* Compass visualization */}
    <div className="compass-display">
      {/* Rotating compass needle based on heading */}
    </div>
  </div>

  {/* GPS Location */}
  <div className="card col-span-2">
    <div className="flex items-center gap-2 mb-4">
      <MapPin className="w-5 h-5" />
      <h3>GPS Location</h3>
    </div>

    {/* Map display using Leaflet */}
    <div id="map" style={{ height: '400px' }} />
  </div>
</div>
```

### Map Integration (Leaflet.js)

```typescript
import L from 'leaflet'
import 'leaflet/dist/leaflet.css'

useEffect(() => {
  if (!location) return

  // Initialize map
  const map = L.map('map').setView([location.latitude, location.longitude], 13)

  // Add tile layer
  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '© OpenStreetMap contributors',
  }).addTo(map)

  // Add marker
  L.marker([location.latitude, location.longitude])
    .addTo(map)
    .bindPopup(`Accuracy: ${location.accuracy.toFixed(2)}m`)
    .openPopup()

  return () => {
    map.remove()
  }
}, [location])
```

## Testing Checklist

### Desktop Testing
- [ ] Geolocation API works in browser
- [ ] Proper error handling for unsupported sensors
- [ ] Map displays correctly
- [ ] Permission prompts appear

### Android Testing
- [ ] Request location permissions at runtime
- [ ] Accelerometer data streams correctly
- [ ] Gyroscope data streams correctly
- [ ] Magnetometer data streams correctly
- [ ] GPS location updates accurately
- [ ] Sensor events properly cleaned up
- [ ] Background location tracking (if implemented)
- [ ] Battery usage is acceptable

### iOS Testing
- [ ] Request location permissions at runtime
- [ ] CoreMotion provides accurate sensor data
- [ ] Compass heading is correct
- [ ] Location accuracy is high
- [ ] App doesn't crash on sensor errors
- [ ] Proper cleanup on app background

### Edge Cases
- [ ] Handle sensor unavailability gracefully
- [ ] Handle permission denial
- [ ] Handle location services disabled
- [ ] Multiple rapid start/stop calls
- [ ] App backgrounding during sensor use
- [ ] Low battery impact

## Implementation Status

### Backend
- [ ] Rust sensor availability command
- [ ] Android sensor plugin setup
- [ ] iOS CoreMotion plugin setup
- [ ] Geolocation permission handling
- [ ] Event emission system

### Frontend
- [ ] Sensor availability check UI
- [ ] Real-time sensor data display
- [ ] 3-axis graph visualization
- [ ] Compass UI component
- [ ] Map integration (Leaflet)
- [ ] Location tracking UI
- [ ] Shake detection
- [ ] Error handling and user feedback

### Features Implemented
- [ ] Accelerometer access
- [ ] Gyroscope access
- [ ] Magnetometer access
- [ ] Geolocation (current position)
- [ ] Geolocation (watch position)
- [ ] Compass heading calculation
- [ ] Motion event detection
- [ ] Map visualization

### Testing
- [ ] Desktop geolocation tested
- [ ] Android sensors tested
- [ ] iOS sensors tested
- [ ] Permission flows tested
- [ ] Error scenarios tested

## Troubleshooting

### Geolocation Not Working
**Issue**: `navigator.geolocation` returns error

**Solutions**:
- Ensure HTTPS connection (required for geolocation API)
- Check browser/WebView permissions
- Verify Android/iOS manifest permissions
- Check if location services are enabled on device

### Sensors Not Available on Android
**Issue**: `getSensorAvailability` returns false

**Solutions**:
- Verify device has physical sensors
- Check SensorManager is properly initialized
- Ensure plugin is registered in Tauri config
- Test on physical device (emulator may not have sensors)

### iOS Sensors Not Working
**Issue**: CoreMotion not providing data

**Solutions**:
- Check Info.plist has required permissions
- Verify motion manager is started on main thread
- Ensure proper delegate setup
- Check device is not in low-power mode

### High Battery Drain
**Issue**: App uses too much battery with sensors

**Solutions**:
- Reduce sensor update frequency
- Stop sensors when app is backgrounded
- Use `SENSOR_DELAY_UI` or `SENSOR_DELAY_NORMAL` instead of `SENSOR_DELAY_FASTEST`
- Implement batching for sensor events

### Map Not Displaying
**Issue**: Leaflet map shows blank

**Solutions**:
- Import Leaflet CSS file
- Ensure map container has explicit height
- Check tile URL is accessible
- Verify coordinates are valid

## Resources

### Official Documentation
- [MDN Geolocation API](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation_API)
- [Android SensorManager](https://developer.android.com/reference/android/hardware/SensorManager)
- [iOS CoreMotion](https://developer.apple.com/documentation/coremotion)
- [iOS CoreLocation](https://developer.apple.com/documentation/corelocation)
- [Leaflet.js Documentation](https://leafletjs.com/)

### Libraries & Tools
- [Leaflet.js](https://leafletjs.com/) - Interactive maps
- [OpenStreetMap](https://www.openstreetmap.org/) - Free map tiles
- [Chart.js](https://www.chartjs.org/) - For sensor data visualization

### Tauri Resources
- [Tauri Mobile Plugin Development](https://tauri.app/develop/plugins/)
- [Tauri Event System](https://tauri.app/develop/calling-frontend/)
- [Tauri Permissions](https://tauri.app/security/capabilities/)

## Platform Support

| Feature | Windows | macOS | Linux | iOS | Android |
|---------|---------|-------|-------|-----|---------|
| Geolocation (Web API) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Accelerometer | ❌* | ❌* | ❌ | ✅ | ✅ |
| Gyroscope | ❌ | ❌ | ❌ | ✅ | ✅ |
| Magnetometer | ❌ | ❌ | ❌ | ✅ | ✅ |
| Compass | ❌ | ❌ | ❌ | ✅ | ✅ |
| Map Display | ✅ | ✅ | ✅ | ✅ | ✅ |

\* Some laptops have accelerometers, but require platform-specific APIs

---

Last Updated: November 2025
Module Version: 1.0.0
Status: Planned
