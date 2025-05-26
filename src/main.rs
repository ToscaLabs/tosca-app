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
mod stream;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use ascot_controller::controller::Controller;

use axum::{
    handler::HandlerWithoutStateExt,
    routing::{get, post},
    Router,
};

use clap::Parser;

use minijinja::Environment;

use tokio::sync::Mutex;

use tower_http::services::ServeDir;

use crate::device::discover_devices;
use crate::error::{missing_assets, missing_route};
use crate::index::index;
use crate::language::lang;
use crate::privacy::privacy;
use crate::request::send_request;
use crate::response::response_log;
use crate::stream::view_stream;
use crate::utils::{add_functions_to_env, create_controller};

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
    // Layout page.
    ("layout", "layout.html"),
    ("head", "head.html"),
    ("navbar", "navbar.html"),
    ("footer", "footer.html"),
    // Index page.
    ("index", "index.html"),
    ("create-devices", "create-devices.html"),
    ("modal-devices", "modal-devices.html"),
    ("modal-hazards", "modal-hazards.html"),
    ("error", "error.html"),
    ("light", "light.html"),
    ("unknown", "unknown.html"),
    ("parameters", "parameters.html"),
    // Privacy page.
    ("privacy", "privacy.html"),
    // Stream page.
    ("stream", "stream.html")
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

    // Add global functions to minijinja environment.
    add_functions_to_env(&mut env);

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
        .route("/discovery", post(discover_devices))
        .route("/request", post(send_request))
        .route("/view-stream/{id}", get(view_stream))
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
        .fallback(missing_route)
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
        tracing::info!(r#"View Stream: GET, "/view-stream/{{id}}"]"#);
        tracing::info!(r#"Discovery: [PUT, "/discovery"]"#);
        tracing::info!(r#"Send request: [PUT, "/request"]"#);
        tracing::info!(r#"Assets: [SERVICE, "/assets"]"#);
        tracing::info!("{}: {listener_bind}", lang::CONTROLLER_ADDRESS_MESSAGE);
        tracing::info!("{}", lang::CONTROLLER_STARTUP_MESSAGE);
    }

    // Runs server.
    axum::serve(listener, app)
        .await
        .expect(lang::SERVER_STARTUP_ERROR);
}
