# System Info & Device Profiling Module

## Overview

The System Info & Device Profiling Module provides comprehensive access to system information, hardware details, and device metrics. This module enables applications to gather diagnostic data, monitor system resources, and profile device capabilities across all supported platforms.

## Current Implementation Status

✅ **Status**: Partially Implemented

**Implemented:**
- Basic system information (OS, architecture, version)
- System resource metrics (CPU, memory, disk, swap)
- Network interface metrics
- WiFi network scanning
- App uptime tracking

**Planned:**
- Device hardware identification
- Detailed CPU information
- GPU information
- Display/screen information
- Mobile-specific device profiling
- Thermal state monitoring
- Power profile information

## Plugin Setup

### Dependencies

**Rust Crates:**
```toml
[dependencies]
sysinfo = "0.32"  # Cross-platform system information
```

### Tauri Capabilities

Custom capability configuration for system information commands.

## Core Features

### Basic System Information
- [x] Operating system name and family
- [x] System architecture (x86_64, arm64, etc.)
- [x] App version
- [x] Process ID
- [ ] Device manufacturer and model
- [ ] Device unique identifier
- [ ] System boot time
- [ ] Kernel version

### Resource Monitoring
- [x] CPU usage percentage
- [x] Memory usage (total, used, available)
- [x] Disk usage (total, used, available)
- [x] Swap memory usage
- [x] Network interface statistics
- [ ] Per-core CPU usage
- [ ] Process-level resource usage
- [ ] GPU usage and memory

### Hardware Information
- [ ] CPU details (name, cores, frequency)
- [ ] GPU details (name, vendor, memory)
- [ ] Display information (resolution, refresh rate)
- [ ] Battery capacity and health
- [ ] Storage devices list
- [ ] Available sensors list

### Network Profiling
- [x] Network interface enumeration
- [x] Data transmitted/received per interface
- [x] WiFi network scanning
- [ ] Active connection details
- [ ] IP address information
- [ ] WiFi signal strength

### App Metrics
- [x] Application uptime
- [ ] App memory footprint
- [ ] App CPU usage
- [ ] App thread count
- [ ] App cache size

## Data Structures

### TypeScript Interfaces

```typescript
interface SystemInfo {
  os: string;
  version: string;
  arch: string;
  app_version: string;
  process_id: number;
}

interface SystemMetrics {
  cpu_usage: number;
  memory_total: number;
  memory_used: number;
  memory_available: number;
  memory_usage_percent: number;
  swap_total: number;
  swap_used: number;
  disk_total: number;
  disk_used: number;
  disk_available: number;
  disk_usage_percent: number;
}

interface NetworkMetrics {
  total_received: number;
  total_transmitted: number;
  interfaces: NetworkInterfaceMetrics[];
}

interface NetworkInterfaceMetrics {
  name: string;
  received: number;
  transmitted: number;
}

interface WiFiNetwork {
  ssid: string;
  bssid: string;
  signal_strength: number;
  channel: number;
  security: string;
}

interface DeviceProfile {
  manufacturer?: string;
  model?: string;
  device_id?: string;
  os_version: string;
  hardware: HardwareProfile;
  capabilities: DeviceCapabilities;
}

interface HardwareProfile {
  cpu: CPUInfo;
  memory: MemoryInfo;
  display: DisplayInfo;
  gpu?: GPUInfo;
}

interface CPUInfo {
  name: string;
  cores: number;
  physical_cores: number;
  frequency: number;
  vendor: string;
}

interface MemoryInfo {
  total: number;
  type?: string;
}

interface DisplayInfo {
  width: number;
  height: number;
  scale_factor: number;
  refresh_rate?: number;
}

interface GPUInfo {
  name: string;
  vendor: string;
  memory?: number;
}

interface DeviceCapabilities {
  has_camera: boolean;
  has_microphone: boolean;
  has_gps: boolean;
  has_nfc: boolean;
  has_bluetooth: boolean;
  has_cellular: boolean;
}
```

### Rust Structures

```rust
#[derive(Debug, Serialize)]
struct SystemInfo {
    os: String,
    version: String,
    arch: String,
    app_version: String,
    process_id: u32,
}

#[derive(Debug, Serialize)]
struct SystemMetrics {
    cpu_usage: f32,
    memory_total: u64,
    memory_used: u64,
    memory_available: u64,
    memory_usage_percent: f32,
    swap_total: u64,
    swap_used: u64,
    disk_total: u64,
    disk_used: u64,
    disk_available: u64,
    disk_usage_percent: f32,
}

#[derive(Debug, Serialize)]
struct NetworkMetrics {
    total_received: u64,
    total_transmitted: u64,
    interfaces: Vec<NetworkInterfaceMetrics>,
}

#[derive(Debug, Serialize)]
struct NetworkInterfaceMetrics {
    name: String,
    received: u64,
    transmitted: u64,
}
```

