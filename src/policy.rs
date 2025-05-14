use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use minijinja::context;

use crate::AppState;

pub(crate) async fn policy(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("policy").unwrap();

    let rendered = template
        .render(context! {
            title => "Policies",
            navbar => crate::template::NAVBAR,
            footer => crate::template::footer(),
        })
        .unwrap();

    Ok(Html(rendered))
}
