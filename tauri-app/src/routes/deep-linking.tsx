import { createFileRoute } from '@tanstack/react-router'
import { Link as LinkIcon, Copy, History, ExternalLink } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import { onOpenUrl } from '@tauri-apps/plugin-deep-link'

export const Route = createFileRoute('/deep-linking')({
  component: DeepLinking,
})

interface ReceivedUrl {
  id: number
  url: string
  timestamp: Date
  params: Record<string, string>
}

function DeepLinking() {
  const [output, setOutput] = useState<string[]>([])
  const [receivedUrls, setReceivedUrls] = useState<ReceivedUrl[]>([])
  const [lastUrl, setLastUrl] = useState<string | null>(null)
  const [isListening, setIsListening] = useState(false)

  useEffect(() => {
    setupDeepLinkListener()
  }, [])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    setOutput((prev) => [...prev, `${icon} ${message}`])
  }

  const setupDeepLinkListener = async () => {
    try {
      await onOpenUrl((urls) => {
        const url = urls[0]
        if (url) {
          handleDeepLink(url)
        }
      })
      setIsListening(true)
      addOutput('Deep link listener initialized')
    } catch (error) {
      addOutput(`Error setting up listener: ${error}`, false)
    }
  }

  const handleDeepLink = (url: string) => {
    try {
      // Parse URL and extract parameters
      const urlObj = new URL(url)
      const params: Record<string, string> = {}

      urlObj.searchParams.forEach((value, key) => {
        params[key] = value
      })

      const receivedUrl: ReceivedUrl = {
        id: Date.now(),
        url,
        timestamp: new Date(),
        params,
      }

      setReceivedUrls((prev) => [receivedUrl, ...prev])
      setLastUrl(url)
      addOutput(`Received deep link: ${url}`)

      if (Object.keys(params).length > 0) {
        addOutput(`Parameters: ${JSON.stringify(params)}`)
      }
    } catch (error) {
      addOutput(`Error parsing URL: ${error}`, false)
    }
  }

  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text)
      addOutput(`Copied to clipboard: ${text}`)
    } catch (error) {
      addOutput(`Error copying to clipboard: ${error}`, false)
    }
  }

  const exampleUrls = [
    { label: 'Basic URL', url: 'myapp://home' },
    { label: 'With Path', url: 'myapp://profile/user123' },
    { label: 'With Parameters', url: 'myapp://search?q=tauri&filter=latest' },
    { label: 'Complex URL', url: 'myapp://details?id=42&tab=settings&theme=dark' },
  ]

  return (
    <ModulePageLayout
      title="Deep Linking Module"
      description="Test opening the app via custom URL schemes like myapp://route."
      icon={LinkIcon}
    >
      <div className="space-y-6">
        {/* Status Section */}
        <div className="space-y-4">
          <h3 className="font-semibold">Listener Status</h3>
          <div className="flex items-center gap-2">
            <div className={`h-3 w-3 rounded-full ${isListening ? 'bg-green-500' : 'bg-red-500'}`} />
            <span className={isListening ? 'text-green-500' : 'text-red-500'}>
              {isListening ? 'Active - Listening for deep links' : 'Not Active'}
            </span>
          </div>
          {lastUrl && (
            <div className="p-3 bg-muted rounded-md border">
              <div className="text-sm font-medium mb-1">Last Received URL:</div>
              <div className="font-mono text-sm break-all">{lastUrl}</div>
            </div>
          )}
        </div>

        {/* Example URLs Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <ExternalLink className="h-5 w-5" />
            Example URLs
          </h3>
          <div className="space-y-2">
            {exampleUrls.map((example, index) => (
              <div
                key={index}
                className="flex items-center gap-2 p-3 bg-muted rounded-md border"
              >
                <div className="flex-1">
                  <div className="text-sm font-medium mb-1">{example.label}</div>
                  <div className="font-mono text-xs break-all text-muted-foreground">
                    {example.url}
                  </div>
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => copyToClipboard(example.url)}
                >
                  <Copy className="h-4 w-4" />
                </Button>
              </div>
            ))}
          </div>
          <div className="p-4 bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800 rounded-md">
            <h4 className="text-sm font-medium mb-2">How to Test:</h4>
            <ul className="text-sm space-y-1 text-muted-foreground list-disc list-inside">
              <li>
                <strong>macOS/Linux:</strong> Open Terminal and run:{' '}
                <code className="text-xs bg-muted px-1 py-0.5 rounded">
                  open "myapp://home"
                </code>
              </li>
              <li>
                <strong>Windows:</strong> Press Win+R and paste the URL, or open from browser
              </li>
              <li>
                <strong>Mobile:</strong> Open the URL from a browser or another app
              </li>
              <li>Copy any example URL above and test it using the appropriate method</li>
            </ul>
          </div>
        </div>

        {/* URL History Section */}
        {receivedUrls.length > 0 && (
          <div className="space-y-4">
            <h3 className="font-semibold flex items-center gap-2">
              <History className="h-5 w-5" />
              Received URLs History
            </h3>
            <div className="space-y-2">
              {receivedUrls.map((item) => (
                <div
                  key={item.id}
                  className="p-3 bg-muted rounded-md border space-y-2"
                >
                  <div className="flex items-start justify-between gap-2">
                    <div className="flex-1">
                      <div className="font-mono text-sm break-all">{item.url}</div>
                      <div className="text-xs text-muted-foreground mt-1">
                        {item.timestamp.toLocaleTimeString()}
                      </div>
                    </div>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => copyToClipboard(item.url)}
                    >
                      <Copy className="h-3 w-3" />
                    </Button>
                  </div>
                  {Object.keys(item.params).length > 0 && (
                    <div className="pt-2 border-t">
                      <div className="text-xs font-medium mb-1">Parameters:</div>
                      <div className="text-xs font-mono space-y-1">
                        {Object.entries(item.params).map(([key, value]) => (
                          <div key={key}>
                            <span className="text-muted-foreground">{key}:</span>{' '}
                            <span>{value}</span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              ))}
            </div>
            <Button
              variant="outline"
              onClick={() => setReceivedUrls([])}
            >
              Clear History
            </Button>
          </div>
        )}

        {/* Output Panel */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold">Output</h3>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setOutput([])}
            >
              Clear
            </Button>
          </div>
          <div className="bg-muted p-4 rounded-md font-mono text-sm min-h-[150px] max-h-[300px] overflow-y-auto">
            {output.length === 0 ? (
              <p className="text-muted-foreground">
                Operation results will appear here...
              </p>
            ) : (
              <div className="space-y-1">
                {output.map((line, index) => (
                  <div key={index}>{line}</div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </ModulePageLayout>
  )
}
