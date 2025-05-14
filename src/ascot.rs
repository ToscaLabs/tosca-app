use std::time::Duration;

use ascot_controller::controller::Controller;
use ascot_controller::discovery::Discovery;

// Default duration for the discovery process.
const DEFAULT_DISCOVERY_DURATION: Duration = Duration::from_secs(1);

pub(crate) fn create_controller() -> Controller {
    // Create discovery searcher
    let discovery = Discovery::new("ascot")
        .timeout(DEFAULT_DISCOVERY_DURATION)
        .disable_ipv6()
        .disable_network_interface("docker0");

    // Create Ascot controller
    Controller::new(discovery)
}

// TODO: Define a function to map English hazards in Italian through hazard id.

// TODO: Add a function to manage hazards and translate them in Italian
