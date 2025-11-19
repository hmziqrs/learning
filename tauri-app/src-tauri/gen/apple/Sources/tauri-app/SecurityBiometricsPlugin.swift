import UIKit
import LocalAuthentication
import Security
import Tauri
import WebKit
import CryptoKit

class SecurityBiometricsPlugin: Plugin {

    private let keychainService = "com.root.tauriapp.securestorage"

    // MARK: - Biometric Authentication

    @objc public func checkBiometricAvailability(_ invoke: Invoke) throws {
        let context = LAContext()
        var error: NSError?

        let canEvaluate = context.canEvaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            error: &error
        )

        var types: [String] = []
        if canEvaluate {
            if #available(iOS 11.0, *) {
                switch context.biometryType {
                case .faceID:
                    types.append("face")
                case .touchID:
                    types.append("fingerprint")
                case .none:
                    break
                @unknown default:
                    break
                }
            } else {
                // For iOS < 11, assume Touch ID if biometrics are available
                types.append("fingerprint")
            }
        }

        let result: [String: Any] = [
            "available": canEvaluate,
            "enrolled": canEvaluate, // If can evaluate, biometrics are enrolled
            "types": types
        ]

        invoke.resolve(result)
    }

    @objc public func authenticateBiometric(_ invoke: Invoke) throws {
        guard let options = invoke.getObject("options") else {
            invoke.reject("Missing 'options' parameter")
            return
        }

        let title = options["title"] as? String ?? "Authenticate"
        let reason = options["description"] as? String ?? "Verify your identity"

        let context = LAContext()
        context.localizedCancelTitle = options["negativeButtonText"] as? String ?? "Cancel"

        var error: NSError?
        guard context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) else {
            let errorMessage = error?.localizedDescription ?? "Biometric authentication not available"
            let result: [String: Any] = [
                "success": false,
                "error": errorMessage
            ]
            invoke.resolve(result)
            return
        }

        context.evaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            localizedReason: reason
        ) { success, error in
            DispatchQueue.main.async {
                var biometricType: String? = nil
                if #available(iOS 11.0, *) {
                    switch context.biometryType {
                    case .faceID:
                        biometricType = "face"
                    case .touchID:
                        biometricType = "fingerprint"
                    case .none:
                        break
                    @unknown default:
                        break
                    }
                }

                let result: [String: Any] = [
                    "success": success,
                    "error": error?.localizedDescription ?? NSNull(),
                    "biometricType": biometricType ?? NSNull()
                ]
                invoke.resolve(result)
            }
        }
    }

    @objc public func getBiometricTypes(_ invoke: Invoke) throws {
        let context = LAContext()
        var types: [String] = []

        if context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: nil) {
            if #available(iOS 11.0, *) {
                switch context.biometryType {
                case .faceID:
                    types.append("face")
                case .touchID:
                    types.append("fingerprint")
                case .none:
                    break
                @unknown default:
                    break
                }
            } else {
                types.append("fingerprint")
            }
        }

        let result: [String: Any] = ["value": types]
        invoke.resolve(result)
    }

    // MARK: - Encryption/Decryption

    @objc public func generateEncryptionKey(_ invoke: Invoke) throws {
        guard let keyName = invoke.getString("keyName") else {
            invoke.reject("Missing 'keyName' parameter")
            return
        }

        // Generate a symmetric key and store it in the Keychain
        if #available(iOS 13.0, *) {
            let key = SymmetricKey(size: .bits256)
            let keyData = key.withUnsafeBytes { Data(Array($0)) }

            // Store key in Keychain
            let query: [String: Any] = [
                kSecClass as String: kSecClassKey,
                kSecAttrApplicationTag as String: "encryption_\(keyName)".data(using: .utf8)!,
                kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
                kSecValueData as String: keyData
            ]

            // Delete existing key if present
            SecItemDelete(query as CFDictionary)

            // Add new key
            let status = SecItemAdd(query as CFDictionary, nil)

            if status == errSecSuccess {
                let result: [String: Any] = ["value": "Key '\(keyName)' generated successfully"]
                invoke.resolve(result)
            } else {
                invoke.reject("Failed to generate key: \(status)")
            }
        } else {
            invoke.reject("Encryption requires iOS 13+")
        }
    }

    @objc public func encryptData(_ invoke: Invoke) throws {
        guard let keyName = invoke.getString("keyName"),
              let data = invoke.getString("data") else {
            invoke.reject("Missing 'keyName' or 'data' parameter")
            return
        }

        if #available(iOS 13.0, *) {
            guard let keyData = getEncryptionKey(keyName: keyName) else {
                invoke.reject("Key '\(keyName)' not found. Generate it first.")
                return
            }

            let key = SymmetricKey(data: keyData)
            guard let plainData = data.data(using: .utf8) else {
                invoke.reject("Failed to encode data")
                return
            }

            do {
                let sealedBox = try AES.GCM.seal(plainData, using: key)
                guard let combined = sealedBox.combined else {
                    invoke.reject("Failed to create sealed box")
                    return
                }

                let base64Encrypted = combined.base64EncodedString()
                let result: [String: Any] = ["value": base64Encrypted]
                invoke.resolve(result)
            } catch {
                invoke.reject("Encryption failed: \(error.localizedDescription)")
            }
        } else {
            invoke.reject("Encryption requires iOS 13+")
        }
    }

    @objc public func decryptData(_ invoke: Invoke) throws {
        guard let keyName = invoke.getString("keyName"),
              let encryptedData = invoke.getString("encryptedData") else {
            invoke.reject("Missing 'keyName' or 'encryptedData' parameter")
            return
        }

        if #available(iOS 13.0, *) {
            guard let keyData = getEncryptionKey(keyName: keyName) else {
                invoke.reject("Key '\(keyName)' not found")
                return
            }

            let key = SymmetricKey(data: keyData)
            guard let combined = Data(base64Encoded: encryptedData) else {
                invoke.reject("Failed to decode encrypted data")
                return
            }

            do {
                let sealedBox = try AES.GCM.SealedBox(combined: combined)
                let decryptedData = try AES.GCM.open(sealedBox, using: key)

                if let decryptedString = String(data: decryptedData, encoding: .utf8) {
                    let result: [String: Any] = ["value": decryptedString]
                    invoke.resolve(result)
                } else {
                    invoke.reject("Failed to decode decrypted data")
                }
            } catch {
                invoke.reject("Decryption failed: \(error.localizedDescription)")
            }
        } else {
            invoke.reject("Decryption requires iOS 13+")
        }
    }

    // MARK: - Secure Storage (Keychain)

    @objc public func secureStorageSet(_ invoke: Invoke) throws {
        guard let key = invoke.getString("key"),
              let value = invoke.getString("value") else {
            invoke.reject("Missing 'key' or 'value' parameter")
            return
        }

        guard let valueData = value.data(using: .utf8) else {
            invoke.reject("Failed to encode value")
            return
        }

        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: keychainService,
            kSecAttrAccount as String: key,
            kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
            kSecValueData as String: valueData
        ]

        // Delete existing item if present
        SecItemDelete(query as CFDictionary)

        // Add new item
        let status = SecItemAdd(query as CFDictionary, nil)

        if status == errSecSuccess {
            invoke.resolve()
        } else {
            invoke.reject("Failed to store data: \(status)")
        }
    }

    @objc public func secureStorageGet(_ invoke: Invoke) throws {
        guard let key = invoke.getString("key") else {
            invoke.reject("Missing 'key' parameter")
            return
        }

        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: keychainService,
            kSecAttrAccount as String: key,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        if status == errSecSuccess,
           let data = result as? Data,
           let value = String(data: data, encoding: .utf8) {
            let response: [String: Any] = ["value": value]
            invoke.resolve(response)
        } else if status == errSecItemNotFound {
            invoke.reject("Key not found")
        } else {
            invoke.reject("Failed to retrieve data: \(status)")
        }
    }

    @objc public func secureStorageDelete(_ invoke: Invoke) throws {
        guard let key = invoke.getString("key") else {
            invoke.reject("Missing 'key' parameter")
            return
        }

        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: keychainService,
            kSecAttrAccount as String: key
        ]

        let status = SecItemDelete(query as CFDictionary)

        if status == errSecSuccess || status == errSecItemNotFound {
            invoke.resolve()
        } else {
            invoke.reject("Failed to delete data: \(status)")
        }
    }

    // MARK: - Helper Methods

    private func getEncryptionKey(keyName: String) -> Data? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassKey,
            kSecAttrApplicationTag as String: "encryption_\(keyName)".data(using: .utf8)!,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        if status == errSecSuccess, let keyData = result as? Data {
            return keyData
        }
        return nil
    }
}

@_cdecl("init_plugin_security_biometrics")
func initPluginSecurityBiometrics() -> Plugin {
    return SecurityBiometricsPlugin()
}
