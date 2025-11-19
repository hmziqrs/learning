# Speech & Media Intelligence Module

## Overview

Process and analyze audio/video content with speech recognition, text-to-speech synthesis, voice activity detection, and media intelligence features. This module provides real-time speech-to-text transcription, natural voice synthesis, speaker identification, and intelligent media analysis capabilities across platforms.

## Current Implementation Status

⚠️ **Planned** - Requires integration with platform speech APIs and/or third-party services

## Plugin Setup

### Web APIs (Basic Implementation)

The simplest approach uses browser-native APIs:

**Web Speech API:**
- Speech Recognition (speech-to-text)
- Speech Synthesis (text-to-speech)
- Available in modern browsers
- Limited offline support

```typescript
// Check browser support
const hasSpeechRecognition = 'webkitSpeechRecognition' in window || 'SpeechRecognition' in window
const hasSpeechSynthesis = 'speechSynthesis' in window
```

**Permissions Required:**
- **Desktop**: Browser microphone permissions
- **Mobile**: Microphone access via system permissions

### Platform-Native APIs

For advanced features beyond Web Speech API:

**Android:**
- `SpeechRecognizer` - Speech recognition
- `TextToSpeech` - Voice synthesis
- `MediaRecorder` - Audio capture
- Permissions: `RECORD_AUDIO`, `INTERNET` (for cloud recognition)

**iOS:**
- `Speech` framework - Speech recognition
- `AVSpeechSynthesizer` - Voice synthesis
- `AVAudioRecorder` - Audio capture
- Permissions: `NSSpeechRecognitionUsageDescription`, `NSMicrophoneUsageDescription`

**Desktop:**
- Windows: `Windows.Media.SpeechRecognition`, `Windows.Media.SpeechSynthesis`
- macOS: `NSSpeechRecognizer`, `NSSpeechSynthesizer`
- Linux: Third-party libraries (espeak, pocketsphinx)

### Third-Party Services

For production-grade features:

**Speech Recognition:**
- OpenAI Whisper (local or API)
- Google Cloud Speech-to-Text
- Amazon Transcribe
- Azure Speech Services
- AssemblyAI

**Text-to-Speech:**
- Google Cloud Text-to-Speech
- Amazon Polly
- Azure Speech Services
- ElevenLabs (realistic voices)

**Media Intelligence:**
- OpenAI (GPT-4 Vision for video analysis)
- Google Cloud Video Intelligence
- Amazon Rekognition
- Azure Video Indexer

### Dependencies

```bash
# Optional: for advanced audio processing
bun add @ffmpeg/ffmpeg
bun add wavesurfer.js

# For API integrations
bun add openai
bun add @google-cloud/speech
bun add @google-cloud/text-to-speech
```

## Permissions Configuration

### Android Manifest

```xml
<uses-permission android:name="android.permission.RECORD_AUDIO" />
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.MODIFY_AUDIO_SETTINGS" />

<queries>
  <intent>
    <action android:name="android.speech.RecognitionService" />
  </intent>
</queries>
```

### iOS Info.plist

```xml
<key>NSSpeechRecognitionUsageDescription</key>
<string>We need speech recognition to transcribe your voice commands</string>
<key>NSMicrophoneUsageDescription</key>
<string>We need microphone access to record and transcribe audio</string>
```

### macOS Info.plist

```xml
<key>NSMicrophoneUsageDescription</key>
<string>This app needs microphone access for speech recognition</string>
<key>NSSpeechRecognitionUsageDescription</key>
<string>This app needs speech recognition for voice commands</string>
```

### Tauri Capabilities

```json
{
  "permissions": [
    "core:default",
    "microphone:allow-record"
  ]
}
```

## Core Features

### Speech Recognition (Speech-to-Text)
- [ ] Real-time speech recognition
- [ ] Continuous recognition mode
- [ ] Interim results (live transcription)
- [ ] Final results with confidence scores
- [ ] Multiple language support
- [ ] Custom vocabulary/commands
- [ ] Punctuation and capitalization
- [ ] Timestamp markers
- [ ] Speaker diarization
- [ ] Noise reduction

### Text-to-Speech Synthesis
- [ ] Convert text to natural speech
- [ ] Multiple voices and languages
- [ ] Voice customization (pitch, rate, volume)
- [ ] SSML support (Speech Synthesis Markup Language)
- [ ] Phoneme control
- [ ] Emotion/style selection
- [ ] Real-time playback controls
- [ ] Export to audio file

### Voice Activity Detection
- [ ] Detect when speech starts/stops
- [ ] Background noise filtering
- [ ] Silence detection
- [ ] Speech/non-speech classification
- [ ] Energy threshold configuration
- [ ] Automatic gain control

### Audio Transcription
- [ ] Upload audio file for transcription
- [ ] Support multiple formats (MP3, WAV, FLAC, OGG)
- [ ] Batch transcription
- [ ] Transcript export (TXT, SRT, VTT)
- [ ] Word-level timestamps
- [ ] Speaker labels
- [ ] Confidence scores per word

