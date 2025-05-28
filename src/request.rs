use ascot::parameters::ParameterId;

use ascot_controller::controller::Controller;
use ascot_controller::parameters::Parameters;
use ascot_controller::response::Response;

use axum::extract::State;
use axum::response::Redirect;

use axum_extra::extract::Form;

use minijinja::Environment;

use serde::Deserialize;
use serde_json::Value;

use crate::error::{error_with_info, Error};
use crate::AppState;

#[derive(Deserialize)]
#[cfg_attr(feature = "logging", derive(Debug))]
pub(crate) struct Request {
    device_id: usize,
    route: String,
    #[serde(default)]
    ids: Vec<ParameterId>,
    #[serde(default)]
    names: Vec<String>,
    #[serde(default)]
    values: Vec<String>,
}

fn create_parameters<'a>(
    env: &Environment<'static>,
    ids: Vec<ParameterId>,
    names: &'a [String],
    values: Vec<String>,
) -> Result<Parameters<'a>, Error> {
    let mut parameters = Parameters::new();
    for (i, value) in values.into_iter().enumerate() {
        match ids[i] {
            ParameterId::Bool => {
                parameters.bool(&names[i], value.is_empty());
            }
            ParameterId::U8 => {
                let value =
                    error_with_info(env, value.parse(), "Error in parsing the `u8` input value")?;
                parameters.u8(&names[i], value);
            }
            ParameterId::U16 => {
                let value =
                    error_with_info(env, value.parse(), "Error in parsing the `u16` input value")?;
                parameters.u16(&names[i], value);
            }
            ParameterId::U32 => {
                let value =
                    error_with_info(env, value.parse(), "Error in parsing the `u32` input value")?;
                parameters.u32(&names[i], value);
            }
            ParameterId::U64 | ParameterId::RangeU64 => {
                let value =
                    error_with_info(env, value.parse(), "Error in parsing the `u64` input value")?;
                parameters.u64(&names[i], value);
            }
            ParameterId::F32 => {
                let value =
                    error_with_info(env, value.parse(), "Error in parsing the `f32` input value")?;
                parameters.f32(&names[i], value);
            }
            ParameterId::F64 | ParameterId::RangeF64 => {
                let value =
                    error_with_info(env, value.parse(), "Error in parsing the `f64` input value")?;
                parameters.f64(&names[i], value);
            }
            ParameterId::CharsSequence => {
                parameters.characters_sequence(&names[i], value);
            }
        }
    }
    Ok(parameters)
}

async fn _send_request(
    env: &Environment<'static>,
    controller: &Controller,
    request: Request,
) -> Result<Response, Error> {
    let Request {
        device_id,
        route,
        ids,
        names,
        values,
    } = request;

    // Find device sender
    let device_sender = error_with_info(
        env,
        controller.device(device_id),
        "Error in finding the device",
    )?;

    // Send request.
    let request_sender = error_with_info(
        env,
        device_sender.request(&route),
        "Error in creating the request for device",
    )?;

    // Obtain response
    if ids.is_empty() {
        // Send request.
        error_with_info(
            env,
            request_sender.send().await,
            "Error in sending the request with default parameters",
        )
    } else {
        // Create parameters.
        let parameters = create_parameters(env, ids, &names, values)?;
        // Send request with parameters.
        error_with_info(
            env,
            request_sender.send_with_parameters(&parameters).await,
            "Error in sending the request with parameters",
        )
    }
}

pub(crate) async fn send_request(
    State(state): State<AppState>,
    Form(request): Form<Request>,
) -> Result<Redirect, Error> {
    #[cfg(feature = "logging")]
    tracing::info!("{:?}", request);

    let env = state.env;

    let controller = state.controller.lock().await;

    // Send a request and obtain a  response.
    let response = _send_request(&env, &controller, request).await?;

    // TODO: Add responses to response log.
    //
    // Check response kind.
    match response {
        Response::OkBody(response) => {
            error_with_info(
                &env,
                response.parse_body().await,
                "Error in retrieving the `Ok` response",
            )?;
        }
        Response::SerialBody(response) => {
            error_with_info(
                &env,
                response.parse_body::<Value>().await,
                "Error in retrieving the serial response",
            )?;
        }
        Response::InfoBody(_response) => {}
        // TODO: How to treat a skip response because of privacy here. Add to
        // response log.
        Response::Skipped => todo!("Add skipped response to response log"),
        Response::StreamBody(_) => {
            return Err(Error::description_page(
                &env,
                "This is a Stream Response, something went really wrong.",
            ))
        }
    }

    // Redirect to index
    Ok(Redirect::to("/"))
}
