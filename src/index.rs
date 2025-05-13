use std::sync::Arc;

use ascot::device::DeviceKind;
use ascot::hazards::{Hazard, ALL_HAZARDS};

use async_lock::Mutex;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use minijinja::context;

use crate::device::Device;
use crate::{footer, AppState, NAVBAR};

pub(crate) async fn index(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Html<String>, StatusCode> {
    let env = &state.lock().await.env;
    let template = env.get_template("index").unwrap();

    let rendered = template
        .render(context! {
            title => "Home",
            navbar => NAVBAR,
            no_devices_message => "No devices found.",
            discover_message => "Discover device",
            devices => vec![Device::new(DeviceKind::Light), Device::new(DeviceKind::Camera)],
            hazards => ALL_HAZARDS.iter().map(Hazard::data).collect::<Vec<_>>(),
            footer => footer(),
        })
        .unwrap();

    Ok(Html(rendered))
}
