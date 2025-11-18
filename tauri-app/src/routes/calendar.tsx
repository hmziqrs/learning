import { createFileRoute } from '@tanstack/react-router'
import { Calendar as CalendarIcon, Plus, Trash2, Download, ExternalLink } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-opener'

export const Route = createFileRoute('/calendar')({
  component: CalendarModule,
})

interface Event {
  id: number
  title: string
  description?: string
  start_time: string
  end_time: string
  is_all_day: boolean
  created_at: string
  updated_at: string
}

function CalendarModule() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)

  // Event form state
  const [eventTitle, setEventTitle] = useState('')
  const [eventDescription, setEventDescription] = useState('')
  const [startDate, setStartDate] = useState('')
  const [startTime, setStartTime] = useState('')
  const [endDate, setEndDate] = useState('')
  const [endTime, setEndTime] = useState('')
  const [isAllDay, setIsAllDay] = useState(false)

  // Events list
  const [events, setEvents] = useState<Event[]>([])

  useEffect(() => {
    initDatabase()
  }, [])

  const initDatabase = async () => {
    try {
      await invoke('init_calendar_db')
      addOutput('Database initialized successfully')
      await loadEvents()
    } catch (error) {
      addOutput(`Error initializing database: ${error}`, false)
    }
  }

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const loadEvents = async () => {
    setLoading('loading')
    try {
      const loadedEvents = await invoke<Event[]>('get_events')
      setEvents(loadedEvents)
      addOutput(`Loaded ${loadedEvents.length} events`)
    } catch (error) {
      addOutput(`Error loading events: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleAddEvent = async () => {
    if (!eventTitle.trim()) {
      addOutput('Event title is required', false)
      return
    }

    if (!startDate || !endDate) {
      addOutput('Start and end dates are required', false)
      return
    }

    const startDateTime = isAllDay
      ? `${startDate}T00:00:00`
      : `${startDate}T${startTime || '00:00'}:00`
    const endDateTime = isAllDay
      ? `${endDate}T23:59:59`
      : `${endDate}T${endTime || '23:59'}:00`

    const startDateObj = new Date(startDateTime)
    const endDateObj = new Date(endDateTime)

    if (endDateObj < startDateObj) {
      addOutput('End time must be after start time', false)
      return
    }

    setLoading('adding')
    try {
      await invoke('create_event', {
        title: eventTitle,
        description: eventDescription || null,
        startTime: startDateTime,
        endTime: endDateTime,
        isAllDay: isAllDay,
      })

      addOutput(`Event "${eventTitle}" created successfully`)

      // Reset form
      setEventTitle('')
      setEventDescription('')
      setStartDate('')
      setStartTime('')
      setEndDate('')
      setEndTime('')
      setIsAllDay(false)

      // Reload events
      await loadEvents()
    } catch (error) {
      addOutput(`Error creating event: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleDeleteEvent = async (eventId: number) => {
    const event = events.find((e) => e.id === eventId)
    try {
      await invoke('delete_event', { id: eventId })
      addOutput(`Event "${event?.title}" deleted`)
      await loadEvents()
    } catch (error) {
      addOutput(`Error deleting event: ${error}`, false)
    }
  }

  const handleExportToICS = async () => {
    if (events.length === 0) {
      addOutput('No events to export', false)
      return
    }

    setLoading('exporting')
    try {
      const filePath = await invoke<string>('export_events_to_ics')
      addOutput(`Exported ${events.length} events to ${filePath}`)
      addOutput('Opening in system calendar...')

      // Open the ICS file with the system calendar app
      await open(filePath)
      addOutput('Calendar file opened successfully')
    } catch (error) {
      addOutput(`Error exporting events: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const formatEventTime = (event: Event): string => {
    if (event.is_all_day) {
      const startDate = new Date(event.start_time).toLocaleDateString()
      const endDate = new Date(event.end_time).toLocaleDateString()
      return startDate === endDate ? `${startDate} (All day)` : `${startDate} - ${endDate} (All day)`
    }

    const startDateTime = new Date(event.start_time)
    const endDateTime = new Date(event.end_time)
    const startStr = startDateTime.toLocaleString()
    const endStr = endDateTime.toLocaleString()

    return `${startStr} - ${endStr}`
  }

  return (
    <ModulePageLayout
      title="Calendar Module"
      description="Internal calendar with event management and ICS export"
      icon={CalendarIcon}
      docPath="calendar-module.md"
    >
      <div className="space-y-6">
        {/* Add Event Section */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Plus className="w-5 h-5" />
            Add Event
          </h2>

          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-1">Event Title *</label>
              <input
                type="text"
                value={eventTitle}
                onChange={(e) => setEventTitle(e.target.value)}
                placeholder="Team meeting"
                className="w-full px-3 py-2 border rounded-md"
              />
            </div>

            <div>
              <label className="block text-sm font-medium mb-1">Description</label>
              <textarea
                value={eventDescription}
                onChange={(e) => setEventDescription(e.target.value)}
                placeholder="Discuss Q1 goals..."
                className="w-full px-3 py-2 border rounded-md h-20"
              />
            </div>

            <div className="flex items-center gap-2">
              <input
                type="checkbox"
                id="allDay"
                checked={isAllDay}
                onChange={(e) => setIsAllDay(e.target.checked)}
                className="w-4 h-4"
              />
              <label htmlFor="allDay" className="text-sm font-medium">
                All-day event
              </label>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium mb-1">Start Date *</label>
                <input
                  type="date"
                  value={startDate}
                  onChange={(e) => setStartDate(e.target.value)}
                  className="w-full px-3 py-2 border rounded-md"
                />
              </div>

              {!isAllDay && (
                <div>
                  <label className="block text-sm font-medium mb-1">Start Time</label>
                  <input
                    type="time"
                    value={startTime}
                    onChange={(e) => setStartTime(e.target.value)}
                    className="w-full px-3 py-2 border rounded-md"
                  />
                </div>
              )}
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium mb-1">End Date *</label>
                <input
                  type="date"
                  value={endDate}
                  onChange={(e) => setEndDate(e.target.value)}
                  className="w-full px-3 py-2 border rounded-md"
                />
              </div>

              {!isAllDay && (
                <div>
                  <label className="block text-sm font-medium mb-1">End Time</label>
                  <input
                    type="time"
                    value={endTime}
                    onChange={(e) => setEndTime(e.target.value)}
                    className="w-full px-3 py-2 border rounded-md"
                  />
                </div>
              )}
            </div>

            <Button
              onClick={handleAddEvent}
              disabled={loading === 'adding'}
              className="w-full"
            >
              {loading === 'adding' ? 'Adding...' : 'Add Event'}
            </Button>
          </div>
        </section>

        {/* Events List Section */}
        <section className="rounded-lg border p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold flex items-center gap-2">
              <CalendarIcon className="w-5 h-5" />
              Events ({events.length})
            </h2>
            <Button
              variant="outline"
              size="sm"
              onClick={handleExportToICS}
              disabled={loading === 'exporting' || events.length === 0}
            >
              <Download className="w-4 h-4 mr-2" />
              {loading === 'exporting' ? 'Exporting...' : 'Export to ICS'}
            </Button>
          </div>

          {events.length === 0 ? (
            <p className="text-center text-muted-foreground py-8">No events yet. Add your first event above.</p>
          ) : (
            <div className="space-y-3">
              {events.map((event) => (
                <div
                  key={event.id}
                  className="flex items-start justify-between p-4 border rounded-lg hover:bg-muted/50"
                >
                  <div className="flex-1">
                    <h3 className="font-semibold">{event.title}</h3>
                    {event.description && (
                      <p className="text-sm text-muted-foreground mt-1">{event.description}</p>
                    )}
                    <p className="text-sm text-muted-foreground mt-2">
                      {formatEventTime(event)}
                    </p>
                  </div>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => handleDeleteEvent(event.id)}
                  >
                    <Trash2 className="w-4 h-4 text-destructive" />
                  </Button>
                </div>
              ))}
            </div>
          )}
        </section>

        {/* Output Panel */}
        <section className="rounded-lg border p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold">Output</h2>
            <Button variant="outline" size="sm" onClick={() => setOutput([])}>
              Clear
            </Button>
          </div>

          <div className="bg-muted rounded-md p-4 h-48 overflow-y-auto font-mono text-sm">
            {output.length === 0 ? (
              <p className="text-muted-foreground">No output yet...</p>
            ) : (
              output.map((line, i) => (
                <div key={i} className="mb-1">
                  {line}
                </div>
              ))
            )}
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
