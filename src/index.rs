use serde::Serialize;

use std::sync::Arc;

use ascot_library::device::DeviceKind;
use ascot_library::hazards::{HazardData, ALL_HAZARDS};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use minijinja::context;

use crate::AppState;

#[derive(Serialize)]
struct Device {
    kind: DeviceKind,
}

impl Device {
    fn new(kind: DeviceKind) -> Self {
        Self { kind }
    }
}

fn create_devices() -> Vec<Device> {
    vec![
        Device::new(DeviceKind::Light),
        Device::new(DeviceKind::Camera),
    ]
}

fn create_hazards() -> Vec<HazardData> {
    ALL_HAZARDS.iter().map(|hazard| hazard.data()).collect()
}

pub(crate) async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("index").unwrap();

    let rendered = template
        .render(context! {
            title => "Home",
            no_devices_message => "No devices found.",
            discover_message => "Discover device",
            devices => create_devices(),
            hazards => create_hazards(),
        })
        .unwrap();

    Ok(Html(rendered))
}
