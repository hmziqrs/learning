import { createFileRoute } from '@tanstack/react-router'
import { PlayCircle } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'

export const Route = createFileRoute('/media')({
  component: Media,
})

function Media() {
  return (
    <ModulePageLayout
      title="Media Module"
      description="Play local videos, audio files, and test OS media controls."
      icon={PlayCircle}
    />
  )
}
