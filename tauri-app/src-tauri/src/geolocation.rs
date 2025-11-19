use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeolocationPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub accuracy: f64,
    pub altitude_accuracy: Option<f64>,
    pub heading: Option<f64>,
    pub speed: Option<f64>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeolocationError {
    pub code: String,
    pub message: String,
}

impl GeolocationError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}

pub type GeolocationResult = Result<GeolocationPosition, GeolocationError>;

// macOS implementation using CoreLocation
#[cfg(target_os = "macos")]
pub mod macos;

// Windows implementation using WinRT Geolocator
#[cfg(target_os = "windows")]
pub mod windows;

// Linux implementation using GeoClue
#[cfg(target_os = "linux")]
pub mod linux;

// Mobile fallback uses Tauri plugin
#[cfg(any(target_os = "android", target_os = "ios"))]
pub async fn get_current_position() -> GeolocationResult {
    Err(GeolocationError::new(
        "MOBILE_USE_PLUGIN",
        "Use Tauri geolocation plugin for mobile platforms",
    ))
}

// Desktop implementations
#[cfg(target_os = "macos")]
pub async fn get_current_position() -> GeolocationResult {
    macos::get_current_position().await
}

#[cfg(target_os = "windows")]
pub async fn get_current_position() -> GeolocationResult {
    windows::get_current_position().await
}

#[cfg(target_os = "linux")]
pub async fn get_current_position() -> GeolocationResult {
    linux::get_current_position().await
}

// Check if native geolocation is available
pub fn is_native_available() -> bool {
    cfg!(any(
        target_os = "macos",
        target_os = "windows",
        target_os = "linux"
    ))
}
