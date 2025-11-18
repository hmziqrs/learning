import { createFileRoute } from '@tanstack/react-router'
import { Database, Save, Trash2, Check, X, HardDrive, Settings } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'

export const Route = createFileRoute('/sql-storage')({
  component: SqlStorage,
})

interface Note {
  id: number
  title: string
  content: string
  created_at: string
}

function SqlStorage() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)

  // User preferences state
  const [userName, setUserName] = useState('')
  const [savedUserName, setSavedUserName] = useState('')
  const [isDarkMode, setIsDarkMode] = useState(false)

  // Notes state (SQLite demo)
  const [noteTitle, setNoteTitle] = useState('')
  const [noteContent, setNoteContent] = useState('')
  const [notes, setNotes] = useState<Note[]>([])

  // Storage stats
  const [preferencesCount, setPreferencesCount] = useState(0)
  const [notesCount, setNotesCount] = useState(0)

  useEffect(() => {
    initializeStorage()
  }, [])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const initializeStorage = async () => {
    setLoading('init')
    try {
      // TODO: Initialize database and store
      // For now, use localStorage as placeholder
      const storedName = localStorage.getItem('userName') || ''
      const storedTheme = localStorage.getItem('isDarkMode') === 'true'
      const storedNotes = localStorage.getItem('notes')

      setSavedUserName(storedName)
      setUserName(storedName)
      setIsDarkMode(storedTheme)

      if (storedNotes) {
        setNotes(JSON.parse(storedNotes))
      }

      updateStats()
      addOutput('Storage initialized successfully')
    } catch (error) {
      addOutput(`Error initializing storage: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const updateStats = () => {
    // Count preferences
    let prefs = 0
    if (localStorage.getItem('userName')) prefs++
    if (localStorage.getItem('isDarkMode')) prefs++
    setPreferencesCount(prefs)

    // Count notes
    const storedNotes = localStorage.getItem('notes')
    if (storedNotes) {
      setNotesCount(JSON.parse(storedNotes).length)
    }
  }

  const handleSaveUserName = async () => {
    setLoading('saveName')
    try {
      // TODO: Save to store plugin
      localStorage.setItem('userName', userName)
      setSavedUserName(userName)
      updateStats()
      addOutput(`User name saved: "${userName}"`)
    } catch (error) {
      addOutput(`Error saving user name: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleToggleDarkMode = async () => {
    setLoading('toggleTheme')
    const newValue = !isDarkMode
    try {
      // TODO: Save to store plugin
      localStorage.setItem('isDarkMode', String(newValue))
      setIsDarkMode(newValue)
      updateStats()
      addOutput(`Dark mode ${newValue ? 'enabled' : 'disabled'}`)
    } catch (error) {
      addOutput(`Error toggling dark mode: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleAddNote = async () => {
    if (!noteTitle.trim()) {
      addOutput('Note title is required', false)
      return
    }

    setLoading('addNote')
    try {
      const newNote: Note = {
        id: Date.now(),
        title: noteTitle,
        content: noteContent,
        created_at: new Date().toISOString(),
      }

      const updatedNotes = [...notes, newNote]
      setNotes(updatedNotes)

      // TODO: Save to SQLite database
      localStorage.setItem('notes', JSON.stringify(updatedNotes))

      setNoteTitle('')
      setNoteContent('')
      updateStats()
      addOutput(`Note added: "${noteTitle}"`)
    } catch (error) {
      addOutput(`Error adding note: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleDeleteNote = async (id: number, title: string) => {
    setLoading(`deleteNote-${id}`)
    try {
      const updatedNotes = notes.filter((note) => note.id !== id)
      setNotes(updatedNotes)

      // TODO: Delete from SQLite database
      localStorage.setItem('notes', JSON.stringify(updatedNotes))

      updateStats()
      addOutput(`Note deleted: "${title}"`)
    } catch (error) {
      addOutput(`Error deleting note: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleClearAllData = async () => {
    if (!confirm('Are you sure you want to clear all data? This action cannot be undone.')) {
      return
    }

    setLoading('clearAll')
    try {
      // TODO: Clear database and store
      localStorage.removeItem('userName')
      localStorage.removeItem('isDarkMode')
      localStorage.removeItem('notes')

      setUserName('')
      setSavedUserName('')
      setIsDarkMode(false)
      setNotes([])
      updateStats()

      addOutput('All data cleared successfully')
    } catch (error) {
      addOutput(`Error clearing data: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const clearOutput = () => setOutput([])

  return (
    <ModulePageLayout
      title="SQL + Storage Module"
      description="Store user state, preferences, and application data using SQLite database and key-value store"
      icon={Database}
    >
      <div className="space-y-6">
        {/* Storage Stats */}
        <div className="bg-card border border-border rounded-lg p-6">
          <h2 className="text-2xl font-semibold mb-4 flex items-center gap-2">
            <HardDrive className="w-6 h-6" />
            Storage Statistics
          </h2>
          <div className="grid grid-cols-2 gap-4">
            <div className="bg-muted p-4 rounded-lg">
              <div className="text-sm text-muted-foreground">Preferences Stored</div>
              <div className="text-2xl font-bold">{preferencesCount}</div>
            </div>
            <div className="bg-muted p-4 rounded-lg">
              <div className="text-sm text-muted-foreground">Notes Stored</div>
              <div className="text-2xl font-bold">{notesCount}</div>
            </div>
          </div>
        </div>

        {/* User Preferences (Store Plugin) */}
        <div className="bg-card border border-border rounded-lg p-6">
          <h2 className="text-2xl font-semibold mb-4 flex items-center gap-2">
            <Settings className="w-6 h-6" />
            User Preferences (Key-Value Store)
          </h2>

          <div className="space-y-4">
            {/* User Name */}
            <div>
              <label className="block text-sm font-medium mb-2">User Name</label>
              <div className="flex gap-2">
                <input
                  type="text"
                  value={userName}
                  onChange={(e) => setUserName(e.target.value)}
                  placeholder="Enter your name"
                  className="flex-1 px-3 py-2 bg-background border border-border rounded-md"
                />
                <Button
                  onClick={handleSaveUserName}
                  disabled={loading === 'saveName'}
                  className="gap-2"
                >
                  <Save className="w-4 h-4" />
                  {loading === 'saveName' ? 'Saving...' : 'Save'}
                </Button>
              </div>
              {savedUserName && (
                <p className="text-sm text-muted-foreground mt-2">
                  Saved name: <span className="font-semibold">{savedUserName}</span>
                </p>
              )}
            </div>

            {/* Dark Mode Toggle */}
            <div className="flex items-center justify-between">
              <div>
                <div className="font-medium">Dark Mode</div>
                <div className="text-sm text-muted-foreground">
                  Theme preference (currently: {isDarkMode ? 'Dark' : 'Light'})
                </div>
              </div>
              <Button
                onClick={handleToggleDarkMode}
                disabled={loading === 'toggleTheme'}
                variant={isDarkMode ? 'default' : 'outline'}
                className="gap-2"
              >
                {isDarkMode ? <Check className="w-4 h-4" /> : <X className="w-4 h-4" />}
                {loading === 'toggleTheme' ? 'Updating...' : isDarkMode ? 'Enabled' : 'Disabled'}
              </Button>
            </div>
          </div>
        </div>

        {/* Notes Demo (SQLite) */}
        <div className="bg-card border border-border rounded-lg p-6">
          <h2 className="text-2xl font-semibold mb-4 flex items-center gap-2">
            <Database className="w-6 h-6" />
            Notes (SQLite Database)
          </h2>

          {/* Add Note Form */}
          <div className="space-y-3 mb-6">
            <div>
              <label className="block text-sm font-medium mb-2">Note Title</label>
              <input
                type="text"
                value={noteTitle}
                onChange={(e) => setNoteTitle(e.target.value)}
                placeholder="Enter note title"
                className="w-full px-3 py-2 bg-background border border-border rounded-md"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-2">Note Content</label>
              <textarea
                value={noteContent}
                onChange={(e) => setNoteContent(e.target.value)}
                placeholder="Enter note content"
                rows={3}
                className="w-full px-3 py-2 bg-background border border-border rounded-md"
              />
            </div>
            <Button
              onClick={handleAddNote}
              disabled={loading === 'addNote'}
              className="gap-2"
            >
              <Database className="w-4 h-4" />
              {loading === 'addNote' ? 'Adding...' : 'Add Note'}
            </Button>
          </div>

          {/* Notes List */}
          <div className="space-y-3">
            <h3 className="font-semibold">Saved Notes ({notes.length})</h3>
            {notes.length === 0 ? (
              <p className="text-sm text-muted-foreground">No notes yet. Add one above!</p>
            ) : (
              <div className="space-y-2">
                {notes.map((note) => (
                  <div
                    key={note.id}
                    className="bg-muted p-4 rounded-lg flex justify-between items-start"
                  >
                    <div className="flex-1">
                      <h4 className="font-semibold">{note.title}</h4>
                      {note.content && (
                        <p className="text-sm text-muted-foreground mt-1">{note.content}</p>
                      )}
                      <p className="text-xs text-muted-foreground mt-2">
                        {new Date(note.created_at).toLocaleString()}
                      </p>
                    </div>
                    <Button
                      onClick={() => handleDeleteNote(note.id, note.title)}
                      disabled={loading === `deleteNote-${note.id}`}
                      variant="destructive"
                      size="sm"
                      className="gap-1"
                    >
                      <Trash2 className="w-3 h-3" />
                    </Button>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Data Management */}
        <div className="bg-card border border-border rounded-lg p-6">
          <h2 className="text-2xl font-semibold mb-4">Data Management</h2>
          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Clear all stored data including preferences and notes. This action cannot be undone.
            </p>
            <Button
              onClick={handleClearAllData}
              disabled={loading === 'clearAll'}
              variant="destructive"
              className="gap-2"
            >
              <Trash2 className="w-4 h-4" />
              {loading === 'clearAll' ? 'Clearing...' : 'Clear All Data'}
            </Button>
          </div>
        </div>

        {/* Output Panel */}
        <div className="bg-card border border-border rounded-lg p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-2xl font-semibold">Output</h2>
            <Button onClick={clearOutput} variant="outline" size="sm">
              Clear
            </Button>
          </div>
          <div className="bg-muted rounded-md p-4 max-h-96 overflow-y-auto font-mono text-sm">
            {output.length === 0 ? (
              <p className="text-muted-foreground">No operations yet</p>
            ) : (
              output.map((line, i) => (
                <div
                  key={i}
                  className={
                    line.includes('✗') ? 'text-red-500' : line.includes('✓') ? 'text-green-500' : ''
                  }
                >
                  {line}
                </div>
              ))
            )}
          </div>
        </div>
      </div>
    </ModulePageLayout>
  )
}
