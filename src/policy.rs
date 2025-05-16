use axum::extract::State;
use axum::response::Html;

use minijinja::context;

use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::layout;
use crate::AppState;

pub(crate) async fn policy(State(state): State<AppState>) -> Result<Html<String>, Error> {
    let template = error_with_info(
        state.env.get_template("policy"),
        lang::POLICY_TEMPLATE_ERROR,
    )?;

    let rendered = error_with_info(
        template.render(context! {
            title => lang::POLICY_TITLE,
            navbar => layout::NAVBAR,
            footer => layout::footer(),
        }),
        lang::POLICY_RENDER_ERROR,
    )?;

    Ok(Html(rendered))
}