### Language & Dialect Detection
- [ ] Automatic language detection
- [ ] Multi-language transcription
- [ ] Dialect identification
- [ ] Language switching detection
- [ ] Translation integration

### Media Intelligence
- [ ] Audio classification (music, speech, noise)
- [ ] Video scene detection
- [ ] Object detection in video frames
- [ ] Content moderation (explicit content detection)
- [ ] Sentiment analysis from speech
- [ ] Topic extraction
- [ ] Key moment identification

### Subtitle & Caption Generation
- [ ] Auto-generate subtitles from video
- [ ] SRT/VTT format export
- [ ] Multi-language captions
- [ ] Timestamp synchronization
- [ ] Caption editing interface

## Data Structures

### Speech Recognition Result

```typescript
interface SpeechRecognitionResult {
  transcript: string
  confidence: number
  isFinal: boolean
  alternatives?: Array<{
    transcript: string
    confidence: number
  }>
  words?: WordTiming[]
  timestamp: number
  language?: string
}

interface WordTiming {
  word: string
  startTime: number
  endTime: number
  confidence: number
}
```

### Speech Synthesis Config

```typescript
interface SpeechSynthesisConfig {
  text: string
  voice: string
  language: string
  pitch: number // 0.0 to 2.0
  rate: number // 0.1 to 10.0
  volume: number // 0.0 to 1.0
  ssml?: string // SSML markup for advanced control
}

interface VoiceInfo {
  id: string
  name: string
  language: string
  gender: 'male' | 'female' | 'neutral'
  quality: 'standard' | 'premium' | 'neural'
  sampleRate: number
}
```

### Transcription Result

```typescript
interface TranscriptionResult {
  id: string
  text: string
  language: string
  duration: number
  segments: TranscriptionSegment[]
  words: WordTiming[]
  speakers?: SpeakerSegment[]
  metadata: {
    audioFormat: string
    sampleRate: number
    channels: number
    size: number
  }
}

interface TranscriptionSegment {
  id: number
  start: number
  end: number
  text: string
  confidence: number
  speaker?: string
}

interface SpeakerSegment {
  speaker: string
  segments: number[]
  totalDuration: number
}
```

### Voice Activity Detection

```typescript
interface VoiceActivityResult {
  isSpeaking: boolean
  energy: number
  timestamp: number
  segments: Array<{
    start: number
    end: number
    duration: number
  }>
}
```

### Media Analysis Result

```typescript
interface MediaAnalysisResult {
  type: 'audio' | 'video'
  duration: number
  classification: {
    category: string
    confidence: number
  }[]
  scenes?: Array<{
    start: number
    end: number
    description: string
    confidence: number
  }>
  objects?: Array<{
    label: string
    confidence: number
    timestamp: number
    boundingBox?: BoundingBox
  }>
  sentiment?: {
    overall: 'positive' | 'negative' | 'neutral'
    score: number
    segments: Array<{
      start: number
      end: number
      sentiment: string
      score: number
    }>
  }
  topics?: string[]
  keyMoments?: Array<{
    timestamp: number
    description: string
    importance: number
  }>
}

interface BoundingBox {
  x: number
  y: number
  width: number
  height: number
}
```

## Rust Backend

### Web Speech API Integration (No Backend Required)

For basic implementation using Web Speech API, no Rust backend is needed. All processing happens in the browser.

### Native Speech Recognition Commands

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RecognitionConfig {
    language: String,
    continuous: bool,
    interim_results: bool,
    max_alternatives: u8,
}

#[derive(Serialize, Deserialize)]
struct RecognitionResult {
    transcript: String,
    confidence: f32,
    is_final: bool,
    timestamp: i64,
}

#[tauri::command]
async fn start_speech_recognition(config: RecognitionConfig) -> Result<String, String> {
    // Platform-specific implementation
    #[cfg(mobile)]
    {
        mobile::speech::start_recognition(config)
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Ok("Use Web Speech API on desktop".to_string())
    }
}

#[tauri::command]
async fn stop_speech_recognition() -> Result<(), String> {
    #[cfg(mobile)]
    {
        mobile::speech::stop_recognition()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Ok(())
    }
}
```

### Text-to-Speech Commands

```rust
#[derive(Serialize, Deserialize)]
struct SynthesisConfig {
    text: String,
    voice: String,
    language: String,
    pitch: f32,
    rate: f32,
    volume: f32,
}

#[tauri::command]
async fn synthesize_speech(config: SynthesisConfig) -> Result<(), String> {
    #[cfg(mobile)]
    {
        mobile::speech::synthesize(config)
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Ok(())
    }
}

#[tauri::command]
async fn get_available_voices() -> Result<Vec<VoiceInfo>, String> {
    #[cfg(mobile)]
    {
        mobile::speech::get_voices()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Ok(vec![])
    }
}

#[tauri::command]
async fn stop_speech_synthesis() -> Result<(), String> {
    #[cfg(mobile)]
    {
        mobile::speech::stop_synthesis()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        Ok(())
    }
}
```

### Audio Transcription (File-based)

```rust
use std::path::PathBuf;

