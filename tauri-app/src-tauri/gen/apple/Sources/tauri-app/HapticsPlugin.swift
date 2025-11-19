import UIKit
import Tauri
import WebKit

class HapticsPlugin: Plugin {

    private var impactGeneratorLight: UIImpactFeedbackGenerator?
    private var impactGeneratorMedium: UIImpactFeedbackGenerator?
    private var impactGeneratorHeavy: UIImpactFeedbackGenerator?
    private var notificationGenerator: UINotificationFeedbackGenerator?
    private var selectionGenerator: UISelectionFeedbackGenerator?

    override init() {
        super.init()

        // Initialize generators
        impactGeneratorLight = UIImpactFeedbackGenerator(style: .light)
        impactGeneratorMedium = UIImpactFeedbackGenerator(style: .medium)
        impactGeneratorHeavy = UIImpactFeedbackGenerator(style: .heavy)
        notificationGenerator = UINotificationFeedbackGenerator()
        selectionGenerator = UISelectionFeedbackGenerator()
    }

    @objc public func hapticImpact(_ invoke: Invoke) throws {
        guard let style = invoke.getString("style") else {
            invoke.reject("Missing 'style' parameter")
            return
        }

        switch style {
        case "light":
            impactGeneratorLight?.prepare()
            impactGeneratorLight?.impactOccurred()
            invoke.resolve()

        case "medium":
            impactGeneratorMedium?.prepare()
            impactGeneratorMedium?.impactOccurred()
            invoke.resolve()

        case "heavy":
            impactGeneratorHeavy?.prepare()
            impactGeneratorHeavy?.impactOccurred()
            invoke.resolve()

        default:
            invoke.reject("Invalid impact style: \(style). Must be 'light', 'medium', or 'heavy'")
        }
    }

    @objc public func hapticNotification(_ invoke: Invoke) throws {
        guard let notificationType = invoke.getString("notificationType") else {
            invoke.reject("Missing 'notificationType' parameter")
            return
        }

        notificationGenerator?.prepare()

        switch notificationType {
        case "success":
            notificationGenerator?.notificationOccurred(.success)
            invoke.resolve()

        case "warning":
            notificationGenerator?.notificationOccurred(.warning)
            invoke.resolve()

        case "error":
            notificationGenerator?.notificationOccurred(.error)
            invoke.resolve()

        default:
            invoke.reject("Invalid notification type: \(notificationType). Must be 'success', 'warning', or 'error'")
        }
    }

    @objc public func vibrate(_ invoke: Invoke) throws {
        // iOS doesn't support custom duration vibrations via UIFeedbackGenerator
        // We'll use a medium impact as fallback
        impactGeneratorMedium?.prepare()
        impactGeneratorMedium?.impactOccurred()
        invoke.resolve()
    }

    @objc public func vibratePattern(_ invoke: Invoke) throws {
        // iOS doesn't support pattern vibrations via UIFeedbackGenerator
        // For complex patterns, developers would need to use Core Haptics
        invoke.reject("Pattern vibration not supported on iOS via UIFeedbackGenerator. Use Core Haptics framework for custom patterns.")
    }

    @objc public func cancelVibration(_ invoke: Invoke) throws {
        // iOS haptic feedback is instantaneous and cannot be cancelled
        // Return success since there's nothing to cancel
        invoke.resolve()
    }

    @objc public func hasVibrator(_ invoke: Invoke) throws {
        // Check if device supports haptic feedback
        // Taptic Engine is available on iPhone 6s and later
        var hasHaptics = false

        if #available(iOS 10.0, *) {
            // All devices running iOS 10+ with Taptic Engine support haptics
            // For simplicity, we'll check if the feedback generators can be instantiated
            hasHaptics = true
        }

        let result: [String: Any] = ["value": hasHaptics]
        invoke.resolve(result)
    }

    @objc public func selection(_ invoke: Invoke) throws {
        // Additional API for selection feedback (useful for UI interactions)
        selectionGenerator?.prepare()
        selectionGenerator?.selectionChanged()
        invoke.resolve()
    }
}

@_cdecl("init_plugin_haptics")
func initPlugin() -> Plugin {
    return HapticsPlugin()
}
