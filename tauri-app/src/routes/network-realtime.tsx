import { createFileRoute } from '@tanstack/react-router'
import { Wifi, Send, Upload, Radio, Globe, Activity } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

export const Route = createFileRoute('/network-realtime')({
  component: NetworkRealtimeModule,
})

interface HttpResponse {
  status: number
  body: string
}

function NetworkRealtimeModule() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)

  // HTTP State
  const [httpUrl, setHttpUrl] = useState('https://jsonplaceholder.typicode.com/posts/1')
  const [httpResponse, setHttpResponse] = useState<string>('')

  // WebSocket State
  const [wsUrl, setWsUrl] = useState('wss://echo.websocket.org')
  const [wsConnected, setWsConnected] = useState(false)
  const [wsMessage, setWsMessage] = useState('')
  const [wsMessages, setWsMessages] = useState<string[]>([])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  // HTTP GET Request
  const handleHttpGet = async () => {
    setLoading('http-get')
    addOutput(`Making GET request to: ${httpUrl}`)

    try {
      // TODO: Implement with real backend command
      // const response = await invoke<HttpResponse>('http_get', { url: httpUrl })

      // Mock implementation for now
      const mockResponse = {
        userId: 1,
        id: 1,
        title: 'sunt aut facere repellat provident occaecati excepturi optio reprehenderit',
        body: 'quia et suscipit\nsuscipit recusandae consequuntur expedita et cum',
      }

      const responseText = JSON.stringify(mockResponse, null, 2)
      setHttpResponse(responseText)
      addOutput('✓ GET request successful')
      addOutput(`Response length: ${responseText.length} bytes`)
      addOutput('Note: Using mock response. Implement backend for real requests.')
    } catch (error) {
      addOutput(`✗ GET request failed: ${error}`, false)
      setHttpResponse('')
    } finally {
      setLoading(null)
    }
  }

  // HTTP POST Request
  const handleHttpPost = async () => {
    setLoading('http-post')
    const postUrl = 'https://jsonplaceholder.typicode.com/posts'
    addOutput(`Making POST request to: ${postUrl}`)

    try {
      // TODO: Implement with real backend command
      // const response = await invoke<HttpResponse>('http_post', {
      //   url: postUrl,
      //   data: { title: 'Test Post', body: 'This is a test', userId: 1 }
      // })

      // Mock implementation
      const mockResponse = {
        id: 101,
        title: 'Test Post',
        body: 'This is a test',
        userId: 1,
      }

      const responseText = JSON.stringify(mockResponse, null, 2)
      setHttpResponse(responseText)
      addOutput('✓ POST request successful')
      addOutput(`Created resource with ID: ${mockResponse.id}`)
      addOutput('Note: Using mock response. Implement backend for real requests.')
    } catch (error) {
      addOutput(`✗ POST request failed: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // WebSocket Connect
  const handleWsConnect = async () => {
    setLoading('ws-connect')
    addOutput(`Connecting to WebSocket: ${wsUrl}`)

    try {
      // TODO: Implement with real backend command
      // await invoke('websocket_connect', { url: wsUrl })

      // Mock implementation
      setWsConnected(true)
      addOutput('✓ WebSocket connected successfully')
      addOutput('You can now send messages')
      addOutput('Note: Using mock connection. Implement backend for real WebSocket.')
    } catch (error) {
      addOutput(`✗ WebSocket connection failed: ${error}`, false)
      setWsConnected(false)
    } finally {
      setLoading(null)
    }
  }

  // WebSocket Disconnect
  const handleWsDisconnect = async () => {
    setLoading('ws-disconnect')
    addOutput('Disconnecting WebSocket...')

    try {
      // TODO: Implement with real backend command
      // await invoke('websocket_close', { connectionId: wsId })

      // Mock implementation
      setWsConnected(false)
      setWsMessages([])
      addOutput('✓ WebSocket disconnected')
    } catch (error) {
      addOutput(`✗ Disconnect failed: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // WebSocket Send Message
  const handleWsSend = async () => {
    if (!wsMessage.trim()) return

    setLoading('ws-send')
    addOutput(`Sending message: "${wsMessage}"`)

    try {
      // TODO: Implement with real backend command
      // await invoke('websocket_send', { connectionId: wsId, message: wsMessage })

      // Mock implementation - echo back
      const timestamp = new Date().toLocaleTimeString()
      const echoMessage = `[${timestamp}] Echo: ${wsMessage}`
      setWsMessages((prev) => [...prev, echoMessage])
      addOutput('✓ Message sent')
      addOutput('✓ Received echo response')
      setWsMessage('')
    } catch (error) {
      addOutput(`✗ Send failed: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  // File Upload
  const handleFileUpload = async () => {
    setLoading('upload')
    addOutput('Opening file picker...')

    try {
      // TODO: Implement file picker and upload
      // const filePath = await open({ multiple: false })
      // await invoke('upload_file', {
      //   url: 'https://httpbin.org/post',
      //   filePath
      // })

      // Mock implementation
      addOutput('✓ File selected: example.txt (2.5 KB)')
      addOutput('Uploading to https://httpbin.org/post...')
      await new Promise((resolve) => setTimeout(resolve, 1500))
      addOutput('✓ Upload successful')
      addOutput('Note: Using mock upload. Implement backend for real file upload.')
    } catch (error) {
      addOutput(`✗ Upload failed: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  return (
    <ModulePageLayout
      title="Network & Realtime Module"
      description="Test HTTP requests, WebSocket connections, and file uploads"
      icon={Wifi}
    >
      <div className="space-y-6">
        {/* Setup Notice */}
        <section className="rounded-lg border border-yellow-500/50 bg-yellow-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-yellow-500">⚠️</span>
            Backend Implementation Required
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">This module requires Rust backend commands:</p>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># Add required dependencies to Cargo.toml</div>
              <div>reqwest = &#123; version = "0.11", features = ["json", "multipart"] &#125;</div>
              <div>tokio = &#123; version = "1", features = ["full"] &#125;</div>
              <div className="mt-2"># Or install Tauri plugins</div>
              <div>bun add @tauri-apps/plugin-http</div>
              <div>bun add @tauri-apps/plugin-websocket</div>
            </div>
            <p className="text-muted-foreground mt-2">
              Currently showing mock responses. Implement backend commands in{' '}
              <code>src-tauri/src/lib.rs</code> for real functionality.
            </p>
          </div>
        </section>

        {/* HTTP Requests Section */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Globe className="w-5 h-5" />
            HTTP Requests
          </h2>

          <div className="space-y-3">
            <div className="flex gap-2">
              <Input
                type="url"
                placeholder="https://api.example.com/endpoint"
                value={httpUrl}
                onChange={(e) => setHttpUrl(e.target.value)}
                className="flex-1"
              />
            </div>

            <div className="flex flex-wrap gap-2">
              <Button
                onClick={handleHttpGet}
                disabled={loading === 'http-get' || !httpUrl}
                variant="outline"
              >
                <Send className={`w-4 h-4 mr-2 ${loading === 'http-get' ? 'animate-pulse' : ''}`} />
                {loading === 'http-get' ? 'Loading...' : 'GET Request'}
              </Button>

              <Button
                onClick={handleHttpPost}
                disabled={loading === 'http-post'}
                variant="outline"
              >
                <Send className={`w-4 h-4 mr-2 ${loading === 'http-post' ? 'animate-pulse' : ''}`} />
                {loading === 'http-post' ? 'Posting...' : 'POST Request'}
              </Button>
            </div>

            {httpResponse && (
              <div className="mt-4">
                <h3 className="text-sm font-semibold mb-2">Response:</h3>
                <div className="bg-muted rounded-md p-4 max-h-64 overflow-y-auto">
                  <pre className="text-xs font-mono whitespace-pre-wrap">{httpResponse}</pre>
                </div>
              </div>
            )}
          </div>
        </section>

        {/* WebSocket Section */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Radio className="w-5 h-5" />
            WebSocket Connection
          </h2>

          <div className="space-y-3">
            <div className="flex gap-2">
              <Input
                type="url"
                placeholder="wss://echo.websocket.org"
                value={wsUrl}
                onChange={(e) => setWsUrl(e.target.value)}
                disabled={wsConnected}
                className="flex-1"
              />
            </div>

            <div className="flex flex-wrap gap-2">
              {!wsConnected ? (
                <Button
                  onClick={handleWsConnect}
                  disabled={loading === 'ws-connect' || !wsUrl}
                  variant="outline"
                >
                  <Activity className={`w-4 h-4 mr-2 ${loading === 'ws-connect' ? 'animate-pulse' : ''}`} />
                  {loading === 'ws-connect' ? 'Connecting...' : 'Connect'}
                </Button>
              ) : (
                <Button
                  onClick={handleWsDisconnect}
                  disabled={loading === 'ws-disconnect'}
                  variant="destructive"
                >
                  <Activity className="w-4 h-4 mr-2" />
                  {loading === 'ws-disconnect' ? 'Disconnecting...' : 'Disconnect'}
                </Button>
              )}

              {wsConnected && (
                <div className="flex items-center gap-2 px-3 py-2 bg-green-500/10 text-green-500 rounded-md text-sm">
                  <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                  Connected
                </div>
              )}
            </div>

            {wsConnected && (
              <div className="space-y-3 pt-3 border-t">
                <div className="flex gap-2">
                  <Input
                    type="text"
                    placeholder="Type a message..."
                    value={wsMessage}
                    onChange={(e) => setWsMessage(e.target.value)}
                    onKeyDown={(e) => e.key === 'Enter' && handleWsSend()}
                    className="flex-1"
                  />
                  <Button
                    onClick={handleWsSend}
                    disabled={loading === 'ws-send' || !wsMessage.trim()}
                  >
                    <Send className="w-4 h-4" />
                  </Button>
                </div>

                {wsMessages.length > 0 && (
                  <div className="bg-muted rounded-md p-4 max-h-48 overflow-y-auto space-y-2">
                    <h3 className="text-sm font-semibold mb-2">Messages:</h3>
                    {wsMessages.map((msg, i) => (
                      <div key={i} className="text-sm font-mono bg-background p-2 rounded">
                        {msg}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>
        </section>

        {/* File Upload Section */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Upload className="w-5 h-5" />
            File Upload
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Upload files to remote servers with progress tracking
            </p>

            <Button
              onClick={handleFileUpload}
              disabled={loading === 'upload'}
              variant="outline"
            >
              <Upload className={`w-4 h-4 mr-2 ${loading === 'upload' ? 'animate-bounce' : ''}`} />
              {loading === 'upload' ? 'Uploading...' : 'Upload File'}
            </Button>
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
              <h4 className="font-semibold">HTTP Requests</h4>
              <p className="text-muted-foreground">
                Use Rust's <code>reqwest</code> crate or Tauri's HTTP plugin to make requests.
                Supports GET, POST, PUT, DELETE with custom headers and body.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div># Add to Cargo.toml</div>
                <div>reqwest = &#123; version = "0.11", features = ["json"] &#125;</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">WebSocket</h4>
              <p className="text-muted-foreground">
                Real-time bidirectional communication for chat, live updates, and streaming data.
                Supports text and binary messages.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div># Frontend</div>
                <div>bun add @tauri-apps/plugin-websocket</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">File Upload</h4>
              <p className="text-muted-foreground">
                Upload files with progress tracking using multipart form data. Supports single
                and multiple file uploads.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div># Add multipart support</div>
                <div>reqwest = &#123; version = "0.11", features = ["multipart"] &#125;</div>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400">
                Security Best Practices
              </h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Always use HTTPS/WSS in production</li>
                <li>Validate SSL certificates</li>
                <li>Sanitize user input before sending</li>
                <li>Implement request timeouts</li>
                <li>Handle errors gracefully</li>
                <li>Use environment variables for API keys</li>
              </ul>
            </div>
          </div>
        </section>

        {/* Test Endpoints */}
        <section className="rounded-lg border border-green-500/50 bg-green-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">🧪 Public Test Endpoints</h3>
          <div className="space-y-3 text-sm">
            <div>
              <h4 className="font-semibold">HTTP APIs</h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
                <li>
                  <strong>JSONPlaceholder:</strong>{' '}
                  <code className="text-xs">https://jsonplaceholder.typicode.com</code>
                </li>
                <li>
                  <strong>HTTPBin:</strong> <code className="text-xs">https://httpbin.org</code>
                </li>
                <li>
                  <strong>ReqRes:</strong> <code className="text-xs">https://reqres.in/api</code>
                </li>
              </ul>
            </div>

            <div>
              <h4 className="font-semibold">WebSocket</h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
                <li>
                  <strong>Echo Server:</strong> <code className="text-xs">wss://echo.websocket.org</code>
                </li>
                <li>
                  <strong>Postman Echo:</strong>{' '}
                  <code className="text-xs">wss://ws.postman-echo.com/raw</code>
                </li>
              </ul>
            </div>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