#[tauri::command]
async fn transcribe_audio_file(
    file_path: String,
    language: Option<String>,
) -> Result<TranscriptionResult, String> {
    // Read audio file
    let audio_data = std::fs::read(&file_path)
        .map_err(|e| format!("Failed to read audio file: {}", e))?;

    // Call transcription service (e.g., OpenAI Whisper API)
    // This is a placeholder - implement actual API call
    Err("Not implemented - integrate with transcription service".to_string())
}

#[tauri::command]
async fn export_transcript(
    transcript: TranscriptionResult,
    format: String, // "txt", "srt", "vtt"
    output_path: String,
) -> Result<(), String> {
    let content = match format.as_str() {
        "txt" => generate_txt(&transcript),
        "srt" => generate_srt(&transcript),
        "vtt" => generate_vtt(&transcript),
        _ => return Err("Unsupported format".to_string()),
    };

    std::fs::write(&output_path, content)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

fn generate_srt(transcript: &TranscriptionResult) -> String {
    let mut srt = String::new();
    for (i, segment) in transcript.segments.iter().enumerate() {
        srt.push_str(&format!("{}\n", i + 1));
        srt.push_str(&format_timestamp_srt(segment.start));
        srt.push_str(" --> ");
        srt.push_str(&format_timestamp_srt(segment.end));
        srt.push_str("\n");
        srt.push_str(&segment.text);
        srt.push_str("\n\n");
    }
    srt
}

fn format_timestamp_srt(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    let millis = ((seconds % 1.0) * 1000.0) as u32;
    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, secs, millis)
}

fn generate_vtt(transcript: &TranscriptionResult) -> String {
    let mut vtt = String::from("WEBVTT\n\n");
    for segment in &transcript.segments {
        vtt.push_str(&format_timestamp_vtt(segment.start));
        vtt.push_str(" --> ");
        vtt.push_str(&format_timestamp_vtt(segment.end));
        vtt.push_str("\n");
        vtt.push_str(&segment.text);
        vtt.push_str("\n\n");
    }
    vtt
}

fn format_timestamp_vtt(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    let millis = ((seconds % 1.0) * 1000.0) as u32;
    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, secs, millis)
}

