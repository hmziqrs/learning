# File Sharing & Social Integration - Usage Guide

This guide shows how to use the file sharing and social integration functionality in your Tauri application.

## Quick Start

### Using the Share Utility Library

```typescript
import {
  share,
  copyToClipboard,
  shareToTwitter,
  shareToFacebook,
  shareViaEmail,
  getShareCapabilities,
} from '@/lib/share'

// Simple share with automatic fallback
const result = await share({
  title: 'Check this out!',
  text: 'Amazing content from my Tauri app',
  url: 'https://tauri.app',
})

if (result.success) {
  console.log(`Shared via ${result.method}`)
} else {
  console.error(`Share failed: ${result.error}`)
}
```

## Features

### 1. Platform Detection

Detect the current platform and available share methods:

```typescript
import { getPlatform, getShareCapabilities } from '@/lib/share'

// Get platform name
const platform = await getPlatform()
console.log(`Running on: ${platform}`) // "windows", "macos", "linux", "ios", "android"

// Get all capabilities
const capabilities = await getShareCapabilities()
console.log(capabilities)
// {
//   platform: "windows",
//   webShareAPI: true,
//   nativeShare: false,
//   clipboard: true,
//   social: true
// }
```

### 2. Web Share API

Share using the native Web Share API (works on HTTPS):

```typescript
import { share } from '@/lib/share'

const result = await share({
  title: 'My Title',
  text: 'Share this content',
  url: 'https://example.com',
})

// Automatically falls back to clipboard if Web Share API is not available
```

### 3. Clipboard Operations

#### Copy to Clipboard

Multi-layered clipboard with automatic fallback:

```typescript
import { copyToClipboard } from '@/lib/share'

const result = await copyToClipboard('Text to copy')

if (result.success) {
  console.log('Copied to clipboard!')
} else {
  console.error(result.error)
}
```

Fallback order:
1. Backend clipboard (tauri-plugin-clipboard-manager)
2. Web Clipboard API (navigator.clipboard)
3. Legacy execCommand

#### Read from Clipboard

```typescript
import { readFromClipboard } from '@/lib/share'

const result = await readFromClipboard()

if (result.success) {
  console.log(`Clipboard content: ${result.content}`)
} else {
  console.error(result.error)
}
```

### 4. Social Media Sharing

#### Twitter

```typescript
import { shareToTwitter } from '@/lib/share'

const result = shareToTwitter('Check out this amazing app!', 'https://tauri.app')
// Opens Twitter share dialog in new window
```

#### Facebook

```typescript
import { shareToFacebook } from '@/lib/share'

const result = shareToFacebook('https://tauri.app')
// Opens Facebook share dialog
```

#### LinkedIn

```typescript
import { shareToLinkedIn } from '@/lib/share'

const result = shareToLinkedIn('https://tauri.app', 'Tauri - Build Apps')
// Opens LinkedIn share dialog
```

#### WhatsApp

```typescript
import { shareToWhatsApp } from '@/lib/share'

const result = shareToWhatsApp('Check out Tauri: https://tauri.app')
// Opens WhatsApp with pre-filled message
```

### 5. Email & SMS

#### Email

```typescript
import { shareViaEmail } from '@/lib/share'

const result = shareViaEmail(
  'Check this out!',
  'I found this amazing app:\n\nhttps://tauri.app'
)
// Opens default email client
```

#### SMS

```typescript
import { shareViaSMS } from '@/lib/share'

const result = shareViaSMS('Check out https://tauri.app')
// Opens SMS app on mobile devices
```

### 6. Native File Sharing (Mobile)

Share files using platform-specific share sheets:

```typescript
import { shareFiles } from '@/lib/share'

const result = await shareFiles(
  ['/path/to/file.pdf', '/path/to/image.jpg'],
  'My Files'
)

if (result.success) {
  console.log('Files shared!')
} else {
  console.error(result.error)
}
```

**Requirements:**
- Android: Requires FileProvider configuration
- iOS: Requires UIActivityViewController implementation
- Desktop: Not supported (returns error)

## React Component Examples

### Basic Share Button

