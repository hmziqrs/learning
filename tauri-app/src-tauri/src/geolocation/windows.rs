use super::{GeolocationError, GeolocationPosition, GeolocationResult};
use std::time::{SystemTime, UNIX_EPOCH};
use windows::Devices::Geolocation::{Geolocator, PositionAccuracy, PositionStatus};
use windows::Security::Authorization::AppCapabilityAccess::AppCapability;

pub async fn get_current_position() -> GeolocationResult {
    // Check if location capability is available
    let capability = AppCapability::Create(&windows::core::HSTRING::from("location"))
        .map_err(|e| {
            GeolocationError::new(
                "CAPABILITY_ERROR",
                &format!("Failed to check location capability: {}", e),
            )
        })?;

    let access = capability.CheckAccess().map_err(|e| {
        GeolocationError::new(
            "ACCESS_CHECK_ERROR",
            &format!("Failed to check location access: {}", e),
        )
    })?;

    use windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus;
    match access {
        AppCapabilityAccessStatus::DeniedBySystem | AppCapabilityAccessStatus::DeniedByUser => {
            return Err(GeolocationError::new(
                "PERMISSION_DENIED",
                "Location access is denied. Please enable in Settings > Privacy > Location",
            ));
        }
        AppCapabilityAccessStatus::NotDeclaredByApp => {
            return Err(GeolocationError::new(
                "NOT_DECLARED",
                "Location capability not declared in app manifest",
            ));
        }
        AppCapabilityAccessStatus::UserPromptRequired => {
            // Will prompt user when we request location
        }
        _ => {
            // Allowed
        }
    }

    // Create geolocator
    let geolocator = Geolocator::new().map_err(|e| {
        GeolocationError::new("INIT_ERROR", &format!("Failed to create Geolocator: {}", e))
    })?;

    // Set desired accuracy to high (GPS)
    geolocator
        .SetDesiredAccuracy(PositionAccuracy::High)
        .map_err(|e| {
            GeolocationError::new(
                "CONFIG_ERROR",
                &format!("Failed to set accuracy: {}", e),
            )
        })?;

    // Check location status
    let status = geolocator.LocationStatus().map_err(|e| {
        GeolocationError::new("STATUS_ERROR", &format!("Failed to get status: {}", e))
    })?;

    match status {
        PositionStatus::Disabled | PositionStatus::NotAvailable => {
            return Err(GeolocationError::new(
                "LOCATION_DISABLED",
                "Location services are disabled or not available",
            ));
        }
        _ => {}
    }

    // Get position with timeout
    let position_future = geolocator.GetGeopositionAsync().map_err(|e| {
        GeolocationError::new(
            "REQUEST_ERROR",
            &format!("Failed to request position: {}", e),
        )
    })?;

    // Wait for position with timeout
    let geoposition = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        async move {
            position_future.await.map_err(|e| {
                GeolocationError::new("POSITION_ERROR", &format!("Failed to get position: {}", e))
            })
        },
    )
    .await
    .map_err(|_| {
        GeolocationError::new("TIMEOUT", "Location request timed out after 30 seconds")
    })??;

    // Extract coordinate
    let coordinate = geoposition.Coordinate().map_err(|e| {
        GeolocationError::new(
            "COORDINATE_ERROR",
            &format!("Failed to get coordinate: {}", e),
        )
    })?;

    let point = coordinate.Point().map_err(|e| {
        GeolocationError::new("POINT_ERROR", &format!("Failed to get point: {}", e))
    })?;

    let position_data = point.Position().map_err(|e| {
        GeolocationError::new(
            "POSITION_DATA_ERROR",
            &format!("Failed to get position data: {}", e),
        )
    })?;

    // Get accuracy
    let accuracy = coordinate.Accuracy().unwrap_or(0.0);

    // Get optional values
    let altitude = point.AltitudeReferenceSystem().ok().and_then(|_| {
        position_data.Altitude.try_into().ok()
    });

    let altitude_accuracy = coordinate.AltitudeAccuracy().ok().and_then(|v| {
        if let Ok(val) = v.Value() {
            Some(val)
        } else {
            None
        }
    });

    let heading = coordinate.Heading().ok().and_then(|v| {
        if let Ok(val) = v.Value() {
            Some(val)
        } else {
            None
        }
    });

    let speed = coordinate.Speed().ok().and_then(|v| {
        if let Ok(val) = v.Value() {
            Some(val)
        } else {
            None
        }
    });

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    Ok(GeolocationPosition {
        latitude: position_data.Latitude,
        longitude: position_data.Longitude,
        altitude,
        accuracy,
        altitude_accuracy,
        heading,
        speed,
        timestamp,
    })
}
