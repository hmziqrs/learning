import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/deep-linking')({
  component: DeepLinking,
})

function DeepLinking() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-4xl font-bold">Deep Linking Module</h1>
        <p className="text-muted-foreground mt-2">
          Test opening the app via custom URL schemes like myapp://route.
        </p>
      </div>

      <div className="border border-border rounded-lg p-6 bg-card">
        <p className="text-muted-foreground">
          This module will demonstrate deep linking functionality to open specific routes in the app.
        </p>
      </div>
    </div>
  )
}
