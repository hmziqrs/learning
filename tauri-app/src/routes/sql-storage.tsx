import { createFileRoute } from '@tanstack/react-router'
import { Database as DatabaseIcon, Save, Trash2, Check, X, HardDrive, Settings, Download, FileJson, FileSpreadsheet, Share2, Upload } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect, useRef } from 'react'
import Database from '@tauri-apps/plugin-sql'
import { Store } from '@tauri-apps/plugin-store'
import { save, open as openDialog } from '@tauri-apps/plugin-dialog'
import { writeTextFile, readTextFile, BaseDirectory } from '@tauri-apps/plugin-fs'
import { openPath } from '@tauri-apps/plugin-opener'
import { tempDir } from '@tauri-apps/api/path'

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
  const dbRef = useRef<Database | null>(null)
  const storeRef = useRef<Store | null>(null)

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

    return () => {
      // Cleanup on unmount
      if (dbRef.current) {
        dbRef.current.close().catch(console.error)
      }
    }
  }, [])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const initializeStorage = async () => {
    setLoading('init')
    try {
      // Initialize Store plugin
      const store = await Store.load('settings.json')
      storeRef.current = store
      addOutput('Store plugin initialized')

      // Load preferences from store
      const storedName = (await store.get('userName')) as string | null
      const storedTheme = (await store.get('isDarkMode')) as boolean | null

      if (storedName) {
        setSavedUserName(storedName)
        setUserName(storedName)
      }
      if (storedTheme !== null) {
        setIsDarkMode(storedTheme)
      }

      // Initialize SQLite database
      const db = await Database.load('sqlite:storage.db')
      dbRef.current = db
      addOutput('SQLite database initialized')

      // Create notes table if not exists
      await db.execute(`
        CREATE TABLE IF NOT EXISTS notes (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          title TEXT NOT NULL,
          content TEXT,
          created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
      `)
      addOutput('Notes table created/verified')

      // Load notes from database
      await loadNotes()

      await updateStats()
      addOutput('Storage initialized successfully')
    } catch (error) {
      addOutput(`Error initializing storage: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const loadNotes = async () => {
    try {
      if (!dbRef.current) {
        throw new Error('Database not initialized')
      }

      const result = await dbRef.current.select<Note[]>('SELECT * FROM notes ORDER BY created_at DESC')
      setNotes(result)
      addOutput(`Loaded ${result.length} notes from database`)
    } catch (error) {
      addOutput(`Error loading notes: ${error}`, false)
    }
  }

  const updateStats = async () => {
    try {
      // Count preferences in store
      let prefs = 0
      if (storeRef.current) {
        const hasUserName = await storeRef.current.has('userName')
        const hasTheme = await storeRef.current.has('isDarkMode')
        if (hasUserName) prefs++
        if (hasTheme) prefs++
      }
      setPreferencesCount(prefs)

      // Count notes in database
      setNotesCount(notes.length)
    } catch (error) {
      console.error('Error updating stats:', error)
    }
  }

  const handleSaveUserName = async () => {
    setLoading('saveName')
    try {
      if (!storeRef.current) {
        throw new Error('Store not initialized')
      }

      await storeRef.current.set('userName', userName)
      await storeRef.current.save()
      setSavedUserName(userName)
      await updateStats()
      addOutput(`User name saved to store: "${userName}"`)
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
      if (!storeRef.current) {
        throw new Error('Store not initialized')
      }

      await storeRef.current.set('isDarkMode', newValue)
      await storeRef.current.save()
      setIsDarkMode(newValue)
      await updateStats()
      addOutput(`Dark mode ${newValue ? 'enabled' : 'disabled'} in store`)
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
      if (!dbRef.current) {
        throw new Error('Database not initialized')
      }

      const result = await dbRef.current.execute(
        'INSERT INTO notes (title, content) VALUES (?, ?)',
        [noteTitle, noteContent]
      )

      addOutput(`Note added to database (ID: ${result.lastInsertId})`)

      // Reload notes from database
      await loadNotes()

      setNoteTitle('')
      setNoteContent('')
      await updateStats()
    } catch (error) {
      addOutput(`Error adding note: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleDeleteNote = async (id: number, title: string) => {
    setLoading(`deleteNote-${id}`)
    try {
      if (!dbRef.current) {
        throw new Error('Database not initialized')
      }

      await dbRef.current.execute('DELETE FROM notes WHERE id = ?', [id])
      addOutput(`Note deleted from database: "${title}"`)

      // Reload notes from database
      await loadNotes()
      await updateStats()
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
      // Clear database
      if (dbRef.current) {
        await dbRef.current.execute('DELETE FROM notes')
        addOutput('All notes deleted from database')
      }

      // Clear store
      if (storeRef.current) {
        await storeRef.current.clear()
        await storeRef.current.save()
        addOutput('All preferences cleared from store')
      }

      // Reset state
      setUserName('')
      setSavedUserName('')
      setIsDarkMode(false)
      setNotes([])
      await updateStats()

      addOutput('All data cleared successfully')
    } catch (error) {
      addOutput(`Error clearing data: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleExportNotesJSON = async () => {
    setLoading('exportJSON')
    try {
      if (notes.length === 0) {
        addOutput('No notes to export', false)
        return
      }

      // Format data for export
      const exportData = {
        exportDate: new Date().toISOString(),
        totalNotes: notes.length,
        notes: notes,
      }

      const jsonString = JSON.stringify(exportData, null, 2)

      // Show save dialog
      const filePath = await save({
        filters: [{
          name: 'JSON',
          extensions: ['json']
        }],
        defaultPath: `notes-export-${Date.now()}.json`
      })

      if (!filePath) {
        addOutput('Export cancelled')
        return
      }

      // Write file
      await writeTextFile(filePath, jsonString)
      addOutput(`Notes exported to JSON: ${filePath}`)
    } catch (error) {
      addOutput(`Error exporting notes to JSON: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleExportNotesCSV = async () => {
    setLoading('exportCSV')
    try {
      if (notes.length === 0) {
        addOutput('No notes to export', false)
        return
      }

      // Generate CSV content
      const headers = ['ID', 'Title', 'Content', 'Created At']
      const csvRows = [headers.join(',')]

      notes.forEach(note => {
        const row = [
          note.id,
          `"${note.title.replace(/"/g, '""')}"`, // Escape quotes
          `"${note.content.replace(/"/g, '""')}"`,
          `"${new Date(note.created_at).toLocaleString()}"`
        ]
        csvRows.push(row.join(','))
      })

      const csvString = csvRows.join('\n')

      // Show save dialog
      const filePath = await save({
        filters: [{
          name: 'CSV',
          extensions: ['csv']
        }],
        defaultPath: `notes-export-${Date.now()}.csv`
      })

      if (!filePath) {
        addOutput('Export cancelled')
        return
      }

      // Write file
      await writeTextFile(filePath, csvString)
      addOutput(`Notes exported to CSV: ${filePath}`)
    } catch (error) {
      addOutput(`Error exporting notes to CSV: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleExportAll = async () => {
    setLoading('exportAll')
    try {
      // Gather all data
      const preferences: Record<string, any> = {}

      if (storeRef.current) {
        const hasUserName = await storeRef.current.has('userName')
        const hasTheme = await storeRef.current.has('isDarkMode')

        if (hasUserName) {
          preferences.userName = await storeRef.current.get('userName')
        }
        if (hasTheme) {
          preferences.isDarkMode = await storeRef.current.get('isDarkMode')
        }
      }

      const exportData = {
        exportDate: new Date().toISOString(),
        version: '1.0',
        data: {
          preferences,
          notes: {
            count: notes.length,
            items: notes
          }
        }
      }

      const jsonString = JSON.stringify(exportData, null, 2)

      // Show save dialog
      const filePath = await save({
        filters: [{
          name: 'JSON',
          extensions: ['json']
        }],
        defaultPath: `complete-export-${Date.now()}.json`
      })

      if (!filePath) {
        addOutput('Export cancelled')
        return
      }

      // Write file
      await writeTextFile(filePath, jsonString)
      addOutput(`Complete data exported to: ${filePath}`)
    } catch (error) {
      addOutput(`Error exporting all data: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleShareData = async () => {
    setLoading('share')
    try {
      // Gather all data
      const preferences: Record<string, any> = {}

      if (storeRef.current) {
        const hasUserName = await storeRef.current.has('userName')
        const hasTheme = await storeRef.current.has('isDarkMode')

        if (hasUserName) {
          preferences.userName = await storeRef.current.get('userName')
        }
        if (hasTheme) {
          preferences.isDarkMode = await storeRef.current.get('isDarkMode')
        }
      }

      const exportData = {
        exportDate: new Date().toISOString(),
        version: '1.0',
        data: {
          preferences,
          notes: {
            count: notes.length,
            items: notes
          }
        }
      }

      const jsonString = JSON.stringify(exportData, null, 2)
      const filename = `tauri-storage-${Date.now()}.json`

      // Write to temp directory
      await writeTextFile(filename, jsonString, { baseDir: BaseDirectory.Temp })
      addOutput(`Data prepared for sharing: ${filename}`)

      // Open with native share sheet (on mobile) or default handler (on desktop)
      try {
        const tempDirPath = await tempDir()
        const fullPath = `${tempDirPath}${filename}`
        await openPath(fullPath)
        addOutput('Share sheet opened (mobile) or file opened with default app (desktop)')
      } catch (err) {
        addOutput(`Note: File saved to temp directory. ${err}`, false)
      }
    } catch (error) {
      addOutput(`Error sharing data: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleImportData = async () => {
    setLoading('import')
    try {
      // Show open dialog
      const selected = await openDialog({
        filters: [{
          name: 'JSON',
          extensions: ['json']
        }],
        multiple: false
      })

      if (!selected || typeof selected !== 'string') {
        addOutput('Import cancelled')
        return
      }

      // Read file
      const fileContent = await readTextFile(selected)
      const importedData = JSON.parse(fileContent)

      // Validate data structure
      if (!importedData.data) {
        throw new Error('Invalid export file format')
      }

      addOutput(`Reading import file: ${selected}`)

      // Import preferences
      if (importedData.data.preferences && storeRef.current) {
        const prefs = importedData.data.preferences

        if (prefs.userName) {
          await storeRef.current.set('userName', prefs.userName)
          setUserName(prefs.userName)
          setSavedUserName(prefs.userName)
        }

        if (prefs.isDarkMode !== undefined) {
          await storeRef.current.set('isDarkMode', prefs.isDarkMode)
          setIsDarkMode(prefs.isDarkMode)
        }

        await storeRef.current.save()
        addOutput('Preferences imported successfully')
      }

      // Import notes
      if (importedData.data.notes?.items && dbRef.current) {
        const importedNotes = importedData.data.notes.items

        for (const note of importedNotes) {
          await dbRef.current.execute(
            'INSERT INTO notes (title, content, created_at) VALUES (?, ?, ?)',
            [note.title, note.content || '', note.created_at]
          )
        }

        addOutput(`Imported ${importedNotes.length} notes`)
        await loadNotes()
      }

      await updateStats()
      addOutput('Import completed successfully')
    } catch (error) {
      addOutput(`Error importing data: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const clearOutput = () => setOutput([])

  return (
    <ModulePageLayout
      title="SQL + Storage Module"
      description="Store user state, preferences, and application data using SQLite database and key-value store"
      icon={DatabaseIcon}
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
            <DatabaseIcon className="w-6 h-6" />
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
              <DatabaseIcon className="w-4 h-4" />
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

        {/* Export Data */}
        <div className="bg-card border border-border rounded-lg p-6">
          <h2 className="text-2xl font-semibold mb-4 flex items-center gap-2">
            <Download className="w-6 h-6" />
            Export Data
          </h2>
          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Export your data to various formats for backup or analysis.
            </p>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              <Button
                onClick={handleExportNotesJSON}
                disabled={loading === 'exportJSON' || notes.length === 0}
                variant="outline"
                className="gap-2"
              >
                <FileJson className="w-4 h-4" />
                {loading === 'exportJSON' ? 'Exporting...' : 'Export Notes (JSON)'}
              </Button>
              <Button
                onClick={handleExportNotesCSV}
                disabled={loading === 'exportCSV' || notes.length === 0}
                variant="outline"
                className="gap-2"
              >
                <FileSpreadsheet className="w-4 h-4" />
                {loading === 'exportCSV' ? 'Exporting...' : 'Export Notes (CSV)'}
              </Button>
              <Button
                onClick={handleExportAll}
                disabled={loading === 'exportAll'}
                variant="outline"
                className="gap-2"
              >
                <Download className="w-4 h-4" />
                {loading === 'exportAll' ? 'Exporting...' : 'Export Everything'}
              </Button>
            </div>
            {notes.length === 0 && (
              <p className="text-xs text-muted-foreground">
                Add some notes first to enable export functionality
              </p>
            )}
          </div>
        </div>

        {/* Share & Import */}
        <div className="bg-card border border-border rounded-lg p-6">
          <h2 className="text-2xl font-semibold mb-4 flex items-center gap-2">
            <Share2 className="w-6 h-6" />
            Share & Import
          </h2>
          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Share your data via native share sheet (mobile) or import backup files.
            </p>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <Button
                onClick={handleShareData}
                disabled={loading === 'share'}
                variant="default"
                className="gap-2"
              >
                <Share2 className="w-4 h-4" />
                {loading === 'share' ? 'Preparing...' : 'Share Data (Native)'}
              </Button>
              <Button
                onClick={handleImportData}
                disabled={loading === 'import'}
                variant="default"
                className="gap-2"
              >
                <Upload className="w-4 h-4" />
                {loading === 'import' ? 'Importing...' : 'Import Backup'}
              </Button>
            </div>
            <div className="text-xs text-muted-foreground space-y-1">
              <p>• Share: Opens native share sheet on mobile, default app on desktop</p>
              <p>• Import: Restore data from previously exported JSON files</p>
              <p>• Import will add notes (not replace) and update preferences</p>
            </div>
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
