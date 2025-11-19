# File Sharing & Social Integration Module

## Overview

Share files, text, and links with other apps using native share dialogs. Integrate with platform-specific social sharing capabilities including share sheets, file providers, and inter-app communication.

## Current Implementation Status

⚠️ **Planned** - Requires platform-specific implementation

## Plugin Setup

### Tauri Plugin Share

The official Tauri plugin for sharing provides cross-platform share functionality:

```bash
cargo add tauri-plugin-share
npm i @tauri-apps/plugin-share
```

**Features:**
- Share text content
- Share files
- Share URLs
- Native share sheet integration
- Platform-specific share dialogs

**Initialization:**
```rust
use tauri_plugin_share::ShareExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_share::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Alternative Approaches

**Web Share API** (Progressive Web Apps):
```typescript
if (navigator.share) {
    await navigator.share({
        title: 'My Title',
        text: 'Check this out!',
        url: 'https://example.com'
    })
}
```

**Custom File Providers** (Advanced):
- Android: Content Provider
- iOS: Share Extension
- Desktop: Custom file handlers

## Permissions Configuration

### Android Manifest

```xml
<!-- No special permissions required for basic sharing -->
<!-- For reading files to share -->
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" android:maxSdkVersion="32" />
<uses-permission android:name="android.permission.READ_MEDIA_IMAGES" />
<uses-permission android:name="android.permission.READ_MEDIA_VIDEO" />
<uses-permission android:name="android.permission.READ_MEDIA_AUDIO" />
```

### iOS Info.plist

```xml
<!-- No special permissions required for basic sharing -->
<!-- For photo sharing -->
<key>NSPhotoLibraryUsageDescription</key>
<string>We need access to share photos from your library</string>
```

### Tauri Capabilities

```json
{
  "permissions": [
    "share:default",
    "share:allow-share",
    "core:path:allow-resolve",
    "core:path:allow-normalize"
  ]
}
```

## Core Features

### Basic Sharing
- [ ] Share plain text
- [ ] Share URLs
- [ ] Share single file
- [ ] Share multiple files
- [ ] Share with specific app
- [ ] Share sheet customization

### File Sharing
- [ ] Share documents
- [ ] Share images
- [ ] Share videos
- [ ] Share audio files
- [ ] Generate shareable file URIs
- [ ] Temporary file cleanup

### Social Integration
- [ ] Quick share to social media
- [ ] Email integration
- [ ] SMS/messaging integration
- [ ] Clipboard copy fallback
- [ ] Share result callback

### Advanced Features
- [ ] Share with custom MIME types
- [ ] File provider implementation
- [ ] Share extensions (iOS)
- [ ] Direct share targets (Android)
- [ ] Activity continuation (iOS)

## Data Structures

### Share Request
```typescript
interface ShareRequest {
  title?: string
  text?: string
  url?: string
  files?: string[]
  mimeType?: string
}
```

### Share Result
```typescript
interface ShareResult {
  success: boolean
  target?: string // App that was shared to (if available)
  error?: string
}
```

### Share Options
```typescript
interface ShareOptions {
  dialogTitle?: string
  excludedApps?: string[] // Package IDs to exclude
  preferredApp?: string   // Suggest specific app
}
```

### File Share Data
```typescript
interface FileShareData {
  filePath: string
  mimeType: string
  displayName?: string
}
```

### Social Share Preset
```typescript
interface SocialSharePreset {
  platform: 'twitter' | 'facebook' | 'instagram' | 'email' | 'sms'
  text?: string
  url?: string
  hashtags?: string[]
  via?: string // Twitter handle
}
```

## Rust Backend

### Tauri Commands

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ShareRequest {
    title: Option<String>,
    text: Option<String>,
    url: Option<String>,
    files: Option<Vec<String>>,
}

#[derive(Serialize)]
struct ShareResult {
    success: bool,
    error: Option<String>,
}

// Share text or URL
#[tauri::command]
async fn share_text(request: ShareRequest) -> Result<ShareResult, String> {
    // Use tauri-plugin-share
    // Platform-specific implementation
    #[cfg(mobile)]
    {
        // Mobile share sheet
        Ok(ShareResult {
            success: true,
            error: None,
        })
    }

    #[cfg(not(mobile))]
    {
        // Desktop: Open share dialog or copy to clipboard
        Err("Sharing not supported on desktop. Use clipboard instead.".to_string())
    }
}

// Share files
#[tauri::command]
async fn share_files(files: Vec<String>, title: Option<String>) -> Result<ShareResult, String> {
    // Validate file paths
    for file_path in &files {
        if !std::path::Path::new(file_path).exists() {
            return Err(format!("File not found: {}", file_path));
        }
    }

    #[cfg(mobile)]
    {
        // Use tauri-plugin-share to share files
        Ok(ShareResult {
            success: true,
            error: None,
        })
    }

    #[cfg(not(mobile))]
    {
        Err("File sharing not supported on desktop".to_string())
    }
}

// Get available share targets (Android only)
#[tauri::command]
async fn get_share_targets() -> Result<Vec<String>, String> {
    #[cfg(target_os = "android")]
    {
        // Query PackageManager for share-capable apps
        Ok(vec![
            "com.twitter.android".to_string(),
            "com.facebook.katana".to_string(),
            "com.instagram.android".to_string(),
        ])
    }

    #[cfg(not(target_os = "android"))]
    {
        Ok(vec![])
    }
}

// Copy to clipboard (fallback)
#[tauri::command]
async fn copy_to_clipboard(text: String) -> Result<(), String> {
    // Use clipboard plugin or system clipboard
    Ok(())
}

// Check if sharing is supported
#[tauri::command]
async fn is_share_supported() -> Result<bool, String> {
    #[cfg(mobile)]
    {
        Ok(true)
    }

    #[cfg(not(mobile))]
    {
        Ok(false)
    }
}
```

