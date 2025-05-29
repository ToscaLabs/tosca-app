// TODO: Saves the responses in a collector and presents them in the log page

use axum::extract::{Path, State};
use axum::response::Html;

use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::layout::RenderLayout;
use crate::AppState;

pub(crate) async fn response_log(
    State(state): State<AppState>,
    Path(_device_id): Path<usize>,
) -> Result<Html<String>, Error> {
    let controller = state.controller.lock().await;

    let template = error_with_info(
        &state.env,
        state.env.get_template("response-log"),
        lang::RESPONSE_TEMPLATE_ERROR,
    )?;

    let rendered = error_with_info(
        &state.env,
        template.render(RenderLayout::new()),
        lang::RESPONSE_RENDER_ERROR,
    )?;

    Ok(Html(rendered))
}
