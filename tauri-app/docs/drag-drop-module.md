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

- [ ] Native Tauri file drop events
- [ ] HTML5 drag and drop API
- [ ] Large drop zone UI
- [ ] File type validation
- [ ] Multiple file drop support
- [ ] Visual drag-over feedback
- [ ] Dropped file list display
- [ ] File metadata display (name, size, type)
- [ ] Toggle between native and HTML5 modes
- [ ] Clear dropped files
- [ ] File preview (images)
- [ ] Drag and drop reordering (HTML5)

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
- [ ] Implement native Tauri file drop listener
- [ ] Implement HTML5 drag & drop handlers
- [ ] Create drop zone UI component
- [ ] Add visual drag-over feedback
- [ ] Implement file list display
- [ ] Add file metadata display (name, size, type)
- [ ] Create mode toggle (Native vs HTML5)
- [ ] Implement clear functionality
- [ ] Add event logging to output panel

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

**Status**: In Progress

### Backend Configuration
- [x] Route: Created at `/drag-drop`
- [ ] Component: Placeholder exists, needs implementation
- [x] Permissions: Configured in capabilities/default.json (core:event:default, core:window:default)
- [x] Window config: dragDropEnabled is set to true

### Frontend Implementation
- [ ] Native file drop: Not implemented
- [ ] HTML5 drag & drop: Not implemented
- [ ] Drop zone UI: Not implemented
- [ ] File list: Not implemented
- [ ] Mode toggle: Not implemented
- [ ] File metadata display: Not implemented
- [ ] Visual feedback: Not implemented

### Testing Results
- [ ] Desktop: Not tested
- [ ] Mobile: Not tested

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