## Rust Backend

### Implemented Commands

```rust
#[tauri::command]
fn get_system_info() -> SystemInfo

#[tauri::command]
fn get_system_metrics() -> Result<SystemMetrics, String>

#[tauri::command]
fn get_network_metrics() -> Result<NetworkMetrics, String>

#[tauri::command]
fn get_app_uptime() -> u64

#[tauri::command]
async fn scan_wifi_networks() -> Result<Vec<WiFiNetwork>, String>
```

### Planned Commands

```rust
#[tauri::command]
async fn get_device_profile() -> Result<DeviceProfile, String>

#[tauri::command]
fn get_cpu_info() -> Result<CPUInfo, String>

#[tauri::command]
fn get_gpu_info() -> Result<GPUInfo, String>

#[tauri::command]
fn get_display_info() -> Result<DisplayInfo, String>

#[tauri::command]
fn get_thermal_state() -> Result<String, String>

#[tauri::command]
fn get_power_profile() -> Result<String, String>

#[tauri::command]
fn get_storage_devices() -> Result<Vec<StorageDevice>, String>
```

## Frontend Implementation

### React Hook Example

```typescript
import { invoke } from '@tauri-apps/api/core';

export function useSystemInfo() {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [metrics, setMetrics] = useState<SystemMetrics | null>(null);
  const [loading, setLoading] = useState(false);

  const fetchSystemInfo = async () => {
    try {
      const info = await invoke<SystemInfo>('get_system_info');
      setSystemInfo(info);
      return info;
    } catch (error) {
      console.error('Failed to get system info:', error);
      throw error;
    }
  };

  const fetchSystemMetrics = async () => {
    try {
      const data = await invoke<SystemMetrics>('get_system_metrics');
      setMetrics(data);
      return data;
    } catch (error) {
      console.error('Failed to get system metrics:', error);
      throw error;
    }
  };

  const getUptime = async () => {
    try {
      return await invoke<number>('get_app_uptime');
    } catch (error) {
      console.error('Failed to get uptime:', error);
      return 0;
    }
  };

  return {
    systemInfo,
    metrics,
    loading,
    fetchSystemInfo,
    fetchSystemMetrics,
    getUptime,
  };
}
```

### Component Usage

```tsx
function SystemInfoPanel() {
  const { systemInfo, metrics, fetchSystemInfo, fetchSystemMetrics } = useSystemInfo();

  useEffect(() => {
    fetchSystemInfo();
    fetchSystemMetrics();
  }, []);

  return (
    <div>
      <h2>System Information</h2>
      {systemInfo && (
        <div>
          <p>OS: {systemInfo.os}</p>
          <p>Architecture: {systemInfo.arch}</p>
          <p>Version: {systemInfo.version}</p>
        </div>
      )}

      <h2>Resource Usage</h2>
      {metrics && (
        <div>
          <p>CPU: {metrics.cpu_usage.toFixed(1)}%</p>
          <p>Memory: {metrics.memory_usage_percent.toFixed(1)}%</p>
          <p>Disk: {metrics.disk_usage_percent.toFixed(1)}%</p>
        </div>
      )}
    </div>
  );
}
```

## UI Components

### Information Display Sections
- **System Overview**: OS, architecture, app version
- **Resource Monitors**: Real-time CPU, memory, disk usage gauges
- **Network Statistics**: Interface list with data transfer metrics
- **Device Profile**: Hardware specifications and capabilities
- **Performance Metrics**: Historical charts for resource usage
- **WiFi Networks**: Nearby networks with signal strength

### Interactive Features
- Real-time metric updates with configurable intervals
- Copy system information to clipboard
- Export device profile as JSON
- Resource usage alerts and notifications
- Network interface selection

## Testing Checklist

### Desktop Testing
- [ ] Verify system info on Windows
- [ ] Verify system info on macOS
- [ ] Verify system info on Linux
- [ ] Test resource monitoring accuracy
- [ ] Test network metrics collection
- [ ] Verify WiFi scanning (macOS/Linux)
- [ ] Test with multiple displays
- [ ] Test with external storage devices

### Mobile Testing
- [ ] Test system info on Android
- [ ] Test system info on iOS
- [ ] Verify mobile-specific device profiling
- [ ] Test battery information integration
- [ ] Test cellular network detection
- [ ] Verify thermal state monitoring
- [ ] Test on various device models

