import { createFileRoute } from '@tanstack/react-router'
import { Info, Cpu, HardDrive, Network, Smartphone, Monitor, Activity } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

export const Route = createFileRoute('/system-info')({
  component: SystemInfoModule,
})

interface SystemInfo {
  os: string
  version: string
  arch: string
  app_version: string
  process_id: number
}

interface SystemMetrics {
  cpu_usage: number
  memory_total: number
  memory_used: number
  memory_available: number
  memory_usage_percent: number
  swap_total: number
  swap_used: number
  disk_total: number
  disk_used: number
  disk_available: number
  disk_usage_percent: number
}

interface NetworkMetrics {
  total_received: number
  total_transmitted: number
  interfaces: NetworkInterfaceMetrics[]
}

interface NetworkInterfaceMetrics {
  name: string
  received: number
  transmitted: number
}

interface WiFiNetwork {
  ssid: string
  bssid: string
  signal_strength: number
  channel: number
  security: string
}

interface CPUInfo {
  name: string
  vendor: string
  brand: string
  physical_cores: number
  logical_cores: number
  frequency: number
}

interface StorageDevice {
  name: string
  mount_point: string
  total_space: number
  available_space: number
  used_space: number
  file_system: string
  is_removable: boolean
}

interface DeviceProfile {
  hostname: string
  username: string
  device_name: string
  os_name: string
  os_version: string
  architecture: string
  cpu: CPUInfo
  total_memory: number
  storage_devices: StorageDevice[]
  kernel_version: string
}

