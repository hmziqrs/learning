import { createFileRoute } from '@tanstack/react-router'
import { useState, useRef, useEffect } from 'react'
import {
  PlayCircle,
  Music,
  Video,
  Upload,
  Play,
  Pause,
  SkipBack,
  Volume2,
  VolumeX,
  Trash2,
  Clock,
  Globe,
} from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { open } from '@tauri-apps/plugin-dialog'
import { convertFileSrc } from '@tauri-apps/api/core'

export const Route = createFileRoute('/media')({
  component: Media,
})

interface MediaFile {
  path: string
  name: string
  type: 'audio' | 'video'
  url: string
}

interface MediaMetadata {
  duration: number
  currentTime: number
  volume: number
  paused: boolean
}

function Media() {
  const [output, setOutput] = useState<string>('')
  const [selectedFile, setSelectedFile] = useState<MediaFile | null>(null)
  const [playlist, setPlaylist] = useState<MediaFile[]>([])
  const [metadata, setMetadata] = useState<MediaMetadata>({
    duration: 0,
    currentTime: 0,
    volume: 1,
    paused: true,
  })
  const [playbackSpeed, setPlaybackSpeed] = useState(1)
  const [loading, setLoading] = useState(false)
  const [muted, setMuted] = useState(false)
  const [mediaUrl, setMediaUrl] = useState('')
  const [mediaType, setMediaType] = useState<'audio' | 'video'>('audio')

  const audioRef = useRef<HTMLAudioElement>(null)
  const videoRef = useRef<HTMLVideoElement>(null)

  const addOutput = (message: string) => {
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => `${prev}\n[${timestamp}] ${message}`)
  }

  const handleSelectAudio = async () => {
    try {
      setLoading(true)
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Audio',
            extensions: ['mp3', 'wav', 'ogg', 'flac', 'm4a', 'aac'],
          },
        ],
      })

      if (selected) {
        const filePath = selected as string
        const fileName = filePath.split('/').pop() || filePath.split('\\').pop() || 'unknown'
        const webViewUrl = convertFileSrc(filePath)

        const mediaFile: MediaFile = {
          path: filePath,
          name: fileName,
          type: 'audio',
          url: webViewUrl,
        }

        setSelectedFile(mediaFile)
        setPlaylist((prev) => {
          const exists = prev.some((f) => f.path === filePath)
          if (!exists) {
            return [...prev, mediaFile]
          }
          return prev
        })
        addOutput(`✓ Selected audio file: ${fileName}`)
      }
    } catch (error) {
      addOutput(`✗ Error selecting audio: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const handleSelectVideo = async () => {
    try {
      setLoading(true)
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Video',
            extensions: ['mp4', 'webm', 'ogg', 'mov', 'avi', 'mkv'],
          },
        ],
      })

      if (selected) {
        const filePath = selected as string
        const fileName = filePath.split('/').pop() || filePath.split('\\').pop() || 'unknown'
        const webViewUrl = convertFileSrc(filePath)

        const mediaFile: MediaFile = {
          path: filePath,
          name: fileName,
          type: 'video',
          url: webViewUrl,
        }

        setSelectedFile(mediaFile)
        setPlaylist((prev) => {
          const exists = prev.some((f) => f.path === filePath)
          if (!exists) {
            return [...prev, mediaFile]
          }
          return prev
        })
        addOutput(`✓ Selected video file: ${fileName}`)
      }
    } catch (error) {
      addOutput(`✗ Error selecting video: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const handleLoadFromUrl = () => {
    try {
      if (!mediaUrl.trim()) {
        addOutput('✗ Please enter a valid URL')
        return
      }

      // Basic URL validation
      try {
        new URL(mediaUrl)
      } catch {
        addOutput('✗ Invalid URL format')
        return
      }

      const fileName = mediaUrl.split('/').pop() || 'remote-media'

      const mediaFile: MediaFile = {
        path: mediaUrl,
        name: fileName,
        type: mediaType,
        url: mediaUrl,
      }

      setSelectedFile(mediaFile)
      setPlaylist((prev) => {
        const exists = prev.some((f) => f.path === mediaUrl)
        if (!exists) {
          return [...prev, mediaFile]
        }
        return prev
      })
      addOutput(`✓ Loaded ${mediaType} from URL: ${fileName}`)
      setMediaUrl('')
    } catch (error) {
      addOutput(`✗ Error loading from URL: ${error}`)
    }
  }

  const handlePlayPause = () => {
    const media = selectedFile?.type === 'audio' ? audioRef.current : videoRef.current
    if (media) {
      if (media.paused) {
        media.play()
        addOutput('▶ Playing media')
      } else {
        media.pause()
        addOutput('⏸ Paused media')
      }
    }
  }

  const handleStop = () => {
    const media = selectedFile?.type === 'audio' ? audioRef.current : videoRef.current
    if (media) {
      media.pause()
      media.currentTime = 0
      addOutput('⏹ Stopped media')
    }
  }

  const handleVolumeChange = (value: number) => {
    const media = selectedFile?.type === 'audio' ? audioRef.current : videoRef.current
    if (media) {
      media.volume = value
      setMetadata((prev) => ({ ...prev, volume: value }))
    }
  }

  const handleToggleMute = () => {
    const media = selectedFile?.type === 'audio' ? audioRef.current : videoRef.current
    if (media) {
      media.muted = !media.muted
      setMuted(media.muted)
      addOutput(media.muted ? '🔇 Muted' : '🔊 Unmuted')
    }
  }

  const handleSpeedChange = (speed: number) => {
    const media = selectedFile?.type === 'audio' ? audioRef.current : videoRef.current
    if (media) {
      media.playbackRate = speed
      setPlaybackSpeed(speed)
      addOutput(`⚡ Playback speed: ${speed}x`)
    }
  }

  const handleSeek = (value: number) => {
    const media = selectedFile?.type === 'audio' ? audioRef.current : videoRef.current
    if (media) {
      media.currentTime = value
    }
  }

  const handleClearPlaylist = () => {
    setPlaylist([])
    setSelectedFile(null)
    setOutput('')
    addOutput('🗑️ Cleared playlist')
  }

  const formatTime = (seconds: number) => {
    if (isNaN(seconds)) return '0:00'
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

  const handlePlaylistSelect = (file: MediaFile) => {
    setSelectedFile(file)
    addOutput(`📋 Loaded from playlist: ${file.name}`)
  }

  // Update metadata on time change
  useEffect(() => {
    const media = selectedFile?.type === 'audio' ? audioRef.current : videoRef.current
    if (!media) return

    const updateMetadata = () => {
      setMetadata({
        duration: media.duration,
        currentTime: media.currentTime,
        volume: media.volume,
        paused: media.paused,
      })
    }

    const handleLoadedMetadata = () => {
      updateMetadata()
      addOutput(
        `📊 Metadata loaded - Duration: ${formatTime(media.duration)}, Format: ${
          selectedFile?.name.split('.').pop()?.toUpperCase()
        }`
      )
    }

    const handleEnded = () => {
      addOutput('✓ Playback finished')
    }

    const handleError = () => {
      addOutput('✗ Error playing media - check if format is supported')
    }

    media.addEventListener('loadedmetadata', handleLoadedMetadata)
    media.addEventListener('timeupdate', updateMetadata)
    media.addEventListener('ended', handleEnded)
    media.addEventListener('error', handleError)

    return () => {
      media.removeEventListener('loadedmetadata', handleLoadedMetadata)
      media.removeEventListener('timeupdate', updateMetadata)
      media.removeEventListener('ended', handleEnded)
      media.removeEventListener('error', handleError)
    }
  }, [selectedFile])

  return (
    <ModulePageLayout
      title="Media Module"
      description="Play local videos, audio files, and test OS media controls."
      icon={PlayCircle}
    >
      <div className="space-y-6">
        {/* File Selection */}
        <div className="grid grid-cols-2 gap-3">
          <Button onClick={handleSelectAudio} disabled={loading} className="gap-2">
            <Music className="w-4 h-4" />
            Select Audio File
          </Button>
          <Button onClick={handleSelectVideo} disabled={loading} className="gap-2">
            <Video className="w-4 h-4" />
            Select Video File
          </Button>
        </div>

        {/* Remote URL Input */}
        <div className="p-4 bg-card border border-border rounded-lg space-y-3">
          <h3 className="text-sm font-semibold flex items-center gap-2">
            <Globe className="w-4 h-4" />
            Load from URL (Remote Media)
          </h3>
          <div className="flex gap-2">
            <div className="flex-1 space-y-2">
              <input
                type="text"
                value={mediaUrl}
                onChange={(e) => setMediaUrl(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleLoadFromUrl()}
                placeholder="https://example.com/audio.mp3 or video.mp4"
                className="w-full px-3 py-2 bg-background border border-border rounded-md text-sm"
              />
              <div className="flex items-center gap-4 text-sm">
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="radio"
                    name="mediaType"
                    value="audio"
                    checked={mediaType === 'audio'}
                    onChange={(e) => setMediaType(e.target.value as 'audio' | 'video')}
                    className="cursor-pointer"
                  />
                  <Music className="w-4 h-4" />
                  Audio
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="radio"
                    name="mediaType"
                    value="video"
                    checked={mediaType === 'video'}
                    onChange={(e) => setMediaType(e.target.value as 'audio' | 'video')}
                    className="cursor-pointer"
                  />
                  <Video className="w-4 h-4" />
                  Video
                </label>
              </div>
            </div>
            <Button onClick={handleLoadFromUrl} className="gap-2 self-start">
              <Upload className="w-4 h-4" />
              Load URL
            </Button>
          </div>
        </div>

        {/* Media Player */}
        {selectedFile && (
          <div className="p-4 bg-card border border-border rounded-lg space-y-4">
            <div className="flex items-center gap-2">
              {selectedFile.type === 'audio' ? (
                <Music className="w-5 h-5 text-blue-500" />
              ) : (
                <Video className="w-5 h-5 text-purple-500" />
              )}
              <div className="flex-1">
                <p className="font-medium">{selectedFile.name}</p>
                <p className="text-sm text-muted-foreground">
                  {selectedFile.type.charAt(0).toUpperCase() + selectedFile.type.slice(1)}
                </p>
              </div>
            </div>

            {/* Audio Player */}
            {selectedFile.type === 'audio' && (
              <div className="space-y-3">
                <audio ref={audioRef} src={selectedFile.url} className="w-full" />

                {/* Progress Bar */}
                <div className="space-y-1">
                  <input
                    type="range"
                    min="0"
                    max={metadata.duration || 0}
                    value={metadata.currentTime}
                    onChange={(e) => handleSeek(parseFloat(e.target.value))}
                    className="w-full"
                  />
                  <div className="flex justify-between text-xs text-muted-foreground">
                    <span>{formatTime(metadata.currentTime)}</span>
                    <span>{formatTime(metadata.duration)}</span>
                  </div>
                </div>
              </div>
            )}

            {/* Video Player */}
            {selectedFile.type === 'video' && (
              <div className="space-y-3">
                <video
                  ref={videoRef}
                  src={selectedFile.url}
                  className="w-full rounded-lg bg-black"
                  controls
                />
              </div>
            )}

            {/* Playback Controls */}
            <div className="flex items-center gap-2">
              <Button onClick={handlePlayPause} size="sm" className="gap-2">
                {metadata.paused ? (
                  <Play className="w-4 h-4" />
                ) : (
                  <Pause className="w-4 h-4" />
                )}
                {metadata.paused ? 'Play' : 'Pause'}
              </Button>
              <Button onClick={handleStop} size="sm" variant="outline" className="gap-2">
                <SkipBack className="w-4 h-4" />
                Stop
              </Button>
              <Button
                onClick={handleToggleMute}
                size="sm"
                variant="outline"
                className="gap-2"
              >
                {muted ? <VolumeX className="w-4 h-4" /> : <Volume2 className="w-4 h-4" />}
              </Button>

              {/* Volume */}
              <div className="flex items-center gap-2 flex-1">
                <span className="text-sm text-muted-foreground">Vol:</span>
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.1"
                  value={metadata.volume}
                  onChange={(e) => handleVolumeChange(parseFloat(e.target.value))}
                  className="flex-1"
                />
                <span className="text-sm w-12">{Math.round(metadata.volume * 100)}%</span>
              </div>
            </div>

            {/* Playback Speed */}
            <div className="flex items-center gap-2">
              <span className="text-sm text-muted-foreground">Speed:</span>
              {[0.5, 0.75, 1, 1.25, 1.5, 2].map((speed) => (
                <Button
                  key={speed}
                  onClick={() => handleSpeedChange(speed)}
                  size="sm"
                  variant={playbackSpeed === speed ? 'default' : 'outline'}
                  className="min-w-16"
                >
                  {speed}x
                </Button>
              ))}
            </div>

            {/* Metadata Display */}
            <div className="grid grid-cols-2 gap-2 p-3 bg-muted rounded-md text-sm">
              <div>
                <span className="text-muted-foreground">Duration:</span>{' '}
                <span className="font-medium">{formatTime(metadata.duration)}</span>
              </div>
              <div>
                <span className="text-muted-foreground">Current:</span>{' '}
                <span className="font-medium">{formatTime(metadata.currentTime)}</span>
              </div>
              <div>
                <span className="text-muted-foreground">Speed:</span>{' '}
                <span className="font-medium">{playbackSpeed}x</span>
              </div>
              <div>
                <span className="text-muted-foreground">Volume:</span>{' '}
                <span className="font-medium">{Math.round(metadata.volume * 100)}%</span>
              </div>
            </div>
          </div>
        )}

        {/* Playlist */}
        {playlist.length > 0 && (
          <div className="p-4 bg-card border border-border rounded-lg">
            <div className="flex items-center justify-between mb-3">
              <h3 className="text-lg font-semibold flex items-center gap-2">
                <Upload className="w-5 h-5" />
                Playlist ({playlist.length})
              </h3>
              <Button
                onClick={handleClearPlaylist}
                size="sm"
                variant="destructive"
                className="gap-2"
              >
                <Trash2 className="w-4 h-4" />
                Clear
              </Button>
            </div>
            <div className="space-y-2">
              {playlist.map((file, index) => (
                <div
                  key={index}
                  onClick={() => handlePlaylistSelect(file)}
                  className={`flex items-center gap-2 p-3 rounded cursor-pointer transition-colors ${
                    selectedFile?.path === file.path
                      ? 'bg-primary/10 border border-primary'
                      : 'bg-muted hover:bg-muted/80'
                  }`}
                >
                  {file.type === 'audio' ? (
                    <Music className="w-4 h-4 text-blue-500" />
                  ) : (
                    <Video className="w-4 h-4 text-purple-500" />
                  )}
                  <div className="flex-1">
                    <p className="text-sm font-medium">{file.name}</p>
                    <p className="text-xs text-muted-foreground">{file.type}</p>
                  </div>
                  {selectedFile?.path === file.path && (
                    <span className="text-xs text-primary font-medium">Now Playing</span>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Output Panel */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold flex items-center gap-2">
              <Clock className="w-5 h-5" />
              Event Log
            </h3>
            <Button onClick={() => setOutput('')} variant="outline" size="sm">
              Clear
            </Button>
          </div>
          <pre className="p-4 bg-muted border border-border rounded-lg text-sm overflow-auto max-h-64 whitespace-pre-wrap">
            {output || 'No events yet. Select and play a media file to see events.'}
          </pre>
        </div>
      </div>
    </ModulePageLayout>
  )
}