### Platform-Specific Implementation

#### Android (Kotlin)

```kotlin
import android.content.Intent
import androidx.core.content.FileProvider
import java.io.File

class SharePlugin {
    @Command
    fun shareText(invoke: Invoke) {
        val title = invoke.getString("title")
        val text = invoke.getString("text")
        val url = invoke.getString("url")

        val shareIntent = Intent().apply {
            action = Intent.ACTION_SEND
            type = "text/plain"
            putExtra(Intent.EXTRA_SUBJECT, title)
            putExtra(Intent.EXTRA_TEXT, text ?: url)
        }

        val chooser = Intent.createChooser(shareIntent, title ?: "Share")
        activity.startActivity(chooser)

        invoke.resolve(mapOf("success" to true))
    }

    @Command
    fun shareFiles(invoke: Invoke) {
        val files = invoke.getArray("files")
        val title = invoke.getString("title")

        val uris = files.map { filePath ->
            val file = File(filePath)
            FileProvider.getUriForFile(
                activity,
                "${activity.packageName}.fileprovider",
                file
            )
        }

        val shareIntent = Intent().apply {
            action = if (uris.size == 1) Intent.ACTION_SEND else Intent.ACTION_SEND_MULTIPLE
            type = getMimeType(files[0])

            if (uris.size == 1) {
                putExtra(Intent.EXTRA_STREAM, uris[0])
            } else {
                putParcelableArrayListExtra(Intent.EXTRA_STREAM, ArrayList(uris))
            }

            putExtra(Intent.EXTRA_SUBJECT, title)
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        }

        val chooser = Intent.createChooser(shareIntent, title ?: "Share")
        activity.startActivity(chooser)

        invoke.resolve(mapOf("success" to true))
    }

    private fun getMimeType(filePath: String): String {
        return when {
            filePath.endsWith(".jpg", true) || filePath.endsWith(".jpeg", true) -> "image/jpeg"
            filePath.endsWith(".png", true) -> "image/png"
            filePath.endsWith(".pdf", true) -> "application/pdf"
            filePath.endsWith(".mp4", true) -> "video/mp4"
            else -> "*/*"
        }
    }
}
```

