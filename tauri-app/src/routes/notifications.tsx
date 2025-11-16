import { createFileRoute } from '@tanstack/react-router'
import { Bell } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'

export const Route = createFileRoute('/notifications')({
  component: Notifications,
})

function Notifications() {
  return (
    <ModulePageLayout
      title="Notifications Module"
      description="Send local notifications + test future scheduling (app alive)."
      icon={Bell}
    />
  )
}
