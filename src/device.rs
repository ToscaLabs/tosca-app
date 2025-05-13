use ascot::device::DeviceKind;

use axum::extract::State;
use axum::response::Redirect;

use serde::Serialize;

use crate::error::{error_with_info, Error};
use crate::AppState;

#[derive(Serialize)]
pub(crate) struct Device {
    kind: DeviceKind,
}

impl Device {
    pub(crate) fn new(kind: DeviceKind) -> Self {
        Self { kind }
    }
}

// Find devices in the network and save their metadata into the database.
pub(crate) async fn discover_devices(State(state): State<AppState>) -> Result<Redirect, Error> {
    let mut controller = state.controller.lock().await;

    // Discover devices
    error_with_info(controller.discover().await, "Error in discovering devices")?;

    // If some devices have been found, delete every old device from the
    // database and insert every discovered devices.
    if !controller.devices().is_empty() {
        // Clear the database
        //query_error(clear_database(&mut db), uri).await?;

        // Save devices into the database.
        //save_devices(db, devices_info, uri).await?;
    }

    // Redirect to index
    Ok(Redirect::to("/"))
}
