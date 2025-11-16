import { createFileRoute } from '@tanstack/react-router'
import { Link as LinkIcon } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'

export const Route = createFileRoute('/deep-linking')({
  component: DeepLinking,
})

function DeepLinking() {
  return (
    <ModulePageLayout
      title="Deep Linking Module"
      description="Test opening the app via custom URL schemes like myapp://route."
      icon={LinkIcon}
    />
  )
}
