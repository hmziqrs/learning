import { createFileRoute } from '@tanstack/react-router'
import { Share2, Copy, Mail, FileText, Image, FileVideo, Twitter, Facebook, MessageSquare } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { useState } from 'react'

export const Route = createFileRoute('/file-sharing')({
  component: FileSharingModule,
})

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

function FileSharingModule() {
  const [output, setOutput] = useState<string[]>([])
  const [shareTitle, setShareTitle] = useState('Check this out!')
  const [shareText, setShareText] = useState('This is shared from my Tauri app')
  const [shareUrl, setShareUrl] = useState('https://tauri.app')
  const [isSupported, setIsSupported] = useState(false)

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  // Check if Web Share API is supported
  const checkShareSupport = () => {
    const supported = 'share' in navigator
    setIsSupported(supported)
    addOutput(`Web Share API: ${supported ? 'Supported' : 'Not supported'}`, supported)
  }

  // Share text using Web Share API
  const handleShareText = async () => {
    if (!('share' in navigator)) {
      addOutput('Web Share API not supported. Use clipboard fallback.', false)
      handleCopyToClipboard()
      return
    }

    try {
      await navigator.share({
        title: shareTitle,
        text: shareText,
        url: shareUrl,
      })
      addOutput('Content shared successfully')
    } catch (error) {
      if (error instanceof Error && error.name === 'AbortError') {
        addOutput('Share cancelled by user')
      } else {
        addOutput(`Share failed: ${error instanceof Error ? error.message : String(error)}`, false)
      }
    }
  }

  // Share URL only
  const handleShareUrl = async () => {
    if (!('share' in navigator)) {
      addOutput('Web Share API not supported. Copying URL to clipboard.', false)
      handleCopyUrl()
      return
    }

    try {
      await navigator.share({
        url: shareUrl,
      })
      addOutput('URL shared successfully')
    } catch (error) {
      if (error instanceof Error && error.name === 'AbortError') {
        addOutput('Share cancelled by user')
      } else {
        addOutput(`Share failed: ${error instanceof Error ? error.message : String(error)}`, false)
      }
    }
  }

  // Copy to clipboard
  const handleCopyToClipboard = async () => {
    const content = `${shareTitle}\n${shareText}\n${shareUrl}`

    try {
      if (navigator.clipboard) {
        await navigator.clipboard.writeText(content)
        addOutput('Content copied to clipboard')
      } else {
        // Fallback for older browsers
        const textArea = document.createElement('textarea')
        textArea.value = content
        document.body.appendChild(textArea)
        textArea.select()
        document.execCommand('copy')
        document.body.removeChild(textArea)
        addOutput('Content copied to clipboard (legacy method)')
      }
    } catch (error) {
      addOutput(`Failed to copy: ${error instanceof Error ? error.message : String(error)}`, false)
    }
  }

  // Copy URL only
  const handleCopyUrl = async () => {
    try {
      if (navigator.clipboard) {
        await navigator.clipboard.writeText(shareUrl)
        addOutput('URL copied to clipboard')
      } else {
        const textArea = document.createElement('textarea')
        textArea.value = shareUrl
        document.body.appendChild(textArea)
        textArea.select()
        document.execCommand('copy')
        document.body.removeChild(textArea)
        addOutput('URL copied to clipboard')
      }
    } catch (error) {
      addOutput(`Failed to copy URL: ${error instanceof Error ? error.message : String(error)}`, false)
    }
  }

  // Share via Twitter
  const handleShareTwitter = () => {
    const twitterUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}&url=${encodeURIComponent(shareUrl)}`
    window.open(twitterUrl, '_blank', 'width=550,height=420')
    addOutput('Opened Twitter share dialog')
  }

  // Share via Facebook
  const handleShareFacebook = () => {
    const facebookUrl = `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(shareUrl)}`
    window.open(facebookUrl, '_blank', 'width=550,height=420')
    addOutput('Opened Facebook share dialog')
  }

  // Share via Email
  const handleShareEmail = () => {
    const mailto = `mailto:?subject=${encodeURIComponent(shareTitle)}&body=${encodeURIComponent(shareText + '\n\n' + shareUrl)}`
    window.location.href = mailto
    addOutput('Opened email client')
  }

  // Share via SMS (mobile only)
  const handleShareSMS = () => {
    const sms = `sms:?body=${encodeURIComponent(shareText + ' ' + shareUrl)}`
    window.location.href = sms
    addOutput('Opened SMS app')
  }

  // Simulate file share
  const handleSimulateFileShare = () => {
    addOutput('File sharing requires native implementation')
    addOutput('On mobile: Use tauri-plugin-share', false)
    addOutput('On desktop: Use system file dialogs or clipboard', false)
  }

  return (
    <ModulePageLayout
      title="File Sharing & Social Integration Module"
      description="Share files, text, and links with other apps using native share dialogs"
      icon={Share2}
    >
      <div className="space-y-6">
        {/* Status Notice */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-blue-500">ℹ️</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Current implementation:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li>
                <strong className="text-green-600">✓ Web Share API</strong> - Browser-based sharing (HTTPS required)
              </li>
              <li>
                <strong className="text-green-600">✓ Clipboard API</strong> - Copy to clipboard fallback
              </li>
              <li>
                <strong className="text-green-600">✓ Social Media</strong> - Twitter, Facebook web intents
              </li>
              <li>
                <strong className="text-yellow-600">⚠ Native Share</strong> - Requires tauri-plugin-share
              </li>
              <li>
                <strong className="text-yellow-600">⚠ File Sharing</strong> - Requires platform-specific implementation
              </li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># For native sharing support:</div>
              <div>cargo add tauri-plugin-share</div>
              <div>npm i @tauri-apps/plugin-share</div>
            </div>
            <p className="text-muted-foreground mt-2">
              Desktop: Web Share API support varies. Mobile: Full native share sheet support available with plugin.
            </p>
          </div>
        </section>

        {/* Share Support Check */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Share2 className="w-5 h-5" />
            Share Support
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Check if Web Share API is supported in your browser
            </p>

            <Button onClick={checkShareSupport} variant="outline">
              <Share2 className="w-4 h-4 mr-2" />
              Check Share Support
            </Button>

            {isSupported ? (
              <div className="bg-green-500/10 border border-green-500/30 rounded-md p-4">
                <p className="text-sm text-green-700 dark:text-green-400 font-medium">
                  ✓ Web Share API is supported on this platform
                </p>
              </div>
            ) : output.length > 0 && (
              <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
                <p className="text-sm text-yellow-700 dark:text-yellow-400 font-medium">
                  ⚠ Web Share API not supported. Fallback to clipboard and social sharing.
                </p>
              </div>
            )}
          </div>
        </section>

        {/* Share Content Form */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <FileText className="w-5 h-5" />
            Share Content
          </h2>

          <div className="space-y-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Title</label>
              <Input
                value={shareTitle}
                onChange={(e) => setShareTitle(e.target.value)}
                placeholder="Enter share title"
              />
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium">Text</label>
              <Textarea
                value={shareText}
                onChange={(e) => setShareText(e.target.value)}
                placeholder="Enter text to share"
                rows={3}
              />
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium">URL</label>
              <Input
                value={shareUrl}
                onChange={(e) => setShareUrl(e.target.value)}
                placeholder="Enter URL to share"
                type="url"
              />
            </div>

            <div className="flex flex-wrap gap-2">
              <Button onClick={handleShareText} variant="default">
                <Share2 className="w-4 h-4 mr-2" />
                Share All
              </Button>

              <Button onClick={handleShareUrl} variant="outline">
                <Share2 className="w-4 h-4 mr-2" />
                Share URL Only
              </Button>

              <Button onClick={handleCopyToClipboard} variant="outline">
                <Copy className="w-4 h-4 mr-2" />
                Copy to Clipboard
              </Button>

              <Button onClick={handleCopyUrl} variant="outline">
                <Copy className="w-4 h-4 mr-2" />
                Copy URL
              </Button>
            </div>
          </div>
        </section>

        {/* Social Media Sharing */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Twitter className="w-5 h-5" />
            Social Media Integration
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Share directly to social media platforms using web intents
            </p>

            <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
              <Button onClick={handleShareTwitter} variant="outline" className="w-full">
                <Twitter className="w-4 h-4 mr-2" />
                Twitter
              </Button>

              <Button onClick={handleShareFacebook} variant="outline" className="w-full">
                <Facebook className="w-4 h-4 mr-2" />
                Facebook
              </Button>

              <Button onClick={handleShareEmail} variant="outline" className="w-full">
                <Mail className="w-4 h-4 mr-2" />
                Email
              </Button>

              <Button onClick={handleShareSMS} variant="outline" className="w-full">
                <MessageSquare className="w-4 h-4 mr-2" />
                SMS
              </Button>
            </div>

            <div className="bg-blue-500/10 border border-blue-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-blue-700 dark:text-blue-400 text-sm">
                Social Sharing Methods
              </h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Twitter & Facebook open in popup window</li>
                <li>Email opens default mail client (mailto:)</li>
                <li>SMS opens messaging app (mobile only)</li>
                <li>All methods work without requiring app installation</li>
              </ul>
            </div>
          </div>
        </section>

        {/* File Sharing Section */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Image className="w-5 h-5" />
            File Sharing
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Share files using native platform capabilities
            </p>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="border rounded-lg p-4 space-y-3">
                <div className="flex items-center gap-2">
                  <FileText className="w-5 h-5 text-blue-600" />
                  <h3 className="font-semibold">Documents</h3>
                </div>
                <Button onClick={handleSimulateFileShare} variant="outline" size="sm" className="w-full">
                  Share Document
                </Button>
                <p className="text-xs text-muted-foreground">
                  PDF, DOC, TXT files
                </p>
              </div>

              <div className="border rounded-lg p-4 space-y-3">
                <div className="flex items-center gap-2">
                  <Image className="w-5 h-5 text-green-600" />
                  <h3 className="font-semibold">Images</h3>
                </div>
                <Button onClick={handleSimulateFileShare} variant="outline" size="sm" className="w-full">
                  Share Image
                </Button>
                <p className="text-xs text-muted-foreground">
                  JPG, PNG, GIF files
                </p>
              </div>

              <div className="border rounded-lg p-4 space-y-3">
                <div className="flex items-center gap-2">
                  <FileVideo className="w-5 h-5 text-purple-600" />
                  <h3 className="font-semibold">Videos</h3>
                </div>
                <Button onClick={handleSimulateFileShare} variant="outline" size="sm" className="w-full">
                  Share Video
                </Button>
                <p className="text-xs text-muted-foreground">
                  MP4, MOV, AVI files
                </p>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400 text-sm">
                Native Implementation Required
              </h4>
              <p className="text-xs text-muted-foreground">
                File sharing requires native platform implementation. Use tauri-plugin-share for mobile platforms
                (Android Share Intent, iOS UIActivityViewController). Desktop platforms should use system file
                dialogs or clipboard for file paths.
              </p>
            </div>
          </div>
        </section>

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
              <h4 className="font-semibold">Web Share API (Progressive Web Apps)</h4>
              <p className="text-muted-foreground">
                Browser-based sharing that works across all platforms with HTTPS.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div>if (navigator.share) {'{'}</div>
                <div>  await navigator.share({'{ title, text, url }'})</div>
                <div>{'}'}</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Tauri Plugin Share (Native Mobile)</h4>
              <p className="text-muted-foreground">
                Native share sheets for Android and iOS using platform-specific APIs
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div>cargo add tauri-plugin-share</div>
                <div>npm i @tauri-apps/plugin-share</div>
                <div className="mt-2">.plugin(tauri_plugin_share::init())</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Social Media Web Intents</h4>
              <p className="text-muted-foreground">
                Direct sharing to social platforms via web URLs
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div>Twitter: twitter.com/intent/tweet?text=...</div>
                <div>Facebook: facebook.com/sharer/sharer.php?u=...</div>
                <div>Email: mailto:?subject=...&body=...</div>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400">
                Platform Considerations
              </h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Web Share API requires HTTPS and user gesture</li>
                <li>Mobile share sheets support files, text, and URLs</li>
                <li>Desktop should provide clipboard fallback</li>
                <li>Always validate and sanitize shared content</li>
                <li>Handle user cancellation gracefully</li>
                <li>Consider privacy when sharing files</li>
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
                <tr className="border-b bg-muted/30">
                  <td className="py-2 px-4 font-semibold" colSpan={6}>Basic Sharing</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Web Share API</td>
                  <td className="text-center py-2 px-4">⚠️*</td>
                  <td className="text-center py-2 px-4">⚠️*</td>
                  <td className="text-center py-2 px-4">⚠️*</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Native Share Sheet</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Clipboard Copy</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
                <tr className="border-b bg-muted/30">
                  <td className="py-2 px-4 font-semibold" colSpan={6}>File Sharing</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Share Files</td>
                  <td className="text-center py-2 px-4">⚠️***</td>
                  <td className="text-center py-2 px-4">⚠️***</td>
                  <td className="text-center py-2 px-4">⚠️***</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Share Multiple Files</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b bg-muted/30">
                  <td className="py-2 px-4 font-semibold" colSpan={6}>Social Integration</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Twitter/Facebook</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
                <tr>
                  <td className="py-2 px-4">Email/SMS</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
              </tbody>
            </table>
            <div className="text-xs text-muted-foreground mt-2 space-y-1">
              <p>* ⚠️ = Browser-dependent, requires HTTPS</p>
              <p>** 🔶 = Requires tauri-plugin-share</p>
              <p>*** Desktop: Use file dialogs or clipboard for file paths</p>
            </div>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
