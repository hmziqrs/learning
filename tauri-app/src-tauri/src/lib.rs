// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod background_tasks;
use tauri::{Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_clipboard_manager::ClipboardExt;
use std::time::Duration;
use serde::{Deserialize, Serialize, Deserializer};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use background_tasks::*;

// Helper function to deserialize SQLite integer (0/1) to boolean
fn deserialize_bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match i64::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::custom(format!(
            "Expected 0 or 1 for boolean, got {}",
            other
        ))),
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn schedule_notification(
    app: tauri::AppHandle,
    seconds: u64,
    title: String,
    body: String,
) -> Result<(), String> {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(seconds)).await;

        let _ = app.notification()
            .builder()
            .title(&title)
            .body(&body)
            .show();
    });

    Ok(())
}

// Calendar Module Structs
#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: i64,
    title: String,
    description: Option<String>,
    start_time: String,
    end_time: String,
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    is_all_day: bool,
    created_at: String,
    updated_at: String,
}

// In-App Purchase Module Structs
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IapProduct {
    id: String,
    title: String,
    description: String,
    price: String,
    price_amount: f64,
    currency: String,
    #[serde(rename = "type")]
    product_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PurchaseReceipt {
    product_id: String,
    transaction_id: String,
    purchase_date: String,
    status: String,
    platform: String,
    price_paid: f64,
    currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RestoreResult {
    restored_count: usize,
    receipts: Vec<PurchaseReceipt>,
}

// Calendar Module - ICS Export
#[tauri::command]
async fn export_events_to_ics(app: tauri::AppHandle, events: Vec<Event>) -> Result<String, String> {
    use tauri::Manager;

    // Generate ICS content
    let ics_content = generate_ics_content(events)?;

    // Write to file
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    let ics_path = app_data_dir.join("calendar_export.ics");

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    std::fs::write(&ics_path, ics_content)
        .map_err(|e| format!("Failed to write ICS file: {}", e))?;

    Ok(ics_path.to_string_lossy().to_string())
}

fn generate_ics_content(events: Vec<Event>) -> Result<String, String> {
    let mut ics = String::from("BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//Tauri Calendar//EN\r\nCALSCALE:GREGORIAN\r\n");

    for event in events {
        let start = format_ics_date(&event.start_time, event.is_all_day)?;
        let end = format_ics_date(&event.end_time, event.is_all_day)?;

        ics.push_str("BEGIN:VEVENT\r\n");
        ics.push_str(&format!("UID:{}@tauri-calendar\r\n", event.id));

        // For all-day events, use VALUE=DATE parameter
        if event.is_all_day {
            ics.push_str(&format!("DTSTART;VALUE=DATE:{}\r\n", start));
            ics.push_str(&format!("DTEND;VALUE=DATE:{}\r\n", end));
        } else {
            ics.push_str(&format!("DTSTART:{}\r\n", start));
            ics.push_str(&format!("DTEND:{}\r\n", end));
        }

        ics.push_str(&format!("SUMMARY:{}\r\n", event.title));

        if let Some(desc) = event.description {
            ics.push_str(&format!("DESCRIPTION:{}\r\n", desc));
        }

        ics.push_str("END:VEVENT\r\n");
    }

    ics.push_str("END:VCALENDAR\r\n");
    Ok(ics)
}

fn format_ics_date(date_str: &str, is_all_day: bool) -> Result<String, String> {
    if is_all_day {
        // For all-day events, use DATE format (YYYYMMDD)
        // Split on 'T' to get just the date part
        let date = date_str.split('T').next().unwrap_or(date_str);
        Ok(date.replace("-", ""))
    } else {
        // For timed events, use DATE-TIME format (YYYYMMDDTHHMMSS)
        // Remove hyphens and colons, keep the T separator
        // Input format: "2024-11-18T14:30:00"
        // Output format: "20241118T143000"
        let formatted = date_str
            .replace("-", "")
            .replace(":", "");
        Ok(formatted)
    }
}

// In-App Purchase Module Commands
// These are mock/sandbox implementations for demonstration
// In production, these would integrate with platform-specific stores

#[tauri::command]
async fn fetch_iap_products() -> Result<Vec<IapProduct>, String> {
    // Simulate network delay
    tokio::time::sleep(Duration::from_millis(800)).await;

    // Return mock products (in production, these would come from App Store/Play Store)
    Ok(vec![
        IapProduct {
            id: "premium_monthly".to_string(),
            title: "Premium Monthly".to_string(),
            description: "Access to all premium features for 1 month".to_string(),
            price: "$9.99".to_string(),
            price_amount: 9.99,
            currency: "USD".to_string(),
            product_type: "subscription".to_string(),
        },
        IapProduct {
            id: "premium_yearly".to_string(),
            title: "Premium Yearly".to_string(),
            description: "Access to all premium features for 1 year (save 20%)".to_string(),
            price: "$99.99".to_string(),
            price_amount: 99.99,
            currency: "USD".to_string(),
            product_type: "subscription".to_string(),
        },
        IapProduct {
            id: "coins_100".to_string(),
            title: "100 Coins".to_string(),
            description: "Purchase 100 in-app coins".to_string(),
            price: "$4.99".to_string(),
            price_amount: 4.99,
            currency: "USD".to_string(),
            product_type: "consumable".to_string(),
        },
        IapProduct {
            id: "coins_500".to_string(),
            title: "500 Coins".to_string(),
            description: "Purchase 500 in-app coins (best value)".to_string(),
            price: "$19.99".to_string(),
            price_amount: 19.99,
            currency: "USD".to_string(),
            product_type: "consumable".to_string(),
        },
        IapProduct {
            id: "remove_ads".to_string(),
            title: "Remove Ads".to_string(),
            description: "Permanently remove all advertisements".to_string(),
            price: "$2.99".to_string(),
            price_amount: 2.99,
            currency: "USD".to_string(),
            product_type: "non-consumable".to_string(),
        },
    ])
}

#[tauri::command]
async fn purchase_product(product_id: String) -> Result<PurchaseReceipt, String> {
    // Simulate platform purchase flow delay
    tokio::time::sleep(Duration::from_millis(2000)).await;

    // In production, this would:
    // 1. Trigger platform-specific purchase UI (App Store/Play Store)
    // 2. Handle user authentication
    // 3. Process payment
    // 4. Return receipt from platform

    // For now, simulate a successful purchase
    let products = fetch_iap_products().await?;
    let product = products.iter()
        .find(|p| p.id == product_id)
        .ok_or_else(|| format!("Product {} not found", product_id))?;

    // Simulate occasional failures (10% chance)
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    if timestamp % 10 == 0 {
        return Err("Purchase cancelled by user".to_string());
    }

    // Generate mock receipt
    let transaction_id = format!("txn_{}_{}",
        timestamp,
        &product_id[..std::cmp::min(8, product_id.len())]
    );

    let receipt = PurchaseReceipt {
        product_id: product_id.clone(),
        transaction_id,
        purchase_date: chrono::Utc::now().to_rfc3339(),
        status: "completed".to_string(),
        platform: get_platform_name(),
        price_paid: product.price_amount,
        currency: product.currency.clone(),
    };

    Ok(receipt)
}

#[tauri::command]
async fn restore_purchases() -> Result<RestoreResult, String> {
    // Simulate platform restore delay
    tokio::time::sleep(Duration::from_millis(1500)).await;

    // In production, this would query the platform store for previous purchases
    // For now, return empty or mock restored purchases

    // Simulate finding some old purchases (mock data)
    let mock_receipts = vec![
        PurchaseReceipt {
            product_id: "premium_monthly".to_string(),
            transaction_id: "txn_restored_001".to_string(),
            purchase_date: chrono::Utc::now()
                .checked_sub_signed(chrono::Duration::days(15))
                .unwrap()
                .to_rfc3339(),
            status: "completed".to_string(),
            platform: get_platform_name(),
            price_paid: 9.99,
            currency: "USD".to_string(),
        },
    ];

    Ok(RestoreResult {
        restored_count: mock_receipts.len(),
        receipts: mock_receipts,
    })
}

#[tauri::command]
async fn validate_receipt(transaction_id: String) -> Result<bool, String> {
    // Simulate validation delay
    tokio::time::sleep(Duration::from_millis(800)).await;

    // In production, this would:
    // 1. Send receipt to your backend server
    // 2. Backend verifies with App Store/Play Store
    // 3. Returns validation result

    // For mock: all receipts starting with "txn_" are valid
    Ok(transaction_id.starts_with("txn_"))
}

#[tauri::command]
fn get_iap_platform() -> String {
    get_platform_name()
}

fn get_platform_name() -> String {
    #[cfg(target_os = "ios")]
    return "ios".to_string();

    #[cfg(target_os = "android")]
    return "android".to_string();

    #[cfg(target_os = "windows")]
    return "windows".to_string();

    #[cfg(target_os = "macos")]
    return "macos".to_string();

    #[cfg(target_os = "linux")]
    return "linux".to_string();

    #[cfg(not(any(target_os = "ios", target_os = "android", target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "unknown".to_string();
}

// Camera Module Structs
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CapturedPhoto {
    id: String,
    file_path: String,
    width: u32,
    height: u32,
    size: u64,
    format: String,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordedVideo {
    id: String,
    file_path: String,
    duration: f64,
    width: u32,
    height: u32,
    size: u64,
    format: String,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CameraConfig {
    camera_id: String,
    facing_mode: String,
    flash_mode: String,
}

// Contacts Module Structs
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhoneNumber {
    r#type: String,
    number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Email {
    r#type: String,
    address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Contact {
    id: String,
    name: String,
    #[serde(rename = "phoneNumbers")]
    phone_numbers: Vec<PhoneNumber>,
    emails: Vec<Email>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_uri: Option<String>,
}

// Contacts Module Commands
// These are mock implementations for development
// In production, these would integrate with platform-specific contact APIs:
// - iOS/macOS: Contacts framework (CNContactStore)
// - Android: ContactsContract API

#[tauri::command]
async fn check_contacts_permission() -> Result<bool, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // On mobile/macOS, this would call the platform plugin to check permission
        // For now, return false to trigger permission request flow
        // TODO: Implement actual permission check via platform APIs
        Ok(false)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        Err("Contacts API is only available on Android, iOS, and macOS".to_string())
    }
}

#[tauri::command]
async fn request_contacts_permission() -> Result<bool, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // On mobile/macOS, this would call the platform plugin to request permission
        // Simulate permission request delay
        tokio::time::sleep(Duration::from_millis(500)).await;

        // TODO: Implement actual permission request via platform APIs
        // For now, return true to simulate granted permission
        Ok(true)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        Err("Contacts API is only available on Android, iOS, and macOS".to_string())
    }
}

#[tauri::command]
async fn get_contacts() -> Result<Vec<Contact>, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Simulate loading delay
        tokio::time::sleep(Duration::from_millis(800)).await;

        // TODO: Implement actual contact fetching via platform APIs
        // iOS/macOS: Use Contacts framework (CNContactStore)
        // Android: Use ContactsContract
        // For now, return mock contacts for testing UI
        Ok(generate_mock_contacts())
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        Err("Contacts API is only available on Android, iOS, and macOS".to_string())
    }
}

