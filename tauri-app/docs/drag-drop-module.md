# Drag & Drop Module Implementation

## Overview

Test Tauri's native file-drop and HTML5 drag & drop functionality. This module demonstrates both native Tauri file drop events and HTML5 drag and drop API for maximum flexibility across platforms.

## Plugin Setup

### Install Dependencies

No additional plugins required - drag & drop is built into Tauri core.

### Cargo Dependencies

No additional Cargo dependencies required.

### Register Plugin

No plugin registration needed - drag & drop is a core Tauri feature.

## Permissions Configuration

### Required Permissions

Add to `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "core:event:default",
    "core:window:default"
  ]
}
```

### Window Configuration

Enable drag & drop in `src-tauri/tauri.conf.json`:

```json
{
  "app": {
    "windows": [
      {
        "title": "tauri-app",
        "width": 800,
        "height": 600,
        "dragDropEnabled": true
      }
    ]
  }
}
```

## Core Features

- [x] Native Tauri file drop events
- [x] HTML5 drag and drop API
- [x] Large drop zone UI
- [ ] File type validation (optional enhancement)
- [x] Multiple file drop support
- [x] Visual drag-over feedback
- [x] Dropped file list display
- [x] File metadata display (name, size, type)
- [x] Toggle between native and HTML5 modes
- [x] Clear dropped files
- [ ] File preview (images) (optional enhancement)
- [ ] Drag and drop reordering (HTML5) (optional enhancement)

## Frontend Implementation

### Native Tauri File Drop

```typescript
import { listen } from '@tauri-apps/api/event'

// Listen for file drop events
const unlisten = await listen('tauri://file-drop', (event) => {
  const paths = event.payload.paths as string[]
  console.log('Files dropped:', paths)
})

// Listen for drag-over events
await listen('tauri://file-drop-hover', (event) => {
  const paths = event.payload.paths as string[]
  console.log('Files hovering:', paths)
})

// Listen for drag-leave events
await listen('tauri://file-drop-cancelled', () => {
  console.log('Drag cancelled')
})

// Clean up
unlisten()
```

### HTML5 Drag & Drop

```typescript
const handleDragOver = (e: React.DragEvent) => {
  e.preventDefault()
  e.stopPropagation()
}

const handleDrop = (e: React.DragEvent) => {
  e.preventDefault()
  e.stopPropagation()

  const files = Array.from(e.dataTransfer.files)
  console.log('Files dropped (HTML5):', files)
}

<div
  onDragOver={handleDragOver}
  onDrop={handleDrop}
  className="drop-zone"
>
  Drop files here
</div>
```

### File Metadata Access

```typescript
interface DroppedFile {
  path?: string        // Native Tauri (full path)
  name: string         // File name
  size: number         // File size in bytes
  type: string         // MIME type
  lastModified: number // Timestamp
}

// From HTML5 File API
const file: File = e.dataTransfer.files[0]
const metadata: DroppedFile = {
  name: file.name,
  size: file.size,
  type: file.type,
  lastModified: file.lastModified
}

// From Tauri (path only, need to read metadata separately)
const path: string = event.payload.paths[0]
```

## UI Components

### Drop Zone Section
- Large visual drop area
- Drag-over state indication
- "Drop files here" placeholder text
- Visual feedback during drag operation
- Support for multiple file drop

### Mode Toggle Section
- Radio buttons: Native vs HTML5
- Display current mode
- Explanation of differences

### Dropped Files List
- File name display
- File size (formatted)
- File type/MIME type
- File path (native mode only)
- Remove individual file button
- Clear all button

### File Preview Section (Optional)
- Image preview for image files
- Thumbnail grid
- Click to view full size

### Output Panel
- Event log with timestamps
- Drag state changes
- Files dropped count
- Error messages

## Testing Checklist

### Desktop Testing
- [ ] Windows - Native file drop
- [ ] Windows - HTML5 drag & drop
- [ ] macOS - Native file drop
- [ ] macOS - HTML5 drag & drop
- [ ] Linux - Native file drop
- [ ] Linux - HTML5 drag & drop

### Mobile Testing
- [ ] Android - File drop (if supported)
- [ ] iOS - File drop (if supported)

### Feature Testing
- [ ] Single file drop
- [ ] Multiple file drop
- [ ] Different file types (images, documents, archives)
- [ ] Large files (>100MB)
- [ ] Drag-over visual feedback
- [ ] Drag-leave behavior
- [ ] File validation/filtering
- [ ] Drop zone boundaries

