import { createFileRoute } from '@tanstack/react-router'
import { Upload } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'

export const Route = createFileRoute('/drag-drop')({
  component: DragDrop,
})

function DragDrop() {
  return (
    <ModulePageLayout
      title="Drag & Drop Module"
      description="Test Tauri's native file-drop + HTML5 drag drop functionality."
      icon={Upload}
    />
  )
}