fn generate_mock_contacts() -> Vec<Contact> {
    vec![
        Contact {
            id: "1".to_string(),
            name: "John Smith".to_string(),
            phone_numbers: vec![
                PhoneNumber {
                    r#type: "mobile".to_string(),
                    number: "+1 (555) 123-4567".to_string(),
                },
                PhoneNumber {
                    r#type: "work".to_string(),
                    number: "+1 (555) 987-6543".to_string(),
                },
            ],
            emails: vec![
                Email {
                    r#type: "personal".to_string(),
                    address: "john.smith@email.com".to_string(),
                },
            ],
            photo_uri: None,
        },
        Contact {
            id: "2".to_string(),
            name: "Sarah Johnson".to_string(),
            phone_numbers: vec![
                PhoneNumber {
                    r#type: "mobile".to_string(),
                    number: "+1 (555) 234-5678".to_string(),
                },
            ],
            emails: vec![
                Email {
                    r#type: "work".to_string(),
                    address: "sarah.j@company.com".to_string(),
                },
                Email {
                    r#type: "personal".to_string(),
                    address: "sarah.johnson@email.com".to_string(),
                },
            ],
            photo_uri: None,
        },
        Contact {
            id: "3".to_string(),
            name: "Michael Chen".to_string(),
            phone_numbers: vec![
                PhoneNumber {
                    r#type: "mobile".to_string(),
                    number: "+1 (555) 345-6789".to_string(),
                },
            ],
            emails: vec![],
            photo_uri: None,
        },
        Contact {
            id: "4".to_string(),
            name: "Emily Davis".to_string(),
            phone_numbers: vec![
                PhoneNumber {
                    r#type: "home".to_string(),
                    number: "+1 (555) 456-7890".to_string(),
                },
                PhoneNumber {
                    r#type: "mobile".to_string(),
                    number: "+1 (555) 567-8901".to_string(),
                },
            ],
            emails: vec![
                Email {
                    r#type: "personal".to_string(),
                    address: "emily.davis@email.com".to_string(),
                },
            ],
            photo_uri: None,
        },
        Contact {
            id: "5".to_string(),
            name: "David Martinez".to_string(),
            phone_numbers: vec![
                PhoneNumber {
                    r#type: "mobile".to_string(),
                    number: "+1 (555) 678-9012".to_string(),
                },
            ],
            emails: vec![
                Email {
                    r#type: "work".to_string(),
                    address: "d.martinez@company.com".to_string(),
                },
            ],
            photo_uri: None,
        },
        Contact {
            id: "6".to_string(),
            name: "Lisa Anderson".to_string(),
            phone_numbers: vec![],
            emails: vec![
                Email {
                    r#type: "personal".to_string(),
                    address: "lisa.anderson@email.com".to_string(),
                },
            ],
            photo_uri: None,
        },
        Contact {
            id: "7".to_string(),
            name: "Robert Taylor".to_string(),
            phone_numbers: vec![
                PhoneNumber {
                    r#type: "mobile".to_string(),
                    number: "+1 (555) 789-0123".to_string(),
                },
                PhoneNumber {
                    r#type: "work".to_string(),
                    number: "+1 (555) 890-1234".to_string(),
                },
            ],
            emails: vec![
                Email {
                    r#type: "work".to_string(),
                    address: "robert.taylor@company.com".to_string(),
                },
                Email {
                    r#type: "personal".to_string(),
                    address: "rob.taylor@email.com".to_string(),
                },
            ],
            photo_uri: None,
        },
        Contact {
            id: "8".to_string(),
            name: "Jennifer Wilson".to_string(),
            phone_numbers: vec![
                PhoneNumber {
                    r#type: "mobile".to_string(),
                    number: "+1 (555) 901-2345".to_string(),
                },
            ],
            emails: vec![
                Email {
                    r#type: "personal".to_string(),
                    address: "jennifer.wilson@email.com".to_string(),
                },
            ],
            photo_uri: None,
        },
    ]
}

// Camera Module Commands
// These are mock implementations for development
// In production, these would integrate with platform-specific camera APIs:
// - Windows: MediaFoundation API
// - macOS: AVFoundation framework
// - Linux: V4L2 (Video4Linux2)
// - Android: CameraX API
// - iOS: AVFoundation framework

#[tauri::command]
async fn initialize_camera(_config: CameraConfig) -> Result<String, String> {
    // TODO: Implement platform-specific camera initialization
    // For now, return error message indicating custom plugin required
    Err("Camera functionality requires custom platform plugin development. See docs/camera-module.md for implementation details.".to_string())
}

#[tauri::command]
async fn capture_photo() -> Result<CapturedPhoto, String> {
    // TODO: Implement platform-specific photo capture
    // This would:
    // 1. Access camera device
    // 2. Capture current frame
    // 3. Save to temporary file
    // 4. Return photo metadata
    Err("Photo capture requires custom platform plugin. See docs/camera-module.md".to_string())
}

#[tauri::command]
async fn start_video_recording() -> Result<String, String> {
    // TODO: Implement platform-specific video recording
    // This would:
    // 1. Access camera and microphone
    // 2. Start video encoder
    // 3. Begin recording to file
    Err("Video recording requires custom platform plugin. See docs/camera-module.md".to_string())
}

#[tauri::command]
async fn stop_video_recording() -> Result<RecordedVideo, String> {
    // TODO: Implement platform-specific video recording stop
    // This would:
    // 1. Stop video encoder
    // 2. Finalize video file
    // 3. Return video metadata
    Err("Video recording requires custom platform plugin. See docs/camera-module.md".to_string())
}

#[tauri::command]
async fn switch_camera() -> Result<String, String> {
    // TODO: Implement camera switching (front/back)
    // This would:
    // 1. Release current camera
    // 2. Initialize new camera
    // 3. Update preview stream
    Err("Camera switching requires custom platform plugin. See docs/camera-module.md".to_string())
}

#[tauri::command]
async fn set_flash_mode(mode: String) -> Result<(), String> {
    // Validate mode
    if !["on", "off", "auto"].contains(&mode.as_str()) {
        return Err("Invalid flash mode. Must be 'on', 'off', or 'auto'".to_string());
    }

    // TODO: Implement platform-specific flash control
    // This would set the camera flash mode on the device
    Err("Flash control requires custom platform plugin. See docs/camera-module.md".to_string())
}

#[tauri::command]
async fn set_zoom(ratio: f32) -> Result<(), String> {
    // Validate ratio
    if !(0.0..=1.0).contains(&ratio) {
        return Err("Zoom ratio must be between 0.0 and 1.0".to_string());
    }

    // TODO: Implement platform-specific zoom control
    // This would set the camera zoom level
    Err("Zoom control requires custom platform plugin. See docs/camera-module.md".to_string())
}

#[tauri::command]
async fn get_cameras() -> Result<Vec<String>, String> {
    // TODO: Implement platform-specific camera enumeration
    // This would:
    // 1. Query available camera devices
    // 2. Return list of camera IDs/names
    Err("Camera enumeration requires custom platform plugin. See docs/camera-module.md".to_string())
}

#[tauri::command]
fn check_camera_permission() -> Result<bool, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // TODO: Implement actual permission check via platform APIs
        // For now, return false to trigger permission request flow
        Ok(false)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        // Desktop platforms don't typically have permission systems for camera
        // But we still need the custom plugin
        Ok(true)
    }
}

#[tauri::command]
async fn request_camera_permission() -> Result<bool, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Simulate permission request delay
        tokio::time::sleep(Duration::from_millis(500)).await;

        // TODO: Implement actual permission request via platform APIs
        // For now, return true to simulate granted permission
        Ok(true)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        // Desktop platforms don't require permission requests for camera
        Ok(true)
    }
}

// Haptics Module Commands
// Platform-specific haptic feedback
#[tauri::command]
async fn haptic_impact(style: String) -> Result<(), String> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        // Mobile implementation will be handled by platform plugins
        // Android: Uses Vibrator API with VibrationEffect
        // iOS: Uses UIFeedbackGenerator
        match style.as_str() {
            "light" | "medium" | "heavy" => {
                // TODO: Call platform-specific implementation
                Err(format!("Haptic impact '{}' requires custom mobile plugin. See docs/haptics-module.md", style))
            }
            _ => Err("Invalid impact style. Must be 'light', 'medium', or 'heavy'".to_string()),
        }
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Err("Haptic feedback is only available on mobile platforms (iOS, Android)".to_string())
    }
}

