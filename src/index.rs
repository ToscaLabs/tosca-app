use ascot::hazards::HazardData;

use ascot_controller::device::Devices;

use axum::extract::State;
use axum::response::Html;

use serde::Serialize;

use crate::error::{error_with_info, Error};
use crate::language::lang;
use crate::layout::RenderLayout;
use crate::utils::retrieve_all_hazards;
use crate::AppState;

#[derive(Serialize)]
struct RenderMessages {
    // No devices message.
    no_devices_message: &'static str,
    // Change device message.
    new_device_name: &'static str,
    // Device name placeholder.
    device_name_placeholder: &'static str,
    // No route description message.
    no_route_description_message: &'static str,
}

impl RenderMessages {
    const fn new() -> Self {
        Self {
            no_devices_message: lang::NO_DEVICES,
            new_device_name: lang::NEW_DEVICE_NAME,
            device_name_placeholder: lang::CHANGE_DEVICE_NAME_PLACEHOLDER,
            no_route_description_message: lang::NO_ROUTE_DESCRIPTION_MESSAGE,
        }
    }
}

#[derive(Serialize)]
#[allow(clippy::struct_field_names)]
struct RenderRoutes {
    request_route: &'static str,
    discovery_route: &'static str,
    stream_route: &'static str,
    info_route: &'static str,
    event_log_route: &'static str,
    response_log_route: &'static str,
    change_device_name_route: &'static str,
}

impl RenderRoutes {
    const fn new() -> Self {
        Self {
            request_route: lang::REQUEST_ROUTE,
            discovery_route: lang::DISCOVERY_ROUTE,
            stream_route: lang::PRINT_STREAM_ROUTE,
            info_route: lang::PRINT_INFO_ROUTE,
            event_log_route: lang::PRINT_EVENT_LOG_ROUTE,
            response_log_route: lang::PRINT_RESPONSE_LOG_ROUTE,
            change_device_name_route: lang::PRINT_CHANGE_DEVICE_NAME_ROUTE,
        }
    }
}

#[derive(Serialize)]
#[allow(clippy::struct_field_names)]
struct RenderButtons {
    // Buttons messages.
    discover_button: &'static str,
    request_button: &'static str,
    change_button: &'static str,
}

impl RenderButtons {
    const fn new() -> Self {
        Self {
            discover_button: lang::DISCOVER_DEVICES_BUTTON_MESSAGE,
            request_button: lang::REQUEST_BUTTON_MESSAGE,
            change_button: lang::CHANGE_BUTTON_MESSAGE,
        }
    }
}

#[derive(Serialize)]
#[allow(clippy::struct_field_names)]
struct RenderLinks {
    // Links messages.
    stream_link: &'static str,
    info_link: &'static str,
    event_log_link: &'static str,
    response_log_link: &'static str,
}

impl RenderLinks {
    const fn new() -> Self {
        Self {
            stream_link: lang::STREAM_LINK_MESSAGE,
            info_link: lang::INFO_LINK_MESSAGE,
            event_log_link: lang::EVENT_LOG_LINK_MESSAGE,
            response_log_link: lang::RESPONSE_LOG_LINK_MESSAGE,
        }
    }
}

#[derive(Serialize)]
struct RenderIndex<'a> {
    #[serde(flatten)]
    layout: RenderLayout,
    #[serde(flatten)]
    general_render: RenderMessages,
    #[serde(flatten)]
    routes_render: RenderRoutes,
    #[serde(flatten)]
    buttons_render: RenderButtons,
    #[serde(flatten)]
    links_render: RenderLinks,
    // Devices.
    devices: &'a Devices,
    // Hazards.
    hazards: &'a [HazardData],
}

impl<'a> RenderIndex<'a> {
    fn new(devices: &'a Devices, hazards: &'a [HazardData]) -> Self {
        Self {
            layout: RenderLayout::new(),
            general_render: RenderMessages::new(),
            routes_render: RenderRoutes::new(),
            buttons_render: RenderButtons::new(),
            links_render: RenderLinks::new(),
            devices,
            hazards,
        }
    }
}

pub(crate) async fn index(State(state): State<AppState>) -> Result<Html<String>, Error> {
    let template = error_with_info(
        &state.env,
        state.env.get_template("index"),
        lang::INDEX_TEMPLATE_ERROR,
    )?;

    // TODO: Only the hazards associated with each discovered device must be considered.
    let all_hazards = retrieve_all_hazards();

    #[cfg(not(feature = "fake-devices"))]
    let controller = state.controller.lock().await;
    #[cfg(not(feature = "fake-devices"))]
    let devices = controller.devices();

    #[cfg(feature = "fake-devices")]
    let devices = &crate::device::fake::create_fake_devices();

    let rendered = error_with_info(
        &state.env,
        template.render(RenderIndex::new(devices, &all_hazards)),
        lang::INDEX_RENDER_ERROR,
    )?;

    Ok(Html(rendered))
}
