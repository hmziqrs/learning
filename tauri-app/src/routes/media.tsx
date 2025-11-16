import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/media')({
  component: Media,
})

function Media() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-4xl font-bold">Media Module</h1>
        <p className="text-muted-foreground mt-2">
          Play local videos, audio files, and test OS media controls.
        </p>
      </div>

      <div className="border border-border rounded-lg p-6 bg-card">
        <p className="text-muted-foreground">
          This module will demonstrate audio and video playback with native media controls.
        </p>
      </div>
    </div>
  )
}
