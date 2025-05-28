use axum::extract::State;
use axum::response::Html;

use minijinja::context;

use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::layout;
use crate::AppState;

pub(crate) async fn view_info(State(state): State<AppState>) -> Result<Html<String>, Error> {
    let template = error_with_info(&state.env, state.env.get_template("info"), "Info error")?;

    let rendered = error_with_info(
        &state.env,
        template.render(context! {
            title => "Ascot Controller",
            navbar => layout::NAVBAR,
            footer => layout::footer(),
        }),
        "Info render error",
    )?;

    Ok(Html(rendered))
}
