import { createFileRoute } from '@tantml:router'
import { Mic, Volume2, FileAudio, Languages, Users, Sparkles } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect, useRef } from 'react'

export const Route = createFileRoute('/speech-media-intelligence')({
  component: SpeechMediaIntelligenceModule,
})

// Speech Recognition Types
interface SpeechRecognitionEvent {
  results: SpeechRecognitionResultList
  resultIndex: number
}

interface SpeechRecognitionResultList {
  length: number
  item(index: number): SpeechRecognitionResult
  [index: number]: SpeechRecognitionResult
}

interface SpeechRecognitionResult {
  isFinal: boolean
  length: number
  item(index: number): SpeechRecognitionAlternative
  [index: number]: SpeechRecognitionAlternative
}

interface SpeechRecognitionAlternative {
  transcript: string
  confidence: number
}

interface SpeechRecognition extends EventTarget {
  continuous: boolean
  interimResults: boolean
  lang: string
  maxAlternatives: number
  start(): void
  stop(): void
  abort(): void
  onresult: ((event: SpeechRecognitionEvent) => void) | null
  onerror: ((event: any) => void) | null
  onend: (() => void) | null
  onstart: (() => void) | null
}

declare global {
  interface Window {
    SpeechRecognition: {
      new (): SpeechRecognition
    }
    webkitSpeechRecognition: {
      new (): SpeechRecognition
    }
  }
}

