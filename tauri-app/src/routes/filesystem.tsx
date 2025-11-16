import { createFileRoute } from '@tanstack/react-router'
import { FileText } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'

export const Route = createFileRoute('/filesystem')({
  component: Filesystem,
})

function Filesystem() {
  return (
    <ModulePageLayout
      title="Filesystem Module"
      description="Read + write files, list directories, and test permissions on desktop & mobile."
      icon={FileText}
    />
  )
}
