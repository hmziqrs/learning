import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/filesystem')({
  component: Filesystem,
})

function Filesystem() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-4xl font-bold">Filesystem Module</h1>
        <p className="text-muted-foreground mt-2">
          Read + write files, list directories, and test permissions on desktop & mobile.
        </p>
      </div>

      <div className="border border-border rounded-lg p-6 bg-card">
        <p className="text-muted-foreground">
          This module will demonstrate file operations like creating folders, reading/writing files,
          and listing directories.
        </p>
      </div>
    </div>
  )
}
