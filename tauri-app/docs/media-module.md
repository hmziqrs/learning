# Media Module (Audio + Video) Implementation

## Overview

Play local videos, audio files, and test OS media controls. This module demonstrates audio/video playback capabilities using HTML5 media elements with Tauri's file system integration.

## Plugin Setup

### Install Dependencies

```bash
# Optional: community media plugins (not required for basic playback)
# bun add tauri-plugin-media
# bun add tauri-plugin-videoplayer
```

### Cargo Dependencies

No additional Cargo dependencies required for basic media playback.

### Register Plugin

For basic HTML5 media playback, no plugin registration needed. Media files can be accessed via filesystem plugin.

## Permissions Configuration

### Required Permissions

Add to `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "core:webview:allow-internal-toggle-devtools",
    "fs:allow-read",
    "fs:default"
  ]
}
```

### Scope Configuration

Configure file access in `src-tauri/tauri.conf.json`:

```json
{
  "bundle": {
    "resources": ["assets/*"]
  },
  "fs": {
    "scope": [
      "$APPDATA/*",
      "$APPLOCALDATA/*",
      "$RESOURCE/*"
    ]
  }
}
```

## Core Features

- [x] File picker for audio/video selection
- [x] Remote URL media playback (HTTP/HTTPS)
- [x] Audio playback with controls (play/pause/seek)
- [x] Video playback with controls
- [x] Metadata display (duration/current time)
- [x] Volume control
- [x] Playback speed control
- [x] Playlist management
- [x] Support for common formats (MP3, MP4, OGG, WebM, WAV)
- [ ] Media session API integration (optional)
- [ ] Native media controls integration (optional)

## Frontend Implementation

### Using HTML5 Media Elements

```typescript
import { convertFileSrc } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

// File picker
const filePath = await open({
  filters: [{
    name: 'Media',
    extensions: ['mp3', 'mp4', 'ogg', 'webm', 'wav', 'avi', 'mov']
  }]
})

// Convert file path to WebView-compatible URL
const mediaUrl = convertFileSrc(filePath)

// Use with HTML5 media elements
<audio src={mediaUrl} controls />
<video src={mediaUrl} controls />
```

### Remote Media Playback

```typescript
// Direct URL loading (no conversion needed)
const remoteUrl = 'https://example.com/audio.mp3'

// Use directly with HTML5 media elements
<audio src={remoteUrl} controls />
<video src={remoteUrl} controls />
```

**Supported Protocols:**
- HTTP/HTTPS URLs
- Direct media file URLs
- Streaming URLs (HLS, DASH if supported by browser)

**CORS Considerations:**
- Remote servers must allow CORS for cross-origin playback
- Some streaming services may require authentication or API keys
- Local files require `convertFileSrc()`, remote URLs do not

### Media Metadata Access

```typescript
const audioElement = document.querySelector('audio')

audioElement.addEventListener('loadedmetadata', () => {
  console.log('Duration:', audioElement.duration)
  console.log('Current Time:', audioElement.currentTime)
})
```

### Media Session API (Optional)

```typescript
// Set metadata for OS media controls
if ('mediaSession' in navigator) {
  navigator.mediaSession.metadata = new MediaMetadata({
    title: 'Track Title',
    artist: 'Artist Name',
    album: 'Album Name',
    artwork: [
      { src: 'cover.jpg', sizes: '512x512', type: 'image/jpeg' }
    ]
  })

  // Handle media controls
  navigator.mediaSession.setActionHandler('play', () => {
    audioElement.play()
  })

  navigator.mediaSession.setActionHandler('pause', () => {
    audioElement.pause()
  })
}
```

## UI Components

### File Picker Section
- Button: "Select Audio File"
- Button: "Select Video File"
- Display: Selected file name and path

### Remote URL Section
- Input: URL text field with validation
- Radio: Media type selection (Audio/Video)
- Button: "Load URL"
- Support: Enter key to load
- Validation: URL format checking

### Audio Player Section
- HTML5 audio element with controls
- Display: Current track metadata
- Slider: Volume control
- Dropdown: Playback speed (0.5x, 1x, 1.5x, 2x)
- Button: Stop playback

### Video Player Section
- HTML5 video element with controls
- Display: Video metadata (duration, dimensions)
- Fullscreen button
- Picture-in-picture button (if supported)

### Playlist Section (Optional)
- List: Recently played files
- Button: Clear history
- Button: Add to queue

### Output Panel
- Log of media events (play, pause, ended, error)
- Display: Current playback status
- Display: File format information

## Testing Checklist

### Desktop Testing
- [ ] Windows - Audio playback
- [ ] Windows - Video playback
- [ ] macOS - Audio playback
- [ ] macOS - Video playback
- [ ] Linux - Audio playback
- [ ] Linux - Video playback

