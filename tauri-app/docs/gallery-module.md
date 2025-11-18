# Gallery / Media Library Module Implementation

## Overview

Access device photo library and gallery to pick single or multiple images and videos from device storage with preview capabilities.

## Current Implementation Status

⚠️ **Planned** - Requires custom plugin development for mobile platforms

## Plugin Setup

### Desktop File Picker

```bash
bun add @tauri-apps/plugin-dialog
```

```toml
# Add to src-tauri/Cargo.toml
[dependencies]
tauri-plugin-dialog = "2.0"
```

### Mobile Custom Plugin

For mobile platforms (Android/iOS), custom native plugins are required:

**Android:**
- Native Intent: `ACTION_PICK` / `ACTION_GET_CONTENT`
- MediaStore API for gallery access
- Permissions: `READ_MEDIA_IMAGES`, `READ_MEDIA_VIDEO`

**iOS:**
- PHPickerViewController for photo selection
- UIImagePickerController (legacy support)
- Permissions: Photo Library access

### File Path Conversion

```bash
bun add @tauri-apps/api
```

Use `convertFileSrc()` to convert file paths to WebView-compatible URLs.

## Core Features

- [ ] Pick single image
- [ ] Pick single video
- [ ] Pick multiple media items
- [ ] Display thumbnail grid
- [ ] Full-screen preview
- [ ] File metadata (size, type, dimensions)
- [ ] Clear selection
- [ ] Delete selected items from view

## Data Structures

### Media Item Schema
```typescript
interface MediaItem {
  id: string
  uri: string
  type: 'image' | 'video'
  name: string
  size: number
  mimeType: string
  dimensions?: {
    width: number
    height: number
  }
  thumbnail?: string
  timestamp: string
}
```

### Picker Options Schema
```typescript
interface PickerOptions {
  mediaType: 'image' | 'video' | 'all'
  allowMultiple: boolean
  maxSelection?: number
  quality?: number // 0-100 for compression
}
```

### Picker Response Schema
```typescript
interface PickerResponse {
  success: boolean
  items: MediaItem[]
  error?: string
}
```

## Rust Backend

### Desktop Implementation (Dialog Plugin)

#### 1. Pick Single Image
```rust
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
async fn pick_image(app: tauri::AppHandle) -> Result<String, String> {
    let file = app.dialog()
        .file()
        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp"])
        .blocking_pick_file()
        .ok_or("No file selected")?;

    Ok(file.path().to_string_lossy().to_string())
}
```

#### 2. Pick Single Video
```rust
#[tauri::command]
async fn pick_video(app: tauri::AppHandle) -> Result<String, String> {
    let file = app.dialog()
        .file()
        .add_filter("Videos", &["mp4", "mov", "avi", "mkv", "webm"])
        .blocking_pick_file()
        .ok_or("No file selected")?;

    Ok(file.path().to_string_lossy().to_string())
}
```

#### 3. Pick Multiple Media
```rust
#[tauri::command]
async fn pick_multiple_media(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let files = app.dialog()
        .file()
        .add_filter("Media", &["png", "jpg", "jpeg", "gif", "webp", "mp4", "mov", "avi", "mkv"])
        .blocking_pick_files()
        .ok_or("No files selected")?;

    let paths: Vec<String> = files
        .iter()
        .map(|f| f.path().to_string_lossy().to_string())
        .collect();

    Ok(paths)
}
```

#### 4. Get File Metadata
```rust
use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
struct FileMetadata {
    name: String,
    size: u64,
    extension: String,
}

#[tauri::command]
async fn get_file_metadata(file_path: String) -> Result<FileMetadata, String> {
    let path = Path::new(&file_path);
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;

    Ok(FileMetadata {
        name: path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        size: metadata.len(),
        extension: path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    })
}
```

### Mobile Implementation (Custom Plugin Required)

#### Android: Gallery Picker
```kotlin
// Custom Tauri plugin for Android
class GalleryPlugin {
    @Command
    fun pickImage(invoke: Invoke) {
        val intent = Intent(Intent.ACTION_PICK).apply {
            type = "image/*"
        }
        startActivityForResult(intent, PICK_IMAGE_REQUEST)
    }

    @Command
    fun pickVideo(invoke: Invoke) {
        val intent = Intent(Intent.ACTION_PICK).apply {
            type = "video/*"
        }
        startActivityForResult(intent, PICK_VIDEO_REQUEST)
    }

    @Command
    fun pickMultipleMedia(invoke: Invoke) {
        val intent = Intent(Intent.ACTION_GET_CONTENT).apply {
            type = "*/*"
            putExtra(Intent.EXTRA_MIME_TYPES, arrayOf("image/*", "video/*"))
            putExtra(Intent.EXTRA_ALLOW_MULTIPLE, true)
        }
        startActivityForResult(intent, PICK_MULTIPLE_REQUEST)
    }
}
```

