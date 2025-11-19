use super::{GeolocationError, GeolocationPosition, GeolocationResult};
use std::time::{SystemTime, UNIX_EPOCH};
use zbus::{Connection, dbus_proxy};

// GeoClue2 D-Bus interfaces
#[dbus_proxy(
    interface = "org.freedesktop.GeoClue2.Manager",
    default_service = "org.freedesktop.GeoClue2",
    default_path = "/org/freedesktop/GeoClue2/Manager"
)]
trait Manager {
    fn get_client(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

#[dbus_proxy(
    interface = "org.freedesktop.GeoClue2.Client",
    default_service = "org.freedesktop.GeoClue2"
)]
trait Client {
    fn start(&self) -> zbus::Result<()>;
    fn stop(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn location(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn set_desktop_id(&self, id: &str) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn set_distance_threshold(&self, threshold: u32) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.freedesktop.GeoClue2.Location",
    default_service = "org.freedesktop.GeoClue2"
)]
trait Location {
    #[dbus_proxy(property)]
    fn latitude(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn longitude(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn accuracy(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn altitude(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn speed(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn heading(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn timestamp(&self) -> zbus::Result<(u64, u64)>;
}

pub async fn get_current_position() -> GeolocationResult {
    // Connect to session bus
    let connection = Connection::session().await.map_err(|e| {
        GeolocationError::new(
            "DBUS_ERROR",
            &format!("Failed to connect to D-Bus: {}", e),
        )
    })?;

    // Get GeoClue2 Manager
    let manager = ManagerProxy::new(&connection).await.map_err(|e| {
        GeolocationError::new(
            "GEOCLUE_ERROR",
            &format!("Failed to connect to GeoClue2 Manager. Is GeoClue2 running? Error: {}", e),
        )
    })?;

    // Create client
    let client_path = manager.get_client().await.map_err(|e| {
        GeolocationError::new(
            "CLIENT_ERROR",
            &format!("Failed to create GeoClue2 client: {}", e),
        )
    })?;

    let client = ClientProxy::builder(&connection)
        .path(client_path)
        .map_err(|e| {
            GeolocationError::new("PATH_ERROR", &format!("Failed to set client path: {}", e))
        })?
        .build()
        .await
        .map_err(|e| {
            GeolocationError::new(
                "CLIENT_BUILD_ERROR",
                &format!("Failed to build client proxy: {}", e),
            )
        })?;

    // Set desktop ID (required for GeoClue2)
    // Using a generic ID - in production, this should match your .desktop file
    client
        .set_desktop_id("tauri-app")
        .await
        .map_err(|e| {
            GeolocationError::new(
                "DESKTOP_ID_ERROR",
                &format!("Failed to set desktop ID: {}", e),
            )
        })?;

    // Set distance threshold to 0 for immediate updates
    client.set_distance_threshold(0).await.map_err(|e| {
        GeolocationError::new(
            "THRESHOLD_ERROR",
            &format!("Failed to set distance threshold: {}", e),
        )
    })?;

    // Start the client
    client.start().await.map_err(|e| {
        GeolocationError::new(
            "START_ERROR",
            &format!("Failed to start GeoClue2 client. Location services may be disabled: {}", e),
        )
    })?;

    // Wait a bit for location to be available
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Get location with retry
    let mut location_path = None;
    for _ in 0..10 {
        match client.location().await {
            Ok(path) => {
                if path.as_str() != "/" {
                    location_path = Some(path);
                    break;
                }
            }
            Err(_) => {}
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    let location_path = location_path.ok_or_else(|| {
        GeolocationError::new(
            "NO_LOCATION",
            "Failed to get location from GeoClue2. Location may not be available",
        )
    })?;

    // Create location proxy
    let location = LocationProxy::builder(&connection)
        .path(location_path)
        .map_err(|e| {
            GeolocationError::new(
                "LOCATION_PATH_ERROR",
                &format!("Failed to set location path: {}", e),
            )
        })?
        .build()
        .await
        .map_err(|e| {
            GeolocationError::new(
                "LOCATION_BUILD_ERROR",
                &format!("Failed to build location proxy: {}", e),
            )
        })?;

    // Get coordinates
    let latitude = location.latitude().await.map_err(|e| {
        GeolocationError::new("LATITUDE_ERROR", &format!("Failed to get latitude: {}", e))
    })?;

    let longitude = location.longitude().await.map_err(|e| {
        GeolocationError::new(
            "LONGITUDE_ERROR",
            &format!("Failed to get longitude: {}", e),
        )
    })?;

    let accuracy = location.accuracy().await.map_err(|e| {
        GeolocationError::new("ACCURACY_ERROR", &format!("Failed to get accuracy: {}", e))
    })?;

    // Optional properties
    let altitude = location.altitude().await.ok().filter(|a| *a != 0.0);
    let speed = location.speed().await.ok().filter(|s| *s >= 0.0);
    let heading = location.heading().await.ok().filter(|h| *h >= 0.0);

    // Stop the client
    let _ = client.stop().await;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    Ok(GeolocationPosition {
        latitude,
        longitude,
        altitude,
        accuracy,
        altitude_accuracy: None, // GeoClue2 doesn't provide altitude accuracy
        heading,
        speed,
        timestamp,
    })
}
