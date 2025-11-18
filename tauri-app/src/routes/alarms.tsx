import { createFileRoute } from '@tanstack/react-router'
import { Clock } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'

export const Route = createFileRoute('/alarms')({
  component: Alarms,
})

function Alarms() {
  return (
    <ModulePageLayout
      title="Alarms (Future Notifications) Module"
      description="Lightweight alarm simulation using scheduled notifications with persistence."
      icon={Clock}
    />
  )
}
