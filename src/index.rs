use std::sync::Arc;

use ascot_library::device::DeviceKind;
use ascot_library::hazards::ALL_HAZARDS;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use minijinja::context;

use serde::Serialize;

use crate::{footer, AppState, NAVBAR};

#[derive(Serialize)]
struct Device {
    kind: DeviceKind,
}

impl Device {
    fn new(kind: DeviceKind) -> Self {
        Self { kind }
    }
}

pub(crate) async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("index").unwrap();

    let rendered = template
        .render(context! {
            title => "Home",
            navbar => NAVBAR,
            no_devices_message => "No devices found.",
            discover_message => "Discover device",
            devices => vec![Device::new(DeviceKind::Light), Device::new(DeviceKind::Camera)],
            hazards => ALL_HAZARDS.iter().map(|hazard| hazard.data()).collect::<Vec<_>>(),
            footer => footer(),
        })
        .unwrap();

    Ok(Html(rendered))
}