### Performance Testing
- [ ] Measure metrics collection overhead
- [ ] Test polling intervals (1s, 5s, 10s)
- [ ] Verify memory leak prevention
- [ ] Test with continuous monitoring
- [ ] Profile CPU usage impact

## Implementation Status

### Phase 1: Core System Information ✅
- [x] Basic system info command
- [x] OS and architecture detection
- [x] App version reporting
- [x] Process ID retrieval

### Phase 2: Resource Monitoring ✅
- [x] CPU usage tracking
- [x] Memory usage monitoring
- [x] Disk usage calculation
- [x] Network interface metrics
- [x] App uptime tracking

### Phase 3: Network Profiling ⚠️
- [x] WiFi network scanning (macOS/Linux)
- [ ] Signal strength measurement
- [ ] Active connection details
- [ ] Connection quality metrics
- [ ] IP address information

### Phase 4: Hardware Profiling ⏳
- [ ] CPU information (name, cores, frequency)
- [ ] GPU detection and information
- [ ] Display specifications
- [ ] Storage device enumeration
- [ ] Sensor availability detection

### Phase 5: Mobile Device Profiling ⏳
- [ ] Device manufacturer and model
- [ ] Mobile OS version details
- [ ] Hardware capability detection
- [ ] Thermal state monitoring
- [ ] Power profile information

### Phase 6: Frontend Integration ⏳
- [ ] Create system info page/route
- [ ] Build resource monitoring UI
- [ ] Implement real-time metric updates
- [ ] Add device profile display
- [ ] Create export functionality

## Troubleshooting

### Common Issues

**Metrics Not Updating**
- Ensure sufficient polling interval (minimum 200ms)
- Check for permission issues on mobile platforms
- Verify sysinfo crate is properly initialized

**Inaccurate CPU Usage**
- CPU usage requires time to stabilize (200ms delay implemented)
- First reading may be inaccurate, use subsequent readings

**WiFi Scanning Failures**
- macOS: Requires location permissions and WiFi enabled
- Linux: May require root privileges or specific capabilities
- Windows: Not yet implemented

**Missing Hardware Information**
- Some information is platform-specific
- Mobile platforms may restrict hardware access
- Virtual machines may report limited information

## Platform Support

| Feature | Android | iOS | Windows | macOS | Linux |
|---------|---------|-----|---------|-------|-------|
| Basic System Info | ✅ | ✅ | ✅ | ✅ | ✅ |
| CPU Usage | ✅ | ✅ | ✅ | ✅ | ✅ |
| Memory Usage | ✅ | ✅ | ✅ | ✅ | ✅ |
| Disk Usage | ✅ | ✅ | ✅ | ✅ | ✅ |
| Network Metrics | ✅ | ✅ | ✅ | ✅ | ✅ |
| WiFi Scanning | ⏳ | ⏳ | ⏳ | ✅ | ✅ |
| CPU Details | ⏳ | ⏳ | ⏳ | ⏳ | ⏳ |
| GPU Info | ⏳ | ⏳ | ⏳ | ⏳ | ⏳ |
| Display Info | ⏳ | ⏳ | ⏳ | ⏳ | ⏳ |
| Device Model | ⏳ | ⏳ | ❌ | ✅ | ❌ |
| Thermal State | ⏳ | ⏳ | ❌ | ⏳ | ❌ |

**Legend:**
- ✅ Implemented
- ⏳ Planned
- ⚠️ Partial
- ❌ Not Supported

## Resources

### System Information
- [sysinfo crate documentation](https://docs.rs/sysinfo/latest/sysinfo/)
- [Rust std::env module](https://doc.rust-lang.org/std/env/)

### Platform-Specific APIs
- [Windows System Info](https://learn.microsoft.com/en-us/windows/win32/sysinfo/system-information)
- [macOS System Info](https://developer.apple.com/documentation/foundation/processinfo)
- [Android Build Info](https://developer.android.com/reference/android/os/Build)
- [iOS UIDevice](https://developer.apple.com/documentation/uikit/uidevice)

### Hardware Detection
- [hwinfo crate](https://crates.io/crates/hwinfo)
- [CPU-ID libraries](https://crates.io/crates/raw-cpuid)
- [GPU detection libraries](https://crates.io/crates/wgpu)

## Privacy Considerations

When collecting device information, be mindful of user privacy:

- **Device Identifiers**: Use responsibly and inform users
- **Network Information**: May reveal location data
- **Hardware Details**: Could be used for fingerprinting
- **App Usage Metrics**: Consider user consent

Always follow platform privacy guidelines and obtain necessary permissions before collecting sensitive device information.
