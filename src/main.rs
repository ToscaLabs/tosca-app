mod ascot;
mod device;
// TODO
mod database;
mod error;
mod index;
mod language;
#[cfg(feature = "logging")]
mod logging;
// TODO: Maintains the response registers and other methods
mod policy;
mod request;
mod response;
mod template;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use ascot_controller::controller::Controller;

use axum::{
    routing::{get, put},
    Router,
};

use clap::Parser;

use minijinja::Environment;

use tokio::sync::Mutex;

use crate::ascot::create_controller;
use crate::device::discover_devices;
use crate::index::index;
use crate::language::lang;
use crate::policy::policy;
use crate::request::{send_ok_request, send_serial_request};
use crate::response::response_log;

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

    let mut env = Environment::new();

    for (name, src) in template::TEMPLATES {
        env.add_template(name, src)
            .expect(lang::LOADING_TEMPLATE_ERROR);
    }

    // Create controller.
    let controller = create_controller();

    // Pass environment to handlers via state
    let app_state = AppState::new(env, controller);

    // Define routes
    let app = Router::new()
        .route("/", get(index))
        .route("/privacy", get(policy))
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
        tracing::info!(r#": [GET, "/"]"#);
        tracing::info!(r#"Privacy: GET, "/privacy"]"#);
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
