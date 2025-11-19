// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Emitter, Manager};
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

    if let Ok(output) = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        .args(["-I"])
        .output()
    {
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

        // Method 1: Try using airport utility
        let airport_result = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
            .args(["-I"])
            .output();

        if let Ok(output) = airport_result {
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
        if wifi_info.ssid.is_empty() {
            if let Ok(output) = Command::new("networksetup")
                .args(["-getairportnetwork", "en0"])
                .output()
            {
                let info = String::from_utf8_lossy(&output.stdout);
                if info.contains("Current Wi-Fi Network:") {
                    if let Some(ssid) = info.split("Current Wi-Fi Network:").nth(1) {
                        wifi_info.ssid = ssid.trim().to_string();
                    }
                }
            }
        }

        // Get IP address
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

        if wifi_info.ssid.is_empty() {
            Err("Not connected to WiFi or WiFi interface not found. Note: On macOS, WiFi information requires the 'airport' utility or 'networksetup' command.".to_string())
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
async fn scan_wifi_networks() -> Result<Vec<WiFiNetwork>, String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
            .args(["-s"])
            .output()
            .map_err(|e| format!("Failed to scan WiFi networks: {}", e))?;

        let info = String::from_utf8_lossy(&output.stdout);
        let mut networks = Vec::new();

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
use std::sync::{Arc, Mutex};

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
            sse_disconnect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
