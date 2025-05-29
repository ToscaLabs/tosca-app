use axum::extract::{Path, State};
use axum::response::Html;

use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::layout::RenderLayout;
use crate::AppState;

pub(crate) async fn view_stream(
    State(state): State<AppState>,
    Path(_device_id): Path<usize>,
) -> Result<Html<String>, Error> {
    let controller = state.controller.lock().await;

    let template = error_with_info(
        &state.env,
        state.env.get_template("stream"),
        lang::STREAM_TEMPLATE_ERROR,
    )?;

    let rendered = error_with_info(
        &state.env,
        template.render(RenderLayout::new()),
        lang::STREAM_RENDER_ERROR,
    )?;

    Ok(Html(rendered))
}
