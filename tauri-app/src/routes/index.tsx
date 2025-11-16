import { createFileRoute } from '@tanstack/react-router'
import { Button } from '@/components/ui/button'

export const Route = createFileRoute('/')({
  component: Index,
})

function Index() {
  return (
    <div className="space-y-4">
      <h1 className="text-4xl font-bold">Welcome to Tauri + React + TanStack Router</h1>
      <p className="text-muted-foreground">
        This is a Tauri application with:
      </p>
      <ul className="list-disc list-inside space-y-2 text-muted-foreground">
        <li>React 19</li>
        <li>TanStack Router (client-side only, no SSR)</li>
        <li>shadcn/ui components</li>
        <li>Tailwind CSS v4</li>
        <li>TypeScript</li>
      </ul>
      <div className="flex gap-2 mt-6">
        <Button>Default Button</Button>
        <Button variant="secondary">Secondary</Button>
        <Button variant="outline">Outline</Button>
        <Button variant="destructive">Destructive</Button>
      </div>
    </div>
  )
}
