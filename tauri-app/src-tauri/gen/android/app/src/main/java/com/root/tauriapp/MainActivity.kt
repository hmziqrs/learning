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

    // Register security biometrics plugin
    registerPlugin(SecurityBiometricsPluginWrapper::class.java)
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

@TauriPlugin
class SecurityBiometricsPluginWrapper(private val activity: MainActivity): Plugin(activity) {

  private val securityPlugin: SecurityBiometricsPlugin by lazy {
    SecurityBiometricsPlugin(activity.applicationContext, activity)
  }

  @Command
  fun checkBiometricAvailability(invoke: Invoke) {
    try {
      val info = securityPlugin.checkBiometricAvailability()
      val result = JSObject()
      result.put("available", info["available"])
      result.put("enrolled", info["enrolled"])
      result.put("types", info["types"])
      invoke.resolve(result)
    } catch (e: Exception) {
      invoke.reject("Check biometric availability failed: ${e.message}")
    }
  }

  @Command
  fun authenticateBiometric(invoke: Invoke) {
    val options = invoke.getObject("options") ?: run {
      invoke.reject("Missing 'options' parameter")
      return
    }

    val title = options.getString("title") ?: "Authenticate"
    val subtitle = options.optString("subtitle", null)
    val description = options.optString("description", null)
    val negativeButtonText = options.getString("negativeButtonText") ?: "Cancel"

    try {
      activity.runOnUiThread {
        securityPlugin.authenticateBiometric(
          title, subtitle, description, negativeButtonText
        ) { success, error ->
          val result = JSObject()
          result.put("success", success)
          if (error != null) {
            result.put("error", error)
          }
          result.put("biometricType", "fingerprint")
          invoke.resolve(result)
        }
      }
    } catch (e: Exception) {
      invoke.reject("Authentication failed: ${e.message}")
    }
  }

  @Command
  fun getBiometricTypes(invoke: Invoke) {
    try {
      val types = securityPlugin.getBiometricTypes()
      val result = JSObject()
      result.put("value", types)
      invoke.resolve(result)
    } catch (e: Exception) {
      invoke.reject("Get biometric types failed: ${e.message}")
    }
  }

  @Command
  fun generateEncryptionKey(invoke: Invoke) {
    val keyName = invoke.getString("keyName") ?: run {
      invoke.reject("Missing 'keyName' parameter")
      return
    }

    try {
      val message = securityPlugin.generateEncryptionKey(keyName)
      val result = JSObject()
      result.put("value", message)
      invoke.resolve(result)
    } catch (e: Exception) {
      invoke.reject("Generate encryption key failed: ${e.message}")
    }
  }

  @Command
  fun encryptData(invoke: Invoke) {
    val keyName = invoke.getString("keyName") ?: run {
      invoke.reject("Missing 'keyName' parameter")
      return
    }

    val data = invoke.getString("data") ?: run {
      invoke.reject("Missing 'data' parameter")
      return
    }

    try {
      val encrypted = securityPlugin.encryptData(keyName, data)
      val result = JSObject()
      result.put("value", encrypted)
      invoke.resolve(result)
    } catch (e: Exception) {
      invoke.reject("Encrypt data failed: ${e.message}")
    }
  }

  @Command
  fun decryptData(invoke: Invoke) {
    val keyName = invoke.getString("keyName") ?: run {
      invoke.reject("Missing 'keyName' parameter")
      return
    }

    val encryptedData = invoke.getString("encryptedData") ?: run {
      invoke.reject("Missing 'encryptedData' parameter")
      return
    }

    try {
      val decrypted = securityPlugin.decryptData(keyName, encryptedData)
      val result = JSObject()
      result.put("value", decrypted)
      invoke.resolve(result)
    } catch (e: Exception) {
      invoke.reject("Decrypt data failed: ${e.message}")
    }
  }

  @Command
  fun secureStorageSet(invoke: Invoke) {
    val key = invoke.getString("key") ?: run {
      invoke.reject("Missing 'key' parameter")
      return
    }

    val value = invoke.getString("value") ?: run {
      invoke.reject("Missing 'value' parameter")
      return
    }

    try {
      securityPlugin.secureStorageSet(key, value)
      invoke.resolve()
    } catch (e: Exception) {
      invoke.reject("Secure storage set failed: ${e.message}")
    }
  }

  @Command
  fun secureStorageGet(invoke: Invoke) {
    val key = invoke.getString("key") ?: run {
      invoke.reject("Missing 'key' parameter")
      return
    }

    try {
      val value = securityPlugin.secureStorageGet(key)
      if (value != null) {
        val result = JSObject()
        result.put("value", value)
        invoke.resolve(result)
      } else {
        invoke.reject("Key not found")
      }
    } catch (e: Exception) {
      invoke.reject("Secure storage get failed: ${e.message}")
    }
  }

  @Command
  fun secureStorageDelete(invoke: Invoke) {
    val key = invoke.getString("key") ?: run {
      invoke.reject("Missing 'key' parameter")
      return
    }

    try {
      securityPlugin.secureStorageDelete(key)
      invoke.resolve()
    } catch (e: Exception) {
      invoke.reject("Secure storage delete failed: ${e.message}")
    }
  }
}