fn generate_txt(transcript: &TranscriptionResult) -> String {
    transcript.segments
        .iter()
        .map(|s| s.text.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}
```

## Android Implementation

### Speech Recognition Plugin

```kotlin
import android.content.Intent
import android.os.Bundle
import android.speech.RecognitionListener
import android.speech.RecognizerIntent
import android.speech.SpeechRecognizer
import android.speech.tts.TextToSpeech
import android.speech.tts.UtteranceProgressListener
import java.util.*

class SpeechPlugin(private val activity: Activity) : RecognitionListener {
    private var speechRecognizer: SpeechRecognizer? = null
    private var textToSpeech: TextToSpeech? = null
    private var isListening = false

    init {
        setupTextToSpeech()
    }

    private fun setupTextToSpeech() {
        textToSpeech = TextToSpeech(activity) { status ->
            if (status == TextToSpeech.SUCCESS) {
                textToSpeech?.language = Locale.US
            }
        }

        textToSpeech?.setOnUtteranceProgressListener(object : UtteranceProgressListener() {
            override fun onStart(utteranceId: String) {
                emitEvent("tts-started", mapOf("id" to utteranceId))
            }

            override fun onDone(utteranceId: String) {
                emitEvent("tts-completed", mapOf("id" to utteranceId))
            }

            override fun onError(utteranceId: String) {
                emitEvent("tts-error", mapOf("id" to utteranceId))
            }
        })
    }

    @Command
    fun startSpeechRecognition(invoke: Invoke) {
        val language = invoke.getString("language") ?: "en-US"
        val continuous = invoke.getBoolean("continuous") ?: false

        if (!SpeechRecognizer.isRecognitionAvailable(activity)) {
            invoke.reject("Speech recognition not available")
            return
        }

        speechRecognizer = SpeechRecognizer.createSpeechRecognizer(activity)
        speechRecognizer?.setRecognitionListener(this)

        val intent = Intent(RecognizerIntent.ACTION_RECOGNIZE_SPEECH).apply {
            putExtra(RecognizerIntent.EXTRA_LANGUAGE_MODEL, RecognizerIntent.LANGUAGE_MODEL_FREE_FORM)
            putExtra(RecognizerIntent.EXTRA_LANGUAGE, language)
            putExtra(RecognizerIntent.EXTRA_PARTIAL_RESULTS, true)
            putExtra(RecognizerIntent.EXTRA_MAX_RESULTS, 5)
        }

        speechRecognizer?.startListening(intent)
        isListening = true
        invoke.resolve(mapOf("success" to true))
    }

    @Command
    fun stopSpeechRecognition(invoke: Invoke) {
        speechRecognizer?.stopListening()
        isListening = false
        invoke.resolve(mapOf("success" to true))
    }

    @Command
    fun synthesizeSpeech(invoke: Invoke) {
        val text = invoke.getString("text") ?: ""
        val language = invoke.getString("language") ?: "en-US"
        val pitch = invoke.getFloat("pitch") ?: 1.0f
        val rate = invoke.getFloat("rate") ?: 1.0f

        textToSpeech?.language = Locale.forLanguageTag(language)
        textToSpeech?.setPitch(pitch)
        textToSpeech?.setSpeechRate(rate)

        val utteranceId = UUID.randomUUID().toString()
        textToSpeech?.speak(text, TextToSpeech.QUEUE_FLUSH, null, utteranceId)

        invoke.resolve(mapOf("utteranceId" to utteranceId))
    }

    @Command
    fun stopSpeechSynthesis(invoke: Invoke) {
        textToSpeech?.stop()
        invoke.resolve(mapOf("success" to true))
    }

    @Command
    fun getAvailableVoices(invoke: Invoke) {
        val voices = textToSpeech?.voices?.map { voice ->
            mapOf(
                "id" to voice.name,
                "name" to voice.name,
                "language" to voice.locale.toLanguageTag(),
                "quality" to if (voice.quality >= Voice.QUALITY_HIGH) "premium" else "standard"
            )
        } ?: emptyList()

        invoke.resolve(mapOf("voices" to voices))
    }

    // RecognitionListener callbacks
    override fun onReadyForSpeech(params: Bundle?) {
        emitEvent("recognition-ready", emptyMap())
    }

    override fun onBeginningOfSpeech() {
        emitEvent("speech-started", emptyMap())
    }

    override fun onRmsChanged(rmsdB: Float) {
        emitEvent("audio-level", mapOf("level" to rmsdB))
    }

    override fun onBufferReceived(buffer: ByteArray?) {
        // Not typically used
    }

    override fun onEndOfSpeech() {
        emitEvent("speech-ended", emptyMap())
        isListening = false
    }

    override fun onError(error: Int) {
        val errorMessage = when (error) {
            SpeechRecognizer.ERROR_AUDIO -> "Audio recording error"
            SpeechRecognizer.ERROR_CLIENT -> "Client error"
            SpeechRecognizer.ERROR_INSUFFICIENT_PERMISSIONS -> "Insufficient permissions"
            SpeechRecognizer.ERROR_NETWORK -> "Network error"
            SpeechRecognizer.ERROR_NETWORK_TIMEOUT -> "Network timeout"
            SpeechRecognizer.ERROR_NO_MATCH -> "No speech match"
            SpeechRecognizer.ERROR_RECOGNIZER_BUSY -> "Recognizer busy"
            SpeechRecognizer.ERROR_SERVER -> "Server error"
            SpeechRecognizer.ERROR_SPEECH_TIMEOUT -> "Speech timeout"
            else -> "Unknown error: $error"
        }

        emitEvent("recognition-error", mapOf("error" to errorMessage))
        isListening = false
    }

    override fun onResults(results: Bundle?) {
        val matches = results?.getStringArrayList(SpeechRecognizer.RESULTS_RECOGNITION)
        val confidences = results?.getFloatArray(SpeechRecognizer.CONFIDENCE_SCORES)

        if (matches != null && matches.isNotEmpty()) {
            val result = mapOf(
                "transcript" to matches[0],
                "confidence" to (confidences?.get(0) ?: 0.0f),
                "isFinal" to true,
                "alternatives" to matches.take(5).mapIndexed { index, text ->
                    mapOf(
                        "transcript" to text,
                        "confidence" to (confidences?.getOrNull(index) ?: 0.0f)
                    )
                },
                "timestamp" to System.currentTimeMillis()
            )

            emitEvent("recognition-result", result)
        }
    }

    override fun onPartialResults(partialResults: Bundle?) {
        val matches = partialResults?.getStringArrayList(SpeechRecognizer.RESULTS_RECOGNITION)

        if (matches != null && matches.isNotEmpty()) {
            val result = mapOf(
                "transcript" to matches[0],
                "isFinal" to false,
                "timestamp" to System.currentTimeMillis()
            )

            emitEvent("recognition-partial", result)
        }
    }

    override fun onEvent(eventType: Int, params: Bundle?) {
        // Not typically used
    }

    fun cleanup() {
        speechRecognizer?.destroy()
        textToSpeech?.shutdown()
    }

    private fun emitEvent(eventName: String, data: Map<String, Any>) {
        // Send to Tauri frontend via plugin channel
    }
}
```

## iOS Implementation

### Speech Recognition Plugin

```swift
import Speech
import AVFoundation

class SpeechPlugin: NSObject, SFSpeechRecognizerDelegate {
    private var speechRecognizer: SFSpeechRecognizer?
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?
    private let audioEngine = AVAudioEngine()
    private let speechSynthesizer = AVSpeechSynthesizer()

    override init() {
        super.init()
        speechSynthesizer.delegate = self
    }

    @objc func requestPermission(_ invoke: Invoke) {
        SFSpeechRecognizer.requestAuthorization { status in
            switch status {
            case .authorized:
                invoke.resolve(["granted": true])
            case .denied, .restricted, .notDetermined:
                invoke.resolve(["granted": false])
            @unknown default:
                invoke.reject("Unknown authorization status")
            }
        }
    }

    @objc func startSpeechRecognition(_ invoke: Invoke) {
        let language = invoke.getString("language") ?? "en-US"
        let continuous = invoke.getBool("continuous") ?? false

        // Check authorization
        guard SFSpeechRecognizer.authorizationStatus() == .authorized else {
            invoke.reject("Speech recognition not authorized")
            return
        }

        // Initialize recognizer
        guard let recognizer = SFSpeechRecognizer(locale: Locale(identifier: language)) else {
            invoke.reject("Speech recognizer not available for language: \(language)")
            return
        }

        guard recognizer.isAvailable else {
            invoke.reject("Speech recognizer not available")
            return
        }

        speechRecognizer = recognizer
        speechRecognizer?.delegate = self

        // Cancel previous task
        recognitionTask?.cancel()
        recognitionTask = nil

        // Setup audio session
        let audioSession = AVAudioSession.sharedInstance()
        do {
            try audioSession.setCategory(.record, mode: .measurement, options: .duckOthers)
            try audioSession.setActive(true, options: .notifyOthersOnDeactivation)
        } catch {
            invoke.reject("Failed to setup audio session: \(error.localizedDescription)")
            return
        }

        // Setup recognition request
        recognitionRequest = SFSpeechAudioBufferRecognitionRequest()
        guard let recognitionRequest = recognitionRequest else {
            invoke.reject("Failed to create recognition request")
            return
        }

        recognitionRequest.shouldReportPartialResults = true

        // Setup audio input
        let inputNode = audioEngine.inputNode
        let recordingFormat = inputNode.outputFormat(forBus: 0)

        inputNode.installTap(onBus: 0, bufferSize: 1024, format: recordingFormat) { buffer, _ in
            recognitionRequest.append(buffer)
        }

        // Start audio engine
        audioEngine.prepare()
        do {
            try audioEngine.start()
        } catch {
            invoke.reject("Failed to start audio engine: \(error.localizedDescription)")
            return
        }

        // Start recognition
        recognitionTask = speechRecognizer?.recognitionTask(with: recognitionRequest) { [weak self] result, error in
            if let result = result {
                let isFinal = result.isFinal
                let bestTranscription = result.bestTranscription

                let data: [String: Any] = [
                    "transcript": bestTranscription.formattedString,
                    "isFinal": isFinal,
                    "confidence": self?.calculateConfidence(from: result) ?? 0.0,
                    "timestamp": Date().timeIntervalSince1970,
                    "segments": bestTranscription.segments.map { segment in
                        [
                            "substring": segment.substring,
                            "timestamp": segment.timestamp,
                            "duration": segment.duration,
                            "confidence": segment.confidence
                        ]
                    }
                ]

                self?.emitEvent(isFinal ? "recognition-result" : "recognition-partial", data: data)

                if isFinal {
                    self?.stopRecording()
                }
            }

            if let error = error {
                self?.emitEvent("recognition-error", data: ["error": error.localizedDescription])
                self?.stopRecording()
            }
        }

        invoke.resolve(["success": true])
    }

    @objc func stopSpeechRecognition(_ invoke: Invoke) {
        stopRecording()
        invoke.resolve(["success": true])
    }

    private func stopRecording() {
        audioEngine.stop()
        audioEngine.inputNode.removeTap(onBus: 0)
        recognitionRequest?.endAudio()
        recognitionTask?.cancel()
        recognitionTask = nil
        recognitionRequest = nil
    }

    @objc func synthesizeSpeech(_ invoke: Invoke) {
        guard let text = invoke.getString("text") else {
            invoke.reject("Text is required")
            return
        }

        let language = invoke.getString("language") ?? "en-US"
        let pitch = invoke.getFloat("pitch") ?? 1.0
        let rate = invoke.getFloat("rate") ?? 0.5
        let volume = invoke.getFloat("volume") ?? 1.0

        let utterance = AVSpeechUtterance(string: text)
        utterance.voice = AVSpeechSynthesisVoice(language: language)
        utterance.pitchMultiplier = pitch
        utterance.rate = rate
        utterance.volume = volume

        speechSynthesizer.speak(utterance)

        invoke.resolve(["success": true])
    }

    @objc func stopSpeechSynthesis(_ invoke: Invoke) {
        speechSynthesizer.stopSpeaking(at: .immediate)
        invoke.resolve(["success": true])
    }

    @objc func getAvailableVoices(_ invoke: Invoke) {
        let voices = AVSpeechSynthesisVoice.speechVoices().map { voice in
            [
                "id": voice.identifier,
                "name": voice.name,
                "language": voice.language,
                "quality": voice.quality == .enhanced ? "premium" : "standard"
            ]
        }

        invoke.resolve(["voices": voices])
    }

    private func calculateConfidence(from result: SFSpeechRecognitionResult) -> Float {
        // Calculate average confidence from segments
        let segments = result.bestTranscription.segments
        guard !segments.isEmpty else { return 0.0 }

        let totalConfidence = segments.reduce(0.0) { $0 + $1.confidence }
        return totalConfidence / Float(segments.count)
    }

    private func emitEvent(_ eventName: String, data: [String: Any]) {
        // Send to Tauri frontend via plugin channel
    }
}

extension SpeechPlugin: AVSpeechSynthesizerDelegate {
    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didStart utterance: AVSpeechUtterance) {
        emitEvent("tts-started", data: ["text": utterance.speechString])
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didFinish utterance: AVSpeechUtterance) {
        emitEvent("tts-completed", data: ["text": utterance.speechString])
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didCancel utterance: AVSpeechUtterance) {
        emitEvent("tts-cancelled", data: ["text": utterance.speechString])
    }
}
```

## Frontend Implementation

### Web Speech API Integration

```typescript
import { useState, useEffect, useRef } from 'react'

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

