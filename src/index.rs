use serde::Serialize;

use std::sync::Arc;

use ascot_library::device::DeviceKind;
use ascot_library::hazards::{Category, Hazards};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use minijinja::context;

use crate::{AppState, TITLE};

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

#[derive(Serialize)]
struct HazardData {
    kind: DeviceKind,
}

fn create_hazards() -> Vec<HazardData> {
    let mut HazardData = Vec::new();
    let mut safety = Hazards::init_with_elements(Category::Safety.hazards());
    let privacy = Hazards::init_with_elements(Category::Privacy.hazards());
    let financial = Hazards::init_with_elements(Category::Financial.hazards());

    safety.merge(&privacy);
    safety.merge(&financial);
    safety
}

pub(crate) async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("index").unwrap();

    let rendered = template
        .render(context! {
            title => TITLE,
            no_devices_message => "No devices found.",
            discover_message => "Discover device",
            devices => create_devices(),
            hazards => create_hazards(),
        })
        .unwrap();

    Ok(Html(rendered))
}
