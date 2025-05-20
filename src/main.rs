mod device;
mod utils;
// TODO: Implement database
mod database;
mod error;
mod index;
mod language;
mod layout;
#[cfg(feature = "logging")]
mod logging;
// TODO: Implement privacy rules
mod privacy;
mod request;
// TODO: Maintains the response log and other methods
mod response;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use ascot::hazards::{Hazard, HazardData};
use ascot_controller::controller::Controller;

use axum::{
    handler::HandlerWithoutStateExt,
    routing::{get, put},
    Router,
};

use clap::Parser;

use minijinja::value::ViaDeserialize;
use minijinja::Environment;

use serde::Deserialize;

use tokio::sync::Mutex;

use tower_http::services::ServeDir;

use crate::device::discover_devices;
use crate::error::missing_assets;
use crate::index::index;
use crate::language::lang;
use crate::privacy::privacy;
use crate::request::{send_ok_request, send_serial_request};
use crate::response::response_log;
use crate::utils::create_controller;

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
    ("layout", "layout.html"),
    ("head", "head.html"),
    ("navbar", "navbar.html"),
    ("footer", "footer.html"),
    ("index", "index.html"),
    ("create-devices", "create-devices.html"),
    ("modal-devices", "modal-devices.html"),
    ("modal-hazards", "modal-hazards.html"),
    ("error", "error.html"),
    ("privacy", "privacy.html")
];

#[derive(Parser)]
#[command(version, about, long_about = "A web controller for Ascot devices.")]
struct Cli {
    /// Web controller `IPv4` address.
    ///
    /// Only `IPv4` addresses are accepted.
    #[arg(long, default_value_t = Ipv4Addr::LOCALHOST)]
    ip: Ipv4Addr,

    /// Web controller port.
    #[arg(long, default_value_t = 8123)]
    port: u16,
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

fn hazard_id(hazard: ViaDeserialize<Hazard>) -> u16 {
    hazard.data().id
}

fn hazard_category(hazard: ViaDeserialize<Hazard>) -> String {
    hazard.data().category_name.into()
}

#[tokio::main]
async fn main() {
    // Initialize subscriber.
    #[cfg(feature = "logging")]
    tracing::subscriber::set_global_default(logging::create_subscriber())
        .expect(lang::SUBSCRIBER_ERROR);

    // Retrieve CLI arguments.
    let cli = Cli::parse();

    #[cfg(feature = "fake-devices")]
    crate::device::fake::output_devices_on_file();

    let mut env = Environment::new();

    for (name, src) in TEMPLATES {
        env.add_template(name, src)
            .expect(lang::LOADING_TEMPLATE_ERROR);
    }

    env.add_function("hazard_id", hazard_id);
    env.add_function("hazard_category", hazard_category);

    // Create controller.
    let controller = create_controller();

    // Pass environment to handlers via state
    let app_state = AppState::new(env, controller);

    // Loads the directory containing assets such as `CSS` or `JS` files.
    let serve_dir = ServeDir::new("assets").not_found_service(missing_assets.into_service());

    // Define routes
    let app = Router::new()
        .route("/", get(index))
        .route("/privacy", get(privacy))
        .route("/response-log", get(response_log))
        .route("/discovery", put(discover_devices))
        .route("/ok", put(send_ok_request))
        .route("/serial", put(send_serial_request))
        // TODO: Implement Info route
        //.route("/info", put(send_info_request))
        // TODO: Implement Stream route
        // <a href="/stream/id">Stream</a>
        // To view the stream associated with this device.
        //.route("/stream/{id}", get(stream_request))
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
        .with_state(app_state);

    // Creates the web controller listener bind.
    let listener_bind = SocketAddr::new(IpAddr::V4(cli.ip), cli.port);

    // Creates listener.
    let listener = tokio::net::TcpListener::bind(&listener_bind)
        .await
        .expect(lang::LISTENER_ERROR);

    // Prints listener bind and controller startup message.
    #[cfg(feature = "logging")]
    {
        tracing::info!(r#"Home: [GET, "/"]"#);
        tracing::info!(r#"Policy: GET, "/privacy"]"#);
        tracing::info!(r#"Response Log: GET, "/response-log"]"#);
        tracing::info!(r#"Discovery: [PUT, "/discovery"]"#);
        tracing::info!(r#"Ok request: [PUT, "/ok"]"#);
        tracing::info!(r#"Serial request: [PUT, "/serial"]"#);
        tracing::info!("{}: {listener_bind}", lang::CONTROLLER_ADDRESS_MESSAGE);
        tracing::info!("{}", lang::CONTROLLER_STARTUP_MESSAGE);
    }

    // Runs server.
    axum::serve(listener, app)
        .await
        .expect(lang::SERVER_STARTUP_ERROR);
}
