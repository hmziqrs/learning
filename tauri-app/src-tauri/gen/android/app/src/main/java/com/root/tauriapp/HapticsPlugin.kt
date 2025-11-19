package com.root.tauriapp

import android.content.Context
import android.os.Build
import android.os.VibrationEffect
import android.os.Vibrator
import android.os.VibratorManager
import androidx.annotation.RequiresApi

/**
 * HapticsPlugin provides haptic feedback functionality for Android devices
 * Supports different vibration patterns and intensities based on Android API level
 */
class HapticsPlugin(private val context: Context) {

    private val vibrator: Vibrator by lazy {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            val vibratorManager = context.getSystemService(Context.VIBRATOR_MANAGER_SERVICE) as VibratorManager
            vibratorManager.defaultVibrator
        } else {
            @Suppress("DEPRECATION")
            context.getSystemService(Context.VIBRATOR_SERVICE) as Vibrator
        }
    }

    /**
     * Check if the device has a vibrator
     */
    fun hasVibrator(): Boolean {
        return vibrator.hasVibrator()
    }

    /**
     * Trigger impact haptic feedback
     * @param style: "light", "medium", or "heavy"
     */
    fun impact(style: String) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val effect = when (style) {
                "light" -> createOneShot(10, 50)
                "medium" -> createOneShot(20, 100)
                "heavy" -> createOneShot(30, 255)
                else -> throw IllegalArgumentException("Invalid impact style: $style")
            }
            vibrator.vibrate(effect)
        } else {
            // Fallback for older Android versions
            @Suppress("DEPRECATION")
            val duration = when (style) {
                "light" -> 10L
                "medium" -> 20L
                "heavy" -> 30L
                else -> throw IllegalArgumentException("Invalid impact style: $style")
            }
            vibrator.vibrate(duration)
        }
    }

    /**
     * Trigger notification haptic feedback
     * @param type: "success", "warning", or "error"
     */
    fun notification(type: String) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val effect = when (type) {
                "success" -> {
                    // Success: double quick pulse
                    createWaveform(longArrayOf(0, 10, 50, 10), -1)
                }
                "warning" -> {
                    // Warning: triple medium pulse
                    createWaveform(longArrayOf(0, 15, 50, 15, 50, 15), -1)
                }
                "error" -> {
                    // Error: single strong pulse
                    createOneShot(50, 255)
                }
                else -> throw IllegalArgumentException("Invalid notification type: $type")
            }
            vibrator.vibrate(effect)
        } else {
            // Fallback for older Android versions
            @Suppress("DEPRECATION")
            val pattern = when (type) {
                "success" -> longArrayOf(0, 10, 50, 10)
                "warning" -> longArrayOf(0, 15, 50, 15, 50, 15)
                "error" -> longArrayOf(0, 50)
                else -> throw IllegalArgumentException("Invalid notification type: $type")
            }
            vibrator.vibrate(pattern, -1)
        }
    }

    /**
     * Vibrate for a specific duration
     * @param duration: Duration in milliseconds
     */
    fun vibrate(duration: Long) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val effect = createOneShot(duration, VibrationEffect.DEFAULT_AMPLITUDE)
            vibrator.vibrate(effect)
        } else {
            @Suppress("DEPRECATION")
            vibrator.vibrate(duration)
        }
    }

    /**
     * Vibrate with a pattern
     * @param pattern: Array of durations alternating between off and on
     */
    fun vibratePattern(pattern: LongArray) {
        if (pattern.isEmpty()) {
            throw IllegalArgumentException("Pattern cannot be empty")
        }

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val effect = createWaveform(pattern, -1)
            vibrator.vibrate(effect)
        } else {
            @Suppress("DEPRECATION")
            vibrator.vibrate(pattern, -1)
        }
    }

    /**
     * Cancel any ongoing vibration
     */
    fun cancel() {
        vibrator.cancel()
    }

    // Helper methods for VibrationEffect (API 26+)
    @RequiresApi(Build.VERSION_CODES.O)
    private fun createOneShot(duration: Long, amplitude: Int): VibrationEffect {
        return VibrationEffect.createOneShot(duration, amplitude)
    }

    @RequiresApi(Build.VERSION_CODES.O)
    private fun createWaveform(pattern: LongArray, repeat: Int): VibrationEffect {
        return VibrationEffect.createWaveform(pattern, repeat)
    }
}
