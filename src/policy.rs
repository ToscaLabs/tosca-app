use std::sync::Arc;

use async_lock::Mutex;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use minijinja::context;

use crate::{footer, AppState, NAVBAR};

pub(crate) async fn policy(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Html<String>, StatusCode> {
    let env = &state.lock().await.env;
    let template = env.get_template("policy").unwrap();

    let rendered = template
        .render(context! {
            title => "Policies",
            navbar => NAVBAR,
            footer => footer(),
        })
        .unwrap();

    Ok(Html(rendered))
}
