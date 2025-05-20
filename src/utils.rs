use std::time::Duration;

use ascot::hazards::{Hazard, HazardData, ALL_HAZARDS};

use ascot_controller::controller::Controller;
use ascot_controller::discovery::Discovery;

use minijinja::value::ViaDeserialize;
use minijinja::Environment;

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

pub(crate) const fn retrieve_all_hazards() -> [HazardData; 24] {
    let mut a: [HazardData; ALL_HAZARDS.len()] = [Hazard::FireHazard.data(); ALL_HAZARDS.len()];
    let mut i = 0;
    while i < ALL_HAZARDS.len() {
        a[i] = ALL_HAZARDS[i].data();
        i += 1;
    }
    a
}

/*pub(crate) fn retrieve_all_hazards() -> Vec<HazardData> {
    ALL_HAZARDS.iter().map(Hazard::data).collect()
}*/

pub(crate) fn add_functions_to_env(env: &mut Environment<'_>) {
    env.add_function("hazard_id", |hazard: ViaDeserialize<Hazard>| {
        hazard.data().id
    });
    env.add_function("hazard_name", |hazard: ViaDeserialize<Hazard>| {
        hazard.data().name
    });
    env.add_function("hazard_category", |hazard: ViaDeserialize<Hazard>| {
        hazard.data().category_name
    });
}

// TODO: Define a function to map English hazards in Italian through hazard id.

// TODO: Add a function to manage hazards and translate them in Italian
