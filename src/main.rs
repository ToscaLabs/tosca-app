mod index;
mod policy;

use std::sync::Arc;

use axum::{routing::get, Router};

use chrono::Datelike;
use chrono::Utc;

use minijinja::Environment;

use crate::index::index;
use crate::policy::policy;

const PROJECT: &str = "Ascot";

macro_rules! builtin_templates {
    ($(($name:expr, $template:expr)),+) => {
        [
        $(
            (
                $name,
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/templates/", $template)),
            )
        ),+
        ]
    }
}

static TEMPLATES: &[(&str, &str)] = &builtin_templates![
    ("css.custom", "custom.css"),
    ("js.custom", "custom.js"),
    ("layout", "layout.html"),
    ("head", "head.html"),
    ("navbar", "navbar.html"),
    ("scripts", "scripts.html"),
    ("footer", "footer.html"),
    ("index", "index.html"),
    ("devices", "devices.html"),
    ("error", "error.html"),
    ("modal-device", "modal-device.html"),
    ("modal-hazards", "modal-hazards.html")
];

pub(crate) fn footer() -> String {
    format!("Copyright © {} {PROJECT}", Utc::now().year())
}

struct AppState {
    env: Environment<'static>,
}

#[tokio::main]
async fn main() {
    let mut env = Environment::new();

    for (name, src) in TEMPLATES.iter() {
        env.add_template(name, src)
            .expect("Internal error, built-in template");
    }

    // Pass environment to handlers via state
    let app_state = Arc::new(AppState { env });

    // define routes
    let app = Router::new()
        .route("/", get(index))
        .route("/privacy", get(policy))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
