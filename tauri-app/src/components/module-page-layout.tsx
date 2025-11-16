import { ArrowLeft, type LucideIcon } from "lucide-react"
import { Link } from "@tanstack/react-router"
import { Button } from "@/components/ui/button"
import { type ReactNode } from "react"

type ModulePageLayoutProps = {
  title: string
  description: string
  icon?: LucideIcon
  children?: ReactNode
}

export function ModulePageLayout({ title, description, icon: Icon, children }: ModulePageLayoutProps) {
  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Link to="/">
          <Button variant="outline" size="icon">
            <ArrowLeft className="h-4 w-4" />
            <span className="sr-only">Back to home</span>
          </Button>
        </Link>
        <div className="flex-1">
          <div className="flex items-center gap-3">
            {Icon && (
              <div className="p-2 bg-primary/10 rounded-lg">
                <Icon className="h-6 w-6 text-primary" />
              </div>
            )}
            <h1 className="text-4xl font-bold">{title}</h1>
          </div>
          <p className="text-muted-foreground mt-2">{description}</p>
        </div>
      </div>

      {children ? (
        children
      ) : (
        <div className="border border-border rounded-lg p-6 bg-card">
          <p className="text-muted-foreground">
            This module is under development. Check back soon for implementation details.
          </p>
        </div>
      )}
    </div>
  )
}
