import { createFileRoute } from '@tanstack/react-router'
import { Camera, Video, SwitchCamera, Zap, ZapOff, Circle, Square, Trash2 } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState } from 'react'

export const Route = createFileRoute('/camera')({
  component: CameraModule,
})

interface CapturedMedia {
  id: string
  type: 'photo' | 'video'
  filePath: string
  timestamp: string
  size?: number
  duration?: number
}

function CameraModule() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)
  const [capturedMedia, setCapturedMedia] = useState<CapturedMedia[]>([])
  const [isRecording, setIsRecording] = useState(false)
  const [flashMode, setFlashMode] = useState<'on' | 'off' | 'auto'>('off')
  const [cameraFacing, setCameraFacing] = useState<'front' | 'back'>('back')

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`
  }

  const formatDuration = (seconds: number): string => {
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

  // Capture Photo
  const handleCapturePhoto = async () => {
    setLoading('capture-photo')
    addOutput('Capturing photo...')

    try {
      // This would invoke the Tauri command once implemented
      // const result = await invoke<CapturedPhoto>('capture_photo')

      addOutput('✗ Camera functionality requires custom plugin development', false)
      addOutput('Desktop: Requires native camera API integration', false)
      addOutput('Mobile: Requires CameraX (Android) / AVFoundation (iOS)', false)
    } catch (error) {
      addOutput(`✗ Failed to capture photo: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // Start Video Recording
  const handleStartRecording = async () => {
    setLoading('start-recording')
    addOutput('Starting video recording...')

    try {
      // This would invoke the Tauri command once implemented
      // await invoke('start_video_recording')

      setIsRecording(true)
      addOutput('✗ Video recording requires custom plugin development', false)
      addOutput('Desktop: Requires MediaFoundation/AVFoundation/V4L2', false)
      addOutput('Mobile: Requires CameraX (Android) / AVFoundation (iOS)', false)
    } catch (error) {
      addOutput(`✗ Failed to start recording: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // Stop Video Recording
  const handleStopRecording = async () => {
    setLoading('stop-recording')
    addOutput('Stopping video recording...')

    try {
      // This would invoke the Tauri command once implemented
      // const result = await invoke<RecordedVideo>('stop_video_recording')

      setIsRecording(false)
      addOutput('Recording stopped (plugin not implemented)', false)
    } catch (error) {
      addOutput(`✗ Failed to stop recording: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // Switch Camera (Front/Back)
  const handleSwitchCamera = async () => {
    setLoading('switch-camera')
    const newFacing = cameraFacing === 'front' ? 'back' : 'front'
    addOutput(`Switching to ${newFacing} camera...`)

    try {
      // This would invoke the Tauri command once implemented
      // await invoke('switch_camera')

      setCameraFacing(newFacing)
      addOutput(`✗ Camera switching requires plugin implementation`, false)
    } catch (error) {
      addOutput(`✗ Failed to switch camera: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // Toggle Flash Mode
  const handleFlashToggle = async () => {
    const modes: Array<'on' | 'off' | 'auto'> = ['off', 'on', 'auto']
    const currentIndex = modes.indexOf(flashMode)
    const newMode = modes[(currentIndex + 1) % modes.length]

    setLoading('toggle-flash')
    addOutput(`Setting flash mode to: ${newMode}`)

    try {
      // This would invoke the Tauri command once implemented
      // await invoke('set_flash_mode', { mode: newMode })

      setFlashMode(newMode)
      addOutput(`Flash mode set to: ${newMode}`)
    } catch (error) {
      addOutput(`✗ Failed to set flash mode: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // Clear All Captured Media
  const handleClearAll = () => {
    setCapturedMedia([])
    addOutput('Cleared all captured media')
  }

  // Remove Single Item
  const handleRemoveItem = (id: string) => {
    const item = capturedMedia.find((i) => i.id === id)
    if (item) {
      setCapturedMedia((prev) => prev.filter((i) => i.id !== id))
      addOutput(`Removed: ${item.type} from ${item.timestamp}`)
    }
  }

  return (
    <ModulePageLayout
      title="Camera Module"
      description="Capture photos and record videos directly from device cameras"
      icon={Camera}
    >
      <div className="space-y-6">
        {/* Status Notice */}
        <section className="rounded-lg border border-yellow-500/50 bg-yellow-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-yellow-500">⚠️</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Camera module requires custom plugin development:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li><strong className="text-yellow-600">Desktop</strong> - No official Tauri camera plugin exists</li>
              <li><strong className="text-yellow-600">Windows</strong> - Requires MediaFoundation API</li>
              <li><strong className="text-yellow-600">macOS</strong> - Requires AVFoundation framework</li>
              <li><strong className="text-yellow-600">Linux</strong> - Requires V4L2 (Video4Linux2)</li>
              <li><strong className="text-yellow-600">Android</strong> - Requires CameraX API</li>
              <li><strong className="text-yellow-600">iOS</strong> - Requires AVFoundation framework</li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># Custom plugin implementation required for all platforms</div>
              <div>See documentation: docs/camera-module.md</div>
            </div>
            <p className="text-muted-foreground mt-2">
              This is a planning/demonstration UI. Full functionality requires native plugin development.
            </p>
          </div>
        </section>

        {/* Camera Preview Area */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Camera className="w-5 h-5" />
            Camera Preview
          </h2>

          <div className="aspect-video bg-muted/50 rounded-lg flex items-center justify-center border-2 border-dashed border-muted-foreground/30">
            <div className="text-center space-y-2">
              <Camera className="w-16 h-16 mx-auto text-muted-foreground/50" />
              <p className="text-muted-foreground text-sm">
                Camera preview will appear here
              </p>
              <p className="text-muted-foreground/70 text-xs">
                Requires native plugin implementation
              </p>
            </div>
          </div>

          {/* Camera Info */}
          <div className="flex items-center justify-between text-sm text-muted-foreground">
            <span>Camera: {cameraFacing === 'front' ? 'Front' : 'Back'}</span>
            <span>Flash: {flashMode === 'on' ? '⚡ On' : flashMode === 'auto' ? '⚡ Auto' : '○ Off'}</span>
            {isRecording && (
              <span className="flex items-center gap-2 text-red-500 font-semibold">
                <span className="w-2 h-2 bg-red-500 rounded-full animate-pulse"></span>
                Recording
              </span>
            )}
          </div>
        </section>

        {/* Camera Controls */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Circle className="w-5 h-5" />
            Camera Controls
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Control camera, capture photos, and record videos
            </p>

            <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
              {/* Capture Photo */}
              <Button
                onClick={handleCapturePhoto}
                disabled={loading === 'capture-photo' || isRecording}
                variant="default"
                size="lg"
              >
                <Camera className={`w-4 h-4 mr-2 ${loading === 'capture-photo' ? 'animate-pulse' : ''}`} />
                {loading === 'capture-photo' ? 'Capturing...' : 'Capture Photo'}
              </Button>

              {/* Video Recording */}
              {!isRecording ? (
                <Button
                  onClick={handleStartRecording}
                  disabled={loading === 'start-recording'}
                  variant="destructive"
                  size="lg"
                >
                  <Circle className={`w-4 h-4 mr-2 ${loading === 'start-recording' ? 'animate-pulse' : ''}`} />
                  {loading === 'start-recording' ? 'Starting...' : 'Start Recording'}
                </Button>
              ) : (
                <Button
                  onClick={handleStopRecording}
                  disabled={loading === 'stop-recording'}
                  variant="destructive"
                  size="lg"
                >
                  <Square className={`w-4 h-4 mr-2 ${loading === 'stop-recording' ? 'animate-pulse' : ''}`} />
                  {loading === 'stop-recording' ? 'Stopping...' : 'Stop Recording'}
                </Button>
              )}

              {/* Switch Camera */}
              <Button
                onClick={handleSwitchCamera}
                disabled={loading === 'switch-camera' || isRecording}
                variant="outline"
              >
                <SwitchCamera className={`w-4 h-4 mr-2 ${loading === 'switch-camera' ? 'animate-pulse' : ''}`} />
                {cameraFacing === 'front' ? 'Back' : 'Front'} Camera
              </Button>

              {/* Flash Toggle */}
              <Button
                onClick={handleFlashToggle}
                disabled={loading === 'toggle-flash' || isRecording}
                variant="outline"
              >
                {flashMode === 'off' ? (
                  <ZapOff className="w-4 h-4 mr-2" />
                ) : (
                  <Zap className="w-4 h-4 mr-2" />
                )}
                Flash: {flashMode}
              </Button>

              {/* Clear All */}
              {capturedMedia.length > 0 && (
                <Button
                  onClick={handleClearAll}
                  variant="destructive"
                  size="sm"
                >
                  <Trash2 className="w-4 h-4 mr-2" />
                  Clear All
                </Button>
              )}
            </div>
          </div>
        </section>

        {/* Captured Media Gallery */}
        {capturedMedia.length > 0 && (
          <section className="rounded-lg border p-6 space-y-4">
            <div className="flex items-center justify-between">
              <h2 className="text-xl font-semibold flex items-center gap-2">
                <Video className="w-5 h-5" />
                Captured Media ({capturedMedia.length})
              </h2>
            </div>

            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
              {capturedMedia.map((item) => (
                <div
                  key={item.id}
                  className="relative group rounded-lg overflow-hidden border bg-muted"
                >
                  {/* Thumbnail Placeholder */}
                  <div className="aspect-square overflow-hidden bg-black/5 flex items-center justify-center">
                    {item.type === 'photo' ? (
                      <Camera className="w-12 h-12 text-muted-foreground" />
                    ) : (
                      <Video className="w-12 h-12 text-muted-foreground" />
                    )}
                  </div>

                  {/* Info Overlay */}
                  <div className="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/80 to-transparent p-3">
                    <p className="text-white text-xs font-medium">{item.type}</p>
                    <p className="text-white/70 text-xs">{item.timestamp}</p>
                    {item.size && (
                      <p className="text-white/70 text-xs">{formatFileSize(item.size)}</p>
                    )}
                    {item.duration && (
                      <p className="text-white/70 text-xs">{formatDuration(item.duration)}</p>
                    )}
                  </div>

                  {/* Remove Button */}
                  <button
                    onClick={() => handleRemoveItem(item.id)}
                    className="absolute top-2 right-2 p-1 bg-destructive text-destructive-foreground rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>

                  {/* Type Badge */}
                  <div className="absolute top-2 left-2 px-2 py-1 bg-black/60 text-white text-xs rounded">
                    {item.type}
                  </div>
                </div>
              ))}
            </div>
          </section>
        )}

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
              <p className="text-muted-foreground">No output yet...</p>
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
              <h4 className="font-semibold">Required Custom Plugins</h4>
              <p className="text-muted-foreground">
                Camera functionality requires platform-specific native plugin development.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div><span className="text-blue-600">Desktop:</span> Native camera API integration</div>
                <div><span className="text-green-600">Android:</span> CameraX or Camera2 API (Kotlin)</div>
                <div><span className="text-purple-600">iOS/macOS:</span> AVFoundation framework (Swift)</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Core Features</h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Real-time camera preview stream</li>
                <li>Photo capture with flash control</li>
                <li>Video recording with audio</li>
                <li>Camera switching (front/back)</li>
                <li>Zoom and focus controls</li>
                <li>Resolution and quality settings</li>
              </ul>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Permissions Required</h4>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div><span className="text-green-600">Android:</span> CAMERA, RECORD_AUDIO</div>
                <div><span className="text-purple-600">iOS:</span> NSCameraUsageDescription, NSMicrophoneUsageDescription</div>
                <div><span className="text-blue-600">macOS:</span> NSCameraUsageDescription, NSMicrophoneUsageDescription</div>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400">
                Security & Privacy
              </h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Request camera permissions at appropriate times</li>
                <li>Show visual indicator when camera is active</li>
                <li>Secure file storage for captured media</li>
                <li>Respect user privacy choices</li>
                <li>Handle permission denials gracefully</li>
                <li>Clear temporary files securely</li>
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
                  <th className="text-center py-2 px-4">Windows</th>
                  <th className="text-center py-2 px-4">macOS</th>
                  <th className="text-center py-2 px-4">Linux</th>
                  <th className="text-center py-2 px-4">iOS</th>
                  <th className="text-center py-2 px-4">Android</th>
                </tr>
              </thead>
              <tbody className="text-muted-foreground">
                <tr className="border-b">
                  <td className="py-2 px-4">Camera Preview</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Capture Photo</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Record Video</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Switch Camera</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Flash Control</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
                <tr>
                  <td className="py-2 px-4">Zoom Control</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
              </tbody>
            </table>
            <p className="text-xs text-muted-foreground mt-2">
              * 🔶 = Requires custom plugin development | ❌ = Not typically available
            </p>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