```typescript
import { useState } from 'react'
import { share } from '@/lib/share'
import { Button } from '@/components/ui/button'
import { Share2 } from 'lucide-react'

export function ShareButton({ title, text, url }) {
  const [status, setStatus] = useState('')

  const handleShare = async () => {
    const result = await share({ title, text, url })

    if (result.success) {
      setStatus(`✓ Shared via ${result.method}`)
    } else {
      setStatus(`✗ ${result.error}`)
    }
  }

  return (
    <div>
      <Button onClick={handleShare}>
        <Share2 className="w-4 h-4 mr-2" />
        Share
      </Button>
      {status && <p className="text-sm mt-2">{status}</p>}
    </div>
  )
}
```

### Social Share Buttons

```typescript
import {
  shareToTwitter,
  shareToFacebook,
  shareToLinkedIn,
} from '@/lib/share'
import { Button } from '@/components/ui/button'
import { Twitter, Facebook } from 'lucide-react'

export function SocialShareButtons({ text, url }) {
  return (
    <div className="flex gap-2">
      <Button
        onClick={() => shareToTwitter(text, url)}
        variant="outline"
        size="sm"
      >
        <Twitter className="w-4 h-4" />
      </Button>

      <Button
        onClick={() => shareToFacebook(url)}
        variant="outline"
        size="sm"
      >
        <Facebook className="w-4 h-4" />
      </Button>

      <Button
        onClick={() => shareToLinkedIn(url, text)}
        variant="outline"
        size="sm"
      >
        LinkedIn
      </Button>
    </div>
  )
}
```

### Clipboard Manager

```typescript
import { useState } from 'react'
import { copyToClipboard, readFromClipboard } from '@/lib/share'
import { Button } from '@/components/ui/button'
import { Copy } from 'lucide-react'

export function ClipboardManager() {
  const [content, setContent] = useState('')

  const handleCopy = async () => {
    await copyToClipboard('Hello from clipboard!')
  }

  const handleRead = async () => {
    const result = await readFromClipboard()
    if (result.success) {
      setContent(result.content || '')
    }
  }

  return (
    <div className="space-y-2">
      <div className="flex gap-2">
        <Button onClick={handleCopy} variant="outline">
          <Copy className="w-4 h-4 mr-2" />
          Copy
        </Button>

        <Button onClick={handleRead} variant="outline">
          <Copy className="w-4 h-4 mr-2" />
          Read
        </Button>
      </div>

      {content && (
        <div className="p-4 bg-muted rounded">
          <pre className="text-xs">{content}</pre>
        </div>
      )}
    </div>
  )
}
```

## Error Handling

All functions return a consistent result format:

```typescript
interface ShareResult {
  success: boolean
  method: string // Which method was used
  error?: string // Error message if failed
}

interface ClipboardResult {
  success: boolean
  content?: string // Clipboard content (for read operations)
  error?: string // Error message if failed
}
```

Example error handling:

```typescript
const result = await share({ text: 'Hello' })

if (!result.success) {
  switch (result.error) {
    case 'User cancelled':
      // User dismissed the share dialog
      console.log('Share was cancelled')
      break

    case 'All clipboard methods failed':
      // No clipboard access available
      console.error('Cannot access clipboard')
      break

    default:
      console.error(`Share failed: ${result.error}`)
  }
}
```

## Platform Support

| Feature | Windows | macOS | Linux | iOS | Android |
|---------|---------|-------|-------|-----|---------|
| Web Share API | ⚠️* | ⚠️* | ⚠️* | ✅ | ✅ |
| Backend Clipboard | ✅ | ✅ | ✅ | ✅ | ✅ |
| Web Clipboard API | ✅** | ✅** | ✅** | ✅** | ✅** |
| Social Web Intents | ✅ | ✅ | ✅ | ✅ | ✅ |
| Email/SMS | ✅ | ✅ | ✅ | ✅ | ✅ |
| File Sharing | ❌ | ❌ | ❌ | 🔶*** | 🔶*** |

**Notes:**
- ⚠️* Browser-dependent, requires HTTPS
- ✅** Requires user permission
- 🔶*** Requires platform plugin implementation

