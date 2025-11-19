import { createFileRoute } from '@tanstack/react-router'
import { Timer, Play, XCircle, RefreshCw, Plus, Trash2 } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

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
  created_at: number
  scheduled_for?: number
  last_run?: number
  next_run?: number
  result?: TaskResult
}

interface TaskResult {
  success: boolean
  data?: any
  error?: string
  executed_at: number
  duration_ms: number
}

interface CreateTaskOptions {
  name: string
  description?: string
  schedule: {
    type: 'one-time' | 'periodic' | 'triggered'
    start_time?: number
    interval_ms?: number
    end_time?: number
  }
  priority?: 'high' | 'normal' | 'low'
  constraints?: {
    requires_network?: boolean
    requires_wifi?: boolean
    requires_charging?: boolean
    requires_battery_not_low?: boolean
    requires_device_idle?: boolean
    requires_storage_not_low?: boolean
  }
  retry_policy?: {
    max_retries: number
    backoff_multiplier?: number
    initial_backoff_ms?: number
    max_backoff_ms?: number
  }
  data?: any
}

function BackgroundTasks() {
  const [tasks, setTasks] = useState<BackgroundTask[]>([])
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)

  // Demo task form state
  const [taskName, setTaskName] = useState('Demo Task')
  const [taskDescription, setTaskDescription] = useState('A simple demo background task')
  const [delaySeconds, setDelaySeconds] = useState('2')

  useEffect(() => {
    // Load tasks on mount
    loadTasks()

    // Poll for task updates every 2 seconds
    const interval = setInterval(() => {
      loadTasks()
    }, 2000)

    return () => clearInterval(interval)
  }, [])

  const loadTasks = async () => {
    try {
      const taskList = await invoke<BackgroundTask[]>('list_background_tasks')
      setTasks(taskList)
    } catch (error) {
      console.error('Failed to load tasks:', error)
    }
  }

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const handleCreateDemoTask = async () => {
    const delay = parseInt(delaySeconds)
    if (isNaN(delay) || delay < 1) {
      addOutput('Please enter a valid delay (minimum 1 second)', false)
      return
    }

    setLoading('create')

    try {
      const options: CreateTaskOptions = {
        name: taskName,
        description: taskDescription,
        schedule: {
          type: 'one-time',
          start_time: Date.now() + delay * 1000,
        },
        priority: 'normal',
      }

      const taskId = await invoke<string>('create_background_task', { options })
      addOutput(`Created task "${taskName}" (ID: ${taskId})`)

      // Load updated tasks
      await loadTasks()

      // Reset form
      setTaskName('Demo Task')
      setTaskDescription('A simple demo background task')
    } catch (error) {
      addOutput(`Failed to create task: ${error}`, false)
    } finally {
      setLoading(null)
    }
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
    addOutput(`Executing task "${task.name}"...`)

    try {
      const delay = parseInt(delaySeconds)
      await invoke('execute_demo_task', {
        id: taskId,
        delaySeconds: delay,
      })
      addOutput(`Task "${task.name}" started`)
    } catch (error) {
      addOutput(`Error executing task: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleCancelTask = async (taskId: string) => {
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

    try {
      await invoke('cancel_background_task', { id: taskId })
      addOutput(`Cancelled task "${task.name}"`)
      await loadTasks()
    } catch (error) {
      addOutput(`Error cancelling task: ${error}`, false)
    }
  }

  const handleClearCompleted = async () => {
    const completedTasks = tasks.filter(
      (t) => t.status === 'completed' || t.status === 'failed' || t.status === 'cancelled'
    )

    if (completedTasks.length === 0) {
      addOutput('No completed tasks to clear', false)
      return
    }

    try {
      for (const task of completedTasks) {
        await invoke('delete_background_task', { id: task.id })
      }
      addOutput(`Cleared ${completedTasks.length} completed/failed/cancelled task(s)`)
      await loadTasks()
    } catch (error) {
      addOutput(`Error clearing tasks: ${error}`, false)
    }
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

  const formatTimestamp = (timestamp?: number) => {
    if (!timestamp) return 'N/A'
    return new Date(timestamp).toLocaleString()
  }

  return (
    <ModulePageLayout
      title="Background Tasks Module"
      description="Schedule and manage background tasks with Rust backend integration"
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
                placeholder="2"
                min="1"
              />
            </div>
            <Button
              onClick={handleCreateDemoTask}
              disabled={loading === 'create'}
            >
              {loading === 'create' ? (
                <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
              ) : (
                <Plus className="h-4 w-4 mr-2" />
              )}
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
                      <div className="flex items-center gap-2 flex-wrap">
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
                        <div className="font-mono">ID: {task.id}</div>
                        <div>Created: {formatTimestamp(task.created_at)}</div>
                        {task.scheduled_for && (
                          <div>Scheduled for: {formatTimestamp(task.scheduled_for)}</div>
                        )}
                        {task.last_run && (
                          <div>Last run: {formatTimestamp(task.last_run)}</div>
                        )}
                        {task.result && (
                          <div className={task.result.success ? 'text-green-600' : 'text-red-600'}>
                            <div>
                              Result: {task.result.success ? 'Success' : `Failed - ${task.result.error}`}
                              {' '}({task.result.duration_ms}ms)
                            </div>
                            {task.result.success && task.result.data && (
                              <div className="mt-1 text-xs bg-green-50 dark:bg-green-950 p-2 rounded">
                                {JSON.stringify(task.result.data, null, 2)}
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
                        title="Execute task"
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
                        title="Cancel task"
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
        <div className="bg-green-50 dark:bg-green-950 border border-green-200 dark:border-green-800 rounded-md p-4">
          <h4 className="font-medium text-green-900 dark:text-green-100 mb-2">
            Implementation Status
          </h4>
          <p className="text-sm text-green-800 dark:text-green-200">
            This module is now integrated with the Rust backend! Tasks are managed
            server-side with real async execution. The task list auto-updates every 2
            seconds to show current status.
          </p>
        </div>
      </div>
    </ModulePageLayout>
  )
}
