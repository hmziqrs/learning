// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_notification::NotificationExt;
use std::time::Duration;
use serde::{Deserialize, Serialize, Deserializer};

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
        .invoke_handler(tauri::generate_handler![
            greet,
            schedule_notification,
            export_events_to_ics,
            fetch_iap_products,
            purchase_product,
            restore_purchases,
            validate_receipt,
            get_iap_platform
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