#### iOS: Photo Picker
```swift
// Custom Tauri plugin for iOS
import PhotosUI

class GalleryPlugin {
    @objc func pickImage(_ invoke: Invoke) {
        var config = PHPickerConfiguration()
        config.filter = .images
        config.selectionLimit = 1

        let picker = PHPickerViewController(configuration: config)
        picker.delegate = self
        present(picker, animated: true)
    }

    @objc func pickVideo(_ invoke: Invoke) {
        var config = PHPickerConfiguration()
        config.filter = .videos
        config.selectionLimit = 1

        let picker = PHPickerViewController(configuration: config)
        picker.delegate = self
        present(picker, animated: true)
    }

    @objc func pickMultipleMedia(_ invoke: Invoke) {
        var config = PHPickerConfiguration()
        config.filter = .any(of: [.images, .videos])
        config.selectionLimit = 0 // 0 = unlimited

        let picker = PHPickerViewController(configuration: config)
        picker.delegate = self
        present(picker, animated: true)
    }
}
```

## Frontend Integration

### React Component Structure

```typescript
import { invoke } from '@tauri-apps/api/core'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useState } from 'react'

interface MediaItem {
  id: string
  uri: string
  type: 'image' | 'video'
  name: string
  webviewUrl: string
}

// Pick Single Image
const [selectedImage, setSelectedImage] = useState<MediaItem | null>(null)

const handlePickImage = async () => {
  try {
    const filePath = await invoke<string>('pick_image')
    const webviewUrl = convertFileSrc(filePath)

    setSelectedImage({
      id: Date.now().toString(),
      uri: filePath,
      type: 'image',
      name: filePath.split('/').pop() || 'image',
      webviewUrl
    })
  } catch (error) {
    console.error('Failed to pick image:', error)
  }
}

// Pick Multiple Media
const [mediaItems, setMediaItems] = useState<MediaItem[]>([])

const handlePickMultiple = async () => {
  try {
    const filePaths = await invoke<string[]>('pick_multiple_media')

    const items = filePaths.map(path => ({
      id: Date.now().toString() + Math.random(),
      uri: path,
      type: path.match(/\.(mp4|mov|avi|mkv)$/i) ? 'video' : 'image',
      name: path.split('/').pop() || 'media',
      webviewUrl: convertFileSrc(path)
    }))

    setMediaItems(items)
  } catch (error) {
    console.error('Failed to pick media:', error)
  }
}

// Display Image
<img
  src={selectedImage?.webviewUrl}
  alt={selectedImage?.name}
  className="w-full h-auto"
/>

// Display Video
<video
  src={videoItem?.webviewUrl}
  controls
  className="w-full h-auto"
/>

// Thumbnail Grid
{mediaItems.map(item => (
  <div key={item.id} className="relative">
    {item.type === 'image' ? (
      <img src={item.webviewUrl} alt={item.name} />
    ) : (
      <video src={item.webviewUrl} />
    )}
  </div>
))}
```

## Security Best Practices

### File Access Security
- ✅ Validate file paths before processing
- ✅ Check file types and extensions
- ✅ Limit file sizes to prevent memory issues
- ✅ Sanitize file names
- ✅ Use secure file path conversion (convertFileSrc)
- ✅ Implement file access permissions properly

### Privacy Considerations
- ✅ Request minimal permissions necessary
- ✅ Clear explanation for permission requests
- ✅ Respect user's privacy choices
- ✅ Don't store sensitive media without consent
- ✅ Implement secure deletion

### Mobile Permissions
- ✅ Request permissions at appropriate times
- ✅ Handle permission denials gracefully
- ✅ Provide fallback options
- ✅ Follow platform-specific permission guidelines

## Permissions Configuration

### Android Permissions

Add to `src-tauri/gen/android/app/src/main/AndroidManifest.xml`:

```xml
<uses-permission android:name="android.permission.READ_MEDIA_IMAGES" />
<uses-permission android:name="android.permission.READ_MEDIA_VIDEO" />
<!-- For Android 12 and below -->
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE"
                 android:maxSdkVersion="32" />
```

### iOS Permissions

Add to `src-tauri/gen/apple/Info.plist`:

```xml
<key>NSPhotoLibraryUsageDescription</key>
<string>This app needs access to your photo library to select images and videos</string>
<key>NSPhotoLibraryAddUsageDescription</key>
<string>This app needs permission to save images to your photo library</string>
```

### Tauri Permissions

Add to `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "dialog:allow-open",
    "dialog:allow-save",
    "core:path:allow-resolve",
    "core:path:allow-normalize"
  ]
}
```

## Common Use Cases

### 1. Profile Picture Upload
```typescript
const selectProfilePicture = async () => {
  const filePath = await invoke<string>('pick_image')
  const webviewUrl = convertFileSrc(filePath)
  // Upload to server or save locally
}
```

### 2. Photo Gallery Browser
```typescript
const browseGallery = async () => {
  const paths = await invoke<string[]>('pick_multiple_media')
  const gallery = paths.map(path => ({
    url: convertFileSrc(path),
    thumbnail: convertFileSrc(path) // Generate thumbnails as needed
  }))
  setGalleryItems(gallery)
}
```

### 3. Video Selection for Editing
```typescript
const selectVideoForEdit = async () => {
  const videoPath = await invoke<string>('pick_video')
  const metadata = await invoke('get_file_metadata', { filePath: videoPath })
  // Load video into editor
}
```

