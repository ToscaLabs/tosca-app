use ascot::parameters::ParameterKind;

use ascot_controller::controller::Controller;
use ascot_controller::parameters::Parameters;
use ascot_controller::response::Response;

use axum::extract::{Form, State};
use axum::response::Redirect;

use serde::Deserialize;
use serde_json::Value;

use crate::error::Error;
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

fn create_parameters(params: &[RequestParameters]) -> Parameters {
    let mut parameters = Parameters::new();
    for param in params {
        match param.kind {
            ParameterKind::Bool { .. } => {
                let value = param.value.parse().unwrap();
                parameters.bool(&param.name, value);
            }
            ParameterKind::U8 { .. } => {
                let value = param.value.parse().unwrap();
                parameters.u8(&param.name, value);
            }
            ParameterKind::U16 { .. } => {
                let value = param.value.parse().unwrap();
                parameters.u16(&param.name, value);
            }
            ParameterKind::U32 { .. } => {
                let value = param.value.parse().unwrap();
                parameters.u32(&param.name, value);
            }
            ParameterKind::U64 { .. } | ParameterKind::RangeU64 { .. } => {
                let value = param.value.parse().unwrap();
                parameters.u64(&param.name, value);
            }
            ParameterKind::F32 { .. } => {
                let value = param.value.parse().unwrap();
                parameters.f32(&param.name, value);
            }
            ParameterKind::F64 { .. } | ParameterKind::RangeF64 { .. } => {
                let value = param.value.parse().unwrap();
                parameters.f64(&param.name, value);
            }
            ParameterKind::CharsSequence { .. } => {
                parameters.characters_sequence(&param.name, param.value.clone());
            }
            _ => unreachable!(),
        }
    }
    parameters
}

async fn send_request(controller: &Controller, request: Request) -> Response {
    // Find device sender
    let device_sender = controller.device(request.device_id).unwrap();

    // Send request.
    let request_sender = device_sender.request(&request.route).unwrap();

    // Obtain response
    if request.parameters.is_empty() {
        // Send request.
        request_sender.send().await.unwrap()
    } else {
        // Create parameters.
        let parameters = create_parameters(&request.parameters);
        // Send request with parameters.
        request_sender
            .send_with_parameters(&parameters)
            .await
            .unwrap()
    }
}

pub(crate) async fn send_ok_request(
    State(state): State<AppState>,
    Form(request): Form<Request>,
) -> Result<Redirect, Error> {
    println!("{:?}", request);
    /*let controller = state.controller.lock().await;

    // Send a request and obtain a  response.
    let response = send_request(&controller, request).await;

    // Check response kind.
    match response {
        // TODO: Add response to response log
        Response::OkBody(response) => {
            response.parse_body().await.unwrap();
        }
        Response::Skipped => todo!("Add skipped response to response log"),
        _ => todo!("This is an error, add to response log"),
    }*/

    // Redirect to index
    Ok(Redirect::to("/"))
}

pub(crate) async fn send_serial_request(
    State(state): State<AppState>,
    Form(request): Form<Request>,
) -> Result<Redirect, Error> {
    println!("{:?}", request);

    /*let controller = state.controller.lock().await;

    // Send a request and obtain a  response.
    let response = send_request(&controller, request).await;

    // Check response kind.
    match response {
        // TODO: Add response to response log
        Response::SerialBody(response) => {
            let serial_response = response.parse_body::<Value>().await.unwrap();
        }
        Response::Skipped => todo!("Add skipped response to response log"),
        _ => todo!("This is an error, add to response log"),
    }*/

    // Redirect to index
    Ok(Redirect::to("/"))
}
