use axum::extract::State;
use axum::response::Redirect;

use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::AppState;

#[cfg(feature = "fake-devices")]
pub(crate) mod fake {
    use std::collections::{HashMap, HashSet};
    use std::fs::File;

    use ascot::device::{DeviceEnvironment, DeviceKind};
    use ascot::hazards::{Hazard, Hazards};
    use ascot::parameters::Parameters;
    use ascot::response::ResponseKind;
    use ascot::route::{Route, RouteConfigs};

    use ascot_controller::device::{Description, Device, Devices, NetworkInformation};

    fn create_network_info(address: &str, port: u16) -> NetworkInformation {
        let ip_address = address.parse().unwrap();

        let complete_address = format!("http://{}:{port}", &ip_address);

        let mut addresses = HashSet::new();
        addresses.insert(ip_address);
        addresses.insert("172.0.0.1".parse().unwrap());

        let mut properties = HashMap::new();
        properties.insert("scheme".into(), "http".into());

        NetworkInformation::new(
            "device-name1._ascot._tcp.local.".into(),
            addresses,
            port,
            properties,
            complete_address,
        )
    }

    fn create_description(device_kind: DeviceKind, main_route: &str) -> Description {
        Description::new(device_kind, DeviceEnvironment::Os, main_route.into())
    }

    pub(crate) fn create_light() -> Device {
        let network_info = create_network_info("192.168.1.174", 3000);
        let description = create_description(DeviceKind::Light, "light/");

        let light_on_route = Route::put("/on")
            .description("Turn light on.")
            .with_hazard(Hazard::ElectricEnergyConsumption);

        let mut light_off_route = Route::put("/off")
            .description("Turn light off.")
            .with_hazard(Hazard::LogEnergyConsumption)
            .serialize_data();
        light_off_route.response_kind = ResponseKind::Serial;

        let toggle_route = Route::get("/toggle")
            .description("Toggle a light.")
            .with_hazards(
                Hazards::new()
                    .insert(Hazard::FireHazard)
                    .insert(Hazard::ElectricEnergyConsumption),
            )
            .with_parameters(Parameters::new().rangeu64("brightness", (0, 20, 1)));

        let route_configs = RouteConfigs::new()
            .insert(light_on_route.serialize_data())
            .insert(light_off_route)
            .insert(toggle_route.serialize_data());

        Device::new(network_info, description, route_configs)
    }

    pub(crate) fn create_unknown() -> Device {
        let network_info = create_network_info("192.168.1.176", 3000);
        let description = create_description(DeviceKind::Unknown, "ip-camera/");

        let mut camera_stream_route = Route::get("/stream")
            .description("View camera stream.")
            .with_hazards(
                Hazards::new()
                    .insert(Hazard::ElectricEnergyConsumption)
                    .insert(Hazard::VideoDisplay)
                    .insert(Hazard::VideoRecordAndStore),
            )
            .serialize_data();
        camera_stream_route.response_kind = ResponseKind::Stream;

        let screenshot_route = Route::get("/take-screenshot")
            .description("Take a screenshot.")
            .with_hazards(
                Hazards::new()
                    .insert(Hazard::ElectricEnergyConsumption)
                    .insert(Hazard::TakeDeviceScreenshots)
                    .insert(Hazard::TakePictures),
            );

        let route_configs = RouteConfigs::new()
            .insert(camera_stream_route)
            .insert(screenshot_route.serialize_data());

        Device::new(network_info, description, route_configs)
    }

    pub(crate) fn create_fake_devices() -> Devices {
        let mut devices = Devices::new();

        devices.add(create_light());
        devices.add(create_unknown());

        devices
    }

    pub(crate) fn output_devices_on_file() {
        // Retrieve devices.
        let devices = create_fake_devices();
        // Create or open the file to write to
        let file = File::create("devices.json").unwrap();

        // Write the JSON data in pretty format to the file
        serde_json::to_writer_pretty(file, &devices).unwrap();
    }
}

// Find devices in the network and save their metadata into the database.
pub(crate) async fn discover_devices(State(state): State<AppState>) -> Result<Redirect, Error> {
    let mut controller = state.controller.lock().await;

    // Discover devices
    error_with_info(controller.discover().await, lang::DISCOVERY_ERROR)?;

    // If some devices have been found, delete every old device from the
    // database and insert every discovered devices.
    /*if !controller.devices().is_empty() {
        // Clear the database
        //query_error(clear_database(&mut db), uri).await?;

        // Save devices into the database.
        //save_devices(db, devices_info, uri).await?;
    }*/

    // Redirect to index
    Ok(Redirect::to("/"))
}
