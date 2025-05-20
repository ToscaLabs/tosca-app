use axum::extract::State;
use axum::response::Html;

use minijinja::context;

use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::layout;
use crate::utils::retrieve_all_hazards;
use crate::AppState;

pub(crate) async fn index(State(state): State<AppState>) -> Result<Html<String>, Error> {
    let controller = state.controller.lock().await;

    let template = error_with_info(state.env.get_template("index"), lang::INDEX_TEMPLATE_ERROR)?;

    // TODO: Only the hazards associated with each discovered device must be considered.
    let all_hazards = retrieve_all_hazards();

    #[cfg(not(feature = "fake-devices"))]
    let devices = controller.devices();

    #[cfg(feature = "fake-devices")]
    let devices = crate::device::fake::create_fake_devices();

    let rendered = error_with_info(
        template.render(context! {
            title => "Ascot Controller",
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
