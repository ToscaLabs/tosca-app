use serde::Serialize;

use std::sync::Arc;

use ascot_library::device::DeviceKind;
use ascot_library::hazards::Category;

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

#[derive(Serialize)]
struct HazardData {
    id: u16,
    name: &'static str,
    description: &'static str,
    category_name: &'static str,
    category_description: &'static str,
}

fn create_hazards() -> Vec<HazardData> {
    let mut hazards = Vec::new();
    for safety in Category::Safety.hazards() {
        hazards.push(HazardData {
            id: safety.id(),
            name: safety.name(),
            description: safety.description(),
            category_name: safety.category().name(),
            category_description: safety.category().description(),
        });
    }
    hazards

    //let privacy = Hazards::init_with_elements(Category::Privacy.hazards());
    //let financial = Hazards::init_with_elements(Category::Financial.hazards());
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
