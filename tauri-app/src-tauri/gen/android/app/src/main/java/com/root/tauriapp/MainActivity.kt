package com.root.tauriapp

import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import app.tauri.plugin.Plugin
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)

    // Register haptics plugin
    registerPlugin(HapticsPluginWrapper::class.java)
  }
}

@TauriPlugin
class HapticsPluginWrapper(private val activity: MainActivity): Plugin(activity) {

  private val hapticsPlugin: HapticsPlugin by lazy {
    HapticsPlugin(activity.applicationContext)
  }

  @Command
  fun hapticImpact(invoke: Invoke) {
    val style = invoke.getString("style") ?: run {
      invoke.reject("Missing 'style' parameter")
      return
    }

    try {
      hapticsPlugin.impact(style)
      invoke.resolve()
    } catch (e: Exception) {
      invoke.reject("Haptic impact failed: ${e.message}")
    }
  }

  @Command
  fun hapticNotification(invoke: Invoke) {
    val notificationType = invoke.getString("notificationType") ?: run {
      invoke.reject("Missing 'notificationType' parameter")
      return
    }

    try {
      hapticsPlugin.notification(notificationType)
      invoke.resolve()
    } catch (e: Exception) {
      invoke.reject("Haptic notification failed: ${e.message}")
    }
  }

  @Command
  fun vibrate(invoke: Invoke) {
    val duration = invoke.getLong("duration") ?: run {
      invoke.reject("Missing 'duration' parameter")
      return
    }

    try {
      hapticsPlugin.vibrate(duration)
      invoke.resolve()
    } catch (e: Exception) {
      invoke.reject("Vibration failed: ${e.message}")
    }
  }

  @Command
  fun vibratePattern(invoke: Invoke) {
    val patternArray = invoke.getArray("pattern") ?: run {
      invoke.reject("Missing 'pattern' parameter")
      return
    }

    try {
      val pattern = LongArray(patternArray.length()) { i ->
        patternArray.getLong(i)
      }
      hapticsPlugin.vibratePattern(pattern)
      invoke.resolve()
    } catch (e: Exception) {
      invoke.reject("Pattern vibration failed: ${e.message}")
    }
  }

  @Command
  fun cancelVibration(invoke: Invoke) {
    try {
      hapticsPlugin.cancel()
      invoke.resolve()
    } catch (e: Exception) {
      invoke.reject("Cancel vibration failed: ${e.message}")
    }
  }

  @Command
  fun hasVibrator(invoke: Invoke) {
    try {
      val result = JSObject()
      result.put("value", hapticsPlugin.hasVibrator())
      invoke.resolve(result)
    } catch (e: Exception) {
      invoke.reject("Check vibrator failed: ${e.message}")
    }
  }
}
