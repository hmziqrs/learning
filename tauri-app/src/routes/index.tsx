import { createFileRoute, Link } from '@tanstack/react-router'
import {
  FileText,
  Bell,
  Link as LinkIcon,
  PlayCircle,
  Upload,
  Clock,
  Calendar,
  Database,
  DollarSign,
  Wifi,
  Images,
  Users,
  Camera,
  Activity,
  Timer,
} from 'lucide-react';

export const Route = createFileRoute('/')({
  component: Index,
})

const modules = [
  {
    name: 'Filesystem',
    description: 'Read + write files, list directories, and test permissions',
    icon: FileText,
    path: '/filesystem',
    color: 'text-blue-500',
  },
  {
    name: 'Notifications',
    description: 'Send local notifications + test future scheduling',
    icon: Bell,
    path: '/notifications',
    color: 'text-green-500',
  },
  {
    name: 'Deep Linking',
    description: 'Test opening the app via custom URL schemes',
    icon: LinkIcon,
    path: '/deep-linking',
    color: 'text-purple-500',
  },
  {
    name: 'Media',
    description: 'Play local videos, audio files, and test OS media controls',
    icon: PlayCircle,
    path: '/media',
    color: 'text-orange-500',
  },
  {
    name: 'Drag & Drop',
    description: "Test Tauri's native file-drop + HTML5 drag drop",
    icon: Upload,
    path: '/drag-drop',
    color: 'text-pink-500',
  },
  {
    name: 'Alarms',
    description: 'Lightweight alarm simulation with scheduled notifications',
    icon: Clock,
    path: '/alarms',
    color: 'text-yellow-500',
  },
  {
    name: 'Calendar',
    description: 'Internal calendar with event management and ICS export',
    icon: Calendar,
    path: '/calendar',
    color: 'text-teal-500',
  },
  {
    name: 'In-App Purchases',
    description: 'Test platform billing: iOS IAP, Android Billing, desktop Stripe',
    icon: DollarSign,
    path: '/in-app-purchases',
    color: 'text-emerald-500',
  },
  {
    name: 'SQL + Storage',
    description: 'Store user state, preferences, and data using SQLite and key-value store',
    icon: Database,
    path: '/sql-storage',
    color: 'text-indigo-500',
  },
  {
    name: 'Networking & Radio Access',
    description: 'HTTP/WebSocket communication, network monitoring, and radio hardware info',
    icon: Wifi,
    path: '/network-realtime',
    color: 'text-violet-500',
  },
  {
    name: 'Contacts',
    description: 'Access device contacts with read capabilities (iOS, Android, macOS)',
    icon: Users,
    path: '/contacts',
    color: 'text-cyan-500',
  },
  {
    name: 'Gallery / Media Library',
    description: 'Pick photos and videos from device storage with preview capabilities',
    icon: Images,
    path: '/gallery',
    color: 'text-rose-500',
  },
  {
    name: 'Camera',
    description: 'Capture photos and record videos directly from device cameras',
    icon: Camera,
    path: '/camera',
    color: 'text-red-500',
  },
  {
    name: 'Sensors & Hardware',
    description: 'Access device sensors for motion tracking, compass, and GPS location',
    icon: Activity,
    path: '/sensors',
    color: 'text-amber-500',
  },
  {
    name: 'Background Tasks',
    description: 'Schedule and manage background tasks with execution monitoring',
    icon: Timer,
    path: '/background-tasks',
    color: 'text-fuchsia-500',
  },
]

function Index() {
  return (
    <div className="space-y-8">
      {/* Hero Section */}
      <div className="text-center space-y-4 py-8">
        <h1 className="text-5xl font-bold bg-gradient-to-r from-primary to-purple-600 bg-clip-text text-transparent">
          Tauri Capability Playground
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Explore and test every Tauri capability in one comprehensive application.
          Built with React 19, TanStack Router, and shadcn/ui.
        </p>
      </div>

      {/* Tech Stack */}
      <div className="flex flex-wrap justify-center gap-3 py-4">
        <div className="px-4 py-2 bg-card border border-border rounded-full text-sm">
          React 19
        </div>
        <div className="px-4 py-2 bg-card border border-border rounded-full text-sm">
          Tauri 2.9
        </div>
        <div className="px-4 py-2 bg-card border border-border rounded-full text-sm">
          TanStack Router
        </div>
        <div className="px-4 py-2 bg-card border border-border rounded-full text-sm">
          shadcn/ui
        </div>
        <div className="px-4 py-2 bg-card border border-border rounded-full text-sm">
          Tailwind CSS v4
        </div>
      </div>

      {/* Modules Grid */}
      <div>
        <h2 className="text-3xl font-bold mb-6">Available Modules</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {modules.map((module) => {
            const Icon = module.icon
            return (
              <Link
                key={module.path}
                to={module.path}
                className="group"
              >
                <div className="h-full p-6 bg-card border border-border rounded-lg hover:shadow-lg hover:border-primary/50 transition-all duration-200">
                  <div className="flex items-start gap-4">
                    <div className={`p-3 bg-muted rounded-lg group-hover:scale-110 transition-transform ${module.color}`}>
                      <Icon className="w-6 h-6" />
                    </div>
                    <div className="flex-1">
                      <h3 className="text-xl font-semibold mb-2 group-hover:text-primary transition-colors">
                        {module.name}
                      </h3>
                      <p className="text-sm text-muted-foreground">
                        {module.description}
                      </p>
                    </div>
                  </div>
                </div>
              </Link>
            )
          })}
        </div>
      </div>

      {/* Info Section */}
      <div className="mt-12 p-6 bg-muted/50 rounded-lg border border-border">
        <h3 className="text-lg font-semibold mb-3">About This Project</h3>
        <p className="text-muted-foreground">
          This playground demonstrates various Tauri capabilities across desktop and mobile platforms.
          Each module provides a hands-on example of specific Tauri features, making it easy to
          understand and implement these capabilities in your own projects.
        </p>
      </div>
    </div>
  )
}