### Mobile Testing
- [ ] Android - Audio playback
- [ ] Android - Video playback
- [ ] iOS - Audio playback
- [ ] iOS - Video playback

### Format Testing
- [ ] MP3 audio format
- [ ] OGG audio format
- [ ] WAV audio format
- [ ] MP4 video format
- [ ] WebM video format
- [ ] Common codec variations

### Edge Cases
- [ ] Large file handling (>100MB)
- [ ] Corrupted file error handling
- [ ] Unsupported format error handling
- [ ] Playback interruption (incoming calls on mobile)
- [ ] Background playback behavior
- [ ] Multiple media elements
- [ ] Network-mounted files (if applicable)

## Progress Tracking

### Setup Phase
- [x] Review plugin options (HTML5 vs native)
- [x] Configure file system permissions
- [x] Add dialog plugin for file picker
- [x] Test file path conversion

### Development Phase
- [x] Implement file picker UI
- [x] Create audio player component
- [x] Create video player component
- [x] Add metadata display
- [x] Implement playback controls
- [x] Add volume control
- [x] Add playback speed control
- [x] Implement playlist/history
- [x] Add error handling
- [ ] Integrate media session API (optional)

### Testing Phase
- [ ] Test all audio formats
- [ ] Test all video formats
- [ ] Test on desktop platforms
- [ ] Test on mobile platforms
- [ ] Verify error handling
- [ ] Test performance with large files
- [ ] Verify media controls integration

### Polish Phase
- [ ] Improve UI/UX
- [ ] Add loading states
- [ ] Add progress indicators
- [ ] Implement keyboard shortcuts
- [ ] Add accessibility features
- [ ] Optimize performance
- [ ] Document known limitations

## Implementation Status

**Status**: ✅ Implemented (Ready for Testing)

### Backend Configuration
- Route: ✅ Active at `/media`
- Component: ✅ Fully implemented in `src/routes/media.tsx`
- Dialog Plugin: ✅ Registered in `src-tauri/src/lib.rs:37`
- Permissions: ✅ Configured in `src-tauri/capabilities/default.json`
- Cargo Dependency: ✅ Added to `src-tauri/Cargo.toml`

### Frontend Implementation
- File picker: ✅ Implemented with audio/video filters
- Audio player: ✅ Implemented with custom controls and progress bar
- Video player: ✅ Implemented with native HTML5 controls
- Media controls: ✅ Play/Pause/Stop/Volume/Mute/Speed controls
- Metadata display: ✅ Duration, current time, speed, volume
- Playlist management: ✅ History and selection UI
- Event logging: ✅ Real-time event log with timestamps
- Error handling: ✅ Try-catch blocks with user feedback

### Features Implemented
- ✅ Audio file selection (MP3, WAV, OGG, FLAC, M4A, AAC)
- ✅ Video file selection (MP4, WebM, OGG, MOV, AVI, MKV)
- ✅ Remote URL playback (HTTP/HTTPS)
- ✅ URL validation and error handling
- ✅ Media type selection for remote URLs (Audio/Video)
- ✅ File path to WebView URL conversion via `convertFileSrc()`
- ✅ Playback controls (Play, Pause, Stop)
- ✅ Volume control with slider and mute toggle
- ✅ Playback speed options (0.5x, 0.75x, 1x, 1.25x, 1.5x, 2x)
- ✅ Progress bar with seek functionality (audio only)
- ✅ Real-time metadata updates
- ✅ Playlist/history with click-to-play
- ✅ Event logging with timestamps
- ✅ Format detection from file extension
- ✅ Keyboard shortcuts (Enter to load URL)

### Testing Results
- Desktop: ⏳ Ready for testing (build successful)
- Mobile: ⏳ Not yet tested

## Known Limitations

- HTML5 media support depends on browser/WebView codecs
- Some formats may not work on all platforms
- DRM-protected content not supported
- Background audio playback requires additional platform-specific configuration
- Media session API support varies by platform
- Remote media requires CORS headers from the server
- Some streaming protocols (HLS, DASH) may have limited support depending on platform
- Network errors for remote media depend on server availability

## Resources

- [HTML5 Audio API - MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio)
- [HTML5 Video API - MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video)
- [Media Session API - MDN](https://developer.mozilla.org/en-US/docs/Web/API/Media_Session_API)
- [Tauri convertFileSrc](https://tauri.app/develop/calling-frontend/#accessing-files-from-the-frontend)
- [Tauri Dialog Plugin](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/dialog)

## Testing URLs

Example public media URLs for testing remote playback:

**Audio:**
- `https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3`
- `https://file-examples.com/wp-content/storage/2017/11/file_example_MP3_700KB.mp3`

**Video:**
- `https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4`
- `https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/360/Big_Buck_Bunny_360_10s_1MB.mp4`

**Note:** These are public test URLs and availability is not guaranteed. Use your own media URLs for production testing.
