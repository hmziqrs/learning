/**
 * File Sharing & Social Integration Utility Library
 *
 * Provides comprehensive sharing functionality with multiple fallback mechanisms:
 * - Native backend sharing (Tauri commands)
 * - Web Share API (progressive web apps)
 * - Clipboard operations (backend + web API + legacy)
 * - Social media web intents
 * - Email and SMS integration
 */

import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// Type Definitions
// ============================================================================

export interface ShareRequest {
  title?: string
  text?: string
  url?: string
  files?: string[]
}

export interface ShareResult {
  success: boolean
  method: string
  error?: string
}

export interface ClipboardResult {
  success: boolean
  content?: string
  error?: string
}

// ============================================================================
// Platform Detection
// ============================================================================

/**
 * Get the current platform name from backend
 */
export async function getPlatform(): Promise<string> {
  try {
    return await invoke<string>('get_share_platform')
  } catch (error) {
    return 'unknown'
  }
}

/**
 * Check if native sharing is supported
 */
export async function isNativeShareSupported(): Promise<boolean> {
  try {
    return await invoke<boolean>('is_share_supported')
  } catch (error) {
    return false
  }
}

/**
 * Check if Web Share API is supported
 */
export function isWebShareSupported(): boolean {
  return 'share' in navigator
}

// ============================================================================
// Share Functions
// ============================================================================

/**
 * Share content using the best available method
 * Priority: Web Share API → Clipboard fallback
 */
export async function share(request: ShareRequest): Promise<ShareResult> {
  // Try Web Share API first
  if (isWebShareSupported()) {
    try {
      await navigator.share({
        title: request.title,
        text: request.text,
        url: request.url,
      })
      return {
        success: true,
        method: 'Web Share API',
      }
    } catch (error) {
      if (error instanceof Error && error.name === 'AbortError') {
        return {
          success: false,
          method: 'Web Share API',
          error: 'User cancelled',
        }
      }
      // Fall through to clipboard fallback
    }
  }

  // Fallback to clipboard
  const content = [request.title, request.text, request.url]
    .filter(Boolean)
    .join('\n')

  const clipboardResult = await copyToClipboard(content)

  return {
    success: clipboardResult.success,
    method: clipboardResult.success ? 'Clipboard (Fallback)' : 'Failed',
    error: clipboardResult.error,
  }
}

/**
 * Share text using native backend (mobile only)
 */
