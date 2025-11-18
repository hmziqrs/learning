import { createFileRoute } from '@tanstack/react-router'
import { Clipboard, Copy, FileText, Clock } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState } from 'react'
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager'

export const Route = createFileRoute('/clipboard')({
  component: ClipboardModule,
})

interface ClipboardHistoryItem {
  id: number
  content: string
  timestamp: Date
  charCount: number
}

function ClipboardModule() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)
  const [clipboardContent, setClipboardContent] = useState<string>('')
  const [textToCopy, setTextToCopy] = useState<string>('Hello from Tauri Clipboard!')
  const [history, setHistory] = useState<ClipboardHistoryItem[]>([])
  const [historyId, setHistoryId] = useState<number>(1)

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    setOutput((prev) => [...prev, `${icon} ${message}`])
  }

  const addToHistory = (content: string) => {
    const newItem: ClipboardHistoryItem = {
      id: historyId,
      content: content,
      timestamp: new Date(),
      charCount: content.length,
    }
    setHistory((prev) => [newItem, ...prev].slice(0, 10)) // Keep only last 10 items
    setHistoryId((prev) => prev + 1)
  }

  const handleCopyText = async () => {
    if (!textToCopy.trim()) {
      addOutput('Cannot copy empty text', false)
      return
    }

    setLoading('copy')
    try {
      await writeText(textToCopy)
      addOutput(`Copied to clipboard: "${textToCopy.substring(0, 50)}${textToCopy.length > 50 ? '...' : ''}"`)
      addToHistory(textToCopy)
    } catch (error) {
      addOutput(`Error copying to clipboard: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleReadClipboard = async () => {
    setLoading('read')
    try {
      const text = await readText()
      if (text === null || text === undefined) {
        setClipboardContent('')
        addOutput('Clipboard is empty or contains non-text data', false)
      } else {
        setClipboardContent(text)
        addOutput(`Read ${text.length} characters from clipboard`)
      }
    } catch (error) {
      addOutput(`Error reading from clipboard: ${error}`, false)
      setClipboardContent('')
    } finally {
      setLoading(null)
    }
  }

  const handleClearClipboard = async () => {
    setLoading('clear')
    try {
      await writeText('')
      setClipboardContent('')
      addOutput('Clipboard cleared')
    } catch (error) {
      addOutput(`Error clearing clipboard: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleCopyQuick = async (text: string, label: string) => {
    setLoading(`quick-${label}`)
    try {
      await writeText(text)
      addOutput(`Copied ${label} to clipboard`)
      addToHistory(text)
    } catch (error) {
      addOutput(`Error copying ${label}: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleCopyFromHistory = async (item: ClipboardHistoryItem) => {
    setLoading(`history-${item.id}`)
    try {
      await writeText(item.content)
      addOutput(`Copied from history: "${item.content.substring(0, 50)}${item.content.length > 50 ? '...' : ''}"`)
    } catch (error) {
      addOutput(`Error copying from history: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const getCurrentTimestamp = () => {
    return new Date().toISOString()
  }

  const getCurrentDate = () => {
    return new Date().toLocaleDateString('en-US', {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    })
  }

  const getLoremIpsum = () => {
    return 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.'
  }

  return (
    <ModulePageLayout
      title="Clipboard Module"
      description="Read and write to the system clipboard. Test text operations and clipboard history."
      icon={Clipboard}
    >
      <div className="space-y-6">
        {/* Copy Text Section */}
        <div className="space-y-4">
          <h3 className="text-lg font-semibold flex items-center gap-2">
            <Copy className="w-5 h-5" />
            Copy Text
          </h3>
          <div className="space-y-3">
            <textarea
              className="w-full p-3 bg-card border border-border rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-primary"
              rows={4}
              value={textToCopy}
              onChange={(e) => setTextToCopy(e.target.value)}
              placeholder="Enter text to copy..."
            />
            <Button
              onClick={handleCopyText}
              disabled={loading === 'copy'}
              className="w-full"
            >
              {loading === 'copy' ? 'Copying...' : 'Copy to Clipboard'}
            </Button>
          </div>
        </div>

        {/* Read Clipboard Section */}
        <div className="space-y-4">
          <h3 className="text-lg font-semibold flex items-center gap-2">
            <FileText className="w-5 h-5" />
            Read Clipboard
          </h3>
          <div className="space-y-3">
            <Button
              onClick={handleReadClipboard}
              disabled={loading === 'read'}
              variant="outline"
              className="w-full"
            >
              {loading === 'read' ? 'Reading...' : 'Read from Clipboard'}
            </Button>
            {clipboardContent && (
              <div className="p-4 bg-muted border border-border rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm font-medium">Clipboard Content:</span>
                  <span className="text-xs text-muted-foreground">
                    {clipboardContent.length} characters
                  </span>
                </div>
                <pre className="text-sm whitespace-pre-wrap break-words">
                  {clipboardContent}
                </pre>
              </div>
            )}
          </div>
        </div>

        {/* Quick Actions Section */}
        <div className="space-y-4">
          <h3 className="text-lg font-semibold flex items-center gap-2">
            <Clock className="w-5 h-5" />
            Quick Copy Actions
          </h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <Button
              onClick={() => handleCopyQuick(getCurrentTimestamp(), 'timestamp')}
              disabled={loading === 'quick-timestamp'}
              variant="outline"
            >
              {loading === 'quick-timestamp' ? 'Copying...' : 'Copy Timestamp'}
            </Button>
            <Button
              onClick={() => handleCopyQuick(getCurrentDate(), 'date')}
              disabled={loading === 'quick-date'}
              variant="outline"
            >
              {loading === 'quick-date' ? 'Copying...' : 'Copy Date'}
            </Button>
            <Button
              onClick={() => handleCopyQuick(getLoremIpsum(), 'Lorem Ipsum')}
              disabled={loading === 'quick-Lorem Ipsum'}
              variant="outline"
            >
              {loading === 'quick-Lorem Ipsum' ? 'Copying...' : 'Copy Lorem Ipsum'}
            </Button>
            <Button
              onClick={handleClearClipboard}
              disabled={loading === 'clear'}
              variant="destructive"
            >
              {loading === 'clear' ? 'Clearing...' : 'Clear Clipboard'}
            </Button>
          </div>
        </div>

        {/* Clipboard History Section */}
        {history.length > 0 && (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-lg font-semibold flex items-center gap-2">
                <Clock className="w-5 h-5" />
                Clipboard History
              </h3>
              <Button
                onClick={() => setHistory([])}
                variant="ghost"
                size="sm"
              >
                Clear History
              </Button>
            </div>
            <div className="space-y-2">
              {history.map((item) => (
                <div
                  key={item.id}
                  className="p-3 bg-card border border-border rounded-lg hover:border-primary/50 transition-colors"
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex-1 min-w-0">
                      <p className="text-sm truncate mb-1">{item.content}</p>
                      <div className="flex items-center gap-3 text-xs text-muted-foreground">
                        <span>{item.timestamp.toLocaleTimeString()}</span>
                        <span>{item.charCount} chars</span>
                      </div>
                    </div>
                    <Button
                      onClick={() => handleCopyFromHistory(item)}
                      disabled={loading === `history-${item.id}`}
                      variant="ghost"
                      size="sm"
                    >
                      {loading === `history-${item.id}` ? '...' : 'Copy'}
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Output Panel */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold">Output</h3>
            {output.length > 0 && (
              <Button onClick={() => setOutput([])} variant="ghost" size="sm">
                Clear
              </Button>
            )}
          </div>
          <div className="p-4 bg-muted border border-border rounded-lg min-h-[100px] max-h-[300px] overflow-y-auto">
            {output.length === 0 ? (
              <p className="text-muted-foreground text-sm">
                Operation results will appear here...
              </p>
            ) : (
              <div className="space-y-1">
                {output.map((line, index) => (
                  <p key={index} className="text-sm font-mono">
                    {line}
                  </p>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </ModulePageLayout>
  )
}
