use ascot::parameters::ParameterKind;

use ascot_controller::controller::Controller;
use ascot_controller::parameters::Parameters;
use ascot_controller::response::Response;

use axum::extract::{Form, State};
use axum::response::Redirect;

use serde::Deserialize;
use serde_json::Value;

use crate::error::{error_with_info, Error};
use crate::AppState;

// TODO: Remove Debug trait

#[derive(Debug, Deserialize)]
pub(crate) struct RequestParameters {
    kind: ParameterKind,
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Request {
    device_id: usize,
    route: String,
    #[serde(default)]
    parameters: Vec<RequestParameters>,
}

fn create_parameters(params: &[RequestParameters]) -> Result<Parameters, Error> {
    let mut parameters = Parameters::new();
    for param in params {
        match param.kind {
            ParameterKind::Bool { .. } => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `bool` input value",
                )?;
                parameters.bool(&param.name, value);
            }
            ParameterKind::U8 { .. } => {
                let value =
                    error_with_info(param.value.parse(), "Error in parsing the `u8` input value")?;
                parameters.u8(&param.name, value);
            }
            ParameterKind::U16 { .. } => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `u16` input value",
                )?;
                parameters.u16(&param.name, value);
            }
            ParameterKind::U32 { .. } => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `u32` input value",
                )?;
                parameters.u32(&param.name, value);
            }
            ParameterKind::U64 { .. } | ParameterKind::RangeU64 { .. } => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `u64` input value",
                )?;
                parameters.u64(&param.name, value);
            }
            ParameterKind::F32 { .. } => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `f32` input value",
                )?;
                parameters.f32(&param.name, value);
            }
            ParameterKind::F64 { .. } | ParameterKind::RangeF64 { .. } => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `f64` input value",
                )?;
                parameters.f64(&param.name, value);
            }
            ParameterKind::CharsSequence { .. } => {
                parameters.characters_sequence(&param.name, param.value.clone());
            }
            _ => unreachable!(),
        }
    }
    Ok(parameters)
}

async fn send_request(controller: &Controller, request: Request) -> Result<Response, Error> {
    // Find device sender
    let device_sender = error_with_info(
        controller.device(request.device_id),
        "Error in finding the device",
    )?;

    // Send request.
    let request_sender = error_with_info(
        device_sender.request(&request.route),
        "Error in creating the request for device",
    )?;

    // Obtain response
    if request.parameters.is_empty() {
        // Send request.
        error_with_info(
            request_sender.send().await,
            "Error in sending the request with default parameters",
        )
    } else {
        // Create parameters.
        let parameters = create_parameters(&request.parameters)?;
        // Send request with parameters.
        error_with_info(
            request_sender.send_with_parameters(&parameters).await,
            "Error in sending the request with parameters",
        )
    }
}

pub(crate) async fn send_ok_request(
    State(state): State<AppState>,
    Form(request): Form<Request>,
) -> Result<Redirect, Error> {
    let controller = state.controller.lock().await;

    // Send a request and obtain a  response.
    let response = send_request(&controller, request).await?;

    // Check response kind.
    match response {
        // TODO: Add response to response log
        Response::OkBody(response) => {
            error_with_info(
                response.parse_body().await,
                "Error in retrieving the `Ok` response",
            )?;
        }
        Response::Skipped => todo!("Add skipped response to response log"),
        _ => todo!("This is an error, add to response log"),
    }

    // Redirect to index
    Ok(Redirect::to("/"))
}

pub(crate) async fn send_serial_request(
    State(state): State<AppState>,
    Form(request): Form<Request>,
) -> Result<Redirect, Error> {
    let controller = state.controller.lock().await;

    // Send a request and obtain a  response.
    let response = send_request(&controller, request).await?;

    // Check response kind.
    match response {
        // TODO: Add response to response log
        Response::SerialBody(response) => {
            let serial_response = error_with_info(
                response.parse_body::<Value>().await,
                "Error in retrieving the serial response",
            )?;
        }
        Response::Skipped => todo!("Add skipped response to response log"),
        _ => todo!("This is an error, add to response log"),
    }

    // Redirect to index
    Ok(Redirect::to("/"))
}