function SystemInfoModule() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null)
  const [systemMetrics, setSystemMetrics] = useState<SystemMetrics | null>(null)
  const [networkMetrics, setNetworkMetrics] = useState<NetworkMetrics | null>(null)
  const [wifiNetworks, setWifiNetworks] = useState<WiFiNetwork[]>([])
  const [uptime, setUptime] = useState<number>(0)
  const [autoRefresh, setAutoRefresh] = useState(false)
  const [cpuInfo, setCpuInfo] = useState<CPUInfo | null>(null)
  const [storageDevices, setStorageDevices] = useState<StorageDevice[]>([])
  const [deviceProfile, setDeviceProfile] = useState<DeviceProfile | null>(null)

  useEffect(() => {
    loadSystemInfo()
    loadUptime()
  }, [])

  useEffect(() => {
    if (!autoRefresh) return

    const interval = setInterval(async () => {
      await loadSystemMetrics()
      await loadNetworkMetrics()
      await loadUptime()
    }, 2000)

    loadSystemMetrics()
    loadNetworkMetrics()

    return () => clearInterval(interval)
  }, [autoRefresh])

  const addOutput = (message: string, success: boolean = true) => {
    const timestamp = new Date().toLocaleTimeString()
    const icon = success ? '✓' : '✗'
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const loadSystemInfo = async () => {
    setLoading('sysinfo')
    try {
      const info = await invoke<SystemInfo>('get_system_info')
      setSystemInfo(info)
      addOutput('System information loaded')
    } catch (error) {
      addOutput(`Error loading system info: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const loadSystemMetrics = async () => {
    try {
      const metrics = await invoke<SystemMetrics>('get_system_metrics')
      setSystemMetrics(metrics)
    } catch (error) {
      console.error('Error loading system metrics:', error)
    }
  }

  const loadNetworkMetrics = async () => {
    try {
      const metrics = await invoke<NetworkMetrics>('get_network_metrics')
      setNetworkMetrics(metrics)
    } catch (error) {
      console.error('Error loading network metrics:', error)
    }
  }

  const loadUptime = async () => {
    try {
      const uptimeSeconds = await invoke<number>('get_app_uptime')
      setUptime(uptimeSeconds)
    } catch (error) {
      console.error('Error loading uptime:', error)
    }
  }

  const scanWifi = async () => {
    setLoading('wifi')
    try {
      const networks = await invoke<WiFiNetwork[]>('scan_wifi_networks')
      setWifiNetworks(networks)
      addOutput(`Found ${networks.length} WiFi network(s)`)
    } catch (error) {
      addOutput(`Error scanning WiFi networks: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const refreshAll = async () => {
    setLoading('refresh')
    try {
      await Promise.all([
        loadSystemInfo(),
        loadSystemMetrics(),
        loadNetworkMetrics(),
        loadUptime(),
      ])
      addOutput('All information refreshed')
    } catch (error) {
      addOutput(`Error refreshing: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const toggleAutoRefresh = () => {
    setAutoRefresh(!autoRefresh)
    addOutput(autoRefresh ? 'Auto-refresh stopped' : 'Auto-refresh started')
  }

  const loadCpuInfo = async () => {
    try {
      const info = await invoke<CPUInfo>('get_cpu_info')
      setCpuInfo(info)
      addOutput('CPU information loaded')
    } catch (error) {
      addOutput(`Error loading CPU info: ${error}`, false)
    }
  }

  const loadStorageDevices = async () => {
    try {
      const devices = await invoke<StorageDevice[]>('get_storage_devices')
      setStorageDevices(devices)
      addOutput(`Loaded ${devices.length} storage device(s)`)
    } catch (error) {
      addOutput(`Error loading storage devices: ${error}`, false)
    }
  }

  const loadDeviceProfile = async () => {
    setLoading('profile')
    try {
      const profile = await invoke<DeviceProfile>('get_device_profile')
      setDeviceProfile(profile)
      setCpuInfo(profile.cpu)
      setStorageDevices(profile.storage_devices)
      addOutput('Device profile loaded')
    } catch (error) {
      addOutput(`Error loading device profile: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const copyToClipboard = () => {
    const data = {
      system: systemInfo,
      metrics: systemMetrics,
      network: networkMetrics,
      uptime: uptime,
      cpu: cpuInfo,
      storage: storageDevices,
      deviceProfile: deviceProfile,
    }

    navigator.clipboard.writeText(JSON.stringify(data, null, 2))
    addOutput('System profile copied to clipboard')
  }

  return (
    <ModulePageLayout
      title="System Info & Device Profiling"
      description="Gather comprehensive system information, hardware details, and device metrics across all platforms."
      icon={Info}
    >
      <div className="space-y-6">
        {/* Quick Actions */}
        <div className="flex gap-2 flex-wrap">
          <Button
            onClick={refreshAll}
            disabled={loading === 'refresh'}
            variant="default"
          >
            Refresh All
          </Button>
          <Button
            onClick={loadDeviceProfile}
            disabled={loading === 'profile'}
            variant="default"
          >
            Load Device Profile
          </Button>
          <Button
            onClick={toggleAutoRefresh}
            variant={autoRefresh ? 'default' : 'outline'}
          >
            {autoRefresh ? 'Stop' : 'Start'} Auto-Refresh
          </Button>
          <Button
            onClick={copyToClipboard}
            variant="outline"
          >
            Copy Profile
          </Button>
        </div>

        {/* Basic System Information */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Smartphone className="h-5 w-5" />
            System Information
          </h3>
          {systemInfo ? (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              <InfoCard label="Operating System" value={systemInfo.os} icon="💻" />
              <InfoCard label="OS Version" value={systemInfo.version} icon="📋" />
              <InfoCard label="Architecture" value={systemInfo.arch} icon="🏗️" />
              <InfoCard label="App Version" value={systemInfo.app_version} icon="📦" />
              <InfoCard label="Process ID" value={systemInfo.process_id.toString()} icon="⚙️" />
              <InfoCard label="Uptime" value={formatUptime(uptime)} icon="⏱️" />
            </div>
          ) : (
            <div className="p-4 bg-muted rounded-md text-muted-foreground">
              Click "Refresh All" to load system information
            </div>
          )}
        </div>

        {/* Resource Metrics */}
        {systemMetrics && (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="font-semibold flex items-center gap-2">
                <Activity className="h-5 w-5" />
                Resource Metrics
              </h3>
              {autoRefresh && (
                <span className="text-sm text-muted-foreground">
                  Auto-updating every 2s
                </span>
              )}
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {/* CPU */}
              <MetricCard
                title="CPU Usage"
                icon={<Cpu className="h-5 w-5 text-blue-500" />}
                percentage={systemMetrics.cpu_usage}
                color="blue"
              />

              {/* Memory */}
              <MetricCard
                title="Memory"
                icon={<Activity className="h-5 w-5 text-green-500" />}
                percentage={systemMetrics.memory_usage_percent}
                details={`${formatBytes(systemMetrics.memory_used)} / ${formatBytes(systemMetrics.memory_total)}`}
                color="green"
              />

              {/* Disk */}
              <MetricCard
                title="Disk Usage"
                icon={<HardDrive className="h-5 w-5 text-purple-500" />}
                percentage={systemMetrics.disk_usage_percent}
                details={`${formatBytes(systemMetrics.disk_used)} / ${formatBytes(systemMetrics.disk_total)}`}
                color="purple"
              />

              {/* Swap */}
              <div className="p-4 bg-muted rounded-md">
                <div className="flex items-center gap-2 mb-3">
                  <Monitor className="h-5 w-5 text-orange-500" />
                  <h4 className="font-semibold">Swap Memory</h4>
                </div>
                <div className="space-y-1 text-sm">
                  <div className="flex justify-between">
                    <span>Total</span>
                    <span className="font-semibold">{formatBytes(systemMetrics.swap_total)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span>Used</span>
                    <span className="font-semibold">{formatBytes(systemMetrics.swap_used)}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Network Metrics */}
        {networkMetrics && (
          <div className="space-y-4">
            <h3 className="font-semibold flex items-center gap-2">
              <Network className="h-5 w-5" />
              Network Metrics
            </h3>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="p-4 bg-muted rounded-md">
                <h4 className="font-semibold mb-3">Total Transfer</h4>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span>Downloaded</span>
                    <span className="font-semibold">{formatBytes(networkMetrics.total_received)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span>Uploaded</span>
                    <span className="font-semibold">{formatBytes(networkMetrics.total_transmitted)}</span>
                  </div>
                </div>
              </div>

              <div className="p-4 bg-muted rounded-md">
                <h4 className="font-semibold mb-3">Interfaces</h4>
                <div className="space-y-1 text-sm max-h-32 overflow-y-auto">
                  {networkMetrics.interfaces.map((iface, idx) => (
                    <div key={idx} className="pb-1 border-b border-border last:border-0">
                      <div className="font-medium">{iface.name}</div>
                      <div className="text-xs text-muted-foreground flex justify-between">
                        <span>↓ {formatBytes(iface.received)}</span>
                        <span>↑ {formatBytes(iface.transmitted)}</span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Device Profile */}
        {deviceProfile && (
          <div className="space-y-4">
            <h3 className="font-semibold flex items-center gap-2">
              <Info className="h-5 w-5" />
              Device Profile
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              <InfoCard label="Hostname" value={deviceProfile.hostname} icon="🖥️" />
              <InfoCard label="Device Name" value={deviceProfile.device_name} icon="📱" />
              <InfoCard label="Username" value={deviceProfile.username} icon="👤" />
              <InfoCard label="OS Name" value={deviceProfile.os_name} icon="💻" />
              <InfoCard label="OS Version" value={deviceProfile.os_version} icon="📋" />
              <InfoCard label="Kernel Version" value={deviceProfile.kernel_version} icon="⚙️" />
              <InfoCard label="Architecture" value={deviceProfile.architecture} icon="🏗️" />
              <InfoCard label="Total Memory" value={formatBytes(deviceProfile.total_memory)} icon="💾" />
            </div>
          </div>
        )}

        {/* CPU Information */}
        {cpuInfo && (
          <div className="space-y-4">
            <h3 className="font-semibold flex items-center gap-2">
              <Cpu className="h-5 w-5" />
              CPU Information
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              <InfoCard label="CPU Model" value={cpuInfo.brand} icon="🔧" />
              <InfoCard label="Vendor" value={cpuInfo.vendor} icon="🏢" />
              <InfoCard label="Physical Cores" value={cpuInfo.physical_cores.toString()} icon="⚡" />
              <InfoCard label="Logical Cores" value={cpuInfo.logical_cores.toString()} icon="🔢" />
              <InfoCard label="Frequency" value={`${cpuInfo.frequency} MHz`} icon="⏱️" />
            </div>
          </div>
        )}

        {/* Storage Devices */}
        {storageDevices.length > 0 && (
          <div className="space-y-4">
            <h3 className="font-semibold flex items-center gap-2">
              <HardDrive className="h-5 w-5" />
              Storage Devices ({storageDevices.length})
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {storageDevices.map((device, idx) => (
                <div key={idx} className="p-4 bg-muted rounded-md">
                  <div className="flex items-center justify-between mb-3">
                    <h4 className="font-semibold flex items-center gap-2">
                      <HardDrive className={`h-4 w-4 ${device.is_removable ? 'text-blue-500' : 'text-gray-500'}`} />
                      {device.name || device.mount_point}
                    </h4>
                    {device.is_removable && (
                      <span className="text-xs bg-blue-500 text-white px-2 py-1 rounded">Removable</span>
                    )}
                  </div>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Mount Point</span>
                      <span className="font-mono text-xs">{device.mount_point}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">File System</span>
                      <span>{device.file_system}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Capacity</span>
                      <span className="font-semibold">{formatBytes(device.total_space)}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Used</span>
                      <span>{formatBytes(device.used_space)}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Available</span>
                      <span className="text-green-600 dark:text-green-400 font-semibold">
                        {formatBytes(device.available_space)}
                      </span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2 mt-2">
                      <div
                        className="bg-purple-500 h-2 rounded-full"
                        style={{
                          width: `${Math.min((device.used_space / device.total_space) * 100, 100)}%`
                        }}
                      />
                    </div>
                    <div className="text-xs text-muted-foreground text-center">
                      {((device.used_space / device.total_space) * 100).toFixed(1)}% used
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* WiFi Scanning */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Network className="h-5 w-5" />
            WiFi Networks
          </h3>
          <Button
            onClick={scanWifi}
            disabled={loading === 'wifi'}
            variant="outline"
          >
            Scan WiFi Networks
          </Button>

          {wifiNetworks.length > 0 && (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
              {wifiNetworks.map((network, idx) => (
                <div key={idx} className="p-3 bg-muted rounded-md">
                  <div className="font-semibold">{network.ssid || '(Hidden)'}</div>
                  <div className="text-sm space-y-1 mt-2">
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Signal</span>
                      <span>{network.signal_strength}%</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Channel</span>
                      <span>{network.channel}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Security</span>
                      <span>{network.security}</span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Output Panel */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold">Output</h3>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setOutput([])}
            >
              Clear
            </Button>
          </div>
          <div className="bg-muted p-4 rounded-md font-mono text-sm min-h-[150px] max-h-[300px] overflow-y-auto">
            {output.length === 0 ? (
              <p className="text-muted-foreground">
                Operation results will appear here...
              </p>
            ) : (
              <div className="space-y-1">
                {output.map((line, index) => (
                  <div key={index}>{line}</div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </ModulePageLayout>
  )
}

// Helper Components
function InfoCard({ label, value, icon }: { label: string; value: string; icon: string }) {
  return (
    <div className="p-4 bg-muted rounded-md">
      <div className="text-2xl mb-2">{icon}</div>
      <div className="text-sm text-muted-foreground">{label}</div>
      <div className="font-semibold text-lg">{value}</div>
    </div>
  )
}

function MetricCard({
  title,
  icon,
  percentage,
  details,
  color,
}: {
  title: string
  icon: React.ReactNode
  percentage: number
  details?: string
  color: 'blue' | 'green' | 'purple'
}) {
  const colorClasses = {
    blue: 'bg-blue-500',
    green: 'bg-green-500',
    purple: 'bg-purple-500',
  }

  return (
    <div className="p-4 bg-muted rounded-md">
      <div className="flex items-center gap-2 mb-3">
        {icon}
        <h4 className="font-semibold">{title}</h4>
      </div>
      <div className="space-y-2">
        <div className="flex justify-between text-sm">
          <span>{details || 'Usage'}</span>
          <span className="font-semibold">{percentage.toFixed(1)}%</span>
        </div>
        <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
          <div
            className={`${colorClasses[color]} h-2 rounded-full transition-all duration-300`}
            style={{ width: `${Math.min(percentage, 100)}%` }}
          />
        </div>
      </div>
    </div>
  )
}

// Helper Functions
function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`
}

function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const secs = Math.floor(seconds % 60)

  const parts = []
  if (days > 0) parts.push(`${days}d`)
  if (hours > 0) parts.push(`${hours}h`)
  if (minutes > 0) parts.push(`${minutes}m`)
  if (secs > 0 || parts.length === 0) parts.push(`${secs}s`)

  return parts.join(' ')
}
