import { createFileRoute } from '@tanstack/react-router'
import { Activity, Check, X, Monitor, Info, MessageSquare, Cpu, HardDrive, Network, Timer } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import { getCurrentWindow, LogicalSize, LogicalPosition } from '@tauri-apps/api/window'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { message, ask, confirm } from '@tauri-apps/plugin-dialog'

export const Route = createFileRoute('/app-lifecycle')({
  component: AppLifecycle,
})

interface WindowState {
  focused: boolean
  minimized: boolean
  maximized: boolean
  visible: boolean
  fullscreen: boolean
}

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

function AppLifecycle() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)
  const [listening, setListening] = useState(false)
  const [eventCount, setEventCount] = useState(0)
  const [windowState, setWindowState] = useState<WindowState | null>(null)
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null)
  const [theme, setTheme] = useState<string>('')
  const [uptime, setUptime] = useState<number>(0)
  const [systemMetrics, setSystemMetrics] = useState<SystemMetrics | null>(null)
  const [networkMetrics, setNetworkMetrics] = useState<NetworkMetrics | null>(null)
  const [isMonitoring, setIsMonitoring] = useState(false)

  // Window property controls
  const [windowTitle, setWindowTitle] = useState('Tauri Capability Playground')
  const [windowWidth, setWindowWidth] = useState('800')
  const [windowHeight, setWindowHeight] = useState('600')
  const [windowX, setWindowX] = useState('100')
  const [windowY, setWindowY] = useState('100')

  // Dialog controls
  const [dialogMessage, setDialogMessage] = useState('Hello from Tauri!')

  const window = getCurrentWindow()
  const unlisteners: UnlistenFn[] = []

  useEffect(() => {
    // Initial loads
    refreshWindowState()
    loadSystemInfo()
    loadUptime()

    // Cleanup listeners on unmount
    return () => {
      unlisteners.forEach(unlisten => unlisten())
    }
  }, [])

  // Auto-refresh monitoring data
  useEffect(() => {
    if (!isMonitoring) return

    const interval = setInterval(async () => {
      await loadSystemMetrics()
      await loadNetworkMetrics()
      await loadUptime()
    }, 2000) // Update every 2 seconds

    // Initial load
    loadSystemMetrics()
    loadNetworkMetrics()

    return () => clearInterval(interval)
  }, [isMonitoring])

  const addOutput = (message: string, success: boolean = true) => {
    const timestamp = new Date().toLocaleTimeString()
    const icon = success ? '✓' : '✗'
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  // Window Event Listeners
  const startListening = async () => {
    if (listening) {
      addOutput('Already listening to events', false)
      return
    }

    setLoading('listen')
    try {
      // Focus event
      const unlistenFocus = await listen('tauri://focus', () => {
        setEventCount(prev => prev + 1)
        addOutput('Window focused')
      })
      unlisteners.push(unlistenFocus)

      // Blur event
      const unlistenBlur = await listen('tauri://blur', () => {
        setEventCount(prev => prev + 1)
        addOutput('Window blurred')
      })
      unlisteners.push(unlistenBlur)

      // Resize event
      const unlistenResize = await listen('tauri://resize', (event) => {
        setEventCount(prev => prev + 1)
        const payload = event.payload as { width: number; height: number }
        addOutput(`Window resized to ${payload.width}x${payload.height}`)
      })
      unlisteners.push(unlistenResize)

      // Move event
      const unlistenMove = await listen('tauri://move', (event) => {
        setEventCount(prev => prev + 1)
        const payload = event.payload as { x: number; y: number }
        addOutput(`Window moved to (${payload.x}, ${payload.y})`)
      })
      unlisteners.push(unlistenMove)

      // Close requested event
      const unlistenCloseRequested = await listen('tauri://close-requested', () => {
        setEventCount(prev => prev + 1)
        addOutput('Window close requested')
      })
      unlisteners.push(unlistenCloseRequested)

      setListening(true)
      addOutput('Started listening to window events')
    } catch (error) {
      addOutput(`Error starting listeners: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const stopListening = () => {
    unlisteners.forEach(unlisten => unlisten())
    unlisteners.length = 0
    setListening(false)
    addOutput('Stopped listening to window events')
  }

  // Window State Management
  const refreshWindowState = async () => {
    setLoading('state')
    try {
      const [focused, minimized, maximized, visible, fullscreen] = await Promise.all([
        window.isFocused(),
        window.isMinimized(),
        window.isMaximized(),
        window.isVisible(),
        window.isFullscreen(),
      ])

      const state: WindowState = {
        focused,
        minimized,
        maximized,
        visible,
        fullscreen,
      }
      setWindowState(state)
      addOutput('Window state refreshed')
    } catch (error) {
      addOutput(`Error getting window state: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleMinimize = async () => {
    setLoading('minimize')
    try {
      await window.minimize()
      addOutput('Window minimized')
      setTimeout(refreshWindowState, 500)
    } catch (error) {
      addOutput(`Error minimizing window: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleMaximize = async () => {
    setLoading('maximize')
    try {
      if (windowState?.maximized) {
        await window.unmaximize()
        addOutput('Window unmaximized')
      } else {
        await window.maximize()
        addOutput('Window maximized')
      }
      setTimeout(refreshWindowState, 500)
    } catch (error) {
      addOutput(`Error toggling maximize: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleCenter = async () => {
    setLoading('center')
    try {
      await window.center()
      addOutput('Window centered')
    } catch (error) {
      addOutput(`Error centering window: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleToggleFullscreen = async () => {
    setLoading('fullscreen')
    try {
      await window.setFullscreen(!windowState?.fullscreen)
      addOutput(`Fullscreen ${windowState?.fullscreen ? 'disabled' : 'enabled'}`)
      setTimeout(refreshWindowState, 500)
    } catch (error) {
      addOutput(`Error toggling fullscreen: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // Window Properties
  const handleSetTitle = async () => {
    setLoading('title')
    try {
      await window.setTitle(windowTitle)
      addOutput(`Window title set to: "${windowTitle}"`)
    } catch (error) {
      addOutput(`Error setting title: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleSetSize = async () => {
    const width = parseInt(windowWidth)
    const height = parseInt(windowHeight)

    if (isNaN(width) || isNaN(height) || width < 100 || height < 100) {
      addOutput('Please enter valid dimensions (minimum 100x100)', false)
      return
    }

    setLoading('size')
    try {
      await window.setSize(new LogicalSize(width, height))
      addOutput(`Window size set to ${width}x${height}`)
    } catch (error) {
      addOutput(`Error setting size: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleGetTheme = async () => {
    setLoading('theme')
    try {
      const currentTheme = await window.theme()
      setTheme(currentTheme || 'unknown')
      addOutput(`Current theme: ${currentTheme || 'unknown'}`)
    } catch (error) {
      addOutput(`Error getting theme: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleToggleDecorations = async () => {
    setLoading('decorations')
    try {
      // Toggle decorations (this is a demo - you'd want to track the state)
      await window.setDecorations(false)
      addOutput('Window decorations toggled (refresh to see effect)')
      setTimeout(async () => {
        await window.setDecorations(true)
      }, 3000)
    } catch (error) {
      addOutput(`Error toggling decorations: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // System Info
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

  const loadUptime = async () => {
    try {
      const uptimeSeconds = await invoke<number>('get_app_uptime')
      setUptime(uptimeSeconds)
    } catch (error) {
      console.error('Error loading uptime:', error)
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

  const toggleMonitoring = () => {
    setIsMonitoring(!isMonitoring)
    if (!isMonitoring) {
      addOutput('Started system monitoring')
    } else {
      addOutput('Stopped system monitoring')
    }
  }

  const handleSetPosition = async () => {
    const x = parseInt(windowX)
    const y = parseInt(windowY)

    if (isNaN(x) || isNaN(y)) {
      addOutput('Please enter valid position coordinates', false)
      return
    }

    setLoading('position')
    try {
      await window.setPosition(new LogicalPosition(x, y))
      addOutput(`Window position set to (${x}, ${y})`)
    } catch (error) {
      addOutput(`Error setting position: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // System Dialogs
  const handleShowMessage = async () => {
    setLoading('message')
    try {
      await message(dialogMessage, 'App Lifecycle')
      addOutput('Message dialog shown')
    } catch (error) {
      addOutput(`Error showing message: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleShowConfirm = async () => {
    setLoading('confirm')
    try {
      const confirmed = await confirm('Are you sure you want to continue?', 'Confirm Action')
      addOutput(`Confirm dialog: User ${confirmed ? 'confirmed' : 'canceled'}`)
    } catch (error) {
      addOutput(`Error showing confirm dialog: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleShowAsk = async () => {
    setLoading('ask')
    try {
      const answer = await ask('What is your name?', 'User Input')
      addOutput(`Ask dialog: User entered "${answer || '(canceled)'}"`)
    } catch (error) {
      addOutput(`Error showing ask dialog: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  return (
    <ModulePageLayout
      title="App Lifecycle & OS Integration"
      description="Monitor app and window lifecycle events, detect system theme and platform info, manage window states, and integrate with OS dialogs."
      icon={Activity}
    >
      <div className="space-y-6">
        {/* Window Events Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Monitor className="h-5 w-5" />
            Window Lifecycle Events
          </h3>
          <div className="flex items-center gap-4 flex-wrap">
            <Button
              onClick={startListening}
              disabled={listening || loading === 'listen'}
              variant={listening ? 'outline' : 'default'}
            >
              Start Listening
            </Button>
            <Button
              onClick={stopListening}
              disabled={!listening}
              variant="outline"
            >
              Stop Listening
            </Button>
            <div className="text-sm text-muted-foreground">
              Events captured: <span className="font-semibold">{eventCount}</span>
            </div>
          </div>
          <p className="text-sm text-muted-foreground">
            Try focusing/blurring the window, resizing, or moving it to see events.
          </p>
        </div>

        {/* Window State Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Monitor className="h-5 w-5" />
            Window State
          </h3>
          <Button
            onClick={refreshWindowState}
            disabled={loading === 'state'}
            variant="outline"
            size="sm"
          >
            Refresh State
          </Button>
          {windowState && (
            <div className="grid grid-cols-2 md:grid-cols-5 gap-3">
              <StateIndicator label="Focused" value={windowState.focused} />
              <StateIndicator label="Minimized" value={windowState.minimized} />
              <StateIndicator label="Maximized" value={windowState.maximized} />
              <StateIndicator label="Visible" value={windowState.visible} />
              <StateIndicator label="Fullscreen" value={windowState.fullscreen} />
            </div>
          )}
        </div>

        {/* Window Controls Section */}
        <div className="space-y-4">
          <h3 className="font-semibold">Window Controls</h3>
          <div className="flex gap-2 flex-wrap">
            <Button
              onClick={handleMinimize}
              disabled={loading === 'minimize'}
              variant="outline"
              size="sm"
            >
              Minimize
            </Button>
            <Button
              onClick={handleMaximize}
              disabled={loading === 'maximize'}
              variant="outline"
              size="sm"
            >
              {windowState?.maximized ? 'Unmaximize' : 'Maximize'}
            </Button>
            <Button
              onClick={handleCenter}
              disabled={loading === 'center'}
              variant="outline"
              size="sm"
            >
              Center
            </Button>
            <Button
              onClick={handleToggleFullscreen}
              disabled={loading === 'fullscreen'}
              variant="outline"
              size="sm"
            >
              Toggle Fullscreen
            </Button>
            <Button
              onClick={handleToggleDecorations}
              disabled={loading === 'decorations'}
              variant="outline"
              size="sm"
            >
              Toggle Decorations
            </Button>
          </div>
        </div>

        {/* Window Properties Section */}
        <div className="space-y-4">
          <h3 className="font-semibold">Window Properties</h3>
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium mb-1">Window Title</label>
              <div className="flex gap-2">
                <input
                  type="text"
                  className="flex-1 px-3 py-2 border rounded-md"
                  value={windowTitle}
                  onChange={(e) => setWindowTitle(e.target.value)}
                  placeholder="Window title"
                />
                <Button
                  onClick={handleSetTitle}
                  disabled={loading === 'title'}
                >
                  Set Title
                </Button>
              </div>
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Window Size</label>
              <div className="flex gap-2">
                <input
                  type="number"
                  className="w-24 px-3 py-2 border rounded-md"
                  value={windowWidth}
                  onChange={(e) => setWindowWidth(e.target.value)}
                  placeholder="Width"
                  min="100"
                />
                <span className="self-center">×</span>
                <input
                  type="number"
                  className="w-24 px-3 py-2 border rounded-md"
                  value={windowHeight}
                  onChange={(e) => setWindowHeight(e.target.value)}
                  placeholder="Height"
                  min="100"
                />
                <Button
                  onClick={handleSetSize}
                  disabled={loading === 'size'}
                >
                  Set Size
                </Button>
              </div>
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Window Position</label>
              <div className="flex gap-2">
                <input
                  type="number"
                  className="w-24 px-3 py-2 border rounded-md"
                  value={windowX}
                  onChange={(e) => setWindowX(e.target.value)}
                  placeholder="X"
                />
                <span className="self-center">,</span>
                <input
                  type="number"
                  className="w-24 px-3 py-2 border rounded-md"
                  value={windowY}
                  onChange={(e) => setWindowY(e.target.value)}
                  placeholder="Y"
                />
                <Button
                  onClick={handleSetPosition}
                  disabled={loading === 'position'}
                >
                  Set Position
                </Button>
              </div>
            </div>
            <div className="flex gap-2 flex-wrap">
              <Button
                onClick={handleGetTheme}
                disabled={loading === 'theme'}
                variant="outline"
                size="sm"
              >
                Get Theme
              </Button>
              {theme && (
                <div className="self-center text-sm">
                  Current theme: <span className="font-semibold">{theme}</span>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* System Info Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Info className="h-5 w-5" />
            System Information
          </h3>
          <Button
            onClick={loadSystemInfo}
            disabled={loading === 'sysinfo'}
            variant="outline"
            size="sm"
          >
            Refresh Info
          </Button>
          {systemInfo && (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3 p-4 bg-muted rounded-md">
              <InfoItem label="Operating System" value={systemInfo.os} />
              <InfoItem label="OS Version" value={systemInfo.version} />
              <InfoItem label="Architecture" value={systemInfo.arch} />
              <InfoItem label="App Version" value={systemInfo.app_version} />
              <InfoItem label="Process ID" value={systemInfo.process_id.toString()} />
              <InfoItem label="App Uptime" value={formatUptime(uptime)} />
            </div>
          )}
        </div>

        {/* Live System Monitoring Section */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold flex items-center gap-2">
              <Activity className="h-5 w-5" />
              Live System Monitoring
            </h3>
            <Button
              onClick={toggleMonitoring}
              variant={isMonitoring ? 'default' : 'outline'}
              size="sm"
            >
              {isMonitoring ? 'Stop Monitoring' : 'Start Monitoring'}
            </Button>
          </div>

          {isMonitoring && systemMetrics && (
            <div className="space-y-4">
              {/* CPU & Memory */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* CPU */}
                <div className="p-4 bg-muted rounded-md">
                  <div className="flex items-center gap-2 mb-3">
                    <Cpu className="h-5 w-5 text-blue-500" />
                    <h4 className="font-semibold">CPU Usage</h4>
                  </div>
                  <div className="space-y-2">
                    <div className="flex justify-between text-sm">
                      <span>Usage</span>
                      <span className="font-semibold">{systemMetrics.cpu_usage.toFixed(1)}%</span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-blue-500 h-2 rounded-full transition-all duration-300"
                        style={{ width: `${Math.min(systemMetrics.cpu_usage, 100)}%` }}
                      />
                    </div>
                  </div>
                </div>

                {/* Memory */}
                <div className="p-4 bg-muted rounded-md">
                  <div className="flex items-center gap-2 mb-3">
                    <Activity className="h-5 w-5 text-green-500" />
                    <h4 className="font-semibold">Memory</h4>
                  </div>
                  <div className="space-y-2">
                    <div className="flex justify-between text-sm">
                      <span>Used / Total</span>
                      <span className="font-semibold">
                        {formatBytes(systemMetrics.memory_used)} / {formatBytes(systemMetrics.memory_total)}
                      </span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-green-500 h-2 rounded-full transition-all duration-300"
                        style={{ width: `${systemMetrics.memory_usage_percent.toFixed(1)}%` }}
                      />
                    </div>
                    <div className="text-xs text-muted-foreground">
                      {systemMetrics.memory_usage_percent.toFixed(1)}% used
                    </div>
                  </div>
                </div>
              </div>

              {/* Disk & Network */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* Disk */}
                <div className="p-4 bg-muted rounded-md">
                  <div className="flex items-center gap-2 mb-3">
                    <HardDrive className="h-5 w-5 text-purple-500" />
                    <h4 className="font-semibold">Disk Usage</h4>
                  </div>
                  <div className="space-y-2">
                    <div className="flex justify-between text-sm">
                      <span>Used / Total</span>
                      <span className="font-semibold">
                        {formatBytes(systemMetrics.disk_used)} / {formatBytes(systemMetrics.disk_total)}
                      </span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-purple-500 h-2 rounded-full transition-all duration-300"
                        style={{ width: `${systemMetrics.disk_usage_percent.toFixed(1)}%` }}
                      />
                    </div>
                    <div className="text-xs text-muted-foreground">
                      {systemMetrics.disk_usage_percent.toFixed(1)}% used
                    </div>
                  </div>
                </div>

                {/* Network */}
                {networkMetrics && (
                  <div className="p-4 bg-muted rounded-md">
                    <div className="flex items-center gap-2 mb-3">
                      <Network className="h-5 w-5 text-orange-500" />
                      <h4 className="font-semibold">Network</h4>
                    </div>
                    <div className="space-y-2 text-sm">
                      <div className="flex justify-between">
                        <span>Downloaded</span>
                        <span className="font-semibold">{formatBytes(networkMetrics.total_received)}</span>
                      </div>
                      <div className="flex justify-between">
                        <span>Uploaded</span>
                        <span className="font-semibold">{formatBytes(networkMetrics.total_transmitted)}</span>
                      </div>
                      <div className="text-xs text-muted-foreground">
                        {networkMetrics.interfaces.length} interface(s) active
                      </div>
                    </div>
                  </div>
                )}
              </div>
            </div>
          )}

          {!isMonitoring && (
            <p className="text-sm text-muted-foreground">
              Click "Start Monitoring" to view live system metrics
            </p>
          )}
        </div>

        {/* System Dialogs Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <MessageSquare className="h-5 w-5" />
            System Dialogs
          </h3>
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium mb-1">Message</label>
              <input
                type="text"
                className="w-full px-3 py-2 border rounded-md"
                value={dialogMessage}
                onChange={(e) => setDialogMessage(e.target.value)}
                placeholder="Dialog message"
              />
            </div>
            <div className="flex gap-2 flex-wrap">
              <Button
                onClick={handleShowMessage}
                disabled={loading === 'message'}
                variant="outline"
                size="sm"
              >
                Show Message
              </Button>
              <Button
                onClick={handleShowConfirm}
                disabled={loading === 'confirm'}
                variant="outline"
                size="sm"
              >
                Show Confirm
              </Button>
              <Button
                onClick={handleShowAsk}
                disabled={loading === 'ask'}
                variant="outline"
                size="sm"
              >
                Show Ask
              </Button>
            </div>
          </div>
        </div>

        {/* Output Panel */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold">Output</h3>
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                setOutput([])
                setEventCount(0)
              }}
            >
              Clear
            </Button>
          </div>
          <div className="bg-muted p-4 rounded-md font-mono text-sm min-h-[150px] max-h-[400px] overflow-y-auto">
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

// Helper Components
function StateIndicator({ label, value }: { label: string; value: boolean }) {
  return (
    <div className="flex items-center gap-2 p-2 bg-card border rounded-md">
      {value ? (
        <Check className="h-4 w-4 text-green-500" />
      ) : (
        <X className="h-4 w-4 text-red-500" />
      )}
      <span className="text-sm">{label}</span>
    </div>
  )
}

function InfoItem({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <div className="text-sm text-muted-foreground">{label}</div>
      <div className="font-semibold">{value}</div>
    </div>
  )
}