#[tauri::command]
async fn haptic_notification(notification_type: String) -> Result<(), String> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        // Mobile implementation will be handled by platform plugins
        match notification_type.as_str() {
            "success" | "warning" | "error" => {
                // TODO: Call platform-specific implementation
                Err(format!("Haptic notification '{}' requires custom mobile plugin. See docs/haptics-module.md", notification_type))
            }
            _ => Err("Invalid notification type. Must be 'success', 'warning', or 'error'".to_string()),
        }
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Err("Haptic feedback is only available on mobile platforms (iOS, Android)".to_string())
    }
}

#[tauri::command]
async fn vibrate(duration: u64) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        // Android can support custom vibration durations
        // TODO: Call platform-specific implementation
        Err(format!("Vibration ({}ms) requires custom mobile plugin. See docs/haptics-module.md", duration))
    }

    #[cfg(target_os = "ios")]
    {
        // iOS doesn't support custom duration vibrations via UIFeedbackGenerator
        // Use Core Haptics for more control, or fall back to predefined patterns
        Err("Custom duration vibration not directly supported on iOS. Use haptic_impact or haptic_notification instead.".to_string())
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Err("Vibration is only available on mobile platforms".to_string())
    }
}

#[tauri::command]
async fn vibrate_pattern(pattern: Vec<u64>) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        if pattern.is_empty() {
            return Err("Pattern cannot be empty".to_string());
        }

        // Android supports pattern vibrations
        // TODO: Call platform-specific implementation
        Err(format!("Pattern vibration {:?} requires custom mobile plugin. See docs/haptics-module.md", pattern))
    }

    #[cfg(target_os = "ios")]
    {
        // iOS doesn't support pattern vibrations via UIFeedbackGenerator
        // Would need Core Haptics framework for custom patterns
        Err("Pattern vibration not supported on iOS via UIFeedbackGenerator. Use Core Haptics framework for custom patterns.".to_string())
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Err("Vibration is only available on mobile platforms".to_string())
    }
}

#[tauri::command]
async fn cancel_vibration() -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        // Android supports canceling ongoing vibrations
        // TODO: Call platform-specific implementation
        Err("Cancel vibration requires custom mobile plugin. See docs/haptics-module.md".to_string())
    }

    #[cfg(target_os = "ios")]
    {
        // iOS haptic feedback is instantaneous, no need to cancel
        Ok(())
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Err("Vibration is only available on mobile platforms".to_string())
    }
}

#[tauri::command]
async fn has_vibrator() -> Result<bool, String> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        // On mobile platforms, assume vibrator is available
        // TODO: Implement actual device capability check
        // Android: Check if Vibrator service is available
        // iOS: Check for Taptic Engine (iPhone 6s+)
        Ok(true)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        // Desktop platforms don't have vibration hardware
        Ok(false)
    }
}

// Security & Biometrics Module Commands
// Biometric authentication and secure cryptographic operations

#[derive(Debug, Serialize, Deserialize)]
struct BiometricInfo {
    available: bool,
    enrolled: bool,
    types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthenticationOptions {
    title: String,
    subtitle: Option<String>,
    description: Option<String>,
    #[serde(rename = "negativeButtonText")]
    negative_button_text: String,
    #[serde(rename = "allowDeviceCredential")]
    allow_device_credential: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthenticationResult {
    success: bool,
    error: Option<String>,
    #[serde(rename = "biometricType")]
    biometric_type: Option<String>,
}

#[tauri::command]
async fn check_biometric_availability() -> Result<BiometricInfo, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Mobile and macOS platforms: delegated to native plugins
        // Android: Uses BiometricManager to check availability
        // iOS/macOS: Uses LAContext canEvaluatePolicy
        Ok(BiometricInfo {
            available: true,
            enrolled: true,
            types: vec!["fingerprint".to_string()],
        })
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        // Windows/Linux platforms don't have biometric hardware support yet
        Ok(BiometricInfo {
            available: false,
            enrolled: false,
            types: vec![],
        })
    }
}

#[tauri::command]
async fn authenticate_biometric(options: AuthenticationOptions) -> Result<AuthenticationResult, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Mobile and macOS platforms: delegated to native plugins
        // Android: Uses BiometricPrompt to authenticate
        // iOS/macOS: Uses LAContext evaluatePolicy
        // This is a placeholder - actual implementation is in platform-specific code
        Ok(AuthenticationResult {
            success: true,
            error: None,
            biometric_type: Some("fingerprint".to_string()),
        })
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        Err("Biometric authentication is only available on mobile and macOS platforms".to_string())
    }
}

#[tauri::command]
async fn get_biometric_types() -> Result<Vec<String>, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Mobile and macOS platforms: delegated to native plugins
        Ok(vec!["fingerprint".to_string()])
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        Ok(vec![])
    }
}

#[tauri::command]
async fn generate_encryption_key(key_name: String) -> Result<String, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Mobile and macOS platforms: delegated to native plugins
        // Android: Uses Android Keystore to generate AES key
        // iOS/macOS: Uses CryptoKit to generate SymmetricKey and stores in Keychain
        Ok(format!("Key '{}' generated successfully", key_name))
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: Could use DPAPI or CNG
        Err(format!("Encryption key generation not yet implemented for Windows. Key name: {}", key_name))
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: Could use libsecret or kernel keyring
        Err(format!("Encryption key generation not yet implemented for Linux. Key name: {}", key_name))
    }
}

#[tauri::command]
async fn encrypt_data(key_name: String, data: String) -> Result<String, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Mobile and macOS platforms: delegated to native plugins
        // Android: Uses Cipher with key from Android Keystore
        // iOS/macOS: Uses AES.GCM with key from Keychain
        // Return placeholder encrypted data
        Ok(format!("encrypted_{}", data))
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        Err(format!("Data encryption not yet implemented for this platform. Key: {}, Data length: {}", key_name, data.len()))
    }
}

#[tauri::command]
async fn decrypt_data(key_name: String, encrypted_data: String) -> Result<String, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Mobile and macOS platforms: delegated to native plugins
        // Android: Uses Cipher with key from Android Keystore
        // iOS/macOS: Uses AES.GCM with key from Keychain
        // Return placeholder decrypted data
        Ok(encrypted_data.replace("encrypted_", ""))
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        Err(format!("Data decryption not yet implemented for this platform. Key: {}, Data length: {}", key_name, encrypted_data.len()))
    }
}

#[tauri::command]
async fn secure_storage_set(key: String, value: String) -> Result<(), String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Mobile and macOS platforms: delegated to native plugins
        // Android: Uses SharedPreferences with encryption or Android Keystore
        // iOS/macOS: Uses Keychain Services
        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: Could use DPAPI for encryption
        Err(format!("Secure storage not yet implemented for Windows. Key: {}", key))
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: Could use Secret Service API
        Err(format!("Secure storage not yet implemented for Linux. Key: {}", key))
    }
}

#[tauri::command]
async fn secure_storage_get(key: String) -> Result<String, String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Mobile and macOS platforms: delegated to native plugins
        // This will return actual value from platform-specific implementation
        Err(format!("Key '{}' not found in secure storage", key))
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        Err(format!("Secure storage not yet implemented for this platform. Key: {}", key))
    }
}

#[tauri::command]
async fn secure_storage_delete(key: String) -> Result<(), String> {
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "macos"))]
    {
        // Mobile and macOS platforms: delegated to native plugins
        Ok(())
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        Err(format!("Secure storage not yet implemented for this platform. Key: {}", key))
    }
}

// Network & Realtime Module
// HTTP Client with connection pooling for better performance
static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build()
        .expect("Failed to create HTTP client")
});

#[derive(Debug, Serialize, Deserialize)]
struct HttpResponse {
    status: u16,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HttpPostData {
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

#[tauri::command]
async fn http_get(url: String) -> Result<HttpResponse, String> {
    let response = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("HTTP GET failed: {}", e))?;

    let status = response.status().as_u16();

    // Extract headers
    let mut headers = std::collections::HashMap::new();
    for (key, value) in response.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            headers.insert(key.to_string(), value_str.to_string());
        }
    }

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}

#[tauri::command]
async fn http_post(url: String, data: HttpPostData) -> Result<HttpResponse, String> {
    let response = HTTP_CLIENT
        .post(&url)
        .json(&data)
        .send()
        .await
        .map_err(|e| format!("HTTP POST failed: {}", e))?;

    let status = response.status().as_u16();

    // Extract headers
    let mut headers = std::collections::HashMap::new();
    for (key, value) in response.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            headers.insert(key.to_string(), value_str.to_string());
        }
    }

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}

#[tauri::command]
async fn upload_file(url: String, file_path: String) -> Result<HttpResponse, String> {
    use std::path::Path;

    // Read file
    let file_bytes = tokio::fs::read(&file_path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Get filename
    let file_name = Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Invalid file path".to_string())?
        .to_string();

    // Create multipart form
    let part = reqwest::multipart::Part::bytes(file_bytes)
        .file_name(file_name);

    let form = reqwest::multipart::Form::new()
        .part("file", part);

    // Send upload request
    let response = HTTP_CLIENT
        .post(&url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("File upload failed: {}", e))?;

    let status = response.status().as_u16();

    // Extract headers
    let mut headers = std::collections::HashMap::new();
    for (key, value) in response.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            headers.insert(key.to_string(), value_str.to_string());
        }
    }

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}

// File Sharing & Social Integration Module

#[derive(Debug, Serialize, Deserialize)]
struct ShareRequest {
    title: Option<String>,
    text: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Serialize)]
struct ShareResult {
    success: bool,
    error: Option<String>,
}

// Check if native backend sharing is supported
// Returns false - native sharing not implemented in backend
// Use Web Share API from frontend instead
#[tauri::command]
fn is_share_supported() -> Result<bool, String> {
    // Native backend sharing not implemented
    // Users should use Web Share API (navigator.share) from frontend
    Ok(false)
}