// Speech Recognition Hook
function useSpeechRecognition() {
  const [transcript, setTranscript] = useState('')
  const [isListening, setIsListening] = useState(false)
  const [interimTranscript, setInterimTranscript] = useState('')
  const recognitionRef = useRef<SpeechRecognition | null>(null)

  useEffect(() => {
    const SpeechRecognitionAPI = window.SpeechRecognition || window.webkitSpeechRecognition

    if (!SpeechRecognitionAPI) {
      console.error('Speech Recognition not supported')
      return
    }

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
      console.error('Speech recognition error:', event.error)
      setIsListening(false)
    }

    recognition.onend = () => {
      setIsListening(false)
    }

    recognitionRef.current = recognition

    return () => {
      recognition.stop()
    }
  }, [])

  const startListening = () => {
    if (recognitionRef.current && !isListening) {
      recognitionRef.current.start()
      setIsListening(true)
    }
  }

  const stopListening = () => {
    if (recognitionRef.current && isListening) {
      recognitionRef.current.stop()
      setIsListening(false)
    }
  }

  const resetTranscript = () => {
    setTranscript('')
    setInterimTranscript('')
  }

  return {
    transcript,
    interimTranscript,
    isListening,
    startListening,
    stopListening,
    resetTranscript,
    supported: !!(window.SpeechRecognition || window.webkitSpeechRecognition),
  }
}

