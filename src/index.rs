use ascot::hazards::HazardData;

use ascot_controller::device::Devices;

use axum::extract::State;
use axum::response::Html;

use serde::Serialize;

use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::layout::RenderLayout;
use crate::utils::retrieve_all_hazards;
use crate::AppState;

#[derive(Serialize)]
pub struct RenderIndex {
    #[serde(flatten)]
    layout: RenderLayout,
    no_devices_message: &'static str,
    discover_message: &'static str,
    devices: Devices,
    hazards: [HazardData; 24],
}

impl RenderIndex {
    fn new(devices: Devices, hazards: [HazardData; 24]) -> Self {
        Self {
            layout: RenderLayout::new(),
            no_devices_message: lang::NO_DEVICES,
            discover_message: lang::DISCOVER_DEVICES,
            devices,
            hazards,
        }
    }
}

pub(crate) async fn index(State(state): State<AppState>) -> Result<Html<String>, Error> {
    let controller = state.controller.lock().await;

    let template = error_with_info(
        &state.env,
        state.env.get_template("index"),
        lang::INDEX_TEMPLATE_ERROR,
    )?;

    // TODO: Only the hazards associated with each discovered device must be considered.
    let all_hazards = retrieve_all_hazards();

    #[cfg(not(feature = "fake-devices"))]
    let devices = controller.devices();

    #[cfg(feature = "fake-devices")]
    let devices = crate::device::fake::create_fake_devices();

    let rendered = error_with_info(
        &state.env,
        template.render(RenderIndex::new(devices, all_hazards)),
        lang::INDEX_RENDER_ERROR,
    )?;

    Ok(Html(rendered))
}
