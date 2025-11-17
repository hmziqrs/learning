import { createFileRoute } from '@tanstack/react-router'
import { useState } from 'react'
import { FileText, FolderPlus, FileEdit, FileSearch, Trash2, CheckCircle, List } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import {
  mkdir,
  writeTextFile,
  readTextFile,
  readDir,
  exists,
  remove,
} from '@tauri-apps/plugin-fs'
import { appDataDir } from '@tauri-apps/api/path'

export const Route = createFileRoute('/filesystem')({
  component: Filesystem,
})

interface FileEntry {
  name: string
  isDirectory: boolean
}

function Filesystem() {
  const [output, setOutput] = useState<string>('')
  const [files, setFiles] = useState<FileEntry[]>([])
  const [loading, setLoading] = useState(false)
  const [folderName, setFolderName] = useState('test-folder')
  const [fileName, setFileName] = useState('sample.txt')
  const [fileContent, setFileContent] = useState('Hello from Tauri!')

  const addOutput = (message: string) => {
    setOutput((prev) => `${prev}\n${message}`)
  }

  const handleCreateFolder = async () => {
    try {
      setLoading(true)
      const appDir = await appDataDir()
      const folderPath = `${appDir}/${folderName}`
      await mkdir(folderPath, { recursive: true })
      addOutput(`✓ Created folder: ${folderPath}`)
    } catch (error) {
      addOutput(`✗ Error creating folder: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const handleWriteFile = async () => {
    try {
      setLoading(true)
      const appDir = await appDataDir()
      const filePath = `${appDir}/${fileName}`
      await writeTextFile(filePath, fileContent)
      addOutput(`✓ Written file: ${filePath}`)
    } catch (error) {
      addOutput(`✗ Error writing file: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const handleReadFile = async () => {
    try {
      setLoading(true)
      const appDir = await appDataDir()
      const filePath = `${appDir}/${fileName}`
      const content = await readTextFile(filePath)
      addOutput(`✓ Read file: ${filePath}`)
      addOutput(`Content: ${content}`)
    } catch (error) {
      addOutput(`✗ Error reading file: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const handleListDirectory = async () => {
    try {
      setLoading(true)
      const appDir = await appDataDir()
      const entries = await readDir(appDir)
      const fileList: FileEntry[] = entries.map((entry) => ({
        name: entry.name || 'unknown',
        isDirectory: entry.isDirectory,
      }))
      setFiles(fileList)
      addOutput(`✓ Listed directory: ${appDir}`)
      addOutput(`Found ${fileList.length} items`)
    } catch (error) {
      addOutput(`✗ Error listing directory: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const handleDeleteFile = async () => {
    try {
      setLoading(true)
      const appDir = await appDataDir()
      const filePath = `${appDir}/${fileName}`
      await remove(filePath)
      addOutput(`✓ Deleted file: ${filePath}`)
    } catch (error) {
      addOutput(`✗ Error deleting file: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const handleCheckExists = async () => {
    try {
      setLoading(true)
      const appDir = await appDataDir()
      const filePath = `${appDir}/${fileName}`
      const fileExists = await exists(filePath)
      addOutput(`File ${filePath} ${fileExists ? 'EXISTS' : 'DOES NOT EXIST'}`)
    } catch (error) {
      addOutput(`✗ Error checking existence: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  const handleClearOutput = () => {
    setOutput('')
    setFiles([])
  }

  return (
    <ModulePageLayout
      title="Filesystem Module"
      description="Read + write files, list directories, and test permissions on desktop & mobile."
      icon={FileText}
    >
      <div className="space-y-6">
        {/* Input Fields */}
        <div className="space-y-4 p-4 bg-card border border-border rounded-lg">
          <div>
            <label className="block text-sm font-medium mb-2">Folder Name</label>
            <input
              data-testid="folder-name-input"
              type="text"
              value={folderName}
              onChange={(e) => setFolderName(e.target.value)}
              className="w-full px-3 py-2 bg-background border border-border rounded-md"
              placeholder="folder-name"
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-2">File Name</label>
            <input
              data-testid="file-name-input"
              type="text"
              value={fileName}
              onChange={(e) => setFileName(e.target.value)}
              className="w-full px-3 py-2 bg-background border border-border rounded-md"
              placeholder="file.txt"
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-2">File Content</label>
            <textarea
              data-testid="file-content-textarea"
              value={fileContent}
              onChange={(e) => setFileContent(e.target.value)}
              className="w-full px-3 py-2 bg-background border border-border rounded-md h-24"
              placeholder="Enter file content..."
            />
          </div>
        </div>

        {/* Action Buttons */}
        <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
          <Button
            onClick={handleCreateFolder}
            disabled={loading}
            className="gap-2"
          >
            <FolderPlus className="w-4 h-4" />
            Create Folder
          </Button>
          <Button
            onClick={handleWriteFile}
            disabled={loading}
            className="gap-2"
          >
            <FileEdit className="w-4 h-4" />
            Write File
          </Button>
          <Button
            onClick={handleReadFile}
            disabled={loading}
            className="gap-2"
          >
            <FileSearch className="w-4 h-4" />
            Read File
          </Button>
          <Button
            onClick={handleListDirectory}
            disabled={loading}
            className="gap-2"
          >
            <List className="w-4 h-4" />
            List Directory
          </Button>
          <Button
            onClick={handleCheckExists}
            disabled={loading}
            className="gap-2"
          >
            <CheckCircle className="w-4 h-4" />
            Check Exists
          </Button>
          <Button
            onClick={handleDeleteFile}
            disabled={loading}
            variant="destructive"
            className="gap-2"
          >
            <Trash2 className="w-4 h-4" />
            Delete File
          </Button>
        </div>

        {/* File List */}
        {files.length > 0 && (
          <div className="p-4 bg-card border border-border rounded-lg">
            <h3 className="text-lg font-semibold mb-3">Directory Contents</h3>
            <div className="space-y-2">
              {files.map((file, index) => (
                <div
                  key={index}
                  className="flex items-center gap-2 p-2 bg-muted rounded"
                >
                  {file.isDirectory ? (
                    <FolderPlus className="w-4 h-4 text-blue-500" />
                  ) : (
                    <FileText className="w-4 h-4 text-green-500" />
                  )}
                  <span className="text-sm">{file.name}</span>
                  <span className="text-xs text-muted-foreground ml-auto">
                    {file.isDirectory ? 'Directory' : 'File'}
                  </span>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Output Panel */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold">Output</h3>
            <Button
              onClick={handleClearOutput}
              variant="outline"
              size="sm"
            >
              Clear
            </Button>
          </div>
          <pre className="p-4 bg-muted border border-border rounded-lg text-sm overflow-auto max-h-96 whitespace-pre-wrap">
            {output || 'No output yet. Try performing some filesystem operations above.'}
          </pre>
        </div>
      </div>
    </ModulePageLayout>
  )
}
