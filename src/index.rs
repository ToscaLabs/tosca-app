use ascot::device::DeviceKind;
use ascot::hazards::{Hazard, ALL_HAZARDS};

use axum::extract::State;
use axum::response::Html;

use minijinja::context;

use crate::device::Device;
use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::AppState;

pub(crate) async fn index(State(state): State<AppState>) -> Result<Html<String>, Error> {
    let template = error_with_info(state.env.get_template("index"), lang::INDEX_TEMPLATE_ERROR)?;

    let rendered = error_with_info(
        template.render(context! {
            title => lang::INDEX_TITLE,
            navbar => crate::template::NAVBAR,
            no_devices_message => lang::NO_DEVICES,
            discover_message => lang::DISCOVER_DEVICES,
            devices => vec![Device::new(DeviceKind::Light), Device::new(DeviceKind::Camera)],
            hazards => ALL_HAZARDS.iter().map(Hazard::data).collect::<Vec<_>>(),
            footer => crate::template::footer(),
        }),
        lang::INDEX_RENDER_ERROR,
    )?;

    Ok(Html(rendered))
}