function SpeechMediaIntelligenceModule() {
  const [output, setOutput] = useState<string[]>([])

  // Speech Recognition State
  const [transcript, setTranscript] = useState('')
  const [interimTranscript, setInterimTranscript] = useState('')
  const [isListening, setIsListening] = useState(false)
  const [recognitionError, setRecognitionError] = useState<string | null>(null)
  const recognitionRef = useRef<SpeechRecognition | null>(null)

  // Text-to-Speech State
  const [textToSpeak, setTextToSpeak] = useState('')
  const [speaking, setSpeaking] = useState(false)
  const [voices, setVoices] = useState<SpeechSynthesisVoice[]>([])
  const [selectedVoice, setSelectedVoice] = useState<SpeechSynthesisVoice | undefined>()
  const [speechRate, setSpeechRate] = useState(1)
  const [speechPitch, setSpeechPitch] = useState(1)
  const [speechVolume, setSpeechVolume] = useState(1)

  // Feature Support Detection
  const [recognitionSupported, setRecognitionSupported] = useState(false)
  const [synthesisSupported, setSynthesisSupported] = useState(false)

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  // Initialize Speech Recognition
  useEffect(() => {
    const SpeechRecognitionAPI = window.SpeechRecognition || window.webkitSpeechRecognition

    if (SpeechRecognitionAPI) {
      setRecognitionSupported(true)
      addOutput('Speech Recognition API available')

      const recognition = new SpeechRecognitionAPI()
      recognition.continuous = true
      recognition.interimResults = true
      recognition.lang = 'en-US'
      recognition.maxAlternatives = 3

      recognition.onresult = (event: SpeechRecognitionEvent) => {
        let interim = ''
        let final = ''

        for (let i = event.resultIndex; i < event.results.length; i++) {
          const result = event.results[i]
          const transcriptText = result[0].transcript

          if (result.isFinal) {
            final += transcriptText + ' '
            addOutput(`Final transcript: "${transcriptText}" (confidence: ${(result[0].confidence * 100).toFixed(1)}%)`)
          } else {
            interim += transcriptText
          }
        }

        if (final) {
          setTranscript(prev => prev + final)
        }
        setInterimTranscript(interim)
      }

      recognition.onerror = (event: any) => {
        const errorMsg = `Speech recognition error: ${event.error}`
        setRecognitionError(errorMsg)
        addOutput(errorMsg, false)
        setIsListening(false)
      }

      recognition.onend = () => {
        setIsListening(false)
        addOutput('Speech recognition stopped')
      }

      recognition.onstart = () => {
        setIsListening(true)
        setRecognitionError(null)
        addOutput('Speech recognition started')
      }

      recognitionRef.current = recognition
    } else {
      setRecognitionSupported(false)
      addOutput('Speech Recognition not supported in this browser', false)
    }

    return () => {
      if (recognitionRef.current) {
        recognitionRef.current.stop()
      }
    }
  }, [])

  // Initialize Speech Synthesis
  useEffect(() => {
    if ('speechSynthesis' in window) {
      setSynthesisSupported(true)
      addOutput('Speech Synthesis API available')

      const loadVoices = () => {
        const availableVoices = window.speechSynthesis.getVoices()
        setVoices(availableVoices)
        if (availableVoices.length > 0) {
          addOutput(`Loaded ${availableVoices.length} voices`)
        }
      }

      loadVoices()
      window.speechSynthesis.onvoiceschanged = loadVoices
    } else {
      setSynthesisSupported(false)
      addOutput('Speech Synthesis not supported in this browser', false)
    }
  }, [])

  // Speech Recognition Controls
  const startListening = () => {
    if (recognitionRef.current && !isListening) {
      recognitionRef.current.start()
    }
  }

  const stopListening = () => {
    if (recognitionRef.current && isListening) {
      recognitionRef.current.stop()
    }
  }

  const resetTranscript = () => {
    setTranscript('')
    setInterimTranscript('')
    addOutput('Transcript cleared')
  }

  // Text-to-Speech Controls
  const handleSpeak = () => {
    if (!textToSpeak || !synthesisSupported) return

    const utterance = new SpeechSynthesisUtterance(textToSpeak)

    if (selectedVoice) utterance.voice = selectedVoice
    utterance.rate = speechRate
    utterance.pitch = speechPitch
    utterance.volume = speechVolume

    utterance.onstart = () => {
      setSpeaking(true)
      addOutput(`Speaking: "${textToSpeak.substring(0, 50)}${textToSpeak.length > 50 ? '...' : ''}"`)
    }

    utterance.onend = () => {
      setSpeaking(false)
      addOutput('Speech synthesis completed')
    }

    utterance.onerror = (event) => {
      setSpeaking(false)
      addOutput(`Speech synthesis error: ${event.error}`, false)
    }

    window.speechSynthesis.speak(utterance)
  }

  const stopSpeaking = () => {
    window.speechSynthesis.cancel()
    setSpeaking(false)
    addOutput('Speech synthesis stopped')
  }

  return (
    <ModulePageLayout
      title="Speech & Media Intelligence Module"
      description="Process and analyze audio/video content with speech recognition, text-to-speech, and AI-powered media intelligence"
      icon={Mic}
    >
      <div className="space-y-6">
        {/* Status Notice */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-blue-500">ℹ️</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Current implementation:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li>
                <strong className={recognitionSupported ? 'text-green-600' : 'text-red-600'}>
                  {recognitionSupported ? '✓' : '✗'} Web Speech API (Speech Recognition)
                </strong>
                {recognitionSupported ? ' - Functional' : ' - Not supported in this browser'}
              </li>
              <li>
                <strong className={synthesisSupported ? 'text-green-600' : 'text-red-600'}>
                  {synthesisSupported ? '✓' : '✗'} Web Speech API (Speech Synthesis)
                </strong>
                {synthesisSupported ? ' - Functional' : ' - Not supported in this browser'}
              </li>
              <li>
                <strong className="text-yellow-600">⚠ Advanced Features</strong> - Require API integrations (Whisper, OpenAI, etc.)
              </li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># Browser Support:</div>
              <div>Chrome/Edge: Full support for both recognition and synthesis</div>
              <div>Safari: Synthesis only (no recognition)</div>
              <div>Firefox: Limited support</div>
            </div>
            <p className="text-muted-foreground mt-2">
              For production apps, consider integrating cloud services like OpenAI Whisper, Google Cloud Speech, or Azure Speech Services for better accuracy and features.
            </p>
          </div>
        </section>

        {/* Speech Recognition Section */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Mic className="w-5 h-5" />
            Speech Recognition (Speech-to-Text)
          </h2>

          {!recognitionSupported && (
            <div className="bg-destructive/10 border border-destructive/30 rounded-md p-4">
              <p className="text-sm text-destructive font-medium">
                Speech recognition is not supported in your browser. Please use Chrome or Edge for full functionality.
              </p>
            </div>
          )}

          {recognitionSupported && (
            <>
              <div className="space-y-3">
                <p className="text-sm text-muted-foreground">
                  Real-time speech recognition using the Web Speech API
                </p>

                <div className="flex flex-wrap gap-2">
                  <Button
                    onClick={isListening ? stopListening : startListening}
                    variant={isListening ? 'destructive' : 'default'}
                  >
                    <Mic className={`w-4 h-4 mr-2 ${isListening ? 'animate-pulse' : ''}`} />
                    {isListening ? 'Stop Listening' : 'Start Listening'}
                  </Button>

                  <Button onClick={resetTranscript} variant="outline">
                    Reset Transcript
                  </Button>
                </div>

                {isListening && (
                  <div className="bg-green-50 dark:bg-green-950 border border-green-200 dark:border-green-800 rounded-md p-4">
                    <div className="flex items-center gap-2">
                      <div className="w-3 h-3 bg-red-500 rounded-full animate-pulse" />
                      <span className="text-sm font-medium text-green-700 dark:text-green-300">
                        Listening... Speak now
                      </span>
                    </div>
                  </div>
                )}

                {recognitionError && (
                  <div className="bg-destructive/10 border border-destructive/30 rounded-md p-3 text-sm text-destructive">
                    {recognitionError}
                  </div>
                )}

                {interimTranscript && (
                  <div className="space-y-2">
                    <label className="text-sm font-medium text-muted-foreground">
                      Interim Results (Live):
                    </label>
                    <div className="bg-muted/50 rounded-md p-3 border border-dashed">
                      <p className="text-gray-500 dark:text-gray-400 italic">{interimTranscript}</p>
                    </div>
                  </div>
                )}

                <div className="space-y-2">
                  <label className="text-sm font-medium">Final Transcript:</label>
                  <div className="bg-muted rounded-md p-4 min-h-[150px] max-h-[300px] overflow-y-auto">
                    {transcript || <span className="text-muted-foreground">No transcript yet... Click "Start Listening" and speak</span>}
                  </div>
                </div>

                <div className="flex gap-2">
                  <Button
                    onClick={() => {
                      if (transcript) {
                        navigator.clipboard.writeText(transcript)
                        addOutput('Transcript copied to clipboard')
                      }
                    }}
                    variant="outline"
                    size="sm"
                    disabled={!transcript}
                  >
                    Copy Transcript
                  </Button>
                </div>
              </div>
            </>
          )}
        </section>

        {/* Text-to-Speech Section */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Volume2 className="w-5 h-5" />
            Text-to-Speech Synthesis
          </h2>

          {!synthesisSupported && (
            <div className="bg-destructive/10 border border-destructive/30 rounded-md p-4">
              <p className="text-sm text-destructive font-medium">
                Speech synthesis is not supported in your browser.
              </p>
            </div>
          )}

          {synthesisSupported && (
            <>
              <div className="space-y-4">
                <div className="space-y-2">
                  <label className="text-sm font-medium">Text to Speak:</label>
                  <textarea
                    value={textToSpeak}
                    onChange={(e) => setTextToSpeak(e.target.value)}
                    className="w-full p-3 border rounded-md bg-background"
                    rows={4}
                    placeholder="Enter text to convert to speech..."
                  />
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <label className="text-sm font-medium">Voice:</label>
                    <select
                      value={selectedVoice?.voiceURI || ''}
                      onChange={(e) => {
                        const voice = voices.find(v => v.voiceURI === e.target.value)
                        setSelectedVoice(voice)
                        if (voice) {
                          addOutput(`Selected voice: ${voice.name} (${voice.lang})`)
                        }
                      }}
                      className="w-full p-2 border rounded-md bg-background"
                    >
                      <option value="">Default Voice</option>
                      {voices.map((voice) => (
                        <option key={voice.voiceURI} value={voice.voiceURI}>
                          {voice.name} ({voice.lang})
                        </option>
                      ))}
                    </select>
                    <p className="text-xs text-muted-foreground">
                      {voices.length} voices available
                    </p>
                  </div>

                  <div className="space-y-2">
                    <label className="text-sm font-medium">
                      Rate: {speechRate.toFixed(1)}x
                    </label>
                    <input
                      type="range"
                      min="0.5"
                      max="2"
                      step="0.1"
                      value={speechRate}
                      onChange={(e) => setSpeechRate(parseFloat(e.target.value))}
                      className="w-full"
                    />
                    <p className="text-xs text-muted-foreground">
                      Playback speed (0.5x - 2x)
                    </p>
                  </div>

                  <div className="space-y-2">
                    <label className="text-sm font-medium">
                      Pitch: {speechPitch.toFixed(1)}
                    </label>
                    <input
                      type="range"
                      min="0.5"
                      max="2"
                      step="0.1"
                      value={speechPitch}
                      onChange={(e) => setSpeechPitch(parseFloat(e.target.value))}
                      className="w-full"
                    />
                    <p className="text-xs text-muted-foreground">
                      Voice pitch (0.5 - 2.0)
                    </p>
                  </div>

                  <div className="space-y-2">
                    <label className="text-sm font-medium">
                      Volume: {Math.round(speechVolume * 100)}%
                    </label>
                    <input
                      type="range"
                      min="0"
                      max="1"
                      step="0.1"
                      value={speechVolume}
                      onChange={(e) => setSpeechVolume(parseFloat(e.target.value))}
                      className="w-full"
                    />
                    <p className="text-xs text-muted-foreground">
                      Playback volume (0-100%)
                    </p>
                  </div>
                </div>

                <div className="flex gap-2">
                  <Button
                    onClick={handleSpeak}
                    disabled={!textToSpeak || speaking}
                    variant="default"
                  >
                    <Volume2 className="w-4 h-4 mr-2" />
                    {speaking ? 'Speaking...' : 'Speak'}
                  </Button>

                  <Button
                    onClick={stopSpeaking}
                    disabled={!speaking}
                    variant="outline"
                  >
                    Stop
                  </Button>

                  <Button
                    onClick={() => {
                      setTextToSpeak(transcript)
                      addOutput('Loaded transcript into text-to-speech')
                    }}
                    variant="outline"
                    disabled={!transcript}
                  >
                    Use Transcript
                  </Button>
                </div>
              </div>
            </>
          )}
        </section>

        {/* Advanced Features Section */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Sparkles className="w-5 h-5" />
            Advanced Features
          </h2>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {/* Audio Transcription */}
            <div className="border rounded-lg p-4 space-y-3">
              <div className="flex items-center gap-2">
                <FileAudio className="w-5 h-5 text-purple-600" />
                <h3 className="font-semibold">Audio Transcription</h3>
              </div>
              <p className="text-sm text-muted-foreground">
                Upload audio files for batch transcription with timestamps
              </p>
              <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-3">
                <p className="text-xs text-yellow-700 dark:text-yellow-400">
                  Requires API integration (Whisper, AssemblyAI, etc.)
                </p>
              </div>
            </div>

            {/* Language Detection */}
            <div className="border rounded-lg p-4 space-y-3">
              <div className="flex items-center gap-2">
                <Languages className="w-5 h-5 text-blue-600" />
                <h3 className="font-semibold">Language Detection</h3>
              </div>
              <p className="text-sm text-muted-foreground">
                Automatic language detection and multi-language support
              </p>
              <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-3">
                <p className="text-xs text-yellow-700 dark:text-yellow-400">
                  Requires API integration
                </p>
              </div>
            </div>

            {/* Speaker Diarization */}
            <div className="border rounded-lg p-4 space-y-3">
              <div className="flex items-center gap-2">
                <Users className="w-5 h-5 text-green-600" />
                <h3 className="font-semibold">Speaker Diarization</h3>
              </div>
              <p className="text-sm text-muted-foreground">
                Identify and label different speakers in audio/video
              </p>
              <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-3">
                <p className="text-xs text-yellow-700 dark:text-yellow-400">
                  Requires API integration
                </p>
              </div>
            </div>
          </div>

          <div className="bg-blue-500/10 border border-blue-500/30 rounded-md p-4">
            <h4 className="font-semibold mb-2 text-blue-700 dark:text-blue-400 text-sm">
              Production-Ready Solutions
            </h4>
            <p className="text-xs text-muted-foreground mb-3">
              For advanced features, integrate with these services:
            </p>
            <ul className="list-disc list-inside space-y-1 text-xs text-muted-foreground ml-2">
              <li><strong>OpenAI Whisper:</strong> High-accuracy transcription, translation, and language detection</li>
              <li><strong>Google Cloud Speech:</strong> Real-time streaming, speaker diarization, and 125+ languages</li>
              <li><strong>Azure Speech Services:</strong> Custom models, neural TTS, and conversation transcription</li>
              <li><strong>AssemblyAI:</strong> Automatic punctuation, content moderation, and topic detection</li>
              <li><strong>ElevenLabs:</strong> Ultra-realistic voice cloning and synthesis</li>
            </ul>
          </div>
        </section>

        {/* Output Panel */}
        <section className="rounded-lg border p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold">Event Log</h2>
            <Button variant="outline" size="sm" onClick={() => setOutput([])}>
              Clear
            </Button>
          </div>

          <div className="bg-muted rounded-md p-4 h-64 overflow-y-auto font-mono text-sm">
            {output.length === 0 ? (
              <p className="text-muted-foreground">No events yet...</p>
            ) : (
              output.map((line, i) => (
                <div key={i} className="mb-1">
                  {line}
                </div>
              ))
            )}
          </div>
        </section>

        {/* Implementation Guide */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Implementation Guide</h3>
          <div className="space-y-4 text-sm">
            <div className="space-y-2">
              <h4 className="font-semibold">Web Speech API (Basic)</h4>
              <p className="text-muted-foreground">
                The current implementation uses browser-native Web Speech API for basic functionality.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div>// Speech Recognition</div>
                <div>const recognition = new webkitSpeechRecognition()</div>
                <div>recognition.continuous = true</div>
                <div>recognition.onresult = (event) =&gt; {"{ ... }"}</div>
                <div className="mt-2">// Speech Synthesis</div>
                <div>const utterance = new SpeechSynthesisUtterance(text)</div>
                <div>speechSynthesis.speak(utterance)</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Cloud Integration (Production)</h4>
              <p className="text-muted-foreground">
                For production apps, integrate with cloud services for better accuracy and features.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div>// Example: OpenAI Whisper API</div>
                <div>const formData = new FormData()</div>
                <div>formData.append('file', audioFile)</div>
                <div>formData.append('model', 'whisper-1')</div>
                <div>const response = await openai.audio.transcriptions.create(formData)</div>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400">
                Permissions & Privacy
              </h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Request microphone permissions before starting recognition</li>
                <li>Provide clear explanation for why audio access is needed</li>
                <li>Show visual indicator when microphone is active</li>
                <li>Handle permission denial gracefully</li>
                <li>Don't record or transmit audio without user consent</li>
                <li>Comply with GDPR, CCPA, and other privacy regulations</li>
              </ul>
            </div>
          </div>
        </section>

        {/* Platform Support */}
        <section className="rounded-lg border border-purple-500/50 bg-purple-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Platform Support</h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b">
                  <th className="text-left py-2 px-4">Feature</th>
                  <th className="text-center py-2 px-4">Chrome</th>
                  <th className="text-center py-2 px-4">Edge</th>
                  <th className="text-center py-2 px-4">Safari</th>
                  <th className="text-center py-2 px-4">Firefox</th>
                </tr>
              </thead>
              <tbody className="text-muted-foreground">
                <tr className="border-b">
                  <td className="py-2 px-4">Speech Recognition</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">🔶*</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Speech Synthesis</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Interim Results</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Voice Selection</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
              </tbody>
            </table>
            <div className="text-xs text-muted-foreground mt-2 space-y-1">
              <p>* 🔶 = Limited or experimental support</p>
              <p>For best compatibility, use Chrome or Edge browsers</p>
              <p>Mobile: Works on Android Chrome and iOS Safari (synthesis only)</p>
            </div>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
