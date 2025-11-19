import { createFileRoute } from '@tanstack/react-router'
import { Timer, Play, XCircle, RefreshCw, Plus, Trash2 } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'

export const Route = createFileRoute('/background-tasks')({
  component: BackgroundTasks,
})

interface BackgroundTask {
  id: string
  name: string
  description?: string
  type: 'one-time' | 'periodic' | 'triggered'
  status: 'pending' | 'running' | 'completed' | 'failed' | 'cancelled'
  priority: 'high' | 'normal' | 'low'
  createdAt: Date
  scheduledFor?: Date
  lastRun?: Date
  nextRun?: Date
  result?: TaskResult
}

interface TaskResult {
  success: boolean
  data?: any
  error?: string
  executedAt: Date
  duration: number
}

function BackgroundTasks() {
  const [tasks, setTasks] = useState<BackgroundTask[]>([])
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)

  // Demo task form state
  const [taskName, setTaskName] = useState('Demo Task')
  const [taskDescription, setTaskDescription] = useState('A simple demo background task')
  const [delaySeconds, setDelaySeconds] = useState('5')

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const handleCreateDemoTask = () => {
    const delay = parseInt(delaySeconds)
    if (isNaN(delay) || delay < 1) {
      addOutput('Please enter a valid delay (minimum 1 second)', false)
      return
    }

    // Create a demo task (frontend-only simulation)
    const newTask: BackgroundTask = {
      id: `task-${Date.now()}`,
      name: taskName,
      description: taskDescription,
      type: 'one-time',
      status: 'pending',
      priority: 'normal',
      createdAt: new Date(),
      scheduledFor: new Date(Date.now() + delay * 1000),
    }

    setTasks((prev) => [...prev, newTask])
    addOutput(`Created task "${taskName}" (ID: ${newTask.id})`)

    // Reset form
    setTaskName('Demo Task')
    setTaskDescription('A simple demo background task')
  }

  const handleExecuteTask = async (taskId: string) => {
    const task = tasks.find((t) => t.id === taskId)
    if (!task) return

    if (task.status === 'running') {
      addOutput(`Task "${task.name}" is already running`, false)
      return
    }

    if (task.status === 'cancelled') {
      addOutput(`Task "${task.name}" is cancelled`, false)
      return
    }

    setLoading(taskId)

    // Update task status to running
    setTasks((prev) =>
      prev.map((t) =>
        t.id === taskId
          ? { ...t, status: 'running' as const, lastRun: new Date() }
          : t
      )
    )

    addOutput(`Executing task "${task.name}"...`)

    try {
      // Simulate task execution
      const startTime = Date.now()
      await new Promise((resolve) => setTimeout(resolve, 2000))
      const duration = Date.now() - startTime

      // Randomly succeed or fail for demo purposes
      const success = Math.random() > 0.2 // 80% success rate

      const result: TaskResult = {
        success,
        data: success ? { message: 'Task completed successfully', executedAt: new Date() } : undefined,
        error: success ? undefined : 'Simulated task failure',
        executedAt: new Date(),
        duration,
      }

      // Update task with result
      setTasks((prev) =>
        prev.map((t) =>
          t.id === taskId
            ? {
                ...t,
                status: success ? ('completed' as const) : ('failed' as const),
                result,
              }
            : t
        )
      )

      if (success) {
        addOutput(`Task "${task.name}" completed successfully (${duration}ms)`)
      } else {
        addOutput(`Task "${task.name}" failed: ${result.error}`, false)
      }
    } catch (error) {
      addOutput(`Error executing task: ${error}`, false)
      setTasks((prev) =>
        prev.map((t) =>
          t.id === taskId
            ? { ...t, status: 'failed' as const }
            : t
        )
      )
    } finally {
      setLoading(null)
    }
  }

  const handleCancelTask = (taskId: string) => {
    const task = tasks.find((t) => t.id === taskId)
    if (!task) return

    if (task.status === 'running') {
      addOutput(`Cannot cancel running task "${task.name}"`, false)
      return
    }

    if (task.status === 'completed' || task.status === 'failed') {
      addOutput(`Cannot cancel ${task.status} task "${task.name}"`, false)
      return
    }

    setTasks((prev) =>
      prev.map((t) =>
        t.id === taskId
          ? { ...t, status: 'cancelled' as const }
          : t
      )
    )

    addOutput(`Cancelled task "${task.name}"`)
  }

  const handleClearCompleted = () => {
    const completedCount = tasks.filter(
      (t) => t.status === 'completed' || t.status === 'failed' || t.status === 'cancelled'
    ).length

    setTasks((prev) =>
      prev.filter((t) => t.status !== 'completed' && t.status !== 'failed' && t.status !== 'cancelled')
    )

    addOutput(`Cleared ${completedCount} completed/failed/cancelled task(s)`)
  }

  const getStatusColor = (status: BackgroundTask['status']) => {
    switch (status) {
      case 'pending':
        return 'text-yellow-500'
      case 'running':
        return 'text-blue-500'
      case 'completed':
        return 'text-green-500'
      case 'failed':
        return 'text-red-500'
      case 'cancelled':
        return 'text-gray-500'
      default:
        return 'text-muted-foreground'
    }
  }

  const getPriorityColor = (priority: BackgroundTask['priority']) => {
    switch (priority) {
      case 'high':
        return 'text-red-500'
      case 'normal':
        return 'text-blue-500'
      case 'low':
        return 'text-gray-500'
      default:
        return 'text-muted-foreground'
    }
  }

  return (
    <ModulePageLayout
      title="Background Tasks Module"
      description="Schedule and manage background tasks with execution monitoring"
      icon={Timer}
    >
      <div className="space-y-6">
        {/* Create Task Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Plus className="h-5 w-5" />
            Create Demo Task
          </h3>
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium mb-1">Task Name</label>
              <input
                type="text"
                className="w-full px-3 py-2 border rounded-md"
                value={taskName}
                onChange={(e) => setTaskName(e.target.value)}
                placeholder="Enter task name"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Description</label>
              <textarea
                className="w-full px-3 py-2 border rounded-md"
                rows={2}
                value={taskDescription}
                onChange={(e) => setTaskDescription(e.target.value)}
                placeholder="Enter task description"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">
                Execution Delay (seconds)
              </label>
              <input
                type="number"
                className="w-full px-3 py-2 border rounded-md"
                value={delaySeconds}
                onChange={(e) => setDelaySeconds(e.target.value)}
                placeholder="5"
                min="1"
              />
            </div>
            <Button onClick={handleCreateDemoTask}>
              <Plus className="h-4 w-4 mr-2" />
              Create Task
            </Button>
          </div>
        </div>

        {/* Task List Section */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold flex items-center gap-2">
              <Timer className="h-5 w-5" />
              Task List ({tasks.length})
            </h3>
            {tasks.length > 0 && (
              <Button
                variant="outline"
                size="sm"
                onClick={handleClearCompleted}
              >
                <Trash2 className="h-4 w-4 mr-2" />
                Clear Completed
              </Button>
            )}
          </div>

          {tasks.length === 0 ? (
            <div className="p-8 text-center border-2 border-dashed rounded-md">
              <Timer className="h-12 w-12 mx-auto mb-2 text-muted-foreground" />
              <p className="text-muted-foreground">
                No tasks created yet. Create a demo task to get started.
              </p>
            </div>
          ) : (
            <div className="space-y-3">
              {tasks.map((task) => (
                <div
                  key={task.id}
                  className="border rounded-md p-4 space-y-2"
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <h4 className="font-medium">{task.name}</h4>
                        <span
                          className={`text-xs px-2 py-0.5 rounded-full border ${getStatusColor(
                            task.status
                          )}`}
                        >
                          {task.status}
                        </span>
                        <span
                          className={`text-xs px-2 py-0.5 rounded-full border ${getPriorityColor(
                            task.priority
                          )}`}
                        >
                          {task.priority}
                        </span>
                      </div>
                      {task.description && (
                        <p className="text-sm text-muted-foreground mt-1">
                          {task.description}
                        </p>
                      )}
                      <div className="text-xs text-muted-foreground mt-2 space-y-1">
                        <div>ID: {task.id}</div>
                        <div>Created: {task.createdAt.toLocaleString()}</div>
                        {task.scheduledFor && (
                          <div>Scheduled for: {task.scheduledFor.toLocaleString()}</div>
                        )}
                        {task.lastRun && (
                          <div>Last run: {task.lastRun.toLocaleString()}</div>
                        )}
                        {task.result && (
                          <div className={task.result.success ? 'text-green-600' : 'text-red-600'}>
                            Result: {task.result.success ? 'Success' : `Failed - ${task.result.error}`}
                            {task.result.success && task.result.data && (
                              <div className="ml-2 text-xs">
                                {JSON.stringify(task.result.data)}
                              </div>
                            )}
                          </div>
                        )}
                      </div>
                    </div>
                    <div className="flex gap-2 ml-4">
                      <Button
                        size="sm"
                        variant="outline"
                        onClick={() => handleExecuteTask(task.id)}
                        disabled={
                          loading === task.id ||
                          task.status === 'running' ||
                          task.status === 'cancelled'
                        }
                      >
                        {loading === task.id ? (
                          <RefreshCw className="h-4 w-4 animate-spin" />
                        ) : (
                          <Play className="h-4 w-4" />
                        )}
                      </Button>
                      <Button
                        size="sm"
                        variant="outline"
                        onClick={() => handleCancelTask(task.id)}
                        disabled={
                          task.status === 'running' ||
                          task.status === 'completed' ||
                          task.status === 'failed' ||
                          task.status === 'cancelled'
                        }
                      >
                        <XCircle className="h-4 w-4" />
                      </Button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Output Panel */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold">Output</h3>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setOutput([])}
            >
              Clear
            </Button>
          </div>
          <div className="bg-muted p-4 rounded-md font-mono text-sm min-h-[150px] max-h-[300px] overflow-y-auto">
            {output.length === 0 ? (
              <p className="text-muted-foreground">
                Task operations and results will appear here...
              </p>
            ) : (
              <div className="space-y-1">
                {output.map((line, index) => (
                  <div key={index}>{line}</div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Info Box */}
        <div className="bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800 rounded-md p-4">
          <h4 className="font-medium text-blue-900 dark:text-blue-100 mb-2">
            Implementation Note
          </h4>
          <p className="text-sm text-blue-800 dark:text-blue-200">
            This is a frontend-only demo of the Background Tasks module. For full
            implementation with Rust backend integration, persistent storage, and
            platform-specific background execution, refer to the module documentation.
          </p>
        </div>
      </div>
    </ModulePageLayout>
  )
}
