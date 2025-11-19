import { createFileRoute } from '@tanstack/react-router'
import { invoke } from '@tauri-apps/api/core'
import { Smartphone } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState } from 'react'

export const Route = createFileRoute('/haptics')({
  component: HapticsModule,
})

type HapticImpactStyle = 'light' | 'medium' | 'heavy'
type HapticNotificationType = 'success' | 'warning' | 'error'

function HapticsModule() {
  const [output, setOutput] = useState<string[]>([])
  const [isAvailable, setIsAvailable] = useState<boolean | null>(null)
  const [isLoading, setIsLoading] = useState(false)

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  // Trigger haptic impact
  const handleImpact = async (style: HapticImpactStyle) => {
    if (isLoading) return
    setIsLoading(true)
    addOutput(`Triggering ${style} impact haptic feedback...`)

    try {
      await invoke('haptic_impact', { style })
      addOutput(`${style.charAt(0).toUpperCase() + style.slice(1)} haptic impact triggered successfully`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Trigger notification haptic
  const handleNotification = async (type: HapticNotificationType) => {
    if (isLoading) return
    setIsLoading(true)
    addOutput(`Triggering ${type} notification haptic...`)

    try {
      await invoke('haptic_notification', { notificationType: type })
      addOutput(`${type.charAt(0).toUpperCase() + type.slice(1)} haptic notification triggered successfully`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Trigger custom vibration
  const handleVibrate = async (duration: number) => {
    if (isLoading) return
    setIsLoading(true)
    addOutput(`Vibrating for ${duration}ms...`)

    try {
      await invoke('vibrate', { duration })
      addOutput(`Vibration triggered successfully`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Trigger pattern vibration
  const handleVibratePattern = async () => {
    if (isLoading) return
    setIsLoading(true)
    const pattern = [100, 50, 100, 50, 200]
    addOutput(`Vibrating with pattern: [${pattern.join(', ')}]ms...`)

    try {
      await invoke('vibrate_pattern', { pattern })
      addOutput(`Pattern vibration triggered successfully`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Check device capability
  const handleCheckAvailability = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Checking vibrator availability...')

    try {
      const available = await invoke<boolean>('has_vibrator')
      setIsAvailable(available)
      addOutput(available ? 'Haptics available on this device' : 'Haptics not available on this platform', available)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
      setIsAvailable(false)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <ModulePageLayout
      title="Haptics / Vibrations Module"
      description="Provide tactile feedback through vibrations and haptic effects on mobile devices"
      icon={Smartphone}
    >
      <div className="space-y-6">
        {/* Status Notice */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-blue-500">ℹ️</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Current implementation:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li>
                <strong className="text-green-600">✓ Rust Commands</strong> - 6 Tauri commands registered and functional
              </li>
              <li>
                <strong className="text-green-600">✓ Android Plugin</strong> - HapticsPlugin.kt with VibrationEffect API
              </li>
              <li>
                <strong className="text-green-600">✓ iOS Plugin</strong> - HapticsPlugin.swift with UIFeedbackGenerator
              </li>
              <li>
                <strong className="text-red-600">✗ Desktop</strong> - Haptics not available on Windows/macOS/Linux
              </li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># Android Implementation (Complete):</div>
              <div>HapticsPlugin.kt - VibrationEffect with legacy fallback</div>
              <div className="mt-2"># iOS Implementation (Complete):</div>
              <div>HapticsPlugin.swift - UIFeedbackGenerator (Impact, Notification, Selection)</div>
            </div>
            <p className="text-muted-foreground mt-2">
              Android: Fully functional with VibrationEffect API. iOS: Fully functional with UIFeedbackGenerator. Desktop: Returns appropriate error messages.
            </p>
          </div>
        </section>

        {/* Device Capability Check */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Smartphone className="w-5 h-5" />
            Device Capability
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Check if the current device supports haptic feedback
            </p>

            <div className="flex flex-wrap gap-2 items-center">
              <Button onClick={handleCheckAvailability} variant="outline" disabled={isLoading}>
                {isLoading ? 'Checking...' : 'Check Availability'}
              </Button>

              {isAvailable !== null && (
                <div className={`px-4 py-2 rounded-md text-sm font-medium ${
                  isAvailable
                    ? 'bg-green-500/10 text-green-700 dark:text-green-400 border border-green-500/30'
                    : 'bg-red-500/10 text-red-700 dark:text-red-400 border border-red-500/30'
                }`}>
                  {isAvailable ? '✓ Haptics Available' : '✗ Haptics Not Available'}
                </div>
              )}
            </div>
          </div>
        </section>

        {/* Impact Haptics */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold">Impact Feedback</h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Trigger haptic feedback at different impact intensities
            </p>

            <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
              <div className="border rounded-lg p-4 space-y-3">
                <div className="text-center">
                  <h3 className="font-semibold mb-1">Light</h3>
                  <p className="text-xs text-muted-foreground">Subtle tap feedback</p>
                </div>
                <Button
                  onClick={() => handleImpact('light')}
                  variant="outline"
                  className="w-full"
                  disabled={isLoading}
                >
                  Trigger Light
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-3">
                <div className="text-center">
                  <h3 className="font-semibold mb-1">Medium</h3>
                  <p className="text-xs text-muted-foreground">Standard feedback</p>
                </div>
                <Button
                  onClick={() => handleImpact('medium')}
                  variant="outline"
                  className="w-full"
                  disabled={isLoading}
                >
                  Trigger Medium
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-3">
                <div className="text-center">
                  <h3 className="font-semibold mb-1">Heavy</h3>
                  <p className="text-xs text-muted-foreground">Strong feedback</p>
                </div>
                <Button
                  onClick={() => handleImpact('heavy')}
                  variant="outline"
                  className="w-full"
                  disabled={isLoading}
                >
                  Trigger Heavy
                </Button>
              </div>
            </div>
          </div>
        </section>

        {/* Notification Haptics */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold">Notification Feedback</h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Trigger haptic patterns for different notification types
            </p>

            <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
              <div className="border rounded-lg p-4 space-y-3 border-green-500/30 bg-green-500/5">
                <div className="text-center">
                  <h3 className="font-semibold mb-1 text-green-700 dark:text-green-400">Success</h3>
                  <p className="text-xs text-muted-foreground">Completion feedback</p>
                </div>
                <Button
                  onClick={() => handleNotification('success')}
                  variant="outline"
                  className="w-full border-green-500/50"
                  disabled={isLoading}
                >
                  Trigger Success
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-3 border-yellow-500/30 bg-yellow-500/5">
                <div className="text-center">
                  <h3 className="font-semibold mb-1 text-yellow-700 dark:text-yellow-400">Warning</h3>
                  <p className="text-xs text-muted-foreground">Caution feedback</p>
                </div>
                <Button
                  onClick={() => handleNotification('warning')}
                  variant="outline"
                  className="w-full border-yellow-500/50"
                  disabled={isLoading}
                >
                  Trigger Warning
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-3 border-red-500/30 bg-red-500/5">
                <div className="text-center">
                  <h3 className="font-semibold mb-1 text-red-700 dark:text-red-400">Error</h3>
                  <p className="text-xs text-muted-foreground">Failure feedback</p>
                </div>
                <Button
                  onClick={() => handleNotification('error')}
                  variant="outline"
                  className="w-full border-red-500/50"
                  disabled={isLoading}
                >
                  Trigger Error
                </Button>
              </div>
            </div>
          </div>
        </section>

        {/* Custom Vibration */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold">Custom Vibration</h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Trigger vibrations with custom durations and patterns
            </p>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div className="space-y-3">
                <h3 className="font-semibold text-sm">Duration-based</h3>
                <div className="flex flex-wrap gap-2">
                  <Button onClick={() => handleVibrate(50)} variant="outline" size="sm" disabled={isLoading}>
                    50ms
                  </Button>
                  <Button onClick={() => handleVibrate(100)} variant="outline" size="sm" disabled={isLoading}>
                    100ms
                  </Button>
                  <Button onClick={() => handleVibrate(200)} variant="outline" size="sm" disabled={isLoading}>
                    200ms
                  </Button>
                  <Button onClick={() => handleVibrate(500)} variant="outline" size="sm" disabled={isLoading}>
                    500ms
                  </Button>
                </div>
              </div>

              <div className="space-y-3">
                <h3 className="font-semibold text-sm">Pattern-based</h3>
                <Button onClick={handleVibratePattern} variant="outline" className="w-full" disabled={isLoading}>
                  Trigger Pattern [100, 50, 100, 50, 200]
                </Button>
                <p className="text-xs text-muted-foreground">
                  Pattern alternates between vibration and pause durations
                </p>
              </div>
            </div>
          </div>
        </section>

        {/* Output Panel */}
        <section className="rounded-lg border p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold">Output</h2>
            <Button variant="outline" size="sm" onClick={() => setOutput([])}>
              Clear
            </Button>
          </div>

          <div className="bg-muted rounded-md p-4 h-64 overflow-y-auto font-mono text-sm">
            {output.length === 0 ? (
              <p className="text-muted-foreground">No output yet. Try triggering some haptic feedback...</p>
            ) : (
              output.map((line, i) => (
                <div key={i} className="mb-1">
                  {line}
                </div>
              ))
            )}
          </div>
        </section>

        {/* Implementation Guide */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Implementation Guide</h3>
          <div className="space-y-4 text-sm">
            <div className="space-y-2">
              <h4 className="font-semibold">Android Implementation</h4>
              <p className="text-muted-foreground">
                Requires custom plugin using Android Vibrator API with VibrationEffect
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div>// Light Impact</div>
                <div>vibrator.vibrate(VibrationEffect.createOneShot(10, 50))</div>
                <div className="mt-2">// Medium Impact</div>
                <div>vibrator.vibrate(VibrationEffect.createOneShot(20, 100))</div>
                <div className="mt-2">// Heavy Impact</div>
                <div>vibrator.vibrate(VibrationEffect.createOneShot(30, 200))</div>
                <div className="mt-2">// Pattern</div>
                <div>val pattern = longArrayOf(0, 100, 50, 100)</div>
                <div>vibrator.vibrate(VibrationEffect.createWaveform(pattern, -1))</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">iOS Implementation</h4>
              <p className="text-muted-foreground">
                Requires custom plugin using UIKit UIFeedbackGenerator
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div>// Impact Feedback</div>
                <div>let generator = UIImpactFeedbackGenerator(style: .medium)</div>
                <div>generator.prepare()</div>
                <div>generator.impactOccurred()</div>
                <div className="mt-2">// Notification Feedback</div>
                <div>let notif = UINotificationFeedbackGenerator()</div>
                <div>notif.prepare()</div>
                <div>notif.notificationOccurred(.success)</div>
                <div className="mt-2">// Selection Feedback</div>
                <div>let selection = UISelectionFeedbackGenerator()</div>
                <div>selection.prepare()</div>
                <div>selection.selectionChanged()</div>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400">
                Permissions & Considerations
              </h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Android: Requires VIBRATE permission in AndroidManifest.xml</li>
                <li>iOS: No special permissions required for haptic feedback</li>
                <li>Test on physical devices (haptics don't work in emulators/simulators)</li>
                <li>Check device haptic settings (may be disabled by user)</li>
                <li>Battery saver mode may disable haptics</li>
                <li>Use haptics sparingly to avoid annoying users</li>
                <li>iOS Taptic Engine available on iPhone 6s and later</li>
              </ul>
            </div>
          </div>
        </section>

        {/* Platform Support */}
        <section className="rounded-lg border border-purple-500/50 bg-purple-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Platform Support</h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b">
                  <th className="text-left py-2 px-4">Feature</th>
                  <th className="text-center py-2 px-4">Android</th>
                  <th className="text-center py-2 px-4">iOS</th>
                  <th className="text-center py-2 px-4">Desktop</th>
                </tr>
              </thead>
              <tbody className="text-muted-foreground">
                <tr className="border-b">
                  <td className="py-2 px-4">Light Impact</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Medium Impact</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Heavy Impact</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Success Haptic</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Warning Haptic</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Error Haptic</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Custom Duration</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">⚠️*</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Pattern Vibration</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">⚠️*</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr>
                  <td className="py-2 px-4">Selection Haptic</td>
                  <td className="text-center py-2 px-4">⚠️**</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
              </tbody>
            </table>
            <div className="text-xs text-muted-foreground mt-2 space-y-1">
              <p>* iOS UIFeedbackGenerator uses predefined patterns, custom durations limited</p>
              <p>** Android can simulate selection haptics with short vibrations</p>
            </div>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
