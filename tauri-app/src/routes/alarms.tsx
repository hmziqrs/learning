import { createFileRoute } from '@tanstack/react-router'
import { Clock, Plus, Trash2, Check, X, Bell } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification'
import { invoke } from '@tauri-apps/api/core'

export const Route = createFileRoute('/alarms')({
  component: Alarms,
})

interface Alarm {
  id: number
  title: string
  scheduledTime: Date
  isActive: boolean
  createdAt: Date
  firedAt?: Date
}

function Alarms() {
  const [permissionGranted, setPermissionGranted] = useState<boolean | null>(null)
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)

  // Alarm form state
  const [alarmTitle, setAlarmTitle] = useState('Wake up!')
  const [alarmDate, setAlarmDate] = useState('')
  const [alarmTime, setAlarmTime] = useState('')

  // Alarms list
  const [alarms, setAlarms] = useState<Alarm[]>([])
  const [nextAlarmId, setNextAlarmId] = useState(1)
  const [currentTime, setCurrentTime] = useState(new Date())

  useEffect(() => {
    checkPermission()
    loadAlarms()

    // Update current time every second
    const interval = setInterval(() => {
      setCurrentTime(new Date())
      checkAndFireAlarms()
    }, 1000)

    return () => clearInterval(interval)
  }, [])

  useEffect(() => {
    saveAlarms()
  }, [alarms])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
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

  const loadAlarms = () => {
    try {
      const saved = localStorage.getItem('tauri-alarms')
      if (saved) {
        const parsed = JSON.parse(saved)
        const loadedAlarms = parsed.alarms.map((a: any) => ({
          ...a,
          scheduledTime: new Date(a.scheduledTime),
          createdAt: new Date(a.createdAt),
          firedAt: a.firedAt ? new Date(a.firedAt) : undefined,
        }))
        setAlarms(loadedAlarms)
        setNextAlarmId(parsed.nextId || 1)
        addOutput(`Loaded ${loadedAlarms.length} alarm(s) from storage`)
      }
    } catch (error) {
      addOutput(`Error loading alarms: ${error}`, false)
    }
  }

  const saveAlarms = () => {
    try {
      localStorage.setItem('tauri-alarms', JSON.stringify({
        alarms,
        nextId: nextAlarmId,
      }))
    } catch (error) {
      console.error('Error saving alarms:', error)
    }
  }

  const handleAddAlarm = async () => {
    if (!permissionGranted) {
      addOutput('Permission not granted. Please request permission first.', false)
      return
    }

    if (!alarmTitle.trim()) {
      addOutput('Please enter an alarm title', false)
      return
    }

    if (!alarmDate || !alarmTime) {
      addOutput('Please select both date and time', false)
      return
    }

    setLoading('add')
    try {
      const scheduledTime = new Date(`${alarmDate}T${alarmTime}`)
      const now = new Date()

      if (scheduledTime <= now) {
        addOutput('Alarm time must be in the future', false)
        setLoading(null)
        return
      }

      const newAlarm: Alarm = {
        id: nextAlarmId,
        title: alarmTitle,
        scheduledTime,
        isActive: true,
        createdAt: now,
      }

      // Schedule the notification
      const seconds = Math.floor((scheduledTime.getTime() - now.getTime()) / 1000)
      await invoke('schedule_notification', {
        seconds,
        title: alarmTitle,
        body: `Alarm: ${alarmTitle}`,
      })

      setAlarms((prev) => [...prev, newAlarm].sort((a, b) =>
        a.scheduledTime.getTime() - b.scheduledTime.getTime()
      ))
      setNextAlarmId((prev) => prev + 1)

      addOutput(`Alarm added: "${alarmTitle}" at ${scheduledTime.toLocaleString()}`)

      // Reset form
      setAlarmTitle('Wake up!')
      setAlarmDate('')
      setAlarmTime('')
    } catch (error) {
      addOutput(`Error adding alarm: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleDeleteAlarm = (id: number) => {
    const alarm = alarms.find((a) => a.id === id)
    if (alarm) {
      setAlarms((prev) => prev.filter((a) => a.id !== id))
      addOutput(`Deleted alarm: "${alarm.title}"`)
    }
  }

  const handleToggleAlarm = (id: number) => {
    setAlarms((prev) =>
      prev.map((a) =>
        a.id === id ? { ...a, isActive: !a.isActive } : a
      )
    )
    const alarm = alarms.find((a) => a.id === id)
    if (alarm) {
      addOutput(`Alarm "${alarm.title}" ${!alarm.isActive ? 'activated' : 'deactivated'}`)
    }
  }

  const checkAndFireAlarms = () => {
    const now = new Date()
    alarms.forEach((alarm) => {
      if (
        alarm.isActive &&
        !alarm.firedAt &&
        alarm.scheduledTime <= now
      ) {
        // Mark as fired
        setAlarms((prev) =>
          prev.map((a) =>
            a.id === alarm.id ? { ...a, firedAt: now, isActive: false } : a
          )
        )
        addOutput(`🔔 Alarm fired: "${alarm.title}"`)
      }
    })
  }

  const getTimeUntil = (scheduledTime: Date): string => {
    const diff = scheduledTime.getTime() - currentTime.getTime()

    if (diff <= 0) return 'Now'

    const days = Math.floor(diff / (1000 * 60 * 60 * 24))
    const hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60))
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60))
    const seconds = Math.floor((diff % (1000 * 60)) / 1000)

    const parts = []
    if (days > 0) parts.push(`${days}d`)
    if (hours > 0) parts.push(`${hours}h`)
    if (minutes > 0) parts.push(`${minutes}m`)
    if (seconds > 0 && days === 0) parts.push(`${seconds}s`)

    return parts.join(' ') || 'Now'
  }

  const getDefaultDate = () => {
    const today = new Date()
    return today.toISOString().split('T')[0]
  }

  const getDefaultTime = () => {
    const now = new Date()
    now.setMinutes(now.getMinutes() + 5)
    return now.toTimeString().slice(0, 5)
  }

  // Quick preset functions
  const setQuickAlarm = (seconds: number, label: string) => {
    const now = new Date()
    const scheduledTime = new Date(now.getTime() + seconds * 1000)

    setAlarmDate(scheduledTime.toISOString().split('T')[0])
    setAlarmTime(scheduledTime.toTimeString().slice(0, 5))
    setAlarmTitle(label)

    addOutput(`Quick alarm set: ${label}`)
  }

  const setQuickDate = (daysAhead: number) => {
    const date = new Date()
    date.setDate(date.getDate() + daysAhead)
    setAlarmDate(date.toISOString().split('T')[0])
    addOutput(`Date set to: ${daysAhead === 0 ? 'Today' : daysAhead === 1 ? 'Tomorrow' : `${daysAhead} days ahead`}`)
  }

  const activeAlarms = alarms.filter((a) => a.isActive && !a.firedAt)
  const firedAlarms = alarms.filter((a) => a.firedAt)

  return (
    <ModulePageLayout
      title="Alarms (Future Notifications) Module"
      description="Lightweight alarm simulation using scheduled notifications with persistence."
      icon={Clock}
    >
      <div className="space-y-6">
        {/* Permission Section */}
        <div className="space-y-4">
          <h3 className="text-lg font-semibold flex items-center gap-2">
            <Bell className="w-5 h-5" />
            Notification Permission
          </h3>
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              <span className="text-sm">Status:</span>
              {permissionGranted === null ? (
                <span className="text-muted-foreground">Checking...</span>
              ) : permissionGranted ? (
                <span className="flex items-center gap-1 text-green-500">
                  <Check className="w-4 h-4" />
                  Granted
                </span>
              ) : (
                <span className="flex items-center gap-1 text-red-500">
                  <X className="w-4 h-4" />
                  Not Granted
                </span>
              )}
            </div>
            {!permissionGranted && (
              <Button
                onClick={handleRequestPermission}
                disabled={loading === 'permission'}
                size="sm"
              >
                {loading === 'permission' ? 'Requesting...' : 'Request Permission'}
              </Button>
            )}
          </div>
        </div>

        {/* Add Alarm Section */}
        <div className="space-y-4">
          <h3 className="text-lg font-semibold flex items-center gap-2">
            <Plus className="w-5 h-5" />
            Add New Alarm
          </h3>
          <div className="space-y-4">
            {/* Quick Presets */}
            <div className="space-y-3">
              <h4 className="text-sm font-medium text-muted-foreground">Quick Alarms</h4>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setQuickAlarm(30, 'Alarm in 30s')}
                  className="text-xs"
                >
                  30 secs
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setQuickAlarm(60, 'Alarm in 1 min')}
                  className="text-xs"
                >
                  1 min
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setQuickAlarm(120, 'Alarm in 2 mins')}
                  className="text-xs"
                >
                  2 mins
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setQuickAlarm(300, 'Alarm in 5 mins')}
                  className="text-xs"
                >
                  5 mins
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setQuickAlarm(600, 'Alarm in 10 mins')}
                  className="text-xs"
                >
                  10 mins
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setQuickAlarm(1800, 'Alarm in 30 mins')}
                  className="text-xs"
                >
                  30 mins
                </Button>
              </div>
            </div>

            <div className="border-t border-border pt-4 space-y-3">
              <h4 className="text-sm font-medium text-muted-foreground">Custom Alarm</h4>

              <div>
                <label className="block text-sm font-medium mb-1">Alarm Title</label>
                <input
                  type="text"
                  className="w-full p-2 bg-card border border-border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                  value={alarmTitle}
                  onChange={(e) => setAlarmTitle(e.target.value)}
                  placeholder="Enter alarm title..."
                />
              </div>

              <div className="space-y-2">
                <label className="block text-sm font-medium">Date</label>
                <div className="flex gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setQuickDate(0)}
                    className="text-xs"
                  >
                    Today
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setQuickDate(1)}
                    className="text-xs"
                  >
                    Tomorrow
                  </Button>
                </div>
                <input
                  type="date"
                  className="w-full p-2 bg-card border border-border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                  value={alarmDate}
                  onChange={(e) => setAlarmDate(e.target.value)}
                  min={getDefaultDate()}
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">Time</label>
                <input
                  type="time"
                  className="w-full p-2 bg-card border border-border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                  value={alarmTime}
                  onChange={(e) => setAlarmTime(e.target.value)}
                />
              </div>
            </div>

            <Button
              onClick={handleAddAlarm}
              disabled={loading === 'add' || !permissionGranted}
              className="w-full"
            >
              {loading === 'add' ? 'Adding...' : 'Add Alarm'}
            </Button>
          </div>
        </div>

        {/* Active Alarms Section */}
        {activeAlarms.length > 0 && (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-lg font-semibold flex items-center gap-2">
                <Clock className="w-5 h-5" />
                Active Alarms ({activeAlarms.length})
              </h3>
            </div>
            <div className="space-y-2">
              {activeAlarms.map((alarm) => (
                <div
                  key={alarm.id}
                  className="p-4 bg-card border border-border rounded-lg"
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex-1 min-w-0">
                      <h4 className="font-semibold mb-1">{alarm.title}</h4>
                      <div className="space-y-1 text-sm text-muted-foreground">
                        <p>📅 {alarm.scheduledTime.toLocaleString()}</p>
                        <p className="font-medium text-primary">
                          ⏱️ {getTimeUntil(alarm.scheduledTime)} remaining
                        </p>
                      </div>
                    </div>
                    <div className="flex gap-2">
                      <Button
                        onClick={() => handleToggleAlarm(alarm.id)}
                        variant="outline"
                        size="sm"
                      >
                        {alarm.isActive ? 'Pause' : 'Resume'}
                      </Button>
                      <Button
                        onClick={() => handleDeleteAlarm(alarm.id)}
                        variant="destructive"
                        size="sm"
                      >
                        <Trash2 className="w-4 h-4" />
                      </Button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Fired Alarms Section */}
        {firedAlarms.length > 0 && (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-lg font-semibold">Alarm History ({firedAlarms.length})</h3>
              <Button
                onClick={() => setAlarms((prev) => prev.filter((a) => !a.firedAt))}
                variant="ghost"
                size="sm"
              >
                Clear History
              </Button>
            </div>
            <div className="space-y-2">
              {firedAlarms.map((alarm) => (
                <div
                  key={alarm.id}
                  className="p-3 bg-muted border border-border rounded-lg opacity-60"
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex-1 min-w-0">
                      <p className="font-medium text-sm">{alarm.title}</p>
                      <p className="text-xs text-muted-foreground">
                        Fired at: {alarm.firedAt?.toLocaleString()}
                      </p>
                    </div>
                    <Button
                      onClick={() => handleDeleteAlarm(alarm.id)}
                      variant="ghost"
                      size="sm"
                    >
                      <Trash2 className="w-3 h-3" />
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Output Panel */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold">Output</h3>
            {output.length > 0 && (
              <Button onClick={() => setOutput([])} variant="ghost" size="sm">
                Clear
              </Button>
            )}
          </div>
          <div className="p-4 bg-muted border border-border rounded-lg min-h-[100px] max-h-[300px] overflow-y-auto">
            {output.length === 0 ? (
              <p className="text-muted-foreground text-sm">
                Operation results will appear here...
              </p>
            ) : (
              <div className="space-y-1">
                {output.map((line, index) => (
                  <p key={index} className="text-sm font-mono">
                    {line}
                  </p>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </ModulePageLayout>
  )
}
