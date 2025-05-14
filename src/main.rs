mod ascot;
mod device;
mod error;
mod index;
mod language;
mod policy;
mod template;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use ascot_controller::controller::Controller;

use axum::{
    routing::{get, put},
    Router,
};

use minijinja::Environment;

use tokio::sync::Mutex;

use crate::ascot::create_controller;
use crate::device::discover_devices;
use crate::index::index;
use crate::language::lang;
use crate::policy::policy;

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
    // Initialize tracing subscriber with custom formatter
    //let subscriber = Registry::default().with(fmt::Layer::default().event_format(LanguageEvent));
    //tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

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
        .route("/discovery", put(discover_devices))
        .route("/privacy", get(policy))
        .with_state(app_state);

    // Creates the web controller listener bind.
    let listener_bind = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8123);

    // Creates listener.
    let listener = tokio::net::TcpListener::bind(&listener_bind)
        .await
        .expect(lang::LISTENER_ERROR);

    // Prints listener bind and controller startup message.
    #[cfg(feature = "logging")]
    {
        tracing::info!("{}: {listener_bind}", lang::CONTROLLER_ADDRESS_MESSAGE);
        tracing::info!("{}", lang::CONTROLLER_STARTUP_MESSAGE);
    }

    // Runs server.
    axum::serve(listener, app)
        .await
        .expect(lang::SERVER_STARTUP_ERROR);
}