### 4. Batch Image Processing
```typescript
const processImages = async () => {
  const imagePaths = await invoke<string[]>('pick_multiple_media')
  for (const path of imagePaths) {
    // Process each image
    await processImage(path)
  }
}
```

## Error Handling

### File Picker Errors
```typescript
const safePickImage = async () => {
  try {
    const filePath = await invoke<string>('pick_image')
    return filePath
  } catch (error) {
    if (error === 'No file selected') {
      console.log('User cancelled selection')
      return null
    }
    console.error('Failed to pick image:', error)
    throw error
  }
}
```

### Permission Errors
```typescript
const handlePermissionError = (error: string) => {
  if (error.includes('permission')) {
    alert('Please grant gallery access in settings')
    // Optionally open settings
  }
}
```

### File Access Errors
```rust
#[tauri::command]
async fn safe_pick_image(app: tauri::AppHandle) -> Result<String, String> {
    let file = app.dialog()
        .file()
        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp"])
        .blocking_pick_file()
        .ok_or("No file selected".to_string())?;

    let path = file.path();

    // Verify file exists and is readable
    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    if !path.is_file() {
        return Err("Path is not a file".to_string());
    }

    Ok(path.to_string_lossy().to_string())
}
```

## Performance Optimization

### Thumbnail Generation
```rust
use image::imageops::FilterType;

#[tauri::command]
async fn generate_thumbnail(
    file_path: String,
    max_size: u32,
) -> Result<Vec<u8>, String> {
    let img = image::open(&file_path)
        .map_err(|e| e.to_string())?;

    let thumbnail = img.resize(max_size, max_size, FilterType::Lanczos3);

    let mut buffer = Vec::new();
    thumbnail
        .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Jpeg)
        .map_err(|e| e.to_string())?;

    Ok(buffer)
}
```

### Lazy Loading for Large Galleries
```typescript
const [visibleItems, setVisibleItems] = useState<MediaItem[]>([])
const [page, setPage] = useState(0)
const itemsPerPage = 20

useEffect(() => {
  const start = page * itemsPerPage
  const end = start + itemsPerPage
  setVisibleItems(allMediaItems.slice(start, end))
}, [page, allMediaItems])
```

### Memory Management
```typescript
// Clean up object URLs when component unmounts
useEffect(() => {
  return () => {
    mediaItems.forEach(item => {
      if (item.objectUrl) {
        URL.revokeObjectURL(item.objectUrl)
      }
    })
  }
}, [mediaItems])
```

## Troubleshooting

### Common Issues

**File picker doesn't open**
- Check permissions configuration
- Verify dialog plugin is installed
- Check console for error messages
- Ensure app has necessary capabilities

**Selected images don't display**
- Use `convertFileSrc()` for file paths
- Verify file path is correct
- Check file format is supported
- Ensure file still exists at path

**Permission denied errors (Mobile)**
- Request permissions before accessing gallery
- Check AndroidManifest.xml / Info.plist configuration
- Guide user to app settings if needed
- Handle permission denial gracefully

**Memory issues with large galleries**
- Implement lazy loading
- Generate and use thumbnails
- Limit selection count
- Clean up unused resources

## Resources

### Official Documentation
- [Tauri Dialog Plugin](https://v2.tauri.app/plugin/dialog/)
- [Android Photo Picker](https://developer.android.com/training/data-storage/shared/photopicker)
- [iOS PHPickerViewController](https://developer.apple.com/documentation/photokit/phpickerviewcontroller)
- [convertFileSrc API](https://v2.tauri.app/reference/javascript/api/namespacecore/#convertfilesrc)

### Libraries & Tools
- [image-rs](https://github.com/image-rs/image) - Image processing in Rust
- [react-dropzone](https://react-dropzone.js.org/) - File drop zone (desktop)
- [MediaStore API](https://developer.android.com/reference/android/provider/MediaStore) - Android media access

## Platform Support

| Feature | Windows | macOS | Linux | iOS | Android |
|---------|---------|-------|-------|-----|---------|
| Pick Image | ✅ | ✅ | ✅ | 🔶* | 🔶* |
| Pick Video | ✅ | ✅ | ✅ | 🔶* | 🔶* |
| Pick Multiple | ✅ | ✅ | ✅ | 🔶* | 🔶* |
| File Metadata | ✅ | ✅ | ✅ | ✅ | ✅ |
| Thumbnails | ✅ | ✅ | ✅ | ✅ | ✅ |

*🔶 = Requires custom plugin development*

## Implementation Status

### Backend
- [ ] Desktop file picker commands
- [ ] File metadata extraction
- [ ] Thumbnail generation
- [ ] Android custom plugin
- [ ] iOS custom plugin
- [ ] File validation

### Frontend
- [ ] Image picker UI
- [ ] Video picker UI
- [ ] Multiple selection UI
- [ ] Thumbnail grid display
- [ ] Full-screen preview
- [ ] File information display
- [ ] Error handling
- [ ] Loading states

---

**Last Updated**: November 2025
**Module Version**: 1.0.0
**Status**: Documentation Complete ✅
