use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use minijinja::context;

use crate::{AppState, TITLE};

pub(crate) async fn policy(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("policy").unwrap();

    let rendered = template
        .render(context! {
            title => TITLE,
        })
        .unwrap();

    Ok(Html(rendered))
}