// Text-to-Speech Hook
function useSpeechSynthesis() {
  const [speaking, setSpeaking] = useState(false)
  const [voices, setVoices] = useState<SpeechSynthesisVoice[]>([])

  useEffect(() => {
    const loadVoices = () => {
      const availableVoices = window.speechSynthesis.getVoices()
      setVoices(availableVoices)
    }

    loadVoices()
    window.speechSynthesis.onvoiceschanged = loadVoices
  }, [])

  const speak = (text: string, options?: {
    voice?: SpeechSynthesisVoice
    rate?: number
    pitch?: number
    volume?: number
  }) => {
    if (!window.speechSynthesis) {
      console.error('Speech Synthesis not supported')
      return
    }

    const utterance = new SpeechSynthesisUtterance(text)

    if (options?.voice) utterance.voice = options.voice
    if (options?.rate) utterance.rate = options.rate
    if (options?.pitch) utterance.pitch = options.pitch
    if (options?.volume) utterance.volume = options.volume

    utterance.onstart = () => setSpeaking(true)
    utterance.onend = () => setSpeaking(false)
    utterance.onerror = () => setSpeaking(false)

    window.speechSynthesis.speak(utterance)
  }

  const stop = () => {
    window.speechSynthesis.cancel()
    setSpeaking(false)
  }

  return {
    speak,
    stop,
    speaking,
    voices,
    supported: !!window.speechSynthesis,
  }
}

