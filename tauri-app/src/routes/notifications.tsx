import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/notifications')({
  component: Notifications,
})

function Notifications() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-4xl font-bold">Notifications Module</h1>
        <p className="text-muted-foreground mt-2">
          Send local notifications + test future scheduling (app alive).
        </p>
      </div>

      <div className="border border-border rounded-lg p-6 bg-card">
        <p className="text-muted-foreground">
          This module will demonstrate sending notifications and scheduling them for later delivery.
        </p>
      </div>
    </div>
  )
}
