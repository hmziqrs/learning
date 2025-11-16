import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/about')({
  component: About,
})

function About() {
  return (
    <div className="space-y-4">
      <h1 className="text-4xl font-bold">About</h1>
      <p className="text-muted-foreground">
        This is an example about page demonstrating TanStack Router's file-based routing.
      </p>
      <div className="bg-card text-card-foreground p-6 rounded-lg border border-border">
        <h2 className="text-2xl font-semibold mb-2">Features</h2>
        <p>
          This setup provides a modern development experience with type-safe routing,
          component-based UI with shadcn/ui, and the power of Tauri for building
          lightweight desktop applications.
        </p>
      </div>
    </div>
  )
}