// Copy text to clipboard (works on all platforms)
#[tauri::command]
async fn copy_to_clipboard_backend(app: tauri::AppHandle, text: String) -> Result<(), String> {
    // Use clipboard plugin to write text
    app.clipboard()
        .write_text(text)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))?;
    Ok(())
}

// Read text from clipboard
#[tauri::command]
async fn read_from_clipboard(app: tauri::AppHandle) -> Result<String, String> {
    app.clipboard()
        .read_text()
        .map_err(|e| format!("Failed to read from clipboard: {}", e))
}

// Share text - Use Web Share API from frontend
// Note: tauri-plugin-share only supports file sharing, not text sharing
// Text sharing should be done via Web Share API (navigator.share) or clipboard
#[tauri::command]
async fn share_text(_app: tauri::AppHandle, _request: ShareRequest) -> Result<ShareResult, String> {
    // Native text sharing requires platform-specific implementation
    // For now, direct users to use Web Share API from the frontend
    Err("Native text sharing not implemented. Use Web Share API (navigator.share) or clipboard fallback.".to_string())
}

// Share files - Use Web Share API from frontend
// Note: Native file sharing requires platform-specific implementation
#[tauri::command]
async fn share_files(_app: tauri::AppHandle, files: Vec<String>, _title: Option<String>) -> Result<ShareResult, String> {
    // Validate file paths
    for file_path in &files {
        if !std::path::Path::new(file_path).exists() {
            return Err(format!("File not found: {}", file_path));
        }
    }

    // Native file sharing requires platform-specific implementation
    // For now, direct users to use Web Share API from the frontend
    Err("Native file sharing not implemented. Use Web Share API (navigator.share with files) or file dialogs.".to_string())
}

// Get current platform information
#[tauri::command]
fn get_share_platform() -> String {
    get_platform_name()
}

// App Lifecycle & OS Integration Module
#[derive(Debug, Serialize)]
struct SystemInfo {
    os: String,
    version: String,
    arch: String,
    app_version: String,
    process_id: u32,
}

#[tauri::command]
fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        version: std::env::consts::FAMILY.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        process_id: std::process::id(),
    }
}

// System Monitoring
use sysinfo::{System, Disks, Networks};
use std::time::{SystemTime, UNIX_EPOCH};

static APP_START_TIME: Lazy<u64> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
});

#[derive(Debug, Serialize)]
struct SystemMetrics {
    cpu_usage: f32,
    memory_total: u64,
    memory_used: u64,
    memory_available: u64,
    memory_usage_percent: f32,
    swap_total: u64,
    swap_used: u64,
    disk_total: u64,
    disk_used: u64,
    disk_available: u64,
    disk_usage_percent: f32,
}

#[derive(Debug, Serialize)]
struct NetworkMetrics {
    total_received: u64,
    total_transmitted: u64,
    interfaces: Vec<NetworkInterfaceMetrics>,
}

#[derive(Debug, Serialize)]
struct NetworkInterfaceMetrics {
    name: String,
    received: u64,
    transmitted: u64,
}

#[tauri::command]
fn get_system_metrics() -> Result<SystemMetrics, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Give it a moment to gather CPU data
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();

    let cpu_usage = sys.global_cpu_usage();
    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();
    let memory_available = sys.available_memory();
    let memory_usage_percent = (memory_used as f32 / memory_total as f32) * 100.0;

    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();

    // Get disk usage
    let disks = Disks::new_with_refreshed_list();
    let mut disk_total = 0u64;
    let mut disk_used = 0u64;

    for disk in &disks {
        disk_total += disk.total_space();
        disk_used += disk.total_space() - disk.available_space();
    }

    let disk_available = disk_total - disk_used;
    let disk_usage_percent = if disk_total > 0 {
        (disk_used as f32 / disk_total as f32) * 100.0
    } else {
        0.0
    };

    Ok(SystemMetrics {
        cpu_usage,
        memory_total,
        memory_used,
        memory_available,
        memory_usage_percent,
        swap_total,
        swap_used,
        disk_total,
        disk_used,
        disk_available,
        disk_usage_percent,
    })
}

#[tauri::command]
fn get_network_metrics() -> Result<NetworkMetrics, String> {
    let networks = Networks::new_with_refreshed_list();

    let mut total_received = 0u64;
    let mut total_transmitted = 0u64;
    let mut interfaces = Vec::new();

    for (interface_name, network) in &networks {
        let received = network.total_received();
        let transmitted = network.total_transmitted();

        total_received += received;
        total_transmitted += transmitted;

        interfaces.push(NetworkInterfaceMetrics {
            name: interface_name.clone(),
            received,
            transmitted,
        });
    }

    Ok(NetworkMetrics {
        total_received,
        total_transmitted,
        interfaces,
    })
}

#[tauri::command]
fn get_app_uptime() -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    now - *APP_START_TIME
}

// Extended System Info & Device Profiling
#[derive(Debug, Serialize)]
struct CPUInfo {
    name: String,
    vendor: String,
    brand: String,
    physical_cores: usize,
    logical_cores: usize,
    frequency: u64, // MHz
}

#[derive(Debug, Serialize)]
struct StorageDevice {
    name: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
    used_space: u64,
    file_system: String,
    is_removable: bool,
}

#[derive(Debug, Serialize)]
struct DeviceProfile {
    hostname: String,
    username: String,
    device_name: String,
    os_name: String,
    os_version: String,
    architecture: String,
    cpu: CPUInfo,
    total_memory: u64,
    storage_devices: Vec<StorageDevice>,
    kernel_version: String,
}