## Best Practices

### 1. Always Handle Errors

```typescript
const result = await share({ text: 'Hello' })

if (!result.success) {
  // Show user-friendly error message
  alert(`Failed to share: ${result.error}`)
}
```

### 2. Provide Fallbacks

```typescript
// Try Web Share API, fallback to clipboard
const result = await share({ text: 'Content' })

// The library automatically handles fallbacks
console.log(`Shared via: ${result.method}`)
```

### 3. Check Capabilities First

```typescript
const capabilities = await getShareCapabilities()

if (capabilities.webShareAPI) {
  // Use Web Share API
} else if (capabilities.clipboard) {
  // Fall back to clipboard
}
```

### 4. Sanitize Content

```typescript
// Remove sensitive data before sharing
const safeContent = sanitizeContent(originalContent)
await share({ text: safeContent })
```

### 5. User Permission

```typescript
// Web Clipboard API requires user gesture
button.addEventListener('click', async () => {
  // Must be triggered by user action
  await copyToClipboard('Text')
})
```

## Security Considerations

### Clipboard Access

- Backend clipboard: Always available
- Web Clipboard API: Requires HTTPS and user permission
- Legacy method: No restrictions but limited functionality

### Social Sharing

- All social platforms use web intents
- No authentication required
- Content is URL-encoded automatically

### File Sharing

- Always validate file paths
- Use FileProvider on Android (never file:// URIs)
- Check file permissions before sharing

## Troubleshooting

### Web Share API Not Working

**Problem:** `navigator.share is undefined`

**Solutions:**
1. Ensure site is served over HTTPS
2. Check browser compatibility
3. Use clipboard fallback

### Clipboard Read Fails

**Problem:** Permission denied

**Solutions:**
1. Ensure user triggered the action (click event)
2. Request clipboard permission
3. Use Web Clipboard API with proper permissions

### Social Share Opens Blank Window

**Problem:** Popup blocked

**Solutions:**
1. Ensure triggered by user gesture
2. User needs to allow popups
3. Check popup blocker settings

### Backend Commands Not Found

**Problem:** `Command not found` error

**Solutions:**
1. Ensure Rust commands are registered in `invoke_handler`
2. Check capabilities configuration includes required permissions
3. Rebuild the application

## Advanced Usage

### Custom Share Dialog

```typescript
import { useState } from 'react'
import { share, copyToClipboard, shareToTwitter } from '@/lib/share'

export function CustomShareDialog({ content }) {
  const [isOpen, setIsOpen] = useState(false)

  const handleShare = async (method: string) => {
    switch (method) {
      case 'web':
        await share(content)
        break
      case 'twitter':
        shareToTwitter(content.text || '', content.url)
        break
      case 'clipboard':
        await copyToClipboard(content.text || '')
        break
    }
    setIsOpen(false)
  }

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Share</DialogTitle>
        </DialogHeader>
        <div className="space-y-2">
          <Button onClick={() => handleShare('web')}>Share</Button>
          <Button onClick={() => handleShare('twitter')}>Twitter</Button>
          <Button onClick={() => handleShare('clipboard')}>Copy</Button>
        </div>
      </DialogContent>
    </Dialog>
  )
}
```

### Share with Analytics

```typescript
import { share } from '@/lib/share'

async function shareWithTracking(content) {
  const result = await share(content)

  // Track share event
  analytics.track('share', {
    method: result.method,
    success: result.success,
    content_type: 'article',
  })

  return result
}
```

### Batch Sharing

```typescript
import { shareToTwitter, shareToFacebook } from '@/lib/share'

async function shareToMultiplePlatforms(text: string, url: string) {
  const results = await Promise.allSettled([
    shareToTwitter(text, url),
    shareToFacebook(url),
  ])

  return results.map((r, i) => ({
    platform: ['Twitter', 'Facebook'][i],
    success: r.status === 'fulfilled',
  }))
}
```

## API Reference

See the [full module documentation](./file-sharing-social-module.md) for complete API reference, platform-specific implementations, and advanced configuration.

---

**Module Version:** 1.0.0
**Last Updated:** November 2025
**Status:** Production Ready
