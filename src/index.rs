use ascot::device::DeviceKind;
use ascot::hazards::{Hazard, ALL_HAZARDS};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use minijinja::context;

use crate::device::Device;
use crate::AppState;

pub(crate) async fn index(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("index").unwrap();

    let rendered = template
        .render(context! {
            title => "Home",
            navbar => crate::template::NAVBAR,
            no_devices_message => "No devices found.",
            discover_message => "Discover device",
            devices => vec![Device::new(DeviceKind::Light), Device::new(DeviceKind::Camera)],
            hazards => ALL_HAZARDS.iter().map(Hazard::data).collect::<Vec<_>>(),
            footer => crate::template::footer(),
        })
        .unwrap();

    Ok(Html(rendered))
}
