use axum::extract::{Path, State};
use axum::response::Html;

use minijinja::context;

use crate::error::{error_with_info, Error};
use crate::layout;
use crate::AppState;

pub(crate) async fn view_stream(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> Result<Html<String>, Error> {
    let controller = state.controller.lock().await;

    let template = error_with_info(state.env.get_template("stream"), "Stream error")?;

    let rendered = error_with_info(
        template.render(context! {
            title => "Ascot Controller",
            navbar => layout::NAVBAR,
            footer => layout::footer(),
        }),
        "Error render Stream",
    )?;

    Ok(Html(rendered))
}
