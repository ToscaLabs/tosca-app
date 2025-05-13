mod device;
mod error;
mod index;
mod policy;

use std::sync::Arc;
use std::time::Duration;

use ascot_controller::controller::Controller;
use ascot_controller::discovery::Discovery;

use axum::{
    routing::{get, put},
    Router,
};

use chrono::Datelike;
use chrono::Utc;

use minijinja::Environment;

use serde::Serialize;

use tokio::sync::Mutex;

use crate::device::discover_devices;
use crate::index::index;
use crate::policy::policy;

const PROJECT: &str = "Ascot";
const NAVBAR: &[NavBar] = &[
    NavBar::new("/", "Devices"),
    NavBar::new("policy.html", "Policy"),
];
const DEFAULT_DISCOVERY_DURATION: Duration = Duration::from_secs(1);

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

#[derive(Serialize)]
struct NavBar {
    href: &'static str,
    name: &'static str,
}

impl NavBar {
    const fn new(href: &'static str, name: &'static str) -> Self {
        Self { href, name }
    }
}

#[derive(Clone)]
struct AppState {
    env: Arc<Environment<'static>>,
    controller: Arc<Mutex<Controller>>,
}

impl AppState {
    fn new(env: Environment<'static>, controller: Controller) -> Self {
        Self {
            env: Arc::new(env),
            controller: Arc::new(Mutex::new(controller)),
        }
    }
}

#[tokio::main]
async fn main() {
    // Create discovery searcher
    let discovery = Discovery::new("ascot")
        .timeout(DEFAULT_DISCOVERY_DURATION)
        .disable_ipv6()
        .disable_network_interface("docker0");

    // Create Ascot controller
    let controller = Controller::new(discovery);

    let mut env = Environment::new();

    for (name, src) in TEMPLATES {
        env.add_template(name, src)
            .expect("Internal error, built-in template");
    }

    // Pass environment to handlers via state
    let app_state = AppState::new(env, controller);

    // Define routes
    let app = Router::new()
        .route("/", get(index))
        .route("/discovery", put(discover_devices))
        .route("/privacy", get(policy))
        .with_state(app_state);

    // The web app must always run on localhost!!!
    //
    // Only the port can be different in case of collisions.
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
