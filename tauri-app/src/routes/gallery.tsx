import { createFileRoute } from '@tanstack/react-router'
import { Images, Image, Video, Grid3x3, Trash2, X } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { convertFileSrc } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

export const Route = createFileRoute('/gallery')({
  component: GalleryModule,
})

interface MediaItem {
  id: string
  uri: string
  type: 'image' | 'video'
  name: string
  webviewUrl: string
  size?: number
  extension?: string
}

interface FileMetadata {
  name: string
  size: number
  extension: string
}

function GalleryModule() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)
  const [mediaItems, setMediaItems] = useState<MediaItem[]>([])
  const [selectedItem, setSelectedItem] = useState<MediaItem | null>(null)

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

  const isVideoFile = (filename: string): boolean => {
    const videoExtensions = ['mp4', 'mov', 'avi', 'mkv', 'webm']
    const extension = filename.split('.').pop()?.toLowerCase() || ''
    return videoExtensions.includes(extension)
  }

  const createMediaItem = async (filePath: string): Promise<MediaItem> => {
    const fileName = filePath.split(/[\\/]/).pop() || 'media'
    const webviewUrl = convertFileSrc(filePath)
    const type = isVideoFile(fileName) ? 'video' : 'image'

    // Try to get metadata
    let metadata: FileMetadata | null = null
    try {
      metadata = await invoke<FileMetadata>('get_file_metadata', { filePath })
    } catch (error) {
      console.warn('Failed to get file metadata:', error)
    }

    return {
      id: `${Date.now()}-${Math.random()}`,
      uri: filePath,
      type,
      name: fileName,
      webviewUrl,
      size: metadata?.size,
      extension: metadata?.extension,
    }
  }

  // Pick Single Image
  const handlePickImage = async () => {
    setLoading('pick-image')
    addOutput('Opening image picker...')

    try {
      const filePath = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: 'Images',
            extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp', 'svg'],
          },
        ],
      })

      if (!filePath) {
        addOutput('Image selection cancelled')
        setLoading(null)
        return
      }

      const mediaItem = await createMediaItem(filePath as string)
      setMediaItems([mediaItem])
      addOutput(`✓ Image selected: ${mediaItem.name}`)
      if (mediaItem.size) {
        addOutput(`File size: ${formatFileSize(mediaItem.size)}`)
      }
    } catch (error) {
      addOutput(`✗ Failed to pick image: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // Pick Single Video
  const handlePickVideo = async () => {
    setLoading('pick-video')
    addOutput('Opening video picker...')

    try {
      const filePath = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: 'Videos',
            extensions: ['mp4', 'mov', 'avi', 'mkv', 'webm', 'flv', 'wmv'],
          },
        ],
      })

      if (!filePath) {
        addOutput('Video selection cancelled')
        setLoading(null)
        return
      }

      const mediaItem = await createMediaItem(filePath as string)
      setMediaItems([mediaItem])
      addOutput(`✓ Video selected: ${mediaItem.name}`)
      if (mediaItem.size) {
        addOutput(`File size: ${formatFileSize(mediaItem.size)}`)
      }
    } catch (error) {
      addOutput(`✗ Failed to pick video: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // Pick Multiple Media
  const handlePickMultiple = async () => {
    setLoading('pick-multiple')
    addOutput('Opening media picker (multiple selection)...')

    try {
      const filePaths = await open({
        multiple: true,
        directory: false,
        filters: [
          {
            name: 'Media',
            extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp', 'mp4', 'mov', 'avi', 'mkv', 'webm'],
          },
        ],
      })

      if (!filePaths || filePaths.length === 0) {
        addOutput('Media selection cancelled')
        setLoading(null)
        return
      }

      addOutput(`Processing ${filePaths.length} file(s)...`)

      const items = await Promise.all(
        (filePaths as string[]).map((path) => createMediaItem(path))
      )

      setMediaItems(items)
      addOutput(`✓ Selected ${items.length} media item(s)`)

      const images = items.filter((item) => item.type === 'image').length
      const videos = items.filter((item) => item.type === 'video').length

      if (images > 0) addOutput(`  - ${images} image(s)`)
      if (videos > 0) addOutput(`  - ${videos} video(s)`)

      const totalSize = items.reduce((acc, item) => acc + (item.size || 0), 0)
      if (totalSize > 0) {
        addOutput(`Total size: ${formatFileSize(totalSize)}`)
      }
    } catch (error) {
      addOutput(`✗ Failed to pick media: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // Clear Selection
  const handleClearSelection = () => {
    setMediaItems([])
    setSelectedItem(null)
    addOutput('Selection cleared')
  }

  // Remove Single Item
  const handleRemoveItem = (id: string) => {
    const item = mediaItems.find((i) => i.id === id)
    if (item) {
      setMediaItems((prev) => prev.filter((i) => i.id !== id))
      if (selectedItem?.id === id) {
        setSelectedItem(null)
      }
      addOutput(`Removed: ${item.name}`)
    }
  }

  // Open Full Screen Preview
  const handlePreview = (item: MediaItem) => {
    setSelectedItem(item)
    addOutput(`Previewing: ${item.name}`)
  }

  // Close Preview
  const handleClosePreview = () => {
    setSelectedItem(null)
  }

  return (
    <ModulePageLayout
      title="Gallery / Media Library Module"
      description="Pick photos and videos from device storage with preview capabilities"
      icon={Images}
    >
      <div className="space-y-6">
        {/* Status Notice */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-blue-500">ℹ️</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Desktop implementation:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li><strong className="text-green-600">✓ File Picker</strong> - Using Tauri Dialog plugin</li>
              <li><strong className="text-green-600">✓ Image Preview</strong> - Display selected images</li>
              <li><strong className="text-green-600">✓ Video Preview</strong> - Display selected videos</li>
              <li><strong className="text-green-600">✓ Multiple Selection</strong> - Select multiple files</li>
              <li><strong className="text-yellow-600">⚠ Mobile</strong> - Requires custom plugin</li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># For mobile support, implement native plugins:</div>
              <div>Android: ACTION_PICK Intent / MediaStore API</div>
              <div>iOS: PHPickerViewController</div>
            </div>
            <p className="text-muted-foreground mt-2">
              Desktop file picker is fully functional. Mobile requires custom native plugin development.
            </p>
          </div>
        </section>

        {/* Picker Actions */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Grid3x3 className="w-5 h-5" />
            Media Picker
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Select images or videos from your device storage
            </p>

            <div className="flex flex-wrap gap-2">
              <Button
                onClick={handlePickImage}
                disabled={loading === 'pick-image'}
                variant="outline"
              >
                <Image className={`w-4 h-4 mr-2 ${loading === 'pick-image' ? 'animate-pulse' : ''}`} />
                {loading === 'pick-image' ? 'Opening...' : 'Pick Image'}
              </Button>

              <Button
                onClick={handlePickVideo}
                disabled={loading === 'pick-video'}
                variant="outline"
              >
                <Video className={`w-4 h-4 mr-2 ${loading === 'pick-video' ? 'animate-pulse' : ''}`} />
                {loading === 'pick-video' ? 'Opening...' : 'Pick Video'}
              </Button>

              <Button
                onClick={handlePickMultiple}
                disabled={loading === 'pick-multiple'}
                variant="outline"
              >
                <Images className={`w-4 h-4 mr-2 ${loading === 'pick-multiple' ? 'animate-pulse' : ''}`} />
                {loading === 'pick-multiple' ? 'Opening...' : 'Pick Multiple'}
              </Button>

              {mediaItems.length > 0 && (
                <Button onClick={handleClearSelection} variant="destructive" size="sm">
                  <Trash2 className="w-4 h-4 mr-2" />
                  Clear All
                </Button>
              )}
            </div>
          </div>
        </section>

        {/* Media Grid */}
        {mediaItems.length > 0 && (
          <section className="rounded-lg border p-6 space-y-4">
            <div className="flex items-center justify-between">
              <h2 className="text-xl font-semibold flex items-center gap-2">
                <Images className="w-5 h-5" />
                Selected Media ({mediaItems.length})
              </h2>
            </div>

            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
              {mediaItems.map((item) => (
                <div
                  key={item.id}
                  className="relative group rounded-lg overflow-hidden border bg-muted cursor-pointer hover:ring-2 hover:ring-primary transition-all"
                  onClick={() => handlePreview(item)}
                >
                  {/* Thumbnail */}
                  <div className="aspect-square overflow-hidden bg-black/5">
                    {item.type === 'image' ? (
                      <img
                        src={item.webviewUrl}
                        alt={item.name}
                        className="w-full h-full object-cover"
                      />
                    ) : (
                      <div className="w-full h-full flex items-center justify-center bg-black/10">
                        <Video className="w-12 h-12 text-muted-foreground" />
                      </div>
                    )}
                  </div>

                  {/* Info Overlay */}
                  <div className="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/80 to-transparent p-3">
                    <p className="text-white text-xs font-medium truncate">{item.name}</p>
                    {item.size && (
                      <p className="text-white/70 text-xs">{formatFileSize(item.size)}</p>
                    )}
                  </div>

                  {/* Remove Button */}
                  <button
                    onClick={(e) => {
                      e.stopPropagation()
                      handleRemoveItem(item.id)
                    }}
                    className="absolute top-2 right-2 p-1 bg-destructive text-destructive-foreground rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
                  >
                    <X className="w-4 h-4" />
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

        {/* Full Screen Preview Modal */}
        {selectedItem && (
          <div
            className="fixed inset-0 z-50 bg-black/90 flex items-center justify-center p-4"
            onClick={handleClosePreview}
          >
            <button
              onClick={handleClosePreview}
              className="absolute top-4 right-4 p-2 bg-white/10 hover:bg-white/20 text-white rounded-full transition-colors"
            >
              <X className="w-6 h-6" />
            </button>

            <div className="max-w-6xl max-h-[90vh] w-full" onClick={(e) => e.stopPropagation()}>
              {selectedItem.type === 'image' ? (
                <img
                  src={selectedItem.webviewUrl}
                  alt={selectedItem.name}
                  className="w-full h-full object-contain"
                />
              ) : (
                <video
                  src={selectedItem.webviewUrl}
                  controls
                  autoPlay
                  className="w-full h-full"
                />
              )}

              {/* Info Bar */}
              <div className="mt-4 p-4 bg-white/10 backdrop-blur-sm rounded-lg text-white">
                <h3 className="font-semibold text-lg mb-2">{selectedItem.name}</h3>
                <div className="flex flex-wrap gap-4 text-sm text-white/80">
                  <span>Type: {selectedItem.type}</span>
                  {selectedItem.extension && <span>Format: {selectedItem.extension}</span>}
                  {selectedItem.size && <span>Size: {formatFileSize(selectedItem.size)}</span>}
                </div>
              </div>
            </div>
          </div>
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
              <h4 className="font-semibold">Desktop Implementation</h4>
              <p className="text-muted-foreground">
                Uses Tauri's Dialog plugin for file selection. Supports filtering by file type
                and multiple file selection.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div>bun add @tauri-apps/plugin-dialog</div>
                <div>import &#123; open &#125; from '@tauri-apps/plugin-dialog'</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">File Path Conversion</h4>
              <p className="text-muted-foreground">
                Use <code>convertFileSrc()</code> to convert file system paths to WebView-compatible URLs
                for displaying images and videos.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div>import &#123; convertFileSrc &#125; from '@tauri-apps/api/core'</div>
                <div>const url = convertFileSrc(filePath)</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Mobile Implementation</h4>
              <p className="text-muted-foreground">
                Requires custom native plugins for Android (Intent) and iOS (PHPickerViewController)
                to access device gallery.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div># Android: ACTION_PICK / MediaStore</div>
                <div># iOS: PHPickerViewController</div>
                <div># Requires Kotlin/Swift bridge implementation</div>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400">
                Security & Privacy
              </h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Request only necessary permissions</li>
                <li>Validate file types and sizes</li>
                <li>Use secure file path conversion</li>
                <li>Respect user privacy choices</li>
                <li>Clear sensitive data when done</li>
                <li>Handle permission denials gracefully</li>
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
                  <td className="py-2 px-4">Pick Image</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Pick Video</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Multiple Selection</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                </tr>
                <tr>
                  <td className="py-2 px-4">File Metadata</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
              </tbody>
            </table>
            <p className="text-xs text-muted-foreground mt-2">
              * 🔶 = Requires custom plugin development
            </p>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
