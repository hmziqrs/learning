import { createFileRoute } from '@tanstack/react-router'
import { Server } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { useState } from 'react'

export const Route = createFileRoute('/local-web-server')({
  component: LocalWebServerModule,
})

interface ServerConfig {
  port?: number
  host?: string
  staticDir?: string
  cors?: boolean
  directoryListing?: boolean
}

interface ServerInfo {
  id: string
  url: string
  port: number
  running: boolean
  staticDir?: string
}

function LocalWebServerModule() {
  const [output, setOutput] = useState<string[]>([])
  const [servers, setServers] = useState<ServerInfo[]>([])
  const [port, setPort] = useState<string>('3000')
  const [staticDir, setStaticDir] = useState<string>('./public')
  const [isLoading, setIsLoading] = useState(false)

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const handleStartServer = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Starting local web server...')

    try {
      // Placeholder - will be implemented with actual Tauri command
      addOutput('Server implementation pending - commands not yet registered', false)

      // TODO: Implement actual server start
      // const config: ServerConfig = {
      //   port: port ? parseInt(port) : undefined,
      //   staticDir,
      //   cors: true,
      //   directoryListing: false,
      // }
      // const serverInfo = await invoke<ServerInfo>('start_server', { config })
      // setServers(prev => [...prev, serverInfo])
      // addOutput(`Server started at ${serverInfo.url}`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  const handleStopServer = async (serverId: string) => {
    if (isLoading) return
    setIsLoading(true)
    addOutput(`Stopping server ${serverId}...`)

    try {
      // Placeholder - will be implemented with actual Tauri command
      addOutput('Server implementation pending - commands not yet registered', false)

      // TODO: Implement actual server stop
      // await invoke('stop_server', { serverId })
      // setServers(prev => prev.filter(s => s.id !== serverId))
      // addOutput('Server stopped successfully')
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  const handleCheckPort = async () => {
    if (isLoading || !port) return
    setIsLoading(true)
    addOutput(`Checking if port ${port} is available...`)

    try {
      // Placeholder - will be implemented with actual Tauri command
      addOutput('Server implementation pending - commands not yet registered', false)

      // TODO: Implement actual port check
      // const available = await invoke<boolean>('is_port_available', { port: parseInt(port) })
      // addOutput(available ? `Port ${port} is available` : `Port ${port} is in use`, available)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  const handleStopAllServers = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Stopping all servers...')

    try {
      // Placeholder - will be implemented with actual Tauri command
      addOutput('Server implementation pending - commands not yet registered', false)

      // TODO: Implement actual stop all
      // await invoke('stop_all_servers')
      // setServers([])
      // addOutput('All servers stopped successfully')
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <ModulePageLayout
      title="Local Web Server Module"
      description="Start and manage local HTTP servers for serving static files and testing web content"
      icon={Server}
    >
      <div className="space-y-6">
        {/* Status Notice */}
        <section className="rounded-lg border border-yellow-500/50 bg-yellow-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-yellow-500">⏳</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Current implementation:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li>
                <strong className="text-yellow-600">⏳ Planned</strong> - Module documentation created
              </li>
              <li>
                <strong className="text-red-600">✗ Rust Commands</strong> - Tauri commands not yet implemented
              </li>
              <li>
                <strong className="text-red-600">✗ HTTP Server</strong> - Server implementation pending
              </li>
              <li>
                <strong className="text-red-600">✗ Frontend Integration</strong> - UI placeholder only
              </li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># Planned Implementation:</div>
              <div>- Rust: Axum/Actix-web HTTP server with static file serving</div>
              <div>- Commands: start_server, stop_server, is_port_available, list_servers</div>
              <div>- Platform Support: All desktop and mobile platforms</div>
            </div>
            <p className="text-muted-foreground mt-2">
              See <code className="bg-muted px-1 rounded">docs/local-web-server-module.md</code> for complete implementation plan.
            </p>
          </div>
        </section>

        {/* Server Configuration */}
        <section className="rounded-lg border border-border p-6">
          <h3 className="text-lg font-semibold mb-4">Server Configuration</h3>
          <div className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label htmlFor="port">Port</Label>
                <Input
                  id="port"
                  type="number"
                  value={port}
                  onChange={(e) => setPort(e.target.value)}
                  placeholder="3000"
                  disabled={isLoading}
                />
                <p className="text-xs text-muted-foreground mt-1">
                  Leave as 0 for auto-assign
                </p>
              </div>
              <div>
                <Label htmlFor="staticDir">Static Directory</Label>
                <Input
                  id="staticDir"
                  type="text"
                  value={staticDir}
                  onChange={(e) => setStaticDir(e.target.value)}
                  placeholder="./public"
                  disabled={isLoading}
                />
                <p className="text-xs text-muted-foreground mt-1">
                  Directory to serve files from
                </p>
              </div>
            </div>

            <div className="flex gap-2">
              <Button
                onClick={handleStartServer}
                disabled={isLoading}
                variant="default"
              >
                Start Server
              </Button>
              <Button
                onClick={handleCheckPort}
                disabled={isLoading || !port}
                variant="outline"
              >
                Check Port Availability
              </Button>
            </div>
          </div>
        </section>

        {/* Running Servers */}
        <section className="rounded-lg border border-border p-6">
          <h3 className="text-lg font-semibold mb-4 flex items-center justify-between">
            <span>Running Servers</span>
            {servers.length > 0 && (
              <Button
                onClick={handleStopAllServers}
                disabled={isLoading}
                variant="destructive"
                size="sm"
              >
                Stop All
              </Button>
            )}
          </h3>
          {servers.length === 0 ? (
            <p className="text-sm text-muted-foreground">No servers running</p>
          ) : (
            <div className="space-y-2">
              {servers.map((server) => (
                <div
                  key={server.id}
                  className="flex items-center justify-between p-3 bg-muted rounded-lg"
                >
                  <div className="flex-1">
                    <p className="font-mono text-sm font-medium">
                      <a
                        href={server.url}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-primary hover:underline"
                      >
                        {server.url}
                      </a>
                    </p>
                    {server.staticDir && (
                      <p className="text-xs text-muted-foreground">
                        Serving: {server.staticDir}
                      </p>
                    )}
                  </div>
                  <Button
                    onClick={() => handleStopServer(server.id)}
                    disabled={isLoading}
                    variant="outline"
                    size="sm"
                  >
                    Stop
                  </Button>
                </div>
              ))}
            </div>
          )}
        </section>

        {/* Features Overview */}
        <section className="rounded-lg border border-border p-6">
          <h3 className="text-lg font-semibold mb-4">Planned Features</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div className="flex items-start gap-2">
              <span className="text-muted-foreground">•</span>
              <div>
                <p className="font-medium text-sm">Static File Serving</p>
                <p className="text-xs text-muted-foreground">
                  Serve HTML, CSS, JS, images, and other static assets
                </p>
              </div>
            </div>
            <div className="flex items-start gap-2">
              <span className="text-muted-foreground">•</span>
              <div>
                <p className="font-medium text-sm">Auto Port Assignment</p>
                <p className="text-xs text-muted-foreground">
                  Automatically find available ports
                </p>
              </div>
            </div>
            <div className="flex items-start gap-2">
              <span className="text-muted-foreground">•</span>
              <div>
                <p className="font-medium text-sm">CORS Support</p>
                <p className="text-xs text-muted-foreground">
                  Enable cross-origin requests for development
                </p>
              </div>
            </div>
            <div className="flex items-start gap-2">
              <span className="text-muted-foreground">•</span>
              <div>
                <p className="font-medium text-sm">Multiple Instances</p>
                <p className="text-xs text-muted-foreground">
                  Run multiple servers simultaneously
                </p>
              </div>
            </div>
            <div className="flex items-start gap-2">
              <span className="text-muted-foreground">•</span>
              <div>
                <p className="font-medium text-sm">Directory Listing</p>
                <p className="text-xs text-muted-foreground">
                  Optional file browser for served directories
                </p>
              </div>
            </div>
            <div className="flex items-start gap-2">
              <span className="text-muted-foreground">•</span>
              <div>
                <p className="font-medium text-sm">MIME Type Detection</p>
                <p className="text-xs text-muted-foreground">
                  Automatic content-type headers for files
                </p>
              </div>
            </div>
          </div>
        </section>

        {/* Output Log */}
        <section className="rounded-lg border border-border p-6">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold">Output Log</h3>
            <Button
              onClick={() => setOutput([])}
              variant="outline"
              size="sm"
            >
              Clear
            </Button>
          </div>
          <div className="bg-muted rounded-lg p-4 font-mono text-xs h-64 overflow-y-auto">
            {output.length === 0 ? (
              <p className="text-muted-foreground">No output yet...</p>
            ) : (
              output.map((line, i) => (
                <div
                  key={i}
                  className={line.includes('✗') ? 'text-red-500' : ''}
                >
                  {line}
                </div>
              ))
            )}
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
