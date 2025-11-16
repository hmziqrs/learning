import { createRootRoute, Link, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'

export const Route = createRootRoute({
  component: () => (
    <div className="min-h-screen bg-background text-foreground">
      <nav className="border-b border-border">
        <div className="container mx-auto px-4 py-4">
          <div className="flex gap-4">
            <Link
              to="/"
              className="text-foreground hover:text-primary transition-colors"
              activeProps={{ className: 'font-bold text-primary' }}
            >
              Home
            </Link>
            <Link
              to="/about"
              className="text-foreground hover:text-primary transition-colors"
              activeProps={{ className: 'font-bold text-primary' }}
            >
              About
            </Link>
          </div>
        </div>
      </nav>
      <main className="container mx-auto px-4 py-8">
        <Outlet />
      </main>
      <TanStackRouterDevtools />
    </div>
  ),
})
