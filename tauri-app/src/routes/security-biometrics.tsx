import { createFileRoute } from '@tanstack/react-router'
import { invoke } from '@tauri-apps/api/core'
import { Shield, Fingerprint } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { useState } from 'react'

export const Route = createFileRoute('/security-biometrics')({
  component: SecurityBiometricsModule,
})

type BiometricType = 'fingerprint' | 'face' | 'iris' | 'none'

interface BiometricInfo {
  available: boolean
  enrolled: boolean
  types: BiometricType[]
}

interface AuthenticationResult {
  success: boolean
  error?: string
  biometricType?: BiometricType
}

function SecurityBiometricsModule() {
  const [output, setOutput] = useState<string[]>([])
  const [biometricInfo, setBiometricInfo] = useState<BiometricInfo | null>(null)
  const [isLoading, setIsLoading] = useState(false)

  // Secure storage state
  const [storageKey, setStorageKey] = useState('')
  const [storageValue, setStorageValue] = useState('')
  const [retrievedValue, setRetrievedValue] = useState('')

  // Encryption state
  const [encryptionKey, setEncryptionKey] = useState('mySecureKey')
  const [plaintext, setPlaintext] = useState('')
  const [encryptedData, setEncryptedData] = useState('')
  const [decryptedData, setDecryptedData] = useState('')

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  // Check biometric availability
  const handleCheckAvailability = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Checking biometric availability...')

    try {
      const info = await invoke<BiometricInfo>('check_biometric_availability')
      setBiometricInfo(info)
      addOutput(`Biometrics ${info.available ? 'available' : 'not available'}`)
      addOutput(`Enrolled: ${info.enrolled}, Types: ${info.types.join(', ')}`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
      setBiometricInfo(null)
    } finally {
      setIsLoading(false)
    }
  }

  // Authenticate with biometrics
  const handleAuthenticate = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Requesting biometric authentication...')

    try {
      const result = await invoke<AuthenticationResult>('authenticate_biometric', {
        options: {
          title: 'Authenticate',
          subtitle: 'Verify your identity',
          description: 'Use biometrics to authenticate',
          negativeButtonText: 'Cancel',
          allowDeviceCredential: false,
        },
      })

      if (result.success) {
        addOutput(`Authentication successful (${result.biometricType || 'unknown'})`)
      } else {
        addOutput(`Authentication failed: ${result.error || 'Unknown error'}`, false)
      }
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Get biometric types
  const handleGetBiometricTypes = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Retrieving available biometric types...')

    try {
      const types = await invoke<string[]>('get_biometric_types')
      addOutput(`Available types: ${types.join(', ')}`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Generate encryption key
  const handleGenerateKey = async () => {
    if (isLoading || !encryptionKey) return
    setIsLoading(true)
    addOutput(`Generating encryption key: ${encryptionKey}...`)

    try {
      const result = await invoke<string>('generate_encryption_key', {
        keyName: encryptionKey,
      })
      addOutput(`Key generated successfully: ${result}`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Encrypt data
  const handleEncrypt = async () => {
    if (isLoading || !encryptionKey || !plaintext) return
    setIsLoading(true)
    addOutput(`Encrypting data with key: ${encryptionKey}...`)

    try {
      const encrypted = await invoke<string>('encrypt_data', {
        keyName: encryptionKey,
        data: plaintext,
      })
      setEncryptedData(encrypted)
      addOutput(`Data encrypted successfully`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Decrypt data
  const handleDecrypt = async () => {
    if (isLoading || !encryptionKey || !encryptedData) return
    setIsLoading(true)
    addOutput(`Decrypting data with key: ${encryptionKey}...`)

    try {
      const decrypted = await invoke<string>('decrypt_data', {
        keyName: encryptionKey,
        encryptedData,
      })
      setDecryptedData(decrypted)
      addOutput(`Data decrypted successfully`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Secure storage operations
  const handleStorageSet = async () => {
    if (isLoading || !storageKey || !storageValue) return
    setIsLoading(true)
    addOutput(`Storing data securely: ${storageKey}...`)

    try {
      await invoke('secure_storage_set', {
        key: storageKey,
        value: storageValue,
      })
      addOutput(`Data stored successfully`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  const handleStorageGet = async () => {
    if (isLoading || !storageKey) return
    setIsLoading(true)
    addOutput(`Retrieving data: ${storageKey}...`)

    try {
      const value = await invoke<string>('secure_storage_get', {
        key: storageKey,
      })
      setRetrievedValue(value)
      addOutput(`Data retrieved successfully`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  const handleStorageDelete = async () => {
    if (isLoading || !storageKey) return
    setIsLoading(true)
    addOutput(`Deleting data: ${storageKey}...`)

    try {
      await invoke('secure_storage_delete', { key: storageKey })
      setRetrievedValue('')
      addOutput(`Data deleted successfully`)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <ModulePageLayout
      title="Security & Biometrics Module"
      description="Biometric authentication and secure cryptographic operations for enhanced app security"
      icon={Shield}
    >
      <div className="space-y-6">
        {/* Status Notice */}
        <section className="rounded-lg border border-green-500/50 bg-green-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-green-500">✓</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Current implementation:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li>
                <strong className="text-green-600">✓ Rust Commands</strong> - 9 Tauri commands registered and functional
              </li>
              <li>
                <strong className="text-green-600">✓ Android Plugin</strong> - SecurityBiometricsPlugin.kt with BiometricPrompt
              </li>
              <li>
                <strong className="text-green-600">✓ iOS Plugin</strong> - SecurityBiometricsPlugin.swift with LocalAuthentication
              </li>
              <li>
                <strong className="text-red-600">✗ Desktop</strong> - Biometrics not available on Windows/macOS/Linux
              </li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># Android Implementation (Complete):</div>
              <div>SecurityBiometricsPlugin.kt - BiometricPrompt + Android Keystore</div>
              <div className="mt-2"># iOS Implementation (Complete):</div>
              <div>SecurityBiometricsPlugin.swift - LocalAuthentication + iOS Keychain</div>
            </div>
            <p className="text-muted-foreground mt-2">
              Android: Fully functional with BiometricPrompt API and Android Keystore. iOS: Fully functional with LocalAuthentication and Keychain. Desktop: Returns appropriate error messages.
            </p>
          </div>
        </section>

        {/* Biometric Availability Check */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Fingerprint className="w-5 h-5" />
            Biometric Availability
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Check if biometric authentication is available on this device
            </p>

            <div className="flex flex-wrap gap-2 items-center">
              <Button onClick={handleCheckAvailability} variant="outline" disabled={isLoading}>
                {isLoading ? 'Checking...' : 'Check Availability'}
              </Button>
              <Button onClick={handleGetBiometricTypes} variant="outline" disabled={isLoading}>
                Get Biometric Types
              </Button>

              {biometricInfo !== null && (
                <div
                  className={`px-4 py-2 rounded-md text-sm font-medium ${
                    biometricInfo.available
                      ? 'bg-green-500/10 text-green-700 dark:text-green-400 border border-green-500/30'
                      : 'bg-red-500/10 text-red-700 dark:text-red-400 border border-red-500/30'
                  }`}
                >
                  {biometricInfo.available ? '✓ Available' : '✗ Not Available'}
                </div>
              )}
            </div>

            {biometricInfo && biometricInfo.available && (
              <div className="bg-muted rounded-md p-3 text-sm">
                <p>
                  <strong>Enrolled:</strong> {biometricInfo.enrolled ? 'Yes' : 'No'}
                </p>
                <p>
                  <strong>Types:</strong> {biometricInfo.types.join(', ') || 'None'}
                </p>
              </div>
            )}
          </div>
        </section>

        {/* Authentication */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold">Biometric Authentication</h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Trigger biometric authentication prompt (fingerprint, face, or iris)
            </p>

            <Button
              onClick={handleAuthenticate}
              variant="default"
              size="lg"
              className="w-full sm:w-auto"
              disabled={isLoading}
            >
              {isLoading ? 'Authenticating...' : 'Authenticate with Biometrics'}
            </Button>
          </div>
        </section>

        {/* Secure Storage */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold">Secure Storage</h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Store and retrieve sensitive data using platform-specific secure storage (Android Keystore / iOS Keychain)
            </p>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Key</label>
                <Input
                  type="text"
                  placeholder="storage_key"
                  value={storageKey}
                  onChange={(e) => setStorageKey(e.target.value)}
                />
              </div>

              <div className="space-y-2">
                <label className="text-sm font-medium">Value</label>
                <Input
                  type="text"
                  placeholder="sensitive_data"
                  value={storageValue}
                  onChange={(e) => setStorageValue(e.target.value)}
                />
              </div>
            </div>

            <div className="flex flex-wrap gap-2">
              <Button onClick={handleStorageSet} variant="outline" disabled={isLoading || !storageKey || !storageValue}>
                Store Data
              </Button>
              <Button onClick={handleStorageGet} variant="outline" disabled={isLoading || !storageKey}>
                Retrieve Data
              </Button>
              <Button onClick={handleStorageDelete} variant="outline" disabled={isLoading || !storageKey}>
                Delete Data
              </Button>
            </div>

            {retrievedValue && (
              <div className="bg-muted rounded-md p-3 text-sm">
                <p className="font-medium">Retrieved Value:</p>
                <p className="font-mono">{retrievedValue}</p>
              </div>
            )}
          </div>
        </section>

        {/* Encryption/Decryption */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold">Encryption & Decryption</h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Generate secure encryption keys and encrypt/decrypt data
            </p>

            <div className="space-y-3">
              <div className="space-y-2">
                <label className="text-sm font-medium">Key Name</label>
                <div className="flex gap-2">
                  <Input
                    type="text"
                    placeholder="mySecureKey"
                    value={encryptionKey}
                    onChange={(e) => setEncryptionKey(e.target.value)}
                  />
                  <Button onClick={handleGenerateKey} variant="outline" disabled={isLoading || !encryptionKey}>
                    Generate Key
                  </Button>
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="space-y-2">
                  <label className="text-sm font-medium">Plaintext Data</label>
                  <Input
                    type="text"
                    placeholder="Secret message"
                    value={plaintext}
                    onChange={(e) => setPlaintext(e.target.value)}
                  />
                  <Button
                    onClick={handleEncrypt}
                    variant="outline"
                    className="w-full"
                    disabled={isLoading || !encryptionKey || !plaintext}
                  >
                    Encrypt Data
                  </Button>
                </div>

                <div className="space-y-2">
                  <label className="text-sm font-medium">Encrypted Data</label>
                  <Input type="text" placeholder="Encrypted output" value={encryptedData} readOnly />
                  <Button
                    onClick={handleDecrypt}
                    variant="outline"
                    className="w-full"
                    disabled={isLoading || !encryptionKey || !encryptedData}
                  >
                    Decrypt Data
                  </Button>
                </div>
              </div>

              {decryptedData && (
                <div className="bg-green-500/10 border border-green-500/30 rounded-md p-3 text-sm">
                  <p className="font-medium text-green-700 dark:text-green-400">Decrypted Result:</p>
                  <p className="font-mono">{decryptedData}</p>
                </div>
              )}
            </div>
          </div>
        </section>

        {/* Output Panel */}
        <section className="rounded-lg border p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold">Output</h2>
            <Button variant="outline" size="sm" onClick={() => setOutput([])}>
              Clear
            </Button>
          </div>

          <div className="bg-muted rounded-md p-4 h-64 overflow-y-auto font-mono text-sm">
            {output.length === 0 ? (
              <p className="text-muted-foreground">No output yet. Try using biometric features...</p>
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
              <h4 className="font-semibold">Android Implementation</h4>
              <p className="text-muted-foreground">Requires custom plugin using BiometricPrompt API and Android Keystore</p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div>// BiometricPrompt</div>
                <div>val biometricPrompt = BiometricPrompt(this, executor, callback)</div>
                <div>val promptInfo = BiometricPrompt.PromptInfo.Builder()</div>
                <div>    .setTitle("Authenticate")</div>
                <div>    .setNegativeButtonText("Cancel")</div>
                <div>    .build()</div>
                <div>biometricPrompt.authenticate(promptInfo)</div>
                <div className="mt-2"># Android Keystore</div>
                <div>val keyStore = KeyStore.getInstance("AndroidKeyStore")</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">iOS Implementation</h4>
              <p className="text-muted-foreground">Requires custom plugin using LocalAuthentication framework and Keychain</p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div>// LocalAuthentication</div>
                <div>let context = LAContext()</div>
                <div>context.evaluatePolicy(</div>
                <div>    .deviceOwnerAuthenticationWithBiometrics,</div>
                <div>    localizedReason: "Authenticate"</div>
                <div>) {'{ success, error in }'}</div>
                <div className="mt-2"># iOS Keychain</div>
                <div>SecItemAdd(query as CFDictionary, nil)</div>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400">Permissions & Security</h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Android: USE_BIOMETRIC and USE_FINGERPRINT permissions required</li>
                <li>iOS: NSFaceIDUsageDescription required in Info.plist</li>
                <li>Test on physical devices (biometrics unavailable in emulators)</li>
                <li>Check if biometrics are enrolled before requesting authentication</li>
                <li>Provide fallback to device credential (PIN/pattern/password)</li>
                <li>Use secure storage for sensitive data only</li>
                <li>Encryption keys stored in hardware-backed keystores when available</li>
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
                  <th className="text-center py-2 px-4">Android</th>
                  <th className="text-center py-2 px-4">iOS</th>
                  <th className="text-center py-2 px-4">Desktop</th>
                </tr>
              </thead>
              <tbody className="text-muted-foreground">
                <tr className="border-b">
                  <td className="py-2 px-4">Fingerprint Auth</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Face Recognition</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Iris Scan</td>
                  <td className="text-center py-2 px-4">⚠️</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Device Credential</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">❌</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Secure Storage</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">⚠️</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Hardware Encryption</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">⚠️</td>
                </tr>
              </tbody>
            </table>
            <div className="text-xs text-muted-foreground mt-2 space-y-1">
              <p>✅ Fully Supported | ⚠️ Limited Support | ❌ Not Supported</p>
              <p>* Desktop secure storage uses platform-specific APIs (Windows DPAPI, macOS Keychain, Linux Secret Service)</p>
            </div>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
