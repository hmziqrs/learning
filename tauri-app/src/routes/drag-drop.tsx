import { createFileRoute } from '@tanstack/react-router'
import { Upload, FileIcon, X, Trash2 } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect, useRef } from 'react'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import type { UnlistenFn } from '@tauri-apps/api/event'

export const Route = createFileRoute('/drag-drop')({
  component: DragDrop,
})

interface DroppedFile {
  id: string
  path?: string // Native Tauri (full path)
  name: string
  size?: number // HTML5 only
  type?: string // HTML5 only
  lastModified?: number // HTML5 only
  source: 'native' | 'html5'
  droppedAt: Date
}

type DropMode = 'native' | 'html5'

function DragDrop() {
  const [mode, setMode] = useState<DropMode>('native')
  const [droppedFiles, setDroppedFiles] = useState<DroppedFile[]>([])
  const [output, setOutput] = useState<string[]>([])
  const [isDragging, setIsDragging] = useState(false)
  const [nativeListenerActive, setNativeListenerActive] = useState(false)

  // Use ref to track current mode for the native listener
  const modeRef = useRef<DropMode>('native')

  // Track last drop to prevent duplicates from React StrictMode - separate refs for each mode
  const lastNativeDropRef = useRef<{ paths: string[], timestamp: number } | null>(null)
  const lastHtml5DropRef = useRef<{ paths: string[], timestamp: number } | null>(null)

  // Update ref when mode changes
  useEffect(() => {
    modeRef.current = mode
  }, [mode])

  useEffect(() => {
    let unlisten: UnlistenFn | null = null

    const setupNativeListeners = async () => {
      // Only set up native listener when in native mode
      if (mode !== 'native') {
        setNativeListenerActive(false)
        const timestamp = new Date().toLocaleTimeString()
        setOutput((prev) => [...prev, `[${timestamp}] ℹ Native listener disabled (HTML5 mode active)`])
        return
      }

      try {
        const webview = getCurrentWebviewWindow()

        // Use the Tauri v2 onDragDropEvent API
        unlisten = await webview.onDragDropEvent((event) => {
          const timestamp = new Date().toLocaleTimeString()
          const payload = event.payload as any

          console.log('Drag event:', payload) // Debug log

          const paths = payload.paths || []
          const position = payload.position || { x: 0, y: 0 }
          const type = payload.type || ''

          // Handle different drag event types
          if (type === 'over') {
            // Drag over event - only update visual state, don't log spam
            setIsDragging(true)
          } else if (type === 'drop') {
            // Drop event
            setIsDragging(false)

            if (paths.length > 0) {
              // Prevent duplicate processing (React StrictMode causes double-invocation)
              const now = Date.now()
              const lastDrop = lastNativeDropRef.current

              // Check if this is the same drop event within 100ms
              if (lastDrop &&
                  now - lastDrop.timestamp < 100 &&
                  JSON.stringify(lastDrop.paths) === JSON.stringify(paths)) {
                console.log('SKIPPING DUPLICATE NATIVE DROP EVENT')
                return
              }

              // Record this drop event
              lastNativeDropRef.current = { paths, timestamp: now }

              console.log('NATIVE DROP:', paths)
              setOutput((prev) => [...prev, `[${timestamp}] ✓ Files dropped (Native): ${paths.length} file(s)`])

              const newFiles: DroppedFile[] = paths.map((path: string, index: number) => ({
                id: `native-${Date.now()}-${Math.random()}-${index}`,
                path,
                name: path.split('/').pop() || path.split('\\').pop() || path,
                source: 'native',
                droppedAt: new Date(),
              }))

              setDroppedFiles((prev) => {
                console.log('Adding native files, current count:', prev.length, 'adding:', newFiles.length)
                return [...prev, ...newFiles]
              })
            }
          } else if (type === 'leave') {
            // Drag leave event
            setIsDragging(false)
          } else if (type === 'enter') {
            // Drag enter event - don't spam logs
          }
        })

        setNativeListenerActive(true)
        const timestamp = new Date().toLocaleTimeString()
        setOutput((prev) => [...prev, `[${timestamp}] ✓ Native Tauri drag & drop listener initialized`])
      } catch (error) {
        const timestamp = new Date().toLocaleTimeString()
        setOutput((prev) => [...prev, `[${timestamp}] ✗ Error setting up listeners: ${error}`])
        setNativeListenerActive(false)
      }
    }

    setupNativeListeners()

    return () => {
      if (unlisten) {
        unlisten()
        const timestamp = new Date().toLocaleTimeString()
        setOutput((prev) => [...prev, `[${timestamp}] ℹ Native listener cleaned up`])
      }
    }
  }, [mode])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const handleHtml5DragOver = (e: React.DragEvent) => {
    // Must always prevent default for drop to work
    e.preventDefault()
    e.stopPropagation()

    if (mode === 'html5') {
      setIsDragging(true)
    }
  }

  const handleHtml5DragLeave = (e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()

    if (mode === 'html5') {
      setIsDragging(false)
    }
  }

  const handleHtml5Drop = (e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()

    if (mode !== 'html5') return

    setIsDragging(false)

    const files = Array.from(e.dataTransfer.files)

    // Prevent duplicate processing
    if (files.length > 0) {
      const now = Date.now()
      const fileNames = files.map(f => f.name)
      const lastDrop = lastHtml5DropRef.current

      // Check if this is the same drop event within 100ms
      if (lastDrop &&
          now - lastDrop.timestamp < 100 &&
          JSON.stringify(lastDrop.paths) === JSON.stringify(fileNames)) {
        console.log('SKIPPING DUPLICATE HTML5 DROP EVENT')
        return
      }

      // Record this drop event
      lastHtml5DropRef.current = { paths: fileNames, timestamp: now }
    }

    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ✓ Files dropped (HTML5): ${files.length} file(s)`])

    const newFiles: DroppedFile[] = files.map((file, index) => ({
      id: `html5-${Date.now()}-${Math.random()}-${index}`,
      name: file.name,
      size: file.size,
      type: file.type,
      lastModified: file.lastModified,
      source: 'html5',
      droppedAt: new Date(),
    }))

    setDroppedFiles((prev) => [...prev, ...newFiles])
  }

  const handleClearAll = () => {
    setDroppedFiles([])
    addOutput('Cleared all dropped files')
  }

  const handleRemoveFile = (id: string) => {
    setDroppedFiles((prev) => prev.filter((file) => file.id !== id))
    addOutput('Removed file from list')
  }

  const handleClearOutput = () => {
    setOutput([])
  }

  const formatFileSize = (bytes?: number): string => {
    if (!bytes) return 'Unknown'
    if (bytes < 1024) return `${bytes} B`
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(2)} MB`
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`
  }

  const formatDate = (date: Date): string => {
    return date.toLocaleString()
  }

  return (
    <ModulePageLayout
      title="Drag & Drop Module"
      description="Test Tauri's native file-drop + HTML5 drag drop functionality."
      icon={Upload}
    >
      <div className="space-y-6">
        {/* Mode Toggle Section */}
        <div className="border border-border rounded-lg p-6 bg-card">
          <h2 className="text-xl font-semibold mb-4">Drop Mode</h2>
          <div className="flex gap-4">
            <Button
              variant={mode === 'native' ? 'default' : 'outline'}
              onClick={() => {
                setMode('native')
                addOutput('Switched to Native Tauri mode')
              }}
            >
              Native Tauri
            </Button>
            <Button
              variant={mode === 'html5' ? 'default' : 'outline'}
              onClick={() => {
                setMode('html5')
                addOutput('Switched to HTML5 mode')
              }}
            >
              HTML5 Drag & Drop
            </Button>
          </div>
          <p className="text-sm text-muted-foreground mt-3">
            <strong>Current Mode:</strong> {mode === 'native' ? 'Native Tauri' : 'HTML5'}
          </p>
          <p className="text-sm text-muted-foreground mt-2">
            {mode === 'native'
              ? '✓ Full file paths • Desktop only • Better OS integration'
              : '✓ Cross-platform • Web compatible • Limited file path access'}
          </p>
          {mode === 'native' && (
            <p className="text-sm text-muted-foreground mt-2">
              Native listener status: {nativeListenerActive ? '🟢 Active' : '🔴 Inactive'}
            </p>
          )}
        </div>

        {/* Drop Zone */}
        <div className="border border-border rounded-lg p-6 bg-card">
          <h2 className="text-xl font-semibold mb-4">Drop Zone</h2>
          <div
            onDragOver={handleHtml5DragOver}
            onDragLeave={handleHtml5DragLeave}
            onDrop={handleHtml5Drop}
            className={`
              border-2 border-dashed rounded-lg p-12 text-center transition-all duration-200
              ${isDragging
                ? 'border-primary bg-primary/10 scale-[1.02]'
                : 'border-border bg-muted/30 hover:border-primary/50 hover:bg-muted/50'
              }
            `}
          >
            <div className="flex flex-col items-center gap-4">
              <div className={`p-4 rounded-full ${isDragging ? 'bg-primary/20' : 'bg-muted'}`}>
                <Upload className={`w-12 h-12 ${isDragging ? 'text-primary' : 'text-muted-foreground'}`} />
              </div>
              <div>
                <p className="text-lg font-medium">
                  {isDragging ? 'Drop files here!' : 'Drag & drop files here'}
                </p>
                <p className="text-sm text-muted-foreground mt-1">
                  {mode === 'native' ? 'Using Native Tauri file drop' : 'Using HTML5 drag & drop API'}
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Dropped Files List */}
        <div className="border border-border rounded-lg p-6 bg-card">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-semibold">
              Dropped Files ({droppedFiles.length})
            </h2>
            {droppedFiles.length > 0 && (
              <Button
                variant="outline"
                size="sm"
                onClick={handleClearAll}
              >
                <Trash2 className="w-4 h-4 mr-2" />
                Clear All
              </Button>
            )}
          </div>

          {droppedFiles.length === 0 ? (
            <p className="text-muted-foreground text-center py-8">
              No files dropped yet. Drop some files in the zone above!
            </p>
          ) : (
            <div className="space-y-3">
              {droppedFiles.map((file) => (
                <div
                  key={file.id}
                  className="border border-border rounded-lg p-4 bg-muted/30 hover:bg-muted/50 transition-colors"
                >
                  <div className="flex items-start gap-3">
                    <div className="p-2 bg-primary/10 rounded-lg">
                      <FileIcon className="w-5 h-5 text-primary" />
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between gap-2">
                        <div className="flex-1 min-w-0">
                          <p className="font-medium truncate">{file.name}</p>
                          <div className="text-sm text-muted-foreground space-y-1 mt-1">
                            <p>Source: <span className="font-mono">{file.source}</span></p>
                            {file.path && <p className="truncate">Path: {file.path}</p>}
                            {file.size !== undefined && <p>Size: {formatFileSize(file.size)}</p>}
                            {file.type && <p>Type: {file.type || 'unknown'}</p>}
                            <p>Dropped: {formatDate(file.droppedAt)}</p>
                          </div>
                        </div>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => handleRemoveFile(file.id)}
                          className="shrink-0"
                        >
                          <X className="w-4 h-4" />
                        </Button>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Output Panel */}
        <div className="border border-border rounded-lg p-6 bg-card">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-semibold">Event Log</h2>
            {output.length > 0 && (
              <Button
                variant="outline"
                size="sm"
                onClick={handleClearOutput}
              >
                Clear Log
              </Button>
            )}
          </div>
          <div className="bg-muted/30 rounded-lg p-4 font-mono text-sm max-h-64 overflow-y-auto">
            {output.length === 0 ? (
              <p className="text-muted-foreground">No events yet...</p>
            ) : (
              <div className="space-y-1">
                {output.map((line, index) => (
                  <div key={index} className="text-foreground">
                    {line}
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Info Section */}
        <div className="border border-border rounded-lg p-6 bg-muted/50">
          <h3 className="text-lg font-semibold mb-3">About Drag & Drop Modes</h3>
          <div className="space-y-4 text-sm text-muted-foreground">
            <div>
              <p className="font-semibold text-foreground mb-1">Native Tauri:</p>
              <ul className="list-disc list-inside space-y-1 ml-2">
                <li>Provides full file system paths</li>
                <li>Better desktop integration</li>
                <li>No browser restrictions</li>
                <li>Desktop platforms only</li>
              </ul>
            </div>
            <div>
              <p className="font-semibold text-foreground mb-1">HTML5 Drag & Drop:</p>
              <ul className="list-disc list-inside space-y-1 ml-2">
                <li>Cross-platform compatible (web-compatible)</li>
                <li>Can access file metadata (size, type, etc.)</li>
                <li>More flexible for in-app operations</li>
                <li>Limited file path access (browser security)</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </ModulePageLayout>
  )
}
