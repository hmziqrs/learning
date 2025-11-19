package com.root.tauriapp

import android.content.Context
import android.content.SharedPreferences
import android.os.Build
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import java.nio.charset.StandardCharsets
import java.security.KeyStore
import java.util.Base64
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

/**
 * SecurityBiometricsPlugin provides biometric authentication and secure storage
 * functionality for Android devices using BiometricPrompt and Android Keystore
 */
class SecurityBiometricsPlugin(private val context: Context, private val activity: FragmentActivity) {

    private val keyStore: KeyStore = KeyStore.getInstance("AndroidKeyStore").apply {
        load(null)
    }

    private val securePrefs: SharedPreferences = context.getSharedPreferences(
        "secure_storage",
        Context.MODE_PRIVATE
    )

    companion object {
        private const val ANDROID_KEYSTORE = "AndroidKeyStore"
        private const val TRANSFORMATION = "AES/GCM/NoPadding"
        private const val IV_SEPARATOR = "]"  // Separator between IV and ciphertext
    }

    /**
     * Check if biometric authentication is available on the device
     */
    fun checkBiometricAvailability(): Map<String, Any> {
        val biometricManager = BiometricManager.from(context)
        val canAuthenticate = biometricManager.canAuthenticate(
            BiometricManager.Authenticators.BIOMETRIC_STRONG
        )

        val available = when (canAuthenticate) {
            BiometricManager.BIOMETRIC_SUCCESS -> true
            else -> false
        }

        val enrolled = when (canAuthenticate) {
            BiometricManager.BIOMETRIC_SUCCESS -> true
            BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED -> false
            else -> false
        }

        val types = mutableListOf<String>()
        if (available) {
            // Android doesn't provide API to distinguish between fingerprint/face/iris
            // We'll return "fingerprint" as the generic type
            types.add("fingerprint")
        }

        return mapOf(
            "available" to available,
            "enrolled" to enrolled,
            "types" to types
        )
    }

    /**
     * Authenticate user with biometrics
     * @param title: Prompt title
     * @param subtitle: Prompt subtitle (optional)
     * @param description: Prompt description (optional)
     * @param negativeButtonText: Text for cancel button
     * @param callback: Callback with authentication result
     */
    fun authenticateBiometric(
        title: String,
        subtitle: String?,
        description: String?,
        negativeButtonText: String,
        callback: (Boolean, String?) -> Unit
    ) {
        val executor = ContextCompat.getMainExecutor(context)

        val biometricPrompt = BiometricPrompt(
            activity,
            executor,
            object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    super.onAuthenticationError(errorCode, errString)
                    callback(false, errString.toString())
                }

                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    super.onAuthenticationSucceeded(result)
                    callback(true, null)
                }

                override fun onAuthenticationFailed() {
                    super.onAuthenticationFailed()
                    callback(false, "Authentication failed")
                }
            }
        )

        val promptInfoBuilder = BiometricPrompt.PromptInfo.Builder()
            .setTitle(title)
            .setNegativeButtonText(negativeButtonText)

        subtitle?.let { promptInfoBuilder.setSubtitle(it) }
        description?.let { promptInfoBuilder.setDescription(it) }

        val promptInfo = promptInfoBuilder.build()
        biometricPrompt.authenticate(promptInfo)
    }

    /**
     * Get available biometric types
     */
    fun getBiometricTypes(): List<String> {
        val biometricManager = BiometricManager.from(context)
        val canAuthenticate = biometricManager.canAuthenticate(
            BiometricManager.Authenticators.BIOMETRIC_STRONG
        )

        return if (canAuthenticate == BiometricManager.BIOMETRIC_SUCCESS) {
            listOf("fingerprint")
        } else {
            emptyList()
        }
    }

    /**
     * Generate an encryption key in the Android Keystore
     * @param keyName: Name of the key to generate
     */
    fun generateEncryptionKey(keyName: String): String {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) {
            throw IllegalStateException("Android Keystore encryption requires API 23+")
        }

        val keyGenerator = KeyGenerator.getInstance(
            KeyProperties.KEY_ALGORITHM_AES,
            ANDROID_KEYSTORE
        )

        val keyGenParameterSpec = KeyGenParameterSpec.Builder(
            keyName,
            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
        )
            .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
            .setKeySize(256)
            .build()

        keyGenerator.init(keyGenParameterSpec)
        keyGenerator.generateKey()

        return "Key '$keyName' generated successfully"
    }

    /**
     * Encrypt data using a key from the Android Keystore
     * @param keyName: Name of the key to use
     * @param data: Plain text data to encrypt
     * @return Base64 encoded encrypted data with IV
     */
    fun encryptData(keyName: String, data: String): String {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) {
            throw IllegalStateException("Android Keystore encryption requires API 23+")
        }

        val key = getSecretKey(keyName)
            ?: throw IllegalArgumentException("Key '$keyName' not found. Generate it first.")

        val cipher = Cipher.getInstance(TRANSFORMATION)
        cipher.init(Cipher.ENCRYPT_MODE, key)

        val iv = cipher.iv
        val encryptedBytes = cipher.doFinal(data.toByteArray(StandardCharsets.UTF_8))

        // Combine IV and encrypted data
        val combined = iv + encryptedBytes

        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Base64.getEncoder().encodeToString(combined)
        } else {
            android.util.Base64.encodeToString(combined, android.util.Base64.NO_WRAP)
        }
    }

    /**
     * Decrypt data using a key from the Android Keystore
     * @param keyName: Name of the key to use
     * @param encryptedData: Base64 encoded encrypted data with IV
     * @return Decrypted plain text data
     */
    fun decryptData(keyName: String, encryptedData: String): String {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) {
            throw IllegalStateException("Android Keystore encryption requires API 23+")
        }

        val key = getSecretKey(keyName)
            ?: throw IllegalArgumentException("Key '$keyName' not found")

        val combined = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Base64.getDecoder().decode(encryptedData)
        } else {
            android.util.Base64.decode(encryptedData, android.util.Base64.NO_WRAP)
        }

        // Extract IV (first 12 bytes for GCM)
        val iv = combined.copyOfRange(0, 12)
        val encryptedBytes = combined.copyOfRange(12, combined.size)

        val cipher = Cipher.getInstance(TRANSFORMATION)
        val spec = GCMParameterSpec(128, iv)
        cipher.init(Cipher.DECRYPT_MODE, key, spec)

        val decryptedBytes = cipher.doFinal(encryptedBytes)
        return String(decryptedBytes, StandardCharsets.UTF_8)
    }

    /**
     * Store data securely in SharedPreferences
     * Note: For enhanced security, data should be encrypted before storage
     * @param key: Storage key
     * @param value: Value to store
     */
    fun secureStorageSet(key: String, value: String) {
        securePrefs.edit().putString(key, value).apply()
    }

    /**
     * Retrieve data from secure storage
     * @param key: Storage key
     * @return Stored value or null if not found
     */
    fun secureStorageGet(key: String): String? {
        return securePrefs.getString(key, null)
    }

    /**
     * Delete data from secure storage
     * @param key: Storage key
     */
    fun secureStorageDelete(key: String) {
        securePrefs.edit().remove(key).apply()
    }

    /**
     * Get a secret key from the Android Keystore
     */
    private fun getSecretKey(keyName: String): SecretKey? {
        return if (keyStore.containsAlias(keyName)) {
            (keyStore.getEntry(keyName, null) as? KeyStore.SecretKeyEntry)?.secretKey
        } else {
            null
        }
    }
}
