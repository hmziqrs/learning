import { createFileRoute } from '@tanstack/react-router'
import { Clipboard, Battery, Volume2, RefreshCw } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export const Route = createFileRoute('/system-services')({
  component: SystemServices,
})

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

function SystemServices() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)

  // Clipboard state
  const [clipboardText, setClipboardText] = useState('')
  const [clipboardHistory, setClipboardHistory] = useState<ClipboardEntry[]>([])

  // Battery state
  const [batteryInfo, setBatteryInfo] = useState<BatteryInfo | null>(null)
  const [batteryError, setBatteryError] = useState<string | null>(null)

  // Audio devices state
  const [audioDevices, setAudioDevices] = useState<AudioDevice[]>([])
  const [audioError, setAudioError] = useState<string | null>(null)

  useEffect(() => {
    // Initialize
    loadBatteryInfo()
    loadAudioDevices()

    // Try to start battery monitoring if available
    invoke('start_battery_monitoring')
      .then(() => {
        addOutput('Battery monitoring started', true)
      })
      .catch(() => {
        // Silent fail - not all platforms support this
      })

    // Listen for battery changes
    const unlistenBattery = listen<BatteryInfo>('battery-changed', (event) => {
      setBatteryInfo(event.payload)
      addOutput('Battery status updated', true)
    })

    return () => {
      unlistenBattery.then((fn) => fn())
    }
  }, [])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    setOutput((prev) => [...prev, `${icon} ${message}`])
  }

  // Clipboard Functions
  const handleCopyToClipboard = async () => {
    if (!clipboardText.trim()) {
      addOutput('Please enter text to copy', false)
      return
    }

    setLoading('copy')
    try {
      // Try using Tauri clipboard plugin
      const { writeText } = await import('@tauri-apps/plugin-clipboard-manager')
      await writeText(clipboardText)

      // Add to history
      const entry: ClipboardEntry = {
        id: Date.now().toString(),
        text: clipboardText,
        timestamp: Date.now(),
      }
      setClipboardHistory((prev) => [entry, ...prev].slice(0, 5))

      addOutput(`Copied: "${clipboardText.substring(0, 30)}${clipboardText.length > 30 ? '...' : ''}"`)
    } catch (error) {
      // Fallback to Web Clipboard API
      try {
        await navigator.clipboard.writeText(clipboardText)

        const entry: ClipboardEntry = {
          id: Date.now().toString(),
          text: clipboardText,
          timestamp: Date.now(),
        }
        setClipboardHistory((prev) => [entry, ...prev].slice(0, 5))

        addOutput(`Copied (Web API): "${clipboardText.substring(0, 30)}${clipboardText.length > 30 ? '...' : ''}"`)
      } catch (webError) {
        addOutput(`Failed to copy: ${error}`, false)
      }
    } finally {
      setLoading(null)
    }
  }

  const handlePasteFromClipboard = async () => {
    setLoading('paste')
    try {
      // Try using Tauri clipboard plugin
      const { readText } = await import('@tauri-apps/plugin-clipboard-manager')
      const text = await readText()
      setClipboardText(text || '')
      addOutput(`Pasted: "${text?.substring(0, 30) || ''}${(text?.length || 0) > 30 ? '...' : ''}"`)
    } catch (error) {
      // Fallback to Web Clipboard API
      try {
        const text = await navigator.clipboard.readText()
        setClipboardText(text)
        addOutput(`Pasted (Web API): "${text.substring(0, 30)}${text.length > 30 ? '...' : ''}"`)
      } catch (webError) {
        addOutput(`Failed to paste: ${error}`, false)
      }
    } finally {
      setLoading(null)
    }
  }

  const handleClearClipboard = async () => {
    setLoading('clear')
    try {
      const { writeText } = await import('@tauri-apps/plugin-clipboard-manager')
      await writeText('')
      setClipboardText('')
      addOutput('Clipboard cleared')
    } catch (error) {
      try {
        await navigator.clipboard.writeText('')
        setClipboardText('')
        addOutput('Clipboard cleared (Web API)')
      } catch (webError) {
        addOutput(`Failed to clear clipboard: ${error}`, false)
      }
    } finally {
      setLoading(null)
    }
  }

  // Battery Functions
  const loadBatteryInfo = async () => {
    setLoading('battery')
    setBatteryError(null)

    try {
      // Try native implementation first
      const info = await invoke<BatteryInfo>('get_battery_info')
      setBatteryInfo(info)
      addOutput('Battery info loaded (native)', true)
    } catch (error) {
      // Try Web Battery API
      try {
        const battery = await (navigator as any).getBattery()
        setBatteryInfo({
          level: Math.round(battery.level * 100),
          charging: battery.charging,
          chargingTime: battery.chargingTime !== Infinity ? battery.chargingTime : null,
          dischargingTime: battery.dischargingTime !== Infinity ? battery.dischargingTime : null,
          temperature: null,
          powerSource: battery.charging ? 'ac' : 'battery',
          batteryState: battery.charging
            ? 'charging'
            : battery.level === 1
              ? 'full'
              : 'discharging',
        })

        // Listen for battery events
        battery.addEventListener('levelchange', () => {
          setBatteryInfo((prev) => ({
            ...prev!,
            level: Math.round(battery.level * 100),
          }))
        })

        battery.addEventListener('chargingchange', () => {
          setBatteryInfo((prev) => ({
            ...prev!,
            charging: battery.charging,
            powerSource: battery.charging ? 'ac' : 'battery',
            batteryState: battery.charging ? 'charging' : 'discharging',
          }))
        })

        addOutput('Battery info loaded (Web API)', true)
      } catch (webError) {
        setBatteryError('Battery API not available on this platform')
        addOutput('Battery API not available', false)
      }
    } finally {
      setLoading(null)
    }
  }

  // Audio Device Functions
  const loadAudioDevices = async () => {
    setLoading('audio')
    setAudioError(null)

    try {
      // Try native implementation first
      const result = await invoke<{ devices: AudioDevice[] }>('get_audio_devices')
      setAudioDevices(result.devices)
      addOutput(`Found ${result.devices.length} audio devices (native)`, true)
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
        addOutput(`Found ${audioDeviceList.length} audio devices (Web API)`, true)
      } catch (webError) {
        setAudioError('Audio device enumeration not available')
        addOutput('Audio device enumeration not available', false)
      }
    } finally {
      setLoading(null)
    }
  }

  const getBatteryIcon = () => {
    if (!batteryInfo) return '🔋'
    if (batteryInfo.charging) return '🔌'
    if (batteryInfo.level > 80) return '🔋'
    if (batteryInfo.level > 50) return '🔋'
    if (batteryInfo.level > 20) return '🪫'
    return '🪫'
  }

  const getBatteryColor = () => {
    if (!batteryInfo) return 'text-gray-500'
    if (batteryInfo.charging) return 'text-green-500'
    if (batteryInfo.level > 20) return 'text-blue-500'
    return 'text-red-500'
  }

  return (
    <ModulePageLayout
      title="System Services Module"
      description="Access clipboard, battery info, and system audio devices."
      icon={Clipboard}
    >
      <div className="space-y-6">
        {/* Clipboard Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Clipboard className="h-5 w-5" />
            Clipboard Manager
          </h3>

          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium mb-1">Text</label>
              <textarea
                className="w-full px-3 py-2 border rounded-md bg-background"
                rows={4}
                value={clipboardText}
                onChange={(e) => setClipboardText(e.target.value)}
                placeholder="Enter text to copy or paste from clipboard..."
              />
            </div>

            <div className="flex gap-2">
              <Button
                onClick={handleCopyToClipboard}
                disabled={loading === 'copy' || !clipboardText.trim()}
              >
                Copy to Clipboard
              </Button>
              <Button
                onClick={handlePasteFromClipboard}
                disabled={loading === 'paste'}
                variant="outline"
              >
                Paste from Clipboard
              </Button>
              <Button
                onClick={handleClearClipboard}
                disabled={loading === 'clear'}
                variant="outline"
              >
                Clear Clipboard
              </Button>
            </div>
          </div>

          {/* Clipboard History */}
          {clipboardHistory.length > 0 && (
            <div>
              <h4 className="text-sm font-medium mb-2">Recent Copies:</h4>
              <div className="space-y-2">
                {clipboardHistory.map((entry) => (
                  <div
                    key={entry.id}
                    className="text-sm p-3 bg-muted rounded-md border cursor-pointer hover:bg-muted/80"
                    onClick={() => setClipboardText(entry.text)}
                  >
                    <div className="truncate font-mono">{entry.text}</div>
                    <div className="text-xs text-muted-foreground mt-1">
                      {new Date(entry.timestamp).toLocaleTimeString()}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        {/* Battery Info Section */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold flex items-center gap-2">
              <Battery className="h-5 w-5" />
              Battery & Power
            </h3>
            <Button
              onClick={loadBatteryInfo}
              disabled={loading === 'battery'}
              variant="outline"
              size="sm"
            >
              <RefreshCw className="h-4 w-4" />
            </Button>
          </div>

          {batteryError ? (
            <div className="p-4 bg-muted rounded-md border">
              <p className="text-muted-foreground text-sm">{batteryError}</p>
              <p className="text-xs text-muted-foreground mt-2">
                Battery API may not be available on desktop browsers or some platforms.
              </p>
            </div>
          ) : batteryInfo ? (
            <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
              <div className="p-4 bg-muted rounded-md border">
                <div className="text-sm text-muted-foreground mb-1">Level</div>
                <div className={`text-3xl font-bold ${getBatteryColor()}`}>
                  {getBatteryIcon()} {batteryInfo.level}%
                </div>
              </div>

              <div className="p-4 bg-muted rounded-md border">
                <div className="text-sm text-muted-foreground mb-1">Status</div>
                <div className="text-lg font-medium capitalize">
                  {batteryInfo.batteryState.replace('_', ' ')}
                </div>
              </div>

              <div className="p-4 bg-muted rounded-md border">
                <div className="text-sm text-muted-foreground mb-1">Charging</div>
                <div className="text-lg font-medium">
                  {batteryInfo.charging ? '✓ Yes' : '✗ No'}
                </div>
              </div>

              <div className="p-4 bg-muted rounded-md border">
                <div className="text-sm text-muted-foreground mb-1">Power Source</div>
                <div className="text-lg font-medium capitalize">
                  {batteryInfo.powerSource}
                </div>
              </div>

              {batteryInfo.temperature !== null && (
                <div className="p-4 bg-muted rounded-md border">
                  <div className="text-sm text-muted-foreground mb-1">Temperature</div>
                  <div className="text-lg font-medium">{batteryInfo.temperature}°C</div>
                </div>
              )}

              {batteryInfo.chargingTime !== null && batteryInfo.chargingTime > 0 && (
                <div className="p-4 bg-muted rounded-md border">
                  <div className="text-sm text-muted-foreground mb-1">Time to Full</div>
                  <div className="text-lg font-medium">
                    {Math.round(batteryInfo.chargingTime / 60)} min
                  </div>
                </div>
              )}

              {batteryInfo.dischargingTime !== null && batteryInfo.dischargingTime > 0 && (
                <div className="p-4 bg-muted rounded-md border">
                  <div className="text-sm text-muted-foreground mb-1">Time Remaining</div>
                  <div className="text-lg font-medium">
                    {Math.round(batteryInfo.dischargingTime / 60)} min
                  </div>
                </div>
              )}
            </div>
          ) : (
            <div className="p-4 bg-muted rounded-md border">
              <p className="text-muted-foreground text-sm">Loading battery information...</p>
            </div>
          )}
        </div>

        {/* Audio Devices Section */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold flex items-center gap-2">
              <Volume2 className="h-5 w-5" />
              Audio Devices
            </h3>
            <Button
              onClick={loadAudioDevices}
              disabled={loading === 'audio'}
              variant="outline"
              size="sm"
            >
              <RefreshCw className="h-4 w-4" />
            </Button>
          </div>

          {audioError ? (
            <div className="p-4 bg-muted rounded-md border">
              <p className="text-muted-foreground text-sm">{audioError}</p>
              <p className="text-xs text-muted-foreground mt-2">
                Audio device enumeration may require microphone permissions or may not be
                available on this platform.
              </p>
            </div>
          ) : (
            <div className="space-y-4">
              {/* Output Devices */}
              <div>
                <h4 className="text-sm font-medium mb-2">Output Devices:</h4>
                <div className="space-y-2">
                  {audioDevices.filter((d) => d.kind === 'audiooutput').length === 0 ? (
                    <div className="p-4 bg-muted rounded-md border">
                      <p className="text-muted-foreground text-sm">No output devices found</p>
                    </div>
                  ) : (
                    audioDevices
                      .filter((d) => d.kind === 'audiooutput')
                      .map((device) => (
                        <div key={device.id} className="p-3 bg-muted rounded-md border">
                          <div className="flex items-center justify-between">
                            <div className="font-medium">{device.name}</div>
                            {device.isDefault && (
                              <span className="text-xs px-2 py-1 bg-primary/10 text-primary rounded">
                                Default
                              </span>
                            )}
                          </div>
                          <div className="text-xs text-muted-foreground mt-1 capitalize">
                            {device.type}
                          </div>
                        </div>
                      ))
                  )}
                </div>
              </div>

              {/* Input Devices */}
              <div>
                <h4 className="text-sm font-medium mb-2">Input Devices:</h4>
                <div className="space-y-2">
                  {audioDevices.filter((d) => d.kind === 'audioinput').length === 0 ? (
                    <div className="p-4 bg-muted rounded-md border">
                      <p className="text-muted-foreground text-sm">No input devices found</p>
                    </div>
                  ) : (
                    audioDevices
                      .filter((d) => d.kind === 'audioinput')
                      .map((device) => (
                        <div key={device.id} className="p-3 bg-muted rounded-md border">
                          <div className="flex items-center justify-between">
                            <div className="font-medium">{device.name}</div>
                            {device.isDefault && (
                              <span className="text-xs px-2 py-1 bg-primary/10 text-primary rounded">
                                Default
                              </span>
                            )}
                          </div>
                          <div className="text-xs text-muted-foreground mt-1 capitalize">
                            {device.type}
                          </div>
                        </div>
                      ))
                  )}
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Output Panel */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold">Output</h3>
            <Button variant="outline" size="sm" onClick={() => setOutput([])}>
              Clear
            </Button>
          </div>
          <div className="bg-muted p-4 rounded-md font-mono text-sm min-h-[150px] max-h-[300px] overflow-y-auto">
            {output.length === 0 ? (
              <p className="text-muted-foreground">Operation results will appear here...</p>
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