Add FileProvider to AndroidManifest.xml:
```xml
<provider
    android:name="androidx.core.content.FileProvider"
    android:authorities="${applicationId}.fileprovider"
    android:exported="false"
    android:grantUriPermissions="true">
    <meta-data
        android:name="android.support.FILE_PROVIDER_PATHS"
        android:resource="@xml/file_paths" />
</provider>
```

Create `res/xml/file_paths.xml`:
```xml
<?xml version="1.0" encoding="utf-8"?>
<paths>
    <cache-path name="shared_files" path="." />
    <files-path name="app_files" path="." />
    <external-files-path name="external_files" path="." />
</paths>
```

#### iOS (Swift)

```swift
import UIKit

class SharePlugin {
    @objc func shareText(_ invoke: Invoke) {
        let title = invoke.getString("title")
        let text = invoke.getString("text")
        let url = invoke.getString("url")

        var items: [Any] = []
        if let text = text {
            items.append(text)
        }
        if let urlString = url, let shareURL = URL(string: urlString) {
            items.append(shareURL)
        }

        DispatchQueue.main.async {
            let activityVC = UIActivityViewController(
                activityItems: items,
                applicationActivities: nil
            )

            // For iPad: set source view
            if let popover = activityVC.popoverPresentationController {
                popover.sourceView = self.getViewController()?.view
                popover.sourceRect = CGRect(x: UIScreen.main.bounds.width / 2,
                                          y: UIScreen.main.bounds.height / 2,
                                          width: 0, height: 0)
                popover.permittedArrowDirections = []
            }

            self.getViewController()?.present(activityVC, animated: true) {
                invoke.resolve(["success": true])
            }
        }
    }

    @objc func shareFiles(_ invoke: Invoke) {
        let filePaths = invoke.getArray("files")
        let title = invoke.getString("title")

        var fileURLs: [URL] = []
        for path in filePaths {
            let url = URL(fileURLWithPath: path)
            if FileManager.default.fileExists(atPath: path) {
                fileURLs.append(url)
            }
        }

        guard !fileURLs.isEmpty else {
            invoke.reject("No valid files to share")
            return
        }

        DispatchQueue.main.async {
            let activityVC = UIActivityViewController(
                activityItems: fileURLs,
                applicationActivities: nil
            )

            if let title = title {
                activityVC.setValue(title, forKey: "subject")
            }

            // For iPad
            if let popover = activityVC.popoverPresentationController {
                popover.sourceView = self.getViewController()?.view
                popover.sourceRect = CGRect(x: UIScreen.main.bounds.width / 2,
                                          y: UIScreen.main.bounds.height / 2,
                                          width: 0, height: 0)
                popover.permittedArrowDirections = []
            }

            self.getViewController()?.present(activityVC, animated: true) {
                invoke.resolve(["success": true])
            }
        }
    }

    private func getViewController() -> UIViewController? {
        guard let windowScene = UIApplication.shared.connectedScenes.first as? UIWindowScene,
              let rootVC = windowScene.windows.first?.rootViewController else {
            return nil
        }
        return rootVC
    }
}
```

#### Desktop Implementation

Desktop platforms typically don't have native share sheets. Alternative approaches:

**Option 1: Clipboard Fallback**
```rust
use clipboard::{ClipboardProvider, ClipboardContext};

#[tauri::command]
async fn share_text_desktop(text: String) -> Result<(), String> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| e.to_string())?;

    ctx.set_contents(text)
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

**Option 2: Open Default App**
```rust
#[tauri::command]
async fn share_via_email(email_body: String, subject: String) -> Result<(), String> {
    let mailto = format!("mailto:?subject={}&body={}",
        urlencoding::encode(&subject),
        urlencoding::encode(&email_body)
    );

    open::that(&mailto).map_err(|e| e.to_string())?;
    Ok(())
}
```

## Frontend Implementation

### TypeScript Integration

```typescript
import { invoke } from '@tauri-apps/api/core'

interface ShareRequest {
  title?: string
  text?: string
  url?: string
  files?: string[]
}

interface ShareResult {
  success: boolean
  error?: string
}