export async function shareNative(request: ShareRequest): Promise<ShareResult> {
  try {
    const result = await invoke<{ success: boolean; error?: string }>('share_text', {
      request,
    })
    return {
      success: result.success,
      method: 'Native Share',
      error: result.error,
    }
  } catch (error) {
    return {
      success: false,
      method: 'Native Share',
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

/**
 * Share files using native backend (mobile only)
 */
export async function shareFiles(
  files: string[],
  title?: string
): Promise<ShareResult> {
  try {
    const result = await invoke<{ success: boolean; error?: string }>('share_files', {
      files,
      title,
    })
    return {
      success: result.success,
      method: 'File Share',
      error: result.error,
    }
  } catch (error) {
    return {
      success: false,
      method: 'File Share',
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

// ============================================================================
// Clipboard Functions
// ============================================================================

/**
 * Copy text to clipboard with multiple fallback mechanisms
 * Priority: Backend → Web Clipboard API → Legacy execCommand
 */
export async function copyToClipboard(text: string): Promise<ClipboardResult> {
  // Try backend clipboard first
  try {
    await invoke('copy_to_clipboard_backend', { text })
    return {
      success: true,
      content: text,
    }
  } catch (backendError) {
    // Backend failed, try Web Clipboard API
    try {
      if (navigator.clipboard) {
        await navigator.clipboard.writeText(text)
        return {
          success: true,
          content: text,
        }
      }
    } catch (webError) {
      // Web API failed, try legacy method
      try {
        const textArea = document.createElement('textarea')
        textArea.value = text
        textArea.style.position = 'fixed'
        textArea.style.left = '-999999px'
        textArea.style.top = '-999999px'
        document.body.appendChild(textArea)
        textArea.focus()
        textArea.select()
        const successful = document.execCommand('copy')
        document.body.removeChild(textArea)

        if (successful) {
          return {
            success: true,
            content: text,
          }
        }
      } catch (legacyError) {
        // All methods failed
      }
    }
  }

  return {
    success: false,
    error: 'All clipboard methods failed',
  }
}

/**
 * Read text from clipboard with multiple fallback mechanisms
 * Priority: Backend → Web Clipboard API
 */
export async function readFromClipboard(): Promise<ClipboardResult> {
  // Try backend clipboard first
  try {
    const text = await invoke<string>('read_from_clipboard')
    return {
      success: true,
      content: text,
    }
  } catch (backendError) {
    // Backend failed, try Web Clipboard API
    try {
      if (navigator.clipboard) {
        const text = await navigator.clipboard.readText()
        return {
          success: true,
          content: text,
        }
      }
    } catch (webError) {
      // Web API failed
    }
  }

  return {
    success: false,
    error: 'Failed to read from clipboard',
  }
}

// ============================================================================
// Social Media Functions
// ============================================================================

/**
 * Share to Twitter via web intent
 */
export function shareToTwitter(text: string, url?: string): ShareResult {
  try {
    const twitterUrl = new URL('https://twitter.com/intent/tweet')
    twitterUrl.searchParams.set('text', text)
    if (url) {
      twitterUrl.searchParams.set('url', url)
    }

    window.open(twitterUrl.toString(), '_blank', 'width=550,height=420')

    return {
      success: true,
      method: 'Twitter Share',
    }
  } catch (error) {
    return {
      success: false,
      method: 'Twitter Share',
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

/**
 * Share to Facebook via web intent
 */
export function shareToFacebook(url: string): ShareResult {
  try {
    const facebookUrl = new URL('https://www.facebook.com/sharer/sharer.php')
    facebookUrl.searchParams.set('u', url)

    window.open(facebookUrl.toString(), '_blank', 'width=550,height=420')

    return {
      success: true,
      method: 'Facebook Share',
    }
  } catch (error) {
    return {
      success: false,
      method: 'Facebook Share',
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

/**
 * Share to LinkedIn via web intent
 */
export function shareToLinkedIn(url: string, title?: string): ShareResult {
  try {
    const linkedinUrl = new URL('https://www.linkedin.com/sharing/share-offsite/')
    linkedinUrl.searchParams.set('url', url)
    if (title) {
      linkedinUrl.searchParams.set('title', title)
    }

    window.open(linkedinUrl.toString(), '_blank', 'width=550,height=420')

    return {
      success: true,
      method: 'LinkedIn Share',
    }
  } catch (error) {
    return {
      success: false,
      method: 'LinkedIn Share',
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

/**
 * Share via email using mailto: protocol
 */
export function shareViaEmail(subject: string, body: string): ShareResult {
  try {
    const mailto = `mailto:?subject=${encodeURIComponent(subject)}&body=${encodeURIComponent(body)}`
    window.location.href = mailto

    return {
      success: true,
      method: 'Email Share',
    }
  } catch (error) {
    return {
      success: false,
      method: 'Email Share',
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

/**
 * Share via SMS using sms: protocol (mobile only)
 */
export function shareViaSMS(body: string): ShareResult {
  try {
    const sms = `sms:?body=${encodeURIComponent(body)}`
    window.location.href = sms

    return {
      success: true,
      method: 'SMS Share',
    }
  } catch (error) {
    return {
      success: false,
      method: 'SMS Share',
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

/**
 * Share via WhatsApp web intent
 */
export function shareToWhatsApp(text: string): ShareResult {
  try {
    const whatsappUrl = new URL('https://wa.me/')
    whatsappUrl.searchParams.set('text', text)

    window.open(whatsappUrl.toString(), '_blank')

    return {
      success: true,
      method: 'WhatsApp Share',
    }
  } catch (error) {
    return {
      success: false,
      method: 'WhatsApp Share',
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Check all available share methods and return support status
 */
export async function getShareCapabilities() {
  const [platform, nativeSupport] = await Promise.all([
    getPlatform(),
    isNativeShareSupported(),
  ])

  return {
    platform,
    webShareAPI: isWebShareSupported(),
    nativeShare: nativeSupport,
    clipboard: true, // Always available with fallbacks
    social: true, // Web intents always available
  }
}

/**
 * Format content for sharing
 */
export function formatShareContent(
  title?: string,
  text?: string,
  url?: string
): string {
  return [title, text, url].filter(Boolean).join('\n\n')
}