// Example Component
function SpeechIntelligencePage() {
  const {
    transcript,
    interimTranscript,
    isListening,
    startListening,
    stopListening,
    resetTranscript,
    supported: recognitionSupported,
  } = useSpeechRecognition()

  const {
    speak,
    stop,
    speaking,
    voices,
    supported: synthesisSupported,
  } = useSpeechSynthesis()

  const [textToSpeak, setTextToSpeak] = useState('')
  const [selectedVoice, setSelectedVoice] = useState<SpeechSynthesisVoice | undefined>()
  const [speechRate, setSpeechRate] = useState(1)
  const [speechPitch, setSpeechPitch] = useState(1)

  const handleSpeak = () => {
    if (textToSpeak) {
      speak(textToSpeak, {
        voice: selectedVoice,
        rate: speechRate,
        pitch: speechPitch,
      })
    }
  }

  return (
    <div className="space-y-6">
      {/* Speech Recognition Section */}
      <div className="card">
        <h2 className="text-2xl font-bold mb-4">Speech Recognition</h2>

        {!recognitionSupported && (
          <div className="alert alert-warning">
            Speech recognition is not supported in your browser
          </div>
        )}

        {recognitionSupported && (
          <>
            <div className="flex gap-4 mb-4">
              <button
                onClick={isListening ? stopListening : startListening}
                className="btn btn-primary"
              >
                {isListening ? 'Stop Listening' : 'Start Listening'}
              </button>
              <button onClick={resetTranscript} className="btn btn-secondary">
                Reset
              </button>
            </div>

            {isListening && (
              <div className="mb-4 p-4 bg-blue-50 border border-blue-200 rounded">
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 bg-red-500 rounded-full animate-pulse" />
                  <span className="text-sm font-medium">Listening...</span>
                </div>
              </div>
            )}

            {interimTranscript && (
              <div className="mb-4">
                <label className="text-sm font-medium text-gray-500">
                  Interim Results:
                </label>
                <p className="text-gray-400 italic">{interimTranscript}</p>
              </div>
            )}

            <div>
              <label className="text-sm font-medium">Transcript:</label>
              <div className="mt-2 p-4 bg-gray-50 border rounded min-h-[100px]">
                {transcript || 'No transcript yet...'}
              </div>
            </div>
          </>
        )}
      </div>

      {/* Text-to-Speech Section */}
      <div className="card">
        <h2 className="text-2xl font-bold mb-4">Text-to-Speech</h2>

        {!synthesisSupported && (
          <div className="alert alert-warning">
            Speech synthesis is not supported in your browser
          </div>
        )}

        {synthesisSupported && (
          <>
            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium">Text to Speak:</label>
                <textarea
                  value={textToSpeak}
                  onChange={(e) => setTextToSpeak(e.target.value)}
                  className="w-full mt-2 p-3 border rounded"
                  rows={4}
                  placeholder="Enter text to convert to speech..."
                />
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="text-sm font-medium">Voice:</label>
                  <select
                    value={selectedVoice?.voiceURI || ''}
                    onChange={(e) => {
                      const voice = voices.find(v => v.voiceURI === e.target.value)
                      setSelectedVoice(voice)
                    }}
                    className="w-full mt-2 p-2 border rounded"
                  >
                    <option value="">Default</option>
                    {voices.map((voice) => (
                      <option key={voice.voiceURI} value={voice.voiceURI}>
                        {voice.name} ({voice.lang})
                      </option>
                    ))}
                  </select>
                </div>

                <div>
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
                    className="w-full mt-2"
                  />
                </div>

                <div>
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
                    className="w-full mt-2"
                  />
                </div>
              </div>

              <div className="flex gap-4">
                <button
                  onClick={handleSpeak}
                  disabled={!textToSpeak || speaking}
                  className="btn btn-primary"
                >
                  {speaking ? 'Speaking...' : 'Speak'}
                </button>
                <button
                  onClick={stop}
                  disabled={!speaking}
                  className="btn btn-secondary"
                >
                  Stop
                </button>
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  )
}
```

## UI Components

### Speech Recognition Section
- [ ] Microphone button (start/stop)
- [ ] Real-time transcript display
- [ ] Interim results (live transcription)
- [ ] Confidence score indicator
- [ ] Language selector
- [ ] Recording indicator animation
- [ ] Clear/reset button
- [ ] Export transcript button

### Text-to-Speech Section
- [ ] Text input area
- [ ] Voice selector dropdown
- [ ] Playback controls (play/pause/stop)
- [ ] Rate slider (0.5x - 2x)
- [ ] Pitch slider (0.5 - 2.0)
- [ ] Volume slider (0-100%)
- [ ] Preview button
- [ ] Export audio button

### Audio Transcription Section
- [ ] File upload button
- [ ] Supported formats display
- [ ] Progress bar
- [ ] Transcript viewer
- [ ] Timestamp navigation
- [ ] Speaker labels (if available)
- [ ] Export options (TXT, SRT, VTT)
- [ ] Edit transcript interface

### Media Intelligence Section
- [ ] Media file upload
- [ ] Analysis progress indicator
- [ ] Classification results display
- [ ] Scene timeline
- [ ] Object detection overlay
- [ ] Sentiment analysis chart
- [ ] Key moments list
- [ ] Export analysis report

## Testing Checklist

### Desktop Testing
- [ ] Web Speech API recognition
- [ ] Web Speech API synthesis
- [ ] Multiple voices
- [ ] Rate/pitch/volume controls
- [ ] Browser compatibility (Chrome, Firefox, Safari, Edge)

### Mobile Testing
- [ ] Android speech recognition
- [ ] Android text-to-speech
- [ ] iOS speech recognition
- [ ] iOS text-to-speech
- [ ] Background audio handling
- [ ] Permission requests

### Feature Testing
- [ ] Continuous recognition
- [ ] Interim results
- [ ] Multiple languages
- [ ] Voice selection
- [ ] Audio file transcription
- [ ] Subtitle generation
- [ ] Export formats

### Edge Cases
- [ ] No speech detected
- [ ] Background noise
- [ ] Network interruption (for cloud services)
- [ ] Long audio files
- [ ] Multiple speakers
- [ ] Non-standard accents
- [ ] Permission denied handling

## Implementation Status

### Backend
- [ ] Web Speech API wrapper
- [ ] Native Android plugin
- [ ] Native iOS plugin
- [ ] Desktop platform integration
- [ ] Rust command interface
- [ ] Audio file processing
- [ ] Transcript export (SRT, VTT, TXT)
- [ ] API service integration (Whisper, etc.)

### Frontend
- [ ] Speech recognition UI
- [ ] Text-to-speech UI
- [ ] Audio transcription UI
- [ ] Media intelligence UI
- [ ] Real-time visualization
- [ ] Language selection
- [ ] Voice customization
- [ ] Export functionality

### Features
- [ ] Speech-to-text (real-time)
- [ ] Text-to-speech synthesis
- [ ] Voice activity detection
- [ ] Audio file transcription
- [ ] Subtitle generation
- [ ] Language detection
- [ ] Speaker diarization
- [ ] Media classification

## Troubleshooting

### Speech Recognition Not Working

**Issue**: Recognition not starting or producing results

**Solutions**:
- Check microphone permissions
- Verify browser support (Chrome/Edge recommended)
- Ensure HTTPS connection (required for Web Speech API)
- Check for background noise interference
- Verify language code is supported
- Try different browsers

### Text-to-Speech Not Playing

**Issue**: No audio output from synthesis

**Solutions**:
- Check system audio settings
- Verify browser support
- Wait for voices to load (use `onvoiceschanged` event)
- Check if audio is muted
- Try different voices
- Verify text content is valid

### Poor Recognition Accuracy

**Issue**: Transcripts are inaccurate

**Solutions**:
- Reduce background noise
- Speak clearly and at moderate pace
- Check microphone quality
- Use appropriate language setting
- Consider using cloud-based services for better accuracy
- Add custom vocabulary (if API supports)

### API Integration Errors

**Issue**: Third-party API calls failing

**Solutions**:
- Verify API credentials
- Check network connectivity
- Verify API quotas/limits
- Check file size limits
- Ensure proper audio format
- Review API error messages

## Resources

### Official Documentation
- [Web Speech API - MDN](https://developer.mozilla.org/en-US/docs/Web/API/Web_Speech_API)
- [Android SpeechRecognizer](https://developer.android.com/reference/android/speech/SpeechRecognizer)
- [Android TextToSpeech](https://developer.android.com/reference/android/speech/tts/TextToSpeech)
- [iOS Speech Framework](https://developer.apple.com/documentation/speech)
- [iOS AVSpeechSynthesizer](https://developer.apple.com/documentation/avfoundation/avspeechsynthesizer)

### Services & APIs
- [OpenAI Whisper](https://openai.com/research/whisper)
- [Google Cloud Speech-to-Text](https://cloud.google.com/speech-to-text)
- [Google Cloud Text-to-Speech](https://cloud.google.com/text-to-speech)
- [Amazon Transcribe](https://aws.amazon.com/transcribe/)
- [Amazon Polly](https://aws.amazon.com/polly/)
- [Azure Speech Services](https://azure.microsoft.com/en-us/services/cognitive-services/speech-services/)
- [AssemblyAI](https://www.assemblyai.com/)
- [ElevenLabs](https://elevenlabs.io/)

### Libraries & Tools
- [Whisper.cpp](https://github.com/ggerganov/whisper.cpp) - Local Whisper implementation
- [WaveSurfer.js](https://wavesurfer-js.org/) - Audio waveform visualization
- [FFmpeg](https://ffmpeg.org/) - Audio/video processing
- [Subtitle Edit](https://www.nikse.dk/subtitleedit/) - Subtitle editing reference

## Platform Support

| Feature | Windows | macOS | Linux | iOS | Android |
|---------|---------|-------|-------|-----|---------|
| **Speech Recognition** |
| Web Speech API | ✅ | ✅ | ✅ | ✅ | ✅ |
| Native Recognition | 🔶* | 🔶* | ❌ | ✅ | ✅ |
| Continuous Mode | ✅ | ✅ | ✅ | ✅ | ✅ |
| Interim Results | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Text-to-Speech** |
| Web Speech Synthesis | ✅ | ✅ | ✅ | ✅ | ✅ |
| Native TTS | ✅ | ✅ | 🔶* | ✅ | ✅ |
| Voice Customization | ✅ | ✅ | ✅ | ✅ | ✅ |
| SSML Support | ❌ | ❌ | ❌ | ✅ | ✅ |
| **Advanced Features** |
| Speaker Diarization | 🔶** | 🔶** | 🔶** | 🔶** | 🔶** |
| Language Detection | 🔶** | 🔶** | 🔶** | 🔶** | 🔶** |
| Media Intelligence | 🔶** | 🔶** | 🔶** | 🔶** | 🔶** |

**Notes:**
- ✅ = Supported
- 🔶* = Requires custom plugin development
- 🔶** = Requires third-party API service
- ❌ = Not typically available

---

Last Updated: November 2025
Module Version: 1.0.0
Status: Planned
