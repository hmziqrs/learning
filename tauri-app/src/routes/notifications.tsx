import { createFileRoute } from '@tanstack/react-router'
import { Bell, Check, X, Clock, Send } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification'
import { invoke } from '@tauri-apps/api/core'

export const Route = createFileRoute('/notifications')({
  component: Notifications,
})

interface ScheduledNotification {
  id: number
  title: string
  body: string
  seconds: number
  scheduledAt: Date
}

function Notifications() {
  const [permissionGranted, setPermissionGranted] = useState<boolean | null>(null)
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)

  // Instant notification state
  const [instantTitle, setInstantTitle] = useState('Test Notification')
  const [instantBody, setInstantBody] = useState('This is a test notification from Tauri!')

  // Scheduled notification state
  const [scheduleTitle, setScheduleTitle] = useState('Scheduled Notification')
  const [scheduleBody, setScheduleBody] = useState('This notification was scheduled!')
  const [scheduleSeconds, setScheduleSeconds] = useState('5')
  const [scheduledNotifications, setScheduledNotifications] = useState<ScheduledNotification[]>([])

  useEffect(() => {
    checkPermission()
  }, [])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    setOutput((prev) => [...prev, `${icon} ${message}`])
  }

  const checkPermission = async () => {
    try {
      const granted = await isPermissionGranted()
      setPermissionGranted(granted)
      addOutput(`Permission status: ${granted ? 'Granted' : 'Not granted'}`, granted)
    } catch (error) {
      addOutput(`Error checking permission: ${error}`, false)
    }
  }

  const handleRequestPermission = async () => {
    setLoading('permission')
    try {
      const permission = await requestPermission()
      const granted = permission === 'granted'
      setPermissionGranted(granted)
      addOutput(`Permission ${granted ? 'granted' : 'denied'}`, granted)
    } catch (error) {
      addOutput(`Error requesting permission: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleSendInstantNotification = async () => {
    if (!permissionGranted) {
      addOutput('Permission not granted. Please request permission first.', false)
      return
    }

    setLoading('instant')
    try {
      await sendNotification({
        title: instantTitle,
        body: instantBody,
      })
      addOutput(`Sent notification: "${instantTitle}"`)
    } catch (error) {
      addOutput(`Error sending notification: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleScheduleNotification = async () => {
    if (!permissionGranted) {
      addOutput('Permission not granted. Please request permission first.', false)
      return
    }

    const seconds = parseInt(scheduleSeconds)
    if (isNaN(seconds) || seconds < 1) {
      addOutput('Please enter a valid number of seconds (minimum 1)', false)
      return
    }

    setLoading('schedule')
    try {
      await invoke('schedule_notification', {
        seconds,
        title: scheduleTitle,
        body: scheduleBody,
      })

      const newNotification: ScheduledNotification = {
        id: Date.now(),
        title: scheduleTitle,
        body: scheduleBody,
        seconds,
        scheduledAt: new Date(),
      }
      setScheduledNotifications((prev) => [...prev, newNotification])
      addOutput(`Scheduled notification "${scheduleTitle}" for ${seconds} seconds from now`)

      // Remove from list after it should have fired
      setTimeout(() => {
        setScheduledNotifications((prev) => prev.filter((n) => n.id !== newNotification.id))
      }, (seconds + 1) * 1000)
    } catch (error) {
      addOutput(`Error scheduling notification: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  return (
    <ModulePageLayout
      title="Notifications Module"
      description="Send local notifications + test future scheduling (app alive)."
      icon={Bell}
    >
      <div className="space-y-6">
        {/* Permission Section */}
        <div className="space-y-4">
          <h3 className="font-semibold">Permission Status</h3>
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              {permissionGranted === null ? (
                <span className="text-muted-foreground">Checking...</span>
              ) : permissionGranted ? (
                <>
                  <Check className="h-5 w-5 text-green-500" />
                  <span className="text-green-500">Granted</span>
                </>
              ) : (
                <>
                  <X className="h-5 w-5 text-red-500" />
                  <span className="text-red-500">Not Granted</span>
                </>
              )}
            </div>
            {!permissionGranted && (
              <Button
                onClick={handleRequestPermission}
                disabled={loading === 'permission'}
              >
                Request Permission
              </Button>
            )}
          </div>
        </div>

        {/* Instant Notification Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Send className="h-5 w-5" />
            Send Instant Notification
          </h3>
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium mb-1">Title</label>
              <input
                type="text"
                className="w-full px-3 py-2 border rounded-md"
                value={instantTitle}
                onChange={(e) => setInstantTitle(e.target.value)}
                placeholder="Notification title"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Body</label>
              <textarea
                className="w-full px-3 py-2 border rounded-md"
                rows={3}
                value={instantBody}
                onChange={(e) => setInstantBody(e.target.value)}
                placeholder="Notification body"
              />
            </div>
            <Button
              onClick={handleSendInstantNotification}
              disabled={loading === 'instant' || !permissionGranted}
            >
              <Send className="h-4 w-4 mr-2" />
              Send Notification
            </Button>
          </div>
        </div>

        {/* Scheduled Notification Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Clock className="h-5 w-5" />
            Schedule Notification
          </h3>
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium mb-1">Delay (seconds)</label>
              <input
                type="number"
                className="w-full px-3 py-2 border rounded-md"
                value={scheduleSeconds}
                onChange={(e) => setScheduleSeconds(e.target.value)}
                placeholder="5"
                min="1"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Title</label>
              <input
                type="text"
                className="w-full px-3 py-2 border rounded-md"
                value={scheduleTitle}
                onChange={(e) => setScheduleTitle(e.target.value)}
                placeholder="Notification title"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Body</label>
              <textarea
                className="w-full px-3 py-2 border rounded-md"
                rows={3}
                value={scheduleBody}
                onChange={(e) => setScheduleBody(e.target.value)}
                placeholder="Notification body"
              />
            </div>
            <Button
              onClick={handleScheduleNotification}
              disabled={loading === 'schedule' || !permissionGranted}
            >
              <Clock className="h-4 w-4 mr-2" />
              Schedule Notification
            </Button>
          </div>

          {/* Scheduled Notifications List */}
          {scheduledNotifications.length > 0 && (
            <div className="mt-4">
              <h4 className="text-sm font-medium mb-2">Scheduled Notifications:</h4>
              <div className="space-y-2">
                {scheduledNotifications.map((notif) => (
                  <div
                    key={notif.id}
                    className="text-sm p-3 bg-muted rounded-md border"
                  >
                    <div className="font-medium">{notif.title}</div>
                    <div className="text-muted-foreground text-xs">
                      Will appear in {notif.seconds} seconds (scheduled at{' '}
                      {notif.scheduledAt.toLocaleTimeString()})
                    </div>
                  </div>
                ))}
              </div>
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
                Operation results will appear here...
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
      </div>
    </ModulePageLayout>
  )
}
