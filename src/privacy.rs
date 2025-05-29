use axum::extract::State;
use axum::response::Html;

use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::layout::RenderLayout;
use crate::AppState;

pub(crate) async fn privacy(State(state): State<AppState>) -> Result<Html<String>, Error> {
    let template = error_with_info(
        &state.env,
        state.env.get_template("privacy"),
        lang::PRIVACY_TEMPLATE_ERROR,
    )?;

    let rendered = error_with_info(
        &state.env,
        template.render(RenderLayout::new()),
        lang::PRIVACY_RENDER_ERROR,
    )?;

    Ok(Html(rendered))
}