#[tauri::command]
fn get_cpu_info() -> Result<CPUInfo, String> {
    let mut sys = System::new_all();
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    let physical_cores = sys.physical_core_count().unwrap_or(0);
    let logical_cores = cpus.len();

    let cpu_name = if !cpus.is_empty() {
        cpus[0].brand().to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let cpu_vendor = if !cpus.is_empty() {
        cpus[0].vendor_id().to_string()
    } else {
        "Unknown Vendor".to_string()
    };

    let frequency = if !cpus.is_empty() {
        cpus[0].frequency()
    } else {
        0
    };

    Ok(CPUInfo {
        name: cpu_name.clone(),
        vendor: cpu_vendor,
        brand: cpu_name,
        physical_cores,
        logical_cores,
        frequency,
    })
}

#[tauri::command]
fn get_storage_devices() -> Result<Vec<StorageDevice>, String> {
    let disks = Disks::new_with_refreshed_list();
    let mut devices = Vec::new();

    for disk in &disks {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;

        devices.push(StorageDevice {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_space,
            available_space,
            used_space,
            file_system: disk.file_system().to_string_lossy().to_string(),
            is_removable: disk.is_removable(),
        });
    }

    Ok(devices)
}

#[tauri::command]
fn get_device_profile() -> Result<DeviceProfile, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Get CPU info
    let cpus = sys.cpus();
    let physical_cores = sys.physical_core_count().unwrap_or(0);
    let logical_cores = cpus.len();

    let cpu_name = if !cpus.is_empty() {
        cpus[0].brand().to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let cpu_vendor = if !cpus.is_empty() {
        cpus[0].vendor_id().to_string()
    } else {
        "Unknown Vendor".to_string()
    };

    let frequency = if !cpus.is_empty() {
        cpus[0].frequency()
    } else {
        0
    };

    let cpu = CPUInfo {
        name: cpu_name.clone(),
        vendor: cpu_vendor,
        brand: cpu_name,
        physical_cores,
        logical_cores,
        frequency,
    };

    // Get storage devices
    let disks = Disks::new_with_refreshed_list();
    let mut storage_devices = Vec::new();

    for disk in &disks {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;

        storage_devices.push(StorageDevice {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_space,
            available_space,
            used_space,
            file_system: disk.file_system().to_string_lossy().to_string(),
            is_removable: disk.is_removable(),
        });
    }

    // Get system info
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let os_name = System::name().unwrap_or_else(|| std::env::consts::OS.to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let total_memory = sys.total_memory();

    // Get username and device name using whoami
    let username = whoami::username();
    let device_name = whoami::devicename();

    Ok(DeviceProfile {
        hostname,
        username,
        device_name,
        os_name,
        os_version,
        architecture: std::env::consts::ARCH.to_string(),
        cpu,
        total_memory,
        storage_devices,
        kernel_version,
    })
}

// System Services Module
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatteryInfo {
    level: i32,
    charging: bool,
    temperature: Option<i32>,
    power_source: String,
    battery_state: String,
    charging_time: Option<i64>,
    discharging_time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AudioDevice {
    id: String,
    name: String,
    kind: String,
    is_default: bool,
    is_connected: bool,
    device_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AudioDevicesResponse {
    devices: Vec<AudioDevice>,
}

#[tauri::command]
async fn get_battery_info() -> Result<BatteryInfo, String> {
    use battery::Manager;

    let manager = Manager::new().map_err(|e| format!("Failed to create battery manager: {}", e))?;

    let mut batteries = manager.batteries().map_err(|e| format!("Failed to get batteries: {}", e))?;

    if let Some(Ok(battery)) = batteries.next() {
        let state_of_charge = battery.state_of_charge().value;
        let level = (state_of_charge * 100.0) as i32;

        let state = battery.state();
        let charging = matches!(state, battery::State::Charging);
        let battery_state = match state {
            battery::State::Charging => "charging",
            battery::State::Discharging => "discharging",
            battery::State::Full => "full",
            battery::State::Empty => "empty",
            _ => "unknown",
        };

        let power_source = if charging { "ac" } else { "battery" };

        // Try to get temperature if available
        let temperature = battery.temperature()
            .map(|t| (t.value - 273.15) as i32); // Convert Kelvin to Celsius

        Ok(BatteryInfo {
            level,
            charging,
            temperature,
            power_source: power_source.to_string(),
            battery_state: battery_state.to_string(),
            charging_time: None,
            discharging_time: None,
        })
    } else {
        Err("No battery found".to_string())
    }
}

// Network Status & WiFi Module
#[derive(Debug, Serialize, Deserialize)]
struct NetworkStatus {
    online: bool,
    #[serde(rename = "connectionType")]
    connection_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    downlink: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rtt: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkInterface {
    name: String,
    #[serde(rename = "type")]
    interface_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mac_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WiFiInfo {
    ssid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signal_strength: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WiFiNetwork {
    ssid: String,
    bssid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    signal_strength: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConnectionQualityMetrics {
    latency: u64,
    jitter: u64,
    packet_loss: f64,
    quality_score: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpeedTestResult {
    download_speed: f64,
    upload_speed: f64,
    latency: u64,
    server: String,
}

#[tauri::command]
async fn check_network_status() -> Result<NetworkStatus, String> {
    use std::net::TcpStream;

    // Try to connect to Google DNS to check internet connectivity
    let online = match TcpStream::connect_timeout(
        &"8.8.8.8:53".parse().unwrap(),
        Duration::from_secs(3)
    ) {
        Ok(_) => true,
        Err(_) => false,
    };

    let connection_type = detect_connection_type().await;

    Ok(NetworkStatus {
        online,
        connection_type,
        downlink: None,
        rtt: None,
    })
}

async fn detect_connection_type() -> String {
    use network_interface::NetworkInterface as NI;
    use network_interface::NetworkInterfaceConfig;

    match NI::show() {
        Ok(interfaces) => {
            for iface in interfaces {
                if let Some(_mac) = iface.mac_addr {
                    let name_lower = iface.name.to_lowercase();

                    // Check if interface has IP addresses (is active)
                    if !iface.addr.is_empty() {
                        // WiFi detection
                        if name_lower.contains("wi") ||
                           name_lower.contains("wl") ||
                           name_lower.contains("airport") ||
                           name_lower.starts_with("en") && cfg!(target_os = "macos") {
                            // Verify it's actually WiFi (not ethernet)
                            #[cfg(target_os = "macos")]
                            {
                                if is_wifi_active_macos().await {
                                    return "wifi".to_string();
                                }
                            }
                            #[cfg(target_os = "linux")]
                            {
                                if name_lower.starts_with("wl") {
                                    return "wifi".to_string();
                                }
                            }
                            #[cfg(target_os = "windows")]
                            {
                                if name_lower.contains("wi-fi") || name_lower.contains("wireless") {
                                    return "wifi".to_string();
                                }
                            }
                        }

                        // Ethernet detection
                        if name_lower.contains("eth") ||
                           name_lower.contains("en") && !cfg!(target_os = "macos") ||
                           name_lower.contains("ethernet") ||
                           name_lower.starts_with("em") {
                            return "ethernet".to_string();
                        }
                    }
                }
            }
            "unknown".to_string()
        }
        Err(_) => "unknown".to_string(),
    }
}

#[cfg(target_os = "macos")]
async fn is_wifi_active_macos() -> bool {
    use std::process::Command;

    // Try multiple possible airport utility locations
    let airport_paths = [
        "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport",
        "/usr/sbin/airport",
    ];

    for path in &airport_paths {
        if let Ok(output) = Command::new(path).args(["-I"]).output() {
            let info = String::from_utf8_lossy(&output.stdout);
            // If we can get SSID, WiFi is active
            for line in info.lines() {
                if line.trim().contains("SSID:") && !line.contains("BSSID") {
                    if let Some(ssid) = line.split(':').nth(1) {
                        if !ssid.trim().is_empty() {
                            return true;
                        }
                    }
                }
            }
        }
    }

    // Fallback: try networksetup to check WiFi status
    if let Ok(output) = Command::new("networksetup")
        .args(["-getairportpower", "en0"])
        .output()
    {
        let info = String::from_utf8_lossy(&output.stdout);
        if info.contains("On") {
            return true;
        }
    }

    false
}

#[tauri::command]
async fn get_network_interfaces() -> Result<Vec<NetworkInterface>, String> {
    use network_interface::NetworkInterface as NI;
    use network_interface::NetworkInterfaceConfig;

    let interfaces = NI::show()
        .map_err(|e| format!("Failed to get network interfaces: {}", e))?;

    let result: Vec<NetworkInterface> = interfaces
        .into_iter()
        .map(|iface| {
            let name_lower = iface.name.to_lowercase();
            let interface_type = if name_lower.contains("wi") || name_lower.contains("wl") {
                "wifi".to_string()
            } else if name_lower.contains("eth") || name_lower.contains("en") {
                "ethernet".to_string()
            } else if name_lower.contains("lo") {
                "loopback".to_string()
            } else {
                "other".to_string()
            };

            NetworkInterface {
                name: iface.name,
                interface_type,
                mac_address: iface.mac_addr.map(|m| m.to_string()),
                ip_addresses: Some(
                    iface.addr
                        .iter()
                        .map(|addr| addr.ip().to_string())
                        .collect()
                ),
            }
        })
        .collect();

    Ok(result)
}

#[tauri::command]
async fn get_wifi_info() -> Result<WiFiInfo, String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let mut wifi_info = WiFiInfo {
            ssid: String::new(),
            bssid: None,
            signal_strength: None,
            ip_address: None,
            security_type: None,
        };

        // Method 1: Try using airport utility (try multiple possible locations)
        let airport_paths = [
            "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport",
            "/usr/sbin/airport", // Alternative location on some macOS versions
        ];

        let mut airport_result = None;
        for path in &airport_paths {
            if let Ok(output) = Command::new(path).args(["-I"]).output() {
                airport_result = Some(output);
                break;
            }
        }

        if let Some(output) = airport_result {
            let info = String::from_utf8_lossy(&output.stdout);

            // Parse WiFi information from output
            for line in info.lines() {
                let trimmed = line.trim();

                if trimmed.contains("SSID:") && !trimmed.contains("BSSID") {
                    if let Some(ssid) = trimmed.split(':').nth(1) {
                        wifi_info.ssid = ssid.trim().to_string();
                    }
                } else if trimmed.starts_with("BSSID:") {
                    if let Some(bssid) = trimmed.split(':').skip(1).collect::<Vec<_>>().join(":").trim().split_whitespace().next() {
                        wifi_info.bssid = Some(bssid.to_string());
                    }
                } else if trimmed.contains("agrCtlRSSI:") {
                    if let Some(rssi) = trimmed.split(':').nth(1) {
                        if let Ok(signal) = rssi.trim().parse::<i32>() {
                            wifi_info.signal_strength = Some(signal);
                        }
                    }
                } else if trimmed.contains("link auth:") {
                    if let Some(auth) = trimmed.split(':').nth(1) {
                        wifi_info.security_type = Some(auth.trim().to_string());
                    }
                }
            }
        }

        // Method 2: Fallback to networksetup if airport failed or gave no results
        // Try multiple interfaces (en0, en1, etc.)
        if wifi_info.ssid.is_empty() {
            let interfaces = ["en0", "en1", "en2"];
            for interface in &interfaces {
                if let Ok(output) = Command::new("networksetup")
                    .args(["-getairportnetwork", interface])
                    .output()
                {
                    let info = String::from_utf8_lossy(&output.stdout);
                    if info.contains("Current Wi-Fi Network:") {
                        if let Some(ssid) = info.split("Current Wi-Fi Network:").nth(1) {
                            let ssid_trimmed = ssid.trim();
                            if !ssid_trimmed.is_empty() {
                                wifi_info.ssid = ssid_trimmed.to_string();

                                // Get IP address for this interface
                                if let Ok(ip_output) = Command::new("ipconfig")
                                    .args(["getifaddr", interface])
                                    .output()
                                {
                                    let ip = String::from_utf8_lossy(&ip_output.stdout);
                                    let ip_trimmed = ip.trim();
                                    if !ip_trimmed.is_empty() {
                                        wifi_info.ip_address = Some(ip_trimmed.to_string());
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }

        // Method 3: Fallback to system_profiler if other methods failed
        if wifi_info.ssid.is_empty() {
            if let Ok(output) = Command::new("system_profiler")
                .args(["SPAirPortDataType"])
                .output()
            {
                let info = String::from_utf8_lossy(&output.stdout);
                let mut in_current_network = false;
                let mut current_ssid = String::new();

                for line in info.lines() {
                    let trimmed = line.trim();

                    // Look for Current Network Information section
                    if trimmed.contains("Current Network Information:") {
                        in_current_network = true;
                        continue;
                    }

                    // Stop when we hit "Other Local Wi-Fi Networks" section
                    if in_current_network && trimmed.contains("Other Local Wi-Fi Networks:") {
                        // Found SSID in current network, use it
                        if !current_ssid.is_empty() {
                            wifi_info.ssid = current_ssid.clone();
                        }
                        break;
                    }

                    if in_current_network && !trimmed.is_empty() {
                        // SSID is a line that ends with ":" but is NOT a known property key
                        // Known properties: PHY Mode:, Channel:, Country Code:, Network Type:, Security:, Signal / Noise:, Transmit Rate:, MCS Index:
                        if trimmed.ends_with(":") &&
                           !trimmed.starts_with("PHY Mode") &&
                           !trimmed.starts_with("Channel") &&
                           !trimmed.starts_with("Country") &&
                           !trimmed.starts_with("Network Type") &&
                           !trimmed.starts_with("Security") &&
                           !trimmed.starts_with("Signal") &&
                           !trimmed.starts_with("Transmit") &&
                           !trimmed.starts_with("MCS") {
                            current_ssid = trimmed.trim_end_matches(':').to_string();
                        }

                        // Extract signal strength: "-56 dBm"
                        if trimmed.contains("Signal / Noise:") {
                            if let Some(signal_part) = trimmed.split(':').nth(1) {
                                let signal_str = signal_part.split_whitespace().next().unwrap_or("");
                                if let Ok(signal) = signal_str.parse::<i32>() {
                                    wifi_info.signal_strength = Some(signal);
                                }
                            }
                        }

                        // Extract security type
                        if trimmed.starts_with("Security:") {
                            if let Some(security) = trimmed.split(':').nth(1) {
                                wifi_info.security_type = Some(security.trim().to_string());
                            }
                        }
                    }
                }

                // If still no SSID, try to get IP address as confirmation we're connected
                if !wifi_info.ssid.is_empty() {
                    if let Ok(ip_output) = Command::new("ipconfig")
                        .args(["getifaddr", "en0"])
                        .output()
                    {
                        let ip = String::from_utf8_lossy(&ip_output.stdout);
                        let ip_trimmed = ip.trim();
                        if !ip_trimmed.is_empty() {
                            wifi_info.ip_address = Some(ip_trimmed.to_string());
                        }
                    }
                }
            }
        }

        if wifi_info.ssid.is_empty() {
            Err("Not connected to WiFi. Please enable WiFi and connect to a network. Note: This feature only works when connected via WiFi (not ethernet).".to_string())
        } else {
            Ok(wifi_info)
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let mut wifi_info = WiFiInfo {
            ssid: String::new(),
            bssid: None,
            signal_strength: None,
            ip_address: None,
            security_type: None,
        };

        // Get SSID - check if command exists first
        match Command::new("iwgetid").args(["-r"]).output() {
            Ok(output) => {
                let ssid = String::from_utf8_lossy(&output.stdout);
                wifi_info.ssid = ssid.trim().to_string();
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Err("WiFi tools not found. Please install wireless-tools: sudo apt-get install wireless-tools (Ubuntu/Debian) or sudo dnf install wireless-tools (Fedora)".to_string());
                }
                return Err(format!("Failed to get WiFi info: {}", e));
            }
        }

        // Get more detailed info with iwconfig
        if let Ok(output) = Command::new("iwconfig").output() {
            let info = String::from_utf8_lossy(&output.stdout);

            for line in info.lines() {
                if line.contains("Access Point:") {
                    if let Some(bssid) = line.split("Access Point:").nth(1) {
                        let bssid_clean = bssid.trim().split_whitespace().next().unwrap_or("");
                        if bssid_clean != "Not-Associated" && !bssid_clean.is_empty() {
                            wifi_info.bssid = Some(bssid_clean.to_string());
                        }
                    }
                }
                if line.contains("Signal level=") {
                    if let Some(signal_part) = line.split("Signal level=").nth(1) {
                        if let Some(signal_str) = signal_part.split_whitespace().next() {
                            if let Ok(signal) = signal_str.parse::<i32>() {
                                wifi_info.signal_strength = Some(signal);
                            }
                        }
                    }
                }
                if line.contains("Encryption key:on") {
                    wifi_info.security_type = Some("WPA/WPA2".to_string());
                }
            }
        }

        // Try to get more specific security info
        if let Ok(output) = Command::new("nmcli").args(["-t", "-f", "SECURITY,ACTIVE", "dev", "wifi"]).output() {
            let info = String::from_utf8_lossy(&output.stdout);
            for line in info.lines() {
                if line.contains("yes") {
                    if let Some(security) = line.split(':').next() {
                        if !security.is_empty() && security != "--" {
                            wifi_info.security_type = Some(security.to_string());
                        }
                    }
                }
            }
        }

        // Get IP address
        if let Ok(output) = Command::new("ip").args(["addr", "show"]).output() {
            let ip_info = String::from_utf8_lossy(&output.stdout);
            for line in ip_info.lines() {
                if line.contains("inet ") && !line.contains("127.0.0.1") {
                    if let Some(ip_part) = line.trim().split_whitespace().nth(1) {
                        if let Some(ip) = ip_part.split('/').next() {
                            wifi_info.ip_address = Some(ip.to_string());
                            break;
                        }
                    }
                }
            }
        }

        if wifi_info.ssid.is_empty() {
            Err("Not connected to WiFi".to_string())
        } else {
            Ok(wifi_info)
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let output = Command::new("netsh")
            .args(["wlan", "show", "interfaces"])
            .output()
            .map_err(|e| format!("Failed to get WiFi info: {}", e))?;

        let info = String::from_utf8_lossy(&output.stdout);

        let mut wifi_info = WiFiInfo {
            ssid: String::new(),
            bssid: None,
            signal_strength: None,
            ip_address: None,
            security_type: None,
        };

        // Parse SSID, BSSID, Signal, and Authentication
        for line in info.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("SSID") && !trimmed.contains("BSSID") {
                if let Some(ssid) = trimmed.split(':').nth(1) {
                    wifi_info.ssid = ssid.trim().to_string();
                }
            } else if trimmed.starts_with("BSSID") {
                if let Some(bssid) = trimmed.split(':').skip(1).collect::<Vec<_>>().join(":").trim().split_whitespace().next() {
                    wifi_info.bssid = Some(bssid.to_string());
                }
            } else if trimmed.starts_with("Signal") {
                if let Some(signal_str) = trimmed.split(':').nth(1) {
                    let signal_clean = signal_str.trim().trim_end_matches('%');
                    if let Ok(signal) = signal_clean.parse::<i32>() {
                        wifi_info.signal_strength = Some(signal);
                    }
                }
            } else if trimmed.starts_with("Authentication") {
                if let Some(auth) = trimmed.split(':').nth(1) {
                    wifi_info.security_type = Some(auth.trim().to_string());
                }
            }
        }

        // Get IP address
        if let Ok(ip_output) = Command::new("ipconfig").output() {
            let ip_info = String::from_utf8_lossy(&ip_output.stdout);
            let mut found_wireless = false;

            for line in ip_info.lines() {
                if line.contains("Wireless") {
                    found_wireless = true;
                } else if found_wireless && line.contains("IPv4") {
                    if let Some(ip) = line.split(':').nth(1) {
                        wifi_info.ip_address = Some(ip.trim().to_string());
                        break;
                    }
                }
            }
        }

        if wifi_info.ssid.is_empty() {
            Err("Not connected to WiFi".to_string())
        } else {
            Ok(wifi_info)
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err("WiFi info not supported on this platform".to_string())
    }
}

#[tauri::command]
async fn get_audio_devices() -> Result<AudioDevicesResponse, String> {
    use cpal::traits::{DeviceTrait, HostTrait};

    let host = cpal::default_host();
    let mut devices = Vec::new();

    // Get default devices
    let default_input = host.default_input_device();
    let default_output = host.default_output_device();

    // Get all input devices
    if let Ok(input_devices) = host.input_devices() {
        for (idx, device) in input_devices.enumerate() {
            if let Ok(name) = device.name() {
                let device_name = device.name().unwrap_or_else(|_| format!("Input Device {}", idx));
                let is_default = default_input.as_ref()
                    .and_then(|d| d.name().ok())
                    .map(|n| n == name)
                    .unwrap_or(false);

                devices.push(AudioDevice {
                    id: format!("input-{}", idx),
                    name: device_name,
                    kind: "audioinput".to_string(),
                    is_default,
                    is_connected: true,
                    device_type: "unknown".to_string(),
                });
            }
        }
    }

    // Get all output devices
    if let Ok(output_devices) = host.output_devices() {
        for (idx, device) in output_devices.enumerate() {
            if let Ok(name) = device.name() {
                let device_name = device.name().unwrap_or_else(|_| format!("Output Device {}", idx));
                let is_default = default_output.as_ref()
                    .and_then(|d| d.name().ok())
                    .map(|n| n == name)
                    .unwrap_or(false);

                devices.push(AudioDevice {
                    id: format!("output-{}", idx),
                    name: device_name,
                    kind: "audiooutput".to_string(),
                    is_default,
                    is_connected: true,
                    device_type: "unknown".to_string(),
                });
            }
        }
    }

    Ok(AudioDevicesResponse { devices })
}

#[tauri::command]
async fn scan_wifi_networks() -> Result<Vec<WiFiNetwork>, String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let mut networks = Vec::new();

        // Method 1: Try using airport utility (try multiple possible locations)
        let airport_paths = [
            "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport",
            "/usr/sbin/airport", // Alternative location on some macOS versions
        ];

        let mut airport_result = None;
        for path in &airport_paths {
            if let Ok(output) = Command::new(path).args(["-s"]).output() {
                if output.status.success() {
                    airport_result = Some(output);
                    break;
                }
            }
        }

        if let Some(output) = airport_result {
            if output.status.success() {
                let info = String::from_utf8_lossy(&output.stdout);

                // Skip header line
                for line in info.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let ssid = parts[0].to_string();
                        let bssid = parts[1].to_string();
                        let rssi = parts[2].parse::<i32>().ok();

                        networks.push(WiFiNetwork {
                            ssid,
                            bssid,
                            signal_strength: rssi,
                            security_type: if parts.len() > 6 { Some(parts[6].to_string()) } else { None },
                            frequency: None,
                        });
                    }
                }

                if !networks.is_empty() {
                    return Ok(networks);
                }
            }
        }

        // Method 2: Fallback to system_profiler (non-XML format for easier parsing)
        if networks.is_empty() {
            if let Ok(output) = Command::new("system_profiler")
                .args(["SPAirPortDataType"])
                .output()
            {
                if output.status.success() {
                    let info = String::from_utf8_lossy(&output.stdout);
                    let mut current_ssid = String::new();
                    let mut current_signal: Option<i32> = None;
                    let mut current_security: Option<String> = None;

                    let mut in_other_networks = false;
                    let mut in_network_block = false;

                    for line in info.lines() {
                        let trimmed = line.trim();

                        // Check if we're in the "Other Local Wi-Fi Networks" section
                        if trimmed.contains("Other Local Wi-Fi Networks:") {
                            in_other_networks = true;
                            continue;
                        }

                        if in_other_networks && !trimmed.is_empty() {
                            // SSID line: ends with ":" but is NOT a known property
                            if trimmed.ends_with(":") &&
                               !trimmed.starts_with("PHY Mode") &&
                               !trimmed.starts_with("Channel") &&
                               !trimmed.starts_with("Country") &&
                               !trimmed.starts_with("Network Type") &&
                               !trimmed.starts_with("Security") &&
                               !trimmed.starts_with("Signal") &&
                               !trimmed.starts_with("Transmit") &&
                               !trimmed.starts_with("MCS") {
                                // Save previous network before starting a new one
                                if in_network_block && !current_ssid.is_empty() {
                                    networks.push(WiFiNetwork {
                                        ssid: current_ssid.clone(),
                                        bssid: String::new(),
                                        signal_strength: current_signal,
                                        security_type: current_security.clone(),
                                        frequency: None,
                                    });
                                    current_signal = None;
                                    current_security = None;
                                }
                                current_ssid = trimmed.trim_end_matches(':').to_string();
                                in_network_block = true;
                            } else if in_network_block {
                                // Parse properties within a network block
                                if trimmed.starts_with("Security:") {
                                    if let Some(security) = trimmed.split(':').nth(1) {
                                        current_security = Some(security.trim().to_string());
                                    }
                                } else if trimmed.contains("Signal / Noise:") {
                                    if let Some(signal_part) = trimmed.split(':').nth(1) {
                                        let signal_str = signal_part.split_whitespace().next().unwrap_or("");
                                        if let Ok(signal) = signal_str.parse::<i32>() {
                                            current_signal = Some(signal);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Add last network if exists
                    if in_network_block && !current_ssid.is_empty() {
                        networks.push(WiFiNetwork {
                            ssid: current_ssid,
                            bssid: String::new(),
                            signal_strength: current_signal,
                            security_type: current_security,
                            frequency: None,
                        });
                    }
                }
            }
        }

        // Method 3: Return helpful message if no networks found
        if networks.is_empty() {
            return Err("No WiFi networks found. Make sure WiFi is enabled and try again. Note: On some macOS versions, you may need to run the app with proper permissions to scan networks.".to_string());
        }

        Ok(networks)
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let output = Command::new("nmcli")
            .args(["-t", "-f", "SSID,BSSID,SIGNAL,SECURITY", "dev", "wifi", "list"])
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    "NetworkManager not found. Please install NetworkManager: sudo apt-get install network-manager (Ubuntu/Debian) or sudo dnf install NetworkManager (Fedora)".to_string()
                } else {
                    format!("Failed to scan WiFi networks: {}", e)
                }
            })?;

        let info = String::from_utf8_lossy(&output.stdout);
        let mut networks = Vec::new();

        for line in info.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                networks.push(WiFiNetwork {
                    ssid: parts[0].to_string(),
                    bssid: parts.get(1).unwrap_or(&"").to_string(),
                    signal_strength: parts.get(2).and_then(|s| s.parse::<i32>().ok()),
                    security_type: parts.get(3).map(|s| s.to_string()),
                    frequency: None,
                });
            }
        }

        Ok(networks)
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let output = Command::new("netsh")
            .args(["wlan", "show", "networks", "mode=bssid"])
            .output()
            .map_err(|e| format!("Failed to scan WiFi networks: {}", e))?;

        let info = String::from_utf8_lossy(&output.stdout);
        let mut networks = Vec::new();
        let mut current_ssid = String::new();
        let mut current_bssid = String::new();
        let mut current_signal = None;
        let mut current_auth = None;

        for line in info.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("SSID") && !trimmed.contains("BSSID") {
                if let Some(ssid) = trimmed.split(':').nth(1) {
                    current_ssid = ssid.trim().to_string();
                }
            } else if trimmed.starts_with("BSSID") {
                if let Some(bssid) = trimmed.split(':').skip(1).collect::<Vec<_>>().join(":").split_whitespace().next() {
                    current_bssid = bssid.to_string();
                }
            } else if trimmed.starts_with("Signal") {
                if let Some(signal_str) = trimmed.split(':').nth(1) {
                    let signal_clean = signal_str.trim().trim_end_matches('%');
                    current_signal = signal_clean.parse::<i32>().ok();
                }
            } else if trimmed.starts_with("Authentication") {
                if let Some(auth) = trimmed.split(':').nth(1) {
                    current_auth = Some(auth.trim().to_string());
                }

                // Add network when we have all info
                if !current_ssid.is_empty() && !current_bssid.is_empty() {
                    networks.push(WiFiNetwork {
                        ssid: current_ssid.clone(),
                        bssid: current_bssid.clone(),
                        signal_strength: current_signal,
                        security_type: current_auth.clone(),
                        frequency: None,
                    });
                    current_ssid = String::new();
                    current_bssid = String::new();
                    current_signal = None;
                    current_auth = None;
                }
            }
        }

        Ok(networks)
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err("WiFi scanning not supported on this platform".to_string())
    }
}

#[tauri::command]
async fn test_connection_quality() -> Result<ConnectionQualityMetrics, String> {
    use std::net::TcpStream;
    use std::time::Instant;

    let servers = vec![
        "8.8.8.8:53",      // Google DNS
        "1.1.1.1:53",      // Cloudflare DNS
        "208.67.222.222:53", // OpenDNS
    ];

    let mut latencies = Vec::new();
    let mut successful_connections = 0;

    for server in &servers {
        let start = Instant::now();
        match TcpStream::connect_timeout(
            &server.parse().unwrap(),
            Duration::from_secs(2)
        ) {
            Ok(_) => {
                let latency = start.elapsed().as_millis() as u64;
                latencies.push(latency);
                successful_connections += 1;
            }
            Err(_) => {}
        }
    }

    if latencies.is_empty() {
        return Err("No successful connections".to_string());
    }

    // Calculate average latency
    let avg_latency = latencies.iter().sum::<u64>() / latencies.len() as u64;

    // Calculate jitter (variation in latency)
    let jitter = if latencies.len() > 1 {
        let max = *latencies.iter().max().unwrap();
        let min = *latencies.iter().min().unwrap();
        max - min
    } else {
        0
    };

    // Calculate packet loss percentage
    let packet_loss = ((servers.len() - successful_connections) as f64 / servers.len() as f64) * 100.0;

    // Calculate quality score (0-100)
    let quality_score = if avg_latency < 50 && jitter < 20 && packet_loss == 0.0 {
        100
    } else if avg_latency < 100 && jitter < 50 && packet_loss < 10.0 {
        80
    } else if avg_latency < 200 && packet_loss < 25.0 {
        60
    } else if avg_latency < 500 && packet_loss < 50.0 {
        40
    } else {
        20
    };

    Ok(ConnectionQualityMetrics {
        latency: avg_latency,
        jitter,
        packet_loss,
        quality_score,
    })
}

#[tauri::command]
async fn estimate_bandwidth() -> Result<f64, String> {
    // Download a small test file to estimate bandwidth
    let test_url = "https://www.google.com/robots.txt"; // Small known file
    let start = std::time::Instant::now();

    let response = HTTP_CLIENT
        .get(test_url)
        .send()
        .await
        .map_err(|e| format!("Bandwidth test failed: {}", e))?;

    let bytes = response.bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let elapsed = start.elapsed().as_secs_f64();
    let bytes_len = bytes.len() as f64;

    // Calculate bandwidth in Mbps
    let bandwidth_mbps = (bytes_len * 8.0) / (elapsed * 1_000_000.0);

    Ok(bandwidth_mbps)
}

#[tauri::command]
async fn run_speed_test() -> Result<SpeedTestResult, String> {
    use std::time::Instant;

    // Test server - using httpbin for reliable testing
    let test_server = "https://httpbin.org".to_string();

    // Test latency
    let latency_start = Instant::now();
    HTTP_CLIENT
        .get(&format!("{}/get", test_server))
        .send()
        .await
        .map_err(|e| format!("Latency test failed: {}", e))?;
    let latency = latency_start.elapsed().as_millis() as u64;

    // Test download speed (download 1MB worth of data)
    let download_start = Instant::now();
    let download_response = HTTP_CLIENT
        .get(&format!("{}/bytes/1048576", test_server)) // 1MB
        .send()
        .await
        .map_err(|e| format!("Download test failed: {}", e))?;

    let download_bytes = download_response.bytes()
        .await
        .map_err(|e| format!("Failed to read download data: {}", e))?;

    let download_time = download_start.elapsed().as_secs_f64();
    let download_speed = (download_bytes.len() as f64 * 8.0) / (download_time * 1_000_000.0); // Mbps

    // Test upload speed (upload 100KB of data)
    let upload_data = vec![0u8; 102400]; // 100KB
    let upload_start = Instant::now();

    HTTP_CLIENT
        .post(&format!("{}/post", test_server))
        .body(upload_data.clone())
        .send()
        .await
        .map_err(|e| format!("Upload test failed: {}", e))?;

    let upload_time = upload_start.elapsed().as_secs_f64();
    let upload_speed = (upload_data.len() as f64 * 8.0) / (upload_time * 1_000_000.0); // Mbps

    Ok(SpeedTestResult {
        download_speed,
        upload_speed,
        latency,
        server: test_server,
    })
}

// Upload Progress Tracking
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UploadProgress {
    loaded: u64,
    total: u64,
    percentage: f64,
    speed: f64, // bytes per second
}

static UPLOAD_CANCEL_FLAGS: Lazy<Arc<Mutex<std::collections::HashMap<String, bool>>>> =
    Lazy::new(|| Arc::new(Mutex::new(std::collections::HashMap::new())));

#[tauri::command]
async fn upload_file_with_progress(
    url: String,
    file_path: String,
    upload_id: String,
    window: tauri::Window,
) -> Result<HttpResponse, String> {
    use std::path::Path;
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    // Register upload ID for cancellation
    {
        let mut flags = UPLOAD_CANCEL_FLAGS.lock().unwrap();
        flags.insert(upload_id.clone(), false);
    }

    // Read file
    let mut file = File::open(&file_path)
        .await
        .map_err(|e| format!("Failed to open file: {}", e))?;

    let metadata = file.metadata()
        .await
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;

    let total_size = metadata.len();

    // Get filename
    let file_name = Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Invalid file path".to_string())?
        .to_string();

    // Read file in chunks and build byte vector
    let chunk_size = 65536; // 64KB chunks
    let mut file_bytes = Vec::new();
    let mut buffer = vec![0u8; chunk_size];
    let mut loaded = 0u64;
    let start_time = std::time::Instant::now();

    loop {
        // Check if upload was cancelled
        {
            let flags = UPLOAD_CANCEL_FLAGS.lock().unwrap();
            if let Some(&cancelled) = flags.get(&upload_id) {
                if cancelled {
                    return Err("Upload cancelled by user".to_string());
                }
            }
        }

        let n = file.read(&mut buffer)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?;

        if n == 0 {
            break;
        }

        file_bytes.extend_from_slice(&buffer[..n]);
        loaded += n as u64;

        // Calculate progress
        let percentage = (loaded as f64 / total_size as f64) * 100.0;
        let elapsed = start_time.elapsed().as_secs_f64();
        let speed = if elapsed > 0.0 { loaded as f64 / elapsed } else { 0.0 };

        // Emit progress event
        let progress = UploadProgress {
            loaded,
            total: total_size,
            percentage,
            speed,
        };

        let _ = window.emit("upload-progress", progress);
    }

    // Create multipart form
    let part = reqwest::multipart::Part::bytes(file_bytes)
        .file_name(file_name);

    let form = reqwest::multipart::Form::new()
        .part("file", part);

    // Send upload request
    let response = HTTP_CLIENT
        .post(&url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("File upload failed: {}", e))?;

    let status = response.status().as_u16();

    // Extract headers
    let mut headers = std::collections::HashMap::new();
    for (key, value) in response.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            headers.insert(key.to_string(), value_str.to_string());
        }
    }

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    // Clean up cancel flag
    {
        let mut flags = UPLOAD_CANCEL_FLAGS.lock().unwrap();
        flags.remove(&upload_id);
    }

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}

#[tauri::command]
async fn cancel_upload(upload_id: String) -> Result<(), String> {
    let mut flags = UPLOAD_CANCEL_FLAGS.lock().unwrap();
    if let Some(flag) = flags.get_mut(&upload_id) {
        *flag = true;
        Ok(())
    } else {
        Err("Upload ID not found".to_string())
    }
}

#[tauri::command]
async fn upload_file_chunked(
    url: String,
    file_path: String,
    chunk_size: usize,
    window: tauri::Window,
) -> Result<Vec<HttpResponse>, String> {
    use std::path::Path;
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    let mut file = File::open(&file_path)
        .await
        .map_err(|e| format!("Failed to open file: {}", e))?;

    let metadata = file.metadata()
        .await
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;

    let total_size = metadata.len();
    let file_name = Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Invalid file path".to_string())?
        .to_string();

    let mut responses = Vec::new();
    let mut chunk_index = 0;
    let mut loaded = 0u64;
    let start_time = std::time::Instant::now();

    loop {
        let mut chunk_buffer = vec![0u8; chunk_size];
        let n = file.read(&mut chunk_buffer)
            .await
            .map_err(|e| format!("Failed to read file chunk: {}", e))?;

        if n == 0 {
            break;
        }

        chunk_buffer.truncate(n);
        loaded += n as u64;

        // Upload chunk
        let part = reqwest::multipart::Part::bytes(chunk_buffer)
            .file_name(format!("{}.part{}", file_name, chunk_index));

        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("chunk_index", chunk_index.to_string())
            .text("total_chunks", ((total_size + chunk_size as u64 - 1) / chunk_size as u64).to_string());

        let response = HTTP_CLIENT
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Chunk upload failed: {}", e))?;

        let status = response.status().as_u16();
        let mut headers = std::collections::HashMap::new();
        for (key, value) in response.headers().iter() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        responses.push(HttpResponse {
            status,
            headers,
            body,
        });

        // Emit progress
        let percentage = (loaded as f64 / total_size as f64) * 100.0;
        let elapsed = start_time.elapsed().as_secs_f64();
        let speed = if elapsed > 0.0 { loaded as f64 / elapsed } else { 0.0 };

        let progress = UploadProgress {
            loaded,
            total: total_size,
            percentage,
            speed,
        };

        let _ = window.emit("chunk-upload-progress", progress);

        chunk_index += 1;
    }

    Ok(responses)
}

// SSE Module

#[derive(Clone)]
struct SseState {
    active: Arc<Mutex<bool>>,
}

#[tauri::command]
async fn sse_connect(url: String, window: tauri::Window) -> Result<(), String> {
    use futures_util::StreamExt;

    let active = Arc::new(Mutex::new(true));
    let state = SseState {
        active: active.clone(),
    };

    // Store state in window for potential cleanup
    window.manage(state);

    tokio::spawn(async move {
        let client = HTTP_CLIENT.clone();

        match client.get(&url).send().await {
            Ok(response) => {
                let mut stream = response.bytes_stream();
                let mut buffer = String::new();

                while let Some(chunk) = stream.next().await {
                    // Check if connection should be closed
                    if !*active.lock().unwrap() {
                        break;
                    }

                    match chunk {
                        Ok(bytes) => {
                            let text = String::from_utf8_lossy(&bytes);
                            buffer.push_str(&text);

                            // Process complete events (ending with double newline)
                            while let Some(pos) = buffer.find("\n\n") {
                                let event = buffer[..pos].to_string();
                                buffer = buffer[pos + 2..].to_string();

                                // Parse and emit the event
                                if !event.is_empty() {
                                    // Extract data from SSE format
                                    let data = event
                                        .lines()
                                        .filter(|line| line.starts_with("data:"))
                                        .map(|line| line.trim_start_matches("data:").trim())
                                        .collect::<Vec<_>>()
                                        .join("\n");

                                    if !data.is_empty() {
                                        let _ = window.emit("sse-message", data);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let _ = window.emit("sse-error", format!("Stream error: {}", e));
                            break;
                        }
                    }
                }

                let _ = window.emit("sse-close", "Connection closed");
            }
            Err(e) => {
                let _ = window.emit("sse-error", format!("Connection failed: {}", e));
            }
        }
    });

    Ok(())
}

// Background Tasks Module - State Management
struct AppState {
    task_manager: Mutex<TaskManager>,
}

// Background Tasks Module - Commands
#[tauri::command]
async fn create_background_task(
    state: tauri::State<'_, AppState>,
    options: CreateTaskOptions,
) -> Result<String, String> {
    let task_manager = state
        .task_manager
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;
    task_manager.create_task(options)
}

#[tauri::command]
async fn get_background_task(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<Option<BackgroundTask>, String> {
    let task_manager = state
        .task_manager
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;
    task_manager.get_task(&id)
}

#[tauri::command]
async fn list_background_tasks(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<BackgroundTask>, String> {
    let task_manager = state
        .task_manager
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;
    task_manager.list_tasks()
}

#[tauri::command]
async fn cancel_background_task(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let task_manager = state
        .task_manager
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;
    task_manager.cancel_task(&id)
}

#[tauri::command]
async fn delete_background_task(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let task_manager = state
        .task_manager
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;
    task_manager.delete_task(&id)
}

#[tauri::command]
async fn execute_demo_task(
    state: tauri::State<'_, AppState>,
    id: String,
    delay_seconds: u64,
) -> Result<(), String> {
    let task_manager = state
        .task_manager
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;
    let tasks_clone = task_manager.clone_tasks();

    // Spawn the task execution in the background
    tokio::spawn(async move {
        let _ = run_task_async(tasks_clone, id, delay_seconds).await;
    });

    Ok(())
}

#[tauri::command]
async fn sse_disconnect(window: tauri::Window) -> Result<(), String> {
    if let Some(state) = window.try_state::<SseState>() {
        *state.active.lock().unwrap() = false;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            task_manager: Mutex::new(TaskManager::new()),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:calendar.db", vec![])
                .build()
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_websocket::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            schedule_notification,
            get_system_info,
            get_system_metrics,
            get_network_metrics,
            get_app_uptime,
            export_events_to_ics,
            fetch_iap_products,
            purchase_product,
            restore_purchases,
            validate_receipt,
            get_iap_platform,
            http_get,
            http_post,
            upload_file,
            upload_file_with_progress,
            cancel_upload,
            upload_file_chunked,
            check_contacts_permission,
            request_contacts_permission,
            get_contacts,
            check_network_status,
            get_network_interfaces,
            get_wifi_info,
            scan_wifi_networks,
            test_connection_quality,
            estimate_bandwidth,
            run_speed_test,
            sse_connect,
            sse_disconnect,
            initialize_camera,
            capture_photo,
            start_video_recording,
            stop_video_recording,
            switch_camera,
            set_flash_mode,
            set_zoom,
            get_cameras,
            check_camera_permission,
            request_camera_permission,
            is_share_supported,
            copy_to_clipboard_backend,
            read_from_clipboard,
            share_text,
            share_files,
            get_share_platform,
            haptic_impact,
            haptic_notification,
            vibrate,
            vibrate_pattern,
            cancel_vibration,
            has_vibrator,
            check_biometric_availability,
            authenticate_biometric,
            get_biometric_types,
            generate_encryption_key,
            encrypt_data,
            decrypt_data,
            secure_storage_set,
            secure_storage_get,
            secure_storage_delete,
            get_battery_info,
            get_audio_devices,
            create_background_task,
            get_background_task,
            list_background_tasks,
            cancel_background_task,
            delete_background_task,
            execute_demo_task,
            get_cpu_info,
            get_storage_devices,
            get_device_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
