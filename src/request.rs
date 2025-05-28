use ascot_controller::controller::Controller;
use ascot_controller::parameters::Parameters;
use ascot_controller::response::Response;

use axum::extract::State;
use axum::response::Redirect;

use axum_extra::extract::Form;

use serde::Deserialize;
use serde_json::Value;

use crate::error::{error_with_info, Error};
use crate::AppState;

// TODO: Remove Debug trait

#[derive(Debug, Deserialize)]
enum ParameterId {
    Bool,
    U8,
    U16,
    U32,
    U64,
    RangeU64,
    F32,
    F64,
    RangeF64,
    CharsSequence,
}

#[derive(Debug, Deserialize)]
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

/*fn create_parameters(params: &[RequestParameters]) -> Result<Parameters, Error> {
    let mut parameters = Parameters::new();
    for param in params {
        match param.id {
            ParameterId::Bool => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `bool` input value",
                )?;
                parameters.bool(&param.name, value);
            }
            ParameterId::U8 => {
                let value =
                    error_with_info(param.value.parse(), "Error in parsing the `u8` input value")?;
                parameters.u8(&param.name, value);
            }
            ParameterId::U16 => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `u16` input value",
                )?;
                parameters.u16(&param.name, value);
            }
            ParameterId::U32 => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `u32` input value",
                )?;
                parameters.u32(&param.name, value);
            }
            ParameterId::U64 | ParameterId::RangeU64 => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `u64` input value",
                )?;
                parameters.u64(&param.name, value);
            }
            ParameterId::F32 => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `f32` input value",
                )?;
                parameters.f32(&param.name, value);
            }
            ParameterId::F64 | ParameterId::RangeF64 => {
                let value = error_with_info(
                    param.value.parse(),
                    "Error in parsing the `f64` input value",
                )?;
                parameters.f64(&param.name, value);
            }
            ParameterId::CharsSequence => {
                parameters.characters_sequence(&param.name, param.value.clone());
            }
        }
    }
    Ok(parameters)
}

async fn _send_request(controller: &Controller, request: Request) -> Result<Response, Error> {
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
}*/

pub(crate) async fn send_request(
    State(state): State<AppState>,
    Form(request): Form<Request>,
) -> Result<Redirect, Error> {
    let controller = state.controller.lock().await;

    println!("{:?}", request);

    // Send a request and obtain a  response.
    /*let response = _send_request(&controller, request).await?;

    // TODO: Add responses to response log.
    //
    // Check response kind.
    match response {
        Response::OkBody(response) => {
            error_with_info(
                response.parse_body().await,
                "Error in retrieving the `Ok` response",
            )?;
        }
        Response::SerialBody(response) => {
            error_with_info(
                response.parse_body::<Value>().await,
                "Error in retrieving the serial response",
            )?;
        }
        Response::InfoBody(_response) => {}
        // TODO: How to treat a skip response because of privacy here. Add to
        // response log.
        Response::Skipped => todo!("Add skipped response to response log"),
        Response::StreamBody(_) => {
            return Err(Error::with_description(
                "This is a Stream Response, something went really wrong.",
            ))
        }
    }*/

    // Redirect to index
    Ok(Redirect::to("/"))
}