### Edge Cases
- [ ] Drop on disabled drop zone
- [ ] Drop non-file items
- [ ] Drop from external applications
- [ ] Drop from browser tabs
- [ ] Drag cancel behavior
- [ ] Memory handling with many files

## Progress Tracking

### Setup Phase
- [x] Verify dragDropEnabled in tauri.conf.json
- [x] Configure window permissions in capabilities/default.json
- [x] Add route to navigation (already in home page)

### Development Phase
- [x] Implement native Tauri file drop listener
- [x] Implement HTML5 drag & drop handlers
- [x] Create drop zone UI component
- [x] Add visual drag-over feedback
- [x] Implement file list display
- [x] Add file metadata display (name, size, type)
- [x] Create mode toggle (Native vs HTML5)
- [x] Implement clear functionality
- [x] Add event logging to output panel

### Testing Phase
- [ ] Test native file drop on macOS
- [ ] Test HTML5 drag & drop
- [ ] Verify visual feedback
- [ ] Test with multiple files
- [ ] Test with different file types

### Polish Phase
- [ ] Improve UI/UX
- [ ] Add file previews for images
- [ ] Code cleanup and documentation

## Implementation Status

**Status**: ✅ Implemented - Ready for Testing

### Backend Configuration
- [x] Route: Created at `/drag-drop`
- [x] Component: Fully implemented at `src/routes/drag-drop.tsx`
- [x] Permissions: Configured in capabilities/default.json (core:event:default, core:window:default)
- [x] Window config: dragDropEnabled is set to true in tauri.conf.json:17

### Frontend Implementation
- [x] Native file drop: Implemented with Tauri event listeners
- [x] HTML5 drag & drop: Implemented with React drag handlers
- [x] Drop zone UI: Implemented with visual feedback
- [x] File list: Implemented with file cards
- [x] Mode toggle: Implemented (Native vs HTML5)
- [x] File metadata display: Implemented (name, size, type, path, timestamp)
- [x] Visual feedback: Implemented (drag hover states, animations)
- [x] Event logging: Implemented with timestamped log panel
- [x] Clear functionality: Implemented (clear all files, clear log, remove individual files)

### Features Implemented

#### Mode Toggle
- Switch between Native Tauri and HTML5 modes
- Visual indicators showing current mode
- Real-time listener status for Native mode

#### Drop Zone
- Large visual drop area with hover effects
- Animated scale and color changes on drag-over
- Clear instructions showing current mode
- Support for multiple file drops

#### File Management
- Display dropped files with metadata
- Native mode: Shows full file paths
- HTML5 mode: Shows size, type, and last modified date
- Individual file removal
- Clear all functionality
- File count display

#### Event Logging
- Timestamped event log
- Success/error indicators
- Mode switch logging
- Drag hover/cancel events
- Clear log functionality
- Auto-scrolling log panel

#### UI Components
- Mode toggle buttons (Native/HTML5)
- Animated drop zone with visual feedback
- File list with individual cards
- Metadata display per file
- Event log with timestamps
- Info section explaining mode differences

### Testing Results
- [ ] Desktop (macOS): Pending
- [ ] Desktop (Windows): Pending
- [ ] Desktop (Linux): Pending
- [ ] HTML5 mode: Pending
- [ ] Native mode: Pending
- [ ] Mobile: Not applicable (limited drag & drop support)

## Known Limitations

- Native file drop provides full file paths (desktop only)
- HTML5 drag & drop works in browsers but may have limited access to file paths
- Mobile platforms have limited drag & drop support
- Some file types may not be droppable (system files, protected files)
- Drag & drop from certain applications may be restricted by OS security
- File path access differs between native and HTML5 modes

## Differences: Native vs HTML5

### Native Tauri File Drop
**Pros:**
- Full file system paths
- Better desktop integration
- No browser restrictions

**Cons:**
- Desktop-only
- Limited to files (no custom data)

### HTML5 Drag & Drop
**Pros:**
- Cross-platform (web-compatible)
- Can handle custom data types
- More flexible for in-app reordering

**Cons:**
- Limited file path access (security)
- Browser-dependent behavior
- More complex API

## Resources

- [Tauri Events API](https://tauri.app/develop/calling-frontend/#events)
- [HTML5 Drag and Drop API - MDN](https://developer.mozilla.org/en-US/docs/Web/API/HTML_Drag_and_Drop_API)
- [File API - MDN](https://developer.mozilla.org/en-US/docs/Web/API/File)
- [Tauri Window Configuration](https://tauri.app/reference/config/#windowconfig)
