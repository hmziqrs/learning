use super::{GeolocationError, GeolocationPosition, GeolocationResult};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_core_location::{
    CLAuthorizationStatus, CLLocation, CLLocationManager, CLLocationManagerDelegate,
};
use objc2_foundation::{MainThreadMarker, NSError, NSObject};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::oneshot;

// Delegate for handling location updates
#[derive(Debug)]
struct LocationDelegate {
    sender: Arc<Mutex<Option<oneshot::Sender<GeolocationResult>>>>,
}

impl LocationDelegate {
    fn new(sender: oneshot::Sender<GeolocationResult>) -> Retained<Self> {
        let delegate = Self {
            sender: Arc::new(Mutex::new(Some(sender))),
        };
        unsafe { Retained::new(NSObject::new()) }
    }
}

unsafe impl CLLocationManagerDelegate for LocationDelegate {
    #[method(locationManager:didUpdateLocations:)]
    fn location_manager_did_update_locations(
        &self,
        _manager: &CLLocationManager,
        locations: &objc2_foundation::NSArray<CLLocation>,
    ) {
        if let Some(location) = locations.first() {
            let coordinate = unsafe { location.coordinate() };
            let altitude = unsafe { location.altitude() };
            let horizontal_accuracy = unsafe { location.horizontalAccuracy() };
            let vertical_accuracy = unsafe { location.verticalAccuracy() };
            let course = unsafe { location.course() };
            let speed = unsafe { location.speed() };

            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let position = GeolocationPosition {
                latitude: coordinate.latitude,
                longitude: coordinate.longitude,
                altitude: if altitude >= 0.0 {
                    Some(altitude)
                } else {
                    None
                },
                accuracy: horizontal_accuracy,
                altitude_accuracy: if vertical_accuracy >= 0.0 {
                    Some(vertical_accuracy)
                } else {
                    None
                },
                heading: if course >= 0.0 { Some(course) } else { None },
                speed: if speed >= 0.0 { Some(speed) } else { None },
                timestamp,
            };

            if let Some(sender) = self.sender.lock().unwrap().take() {
                let _ = sender.send(Ok(position));
            }
        }
    }

    #[method(locationManager:didFailWithError:)]
    fn location_manager_did_fail_with_error(
        &self,
        _manager: &CLLocationManager,
        error: &NSError,
    ) {
        let error_code = unsafe { error.code() };
        let message = format!("CoreLocation error: {}", error_code);

        if let Some(sender) = self.sender.lock().unwrap().take() {
            let _ = sender.send(Err(GeolocationError::new("LOCATION_ERROR", &message)));
        }
    }

    #[method(locationManager:didChangeAuthorizationStatus:)]
    fn location_manager_did_change_authorization_status(
        &self,
        manager: &CLLocationManager,
        status: CLAuthorizationStatus,
    ) {
        match status {
            CLAuthorizationStatus::AuthorizedAlways | CLAuthorizationStatus::AuthorizedWhenInUse => {
                // Authorization granted, location updates will start
            }
            CLAuthorizationStatus::Denied | CLAuthorizationStatus::Restricted => {
                if let Some(sender) = self.sender.lock().unwrap().take() {
                    let _ = sender.send(Err(GeolocationError::new(
                        "PERMISSION_DENIED",
                        "Location permission denied or restricted",
                    )));
                }
            }
            _ => {
                // NotDetermined or other status
            }
        }
    }
}

pub async fn get_current_position() -> GeolocationResult {
    // This needs to run on the main thread
    let (tx, rx) = oneshot::channel();

    std::thread::spawn(move || {
        let mtm = MainThreadMarker::new().expect("Not running on main thread");

        let manager = unsafe { CLLocationManager::new(mtm) };

        // Check authorization status
        let status = unsafe { CLLocationManager::authorizationStatus() };

        match status {
            CLAuthorizationStatus::NotDetermined => {
                // Request authorization
                unsafe {
                    manager.requestWhenInUseAuthorization();
                }
            }
            CLAuthorizationStatus::Denied | CLAuthorizationStatus::Restricted => {
                let _ = tx.send(Err(GeolocationError::new(
                    "PERMISSION_DENIED",
                    "Location permission denied. Please enable in System Settings > Privacy & Security > Location Services",
                )));
                return;
            }
            _ => {
                // Already authorized
            }
        }

        // Set up delegate
        let delegate = LocationDelegate::new(tx);
        let delegate_protocol: Retained<ProtocolObject<dyn CLLocationManagerDelegate>> =
            unsafe { ProtocolObject::from_retained_unchecked(delegate.into()) };

        unsafe {
            manager.setDelegate(Some(&delegate_protocol));
            manager.setDesiredAccuracy(objc2_core_location::kCLLocationAccuracyBest);
            manager.requestLocation();
        }

        // Keep the manager and delegate alive
        // They will be dropped when the location is received or error occurs
        std::mem::forget(manager);
        std::mem::forget(delegate_protocol);
    });

    // Wait for the result with timeout
    match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
        Ok(Ok(result)) => result,
        Ok(Err(_)) => Err(GeolocationError::new(
            "INTERNAL_ERROR",
            "Channel communication failed",
        )),
        Err(_) => Err(GeolocationError::new(
            "TIMEOUT",
            "Location request timed out after 30 seconds",
        )),
    }
}
