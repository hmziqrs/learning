import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/drag-drop')({
  component: DragDrop,
})

function DragDrop() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-4xl font-bold">Drag & Drop Module</h1>
        <p className="text-muted-foreground mt-2">
          Test Tauri's native file-drop + HTML5 drag drop functionality.
        </p>
      </div>

      <div className="border border-border rounded-lg p-6 bg-card">
        <p className="text-muted-foreground">
          This module will demonstrate drag and drop functionality for files.
        </p>
      </div>
    </div>
  )
}
