use serde::Serialize;

use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use minijinja::context;

use crate::{AppState, TITLE};

#[derive(Serialize)]
struct Device {
    title: &'static str,
}

impl Device {
    fn new() -> Self {
        Self { title: "hello" }
    }
}

fn create_devices() -> Vec<Device> {
    vec![Device::new()]
}

pub(crate) async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("index").unwrap();

    let rendered = template
        .render(context! {
            title => TITLE,
            discover_message => "Discover device",
            devices => create_devices(),
        })
        .unwrap();

    Ok(Html(rendered))
}
