// TODO: Saves the responses in a collector and presents them in the log page

use axum::extract::{Path, State};
use axum::response::Html;

use minijinja::context;

use crate::error::{error_with_info, Error};
use crate::layout;
use crate::AppState;

pub(crate) async fn response_log(
    State(state): State<AppState>,
    Path(_device_id): Path<usize>,
) -> Result<Html<String>, Error> {
    let controller = state.controller.lock().await;

    let template = error_with_info(state.env.get_template("response-log"), "Response log error")?;

    let rendered = error_with_info(
        template.render(context! {
            title => "Ascot Controller",
            navbar => layout::NAVBAR,
            footer => layout::footer(),
        }),
        "Response log render error",
    )?;

    Ok(Html(rendered))
}
