// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_notification::NotificationExt;
use std::time::Duration;
use serde::{Deserialize, Serialize, Deserializer};
use once_cell::sync::Lazy;

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

// Network Status & WiFi Module
#[derive(Debug, Serialize, Deserialize)]
struct NetworkStatus {
    online: bool,
    #[serde(rename = "connectionType")]
    connection_type: String,
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

    Ok(NetworkStatus {
        online,
        connection_type: if online { "unknown".to_string() } else { "none".to_string() },
    })
}

#[tauri::command]
async fn get_wifi_info() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
            .args(["-I"])
            .output()
            .map_err(|e| format!("Failed to get WiFi info: {}", e))?;

        let info = String::from_utf8_lossy(&output.stdout);

        // Parse SSID from output
        for line in info.lines() {
            if line.contains("SSID:") && !line.contains("BSSID") {
                let ssid = line.split(':').nth(1).unwrap_or("").trim();
                if !ssid.is_empty() {
                    return Ok(ssid.to_string());
                }
            }
        }

        Err("Not connected to WiFi".to_string())
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let output = Command::new("iwgetid")
            .args(["-r"])
            .output()
            .map_err(|e| format!("Failed to get WiFi info: {}", e))?;

        let ssid = String::from_utf8_lossy(&output.stdout);
        let trimmed = ssid.trim();

        if trimmed.is_empty() {
            Err("Not connected to WiFi".to_string())
        } else {
            Ok(trimmed.to_string())
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

        // Parse SSID from output
        for line in info.lines() {
            if line.trim().starts_with("SSID") && !line.contains("BSSID") {
                let ssid = line.split(':').nth(1).unwrap_or("").trim();
                if !ssid.is_empty() {
                    return Ok(ssid.to_string());
                }
            }
        }

        Err("Not connected to WiFi".to_string())
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err("WiFi info not supported on this platform".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        .plugin(tauri_plugin_websocket::init())
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
            check_network_status,
            get_wifi_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