// Share text or URL
export async function shareText(request: ShareRequest): Promise<ShareResult> {
  try {
    // Check if Web Share API is available (PWA)
    if (navigator.share && !request.files) {
      await navigator.share({
        title: request.title,
        text: request.text,
        url: request.url,
      })
      return { success: true }
    }

    // Fall back to Tauri command
    const result = await invoke<ShareResult>('share_text', { request })
    return result
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

// Share files
export async function shareFiles(
  files: string[],
  title?: string
): Promise<ShareResult> {
  try {
    const result = await invoke<ShareResult>('share_files', { files, title })
    return result
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

// Check if sharing is supported
export async function isShareSupported(): Promise<boolean> {
  try {
    // Check Web Share API
    if (navigator.share) {
      return true
    }

    // Check Tauri support
    const supported = await invoke<boolean>('is_share_supported')
    return supported
  } catch {
    return false
  }
}

// Copy to clipboard (fallback)
export async function copyToClipboard(text: string): Promise<boolean> {
  try {
    if (navigator.clipboard) {
      await navigator.clipboard.writeText(text)
      return true
    }

    await invoke('copy_to_clipboard', { text })
    return true
  } catch {
    return false
  }
}
```

### React Component Example

```typescript
import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Share2, Copy, Mail } from 'lucide-react'
import { shareText, shareFiles, isShareSupported, copyToClipboard } from '@/lib/share'

export function ShareExample() {
  const [isSupported, setIsSupported] = useState(false)
  const [output, setOutput] = useState<string[]>([])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  useEffect(() => {
    isShareSupported().then(setIsSupported)
  }, [])

  const handleShareText = async () => {
    const result = await shareText({
      title: 'Check this out!',
      text: 'This is shared from my Tauri app',
      url: 'https://tauri.app',
    })

    if (result.success) {
      addOutput('Text shared successfully')
    } else {
      addOutput(`Share failed: ${result.error}`, false)
    }
  }

  const handleShareFile = async () => {
    // Assume we have a file path
    const filePath = '/path/to/file.pdf'

    const result = await shareFiles([filePath], 'My Document')

    if (result.success) {
      addOutput('File shared successfully')
    } else {
      addOutput(`Share failed: ${result.error}`, false)
    }
  }

  const handleCopyLink = async () => {
    const success = await copyToClipboard('https://tauri.app')

    if (success) {
      addOutput('Link copied to clipboard')
    } else {
      addOutput('Failed to copy to clipboard', false)
    }
  }

  return (
    <div className="space-y-6">
      <section className="rounded-lg border p-6 space-y-4">
        <h2 className="text-xl font-semibold flex items-center gap-2">
          <Share2 className="w-5 h-5" />
          Share Content
        </h2>

        <div className="flex flex-wrap gap-2">
          <Button
            onClick={handleShareText}
            disabled={!isSupported}
            variant="outline"
          >
            <Share2 className="w-4 h-4 mr-2" />
            Share Text
          </Button>

          <Button
            onClick={handleShareFile}
            disabled={!isSupported}
            variant="outline"
          >
            <Share2 className="w-4 h-4 mr-2" />
            Share File
          </Button>

          <Button onClick={handleCopyLink} variant="outline">
            <Copy className="w-4 h-4 mr-2" />
            Copy Link
          </Button>
        </div>

        {!isSupported && (
          <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
            <p className="text-sm text-yellow-700 dark:text-yellow-400">
              Native sharing not supported on this platform. Using clipboard fallback.
            </p>
          </div>
        )}
      </section>

      {/* Output Panel */}
      <section className="rounded-lg border p-6 space-y-4">
        <h2 className="text-xl font-semibold">Output</h2>
        <div className="bg-muted rounded-md p-4 h-48 overflow-y-auto font-mono text-sm">
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
    </div>
  )
}
```

## UI Components

### Share Button Component

```typescript
import { Share2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { shareText } from '@/lib/share'

interface ShareButtonProps {
  title?: string
  text?: string
  url?: string
  onSuccess?: () => void
  onError?: (error: string) => void
}

export function ShareButton({ title, text, url, onSuccess, onError }: ShareButtonProps) {
  const handleShare = async () => {
    const result = await shareText({ title, text, url })

    if (result.success) {
      onSuccess?.()
    } else {
      onError?.(result.error || 'Share failed')
    }
  }

  return (
    <Button onClick={handleShare} variant="outline" size="sm">
      <Share2 className="w-4 h-4 mr-2" />
      Share
    </Button>
  )
}
```

### Social Share Buttons

```typescript
import { Facebook, Twitter, Mail, MessageSquare } from 'lucide-react'
import { Button } from '@/components/ui/button'

interface SocialShareProps {
  url: string
  text: string
  title?: string
}

export function SocialShareButtons({ url, text, title }: SocialShareProps) {
  const shareViaTwitter = () => {
    const twitterUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}&url=${encodeURIComponent(url)}`
    window.open(twitterUrl, '_blank')
  }

  const shareViaFacebook = () => {
    const fbUrl = `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(url)}`
    window.open(fbUrl, '_blank')
  }

  const shareViaEmail = () => {
    const mailto = `mailto:?subject=${encodeURIComponent(title || '')}&body=${encodeURIComponent(text + ' ' + url)}`
    window.location.href = mailto
  }

  return (
    <div className="flex gap-2">
      <Button onClick={shareViaTwitter} variant="outline" size="sm">
        <Twitter className="w-4 h-4" />
      </Button>

      <Button onClick={shareViaFacebook} variant="outline" size="sm">
        <Facebook className="w-4 h-4" />
      </Button>

      <Button onClick={shareViaEmail} variant="outline" size="sm">
        <Mail className="w-4 h-4" />
      </Button>
    </div>
  )
}
```

## Security Best Practices

### File Sharing Security
- ✅ Validate file paths before sharing
- ✅ Use FileProvider on Android (never share file:// URIs)
- ✅ Limit file size for sharing
- ✅ Clean up temporary shared files
- ✅ Verify file MIME types
- ✅ Set appropriate file permissions

### Privacy Considerations
- ✅ Request user confirmation before sharing
- ✅ Don't automatically share sensitive data
- ✅ Respect user privacy preferences
- ✅ Clear share history when appropriate
- ✅ Sanitize shared content (remove metadata)
- ✅ Inform users which app will receive shared content

### Content Security
- ✅ Sanitize URLs before sharing
- ✅ Validate text content length
- ✅ Prevent sharing of sensitive file paths
- ✅ Implement rate limiting for share actions
- ✅ Log share activities for audit

## Error Handling

### Common Errors

```typescript
const handleShare = async () => {
  try {
    const result = await shareText({ text: 'Hello' })

    if (!result.success) {
      switch (result.error) {
        case 'User cancelled':
          // User dismissed share sheet
          break
        case 'Not supported':
          // Fall back to clipboard
          await copyToClipboard('Hello')
          break
        default:
          console.error('Share error:', result.error)
      }
    }
  } catch (error) {
    // Handle network errors, file not found, etc.
    console.error('Unexpected error:', error)
  }
}
```

### Platform-Specific Errors

**Android:**
- FileNotFoundException - File doesn't exist or not accessible
- SecurityException - Missing permissions or invalid FileProvider
- ActivityNotFoundException - No app can handle the share intent

**iOS:**
- File not found
- Invalid file URL
- Share sheet dismissed
- App not authorized

**Desktop:**
- Feature not supported
- Clipboard access denied
- Email client not configured

## Performance Optimization

### File Sharing Optimization
- Use file streaming for large files
- Compress images before sharing
- Generate thumbnails for previews
- Cache file URIs
- Batch multiple files efficiently

### Memory Management
- Clean up temporary files after sharing
- Avoid loading entire file into memory
- Use file descriptors instead of full paths when possible
- Implement background sharing for large files

## Troubleshooting

### Share Sheet Not Appearing

**Issue**: Share sheet doesn't show on mobile

**Solutions**:
- Verify plugin initialization
- Check permissions in manifest/Info.plist
- Ensure running on main thread (iOS)
- Validate share intent data
- Check for valid activity context (Android)

### Files Not Sharing

**Issue**: Files fail to share or receiver can't open them

**Solutions**:
- Use FileProvider on Android (not file:// URIs)
- Verify file exists and is readable
- Check correct MIME type
- Ensure FLAG_GRANT_READ_URI_PERMISSION (Android)
- Validate file paths are accessible
- Check file_paths.xml configuration (Android)

### Desktop Sharing Issues

**Issue**: Sharing doesn't work on desktop

**Solutions**:
- Implement clipboard fallback
- Use mailto: for email sharing
- Open browser for social sharing
- Provide manual copy/paste option
- Show appropriate error messages

### Web Share API Not Working

**Issue**: navigator.share() fails

**Solutions**:
- Check HTTPS requirement
- Verify user gesture (must be triggered by user action)
- Ensure supported browser/platform
- Validate share data format
- Handle user cancellation

## Resources

### Official Documentation
- [Tauri Plugin Share](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/share)
- [Web Share API](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share)
- [Android Share Intent](https://developer.android.com/training/sharing/send)
- [iOS UIActivityViewController](https://developer.apple.com/documentation/uikit/uiactivityviewcontroller)
- [Android FileProvider](https://developer.android.com/reference/androidx/core/content/FileProvider)

### Libraries & Tools
- [react-share](https://www.npmjs.com/package/react-share) - Social share buttons for React
- [share-api-polyfill](https://www.npmjs.com/package/share-api-polyfill) - Web Share API polyfill

### Social Media APIs
- [Twitter Web Intent](https://developer.twitter.com/en/docs/twitter-for-websites/tweet-button/overview)
- [Facebook Share Dialog](https://developers.facebook.com/docs/sharing/reference/share-dialog)
- [LinkedIn Share](https://www.linkedin.com/sharing/share-offsite/)

## Platform Support

| Feature | Windows | macOS | Linux | iOS | Android |
|---------|---------|-------|-------|-----|---------|
| **Basic Sharing** |
| Share Text | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| Share URL | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| Share Files | 🔶* | 🔶* | 🔶* | ✅ | ✅ |
| Native Share Sheet | ❌ | ❌ | ❌ | ✅ | ✅ |
| **Advanced Features** |
| Share to Specific App | ❌ | ❌ | ❌ | ✅ | ✅ |
| Share Multiple Files | ❌ | ❌ | ❌ | ✅ | ✅ |
| Custom MIME Types | ❌ | ❌ | ❌ | ✅ | ✅ |
| Share Result Callback | ❌ | ❌ | ❌ | ⚠️** | ⚠️** |
| **Fallback Options** |
| Clipboard Copy | ✅ | ✅ | ✅ | ✅ | ✅ |
| Email Intent | ✅ | ✅ | ✅ | ✅ | ✅ |
| Web Share API | ✅*** | ✅*** | ✅*** | ✅*** | ✅*** |

**Notes:**
- 🔶* Desktop: Use clipboard or system-specific sharing
- ⚠️** Limited callback information on share completion
- ✅*** Web Share API requires HTTPS and user gesture

## Implementation Status

### Backend
- [ ] Tauri plugin share integration
- [ ] Android share intent implementation
- [ ] iOS UIActivityViewController implementation
- [ ] Desktop clipboard fallback
- [ ] File URI generation
- [ ] MIME type detection
- [ ] Share validation
- [ ] Error handling

### Frontend
- [ ] Share text function
- [ ] Share file function
- [ ] Share multiple files
- [ ] Social share buttons
- [ ] Share button component
- [ ] Clipboard fallback UI
- [ ] Share success/error feedback
- [ ] Platform detection
- [ ] Web Share API integration

### Features Implemented
- [ ] Basic text sharing
- [ ] URL sharing
- [ ] Single file sharing
- [ ] Multiple file sharing
- [ ] Social media quick share
- [ ] Email integration
- [ ] Clipboard fallback
- [ ] Share result handling

### Testing
- [ ] Mobile share sheet tested
- [ ] File sharing tested
- [ ] Desktop fallback tested
- [ ] Web Share API tested
- [ ] Error scenarios tested
- [ ] Cross-platform compatibility verified

---

Last Updated: November 2025
Module Version: 1.0.0
Status: Planned
