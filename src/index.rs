use ascot::device::DeviceKind;
use ascot::hazards::{Hazard, ALL_HAZARDS};

use axum::extract::State;
use axum::response::Html;

use minijinja::context;

use crate::device::Device;
use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::layout;
use crate::AppState;

pub(crate) async fn index(State(state): State<AppState>) -> Result<Html<String>, Error> {
    let template = error_with_info(state.env.get_template("index"), lang::INDEX_TEMPLATE_ERROR)?;

    // TODO: Only the hazards associated with each discovered device must be considered.
    let all_hazards = ALL_HAZARDS.iter().map(Hazard::data).collect::<Vec<_>>();

    let devices = vec![
        Device::new(DeviceKind::Light),
        Device::new(DeviceKind::Camera),
    ];

    let rendered = error_with_info(
        template.render(context! {
            title => lang::INDEX_TITLE,
            navbar => layout::NAVBAR,
            no_devices_message => lang::NO_DEVICES,
            discover_message => lang::DISCOVER_DEVICES,
            devices => devices,
            hazards => all_hazards,
            footer => layout::footer(),
        }),
        lang::INDEX_RENDER_ERROR,
    )?;

    Ok(Html(rendered))
}
