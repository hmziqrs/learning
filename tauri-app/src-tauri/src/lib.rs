// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod background_tasks;

use tauri_plugin_notification::NotificationExt;
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
        let _ = execute_demo_task(tasks_clone, id, delay_seconds).await;
    });

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
        .invoke_handler(tauri::generate_handler![
            greet,
            schedule_notification,
            export_events_to_ics,
            fetch_iap_products,
            purchase_product,
            restore_purchases,
            validate_receipt,
            get_iap_platform,
            http_get,
            http_post,
            upload_file,
            check_contacts_permission,
            request_contacts_permission,
            get_contacts,
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
            create_background_task,
            get_background_task,
            list_background_tasks,
            cancel_background_task,
            delete_background_task,
            execute_demo_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
