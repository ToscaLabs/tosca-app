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
use crate::language::lang;
use crate::layout;
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
    ids: &[ParameterId],
    names: &'a [String],
    values: Vec<String>,
) -> Result<Parameters<'a>, Error> {
    let mut parameters = Parameters::new();
    for (i, (value, id)) in values.into_iter().zip(ids).enumerate() {
        match id {
            ParameterId::Bool => {
                parameters.bool(&names[i], value.is_empty());
            }
            ParameterId::U8 => {
                let value = error_with_info(env, value.parse(), lang::U8_ERROR)?;
                parameters.u8(&names[i], value);
            }
            ParameterId::U16 => {
                let value = error_with_info(env, value.parse(), lang::U16_ERROR)?;
                parameters.u16(&names[i], value);
            }
            ParameterId::U32 => {
                let value = error_with_info(env, value.parse(), lang::U32_ERROR)?;
                parameters.u32(&names[i], value);
            }
            ParameterId::U64 | ParameterId::RangeU64 => {
                let value = error_with_info(env, value.parse(), lang::U64_ERROR)?;
                parameters.u64(&names[i], value);
            }
            ParameterId::F32 => {
                let value = error_with_info(env, value.parse(), lang::F32_ERROR)?;
                parameters.f32(&names[i], value);
            }
            ParameterId::F64 | ParameterId::RangeF64 => {
                let value = error_with_info(env, value.parse(), lang::F64_ERROR)?;
                parameters.f64(&names[i], value);
            }
            ParameterId::CharsSequence => {
                parameters.characters_sequence(&names[i], value);
            }
        }
    }
    Ok(parameters)
}

async fn send_controller_request(
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

    // Find device sender.
    let device_sender = error_with_info(
        env,
        controller.device(device_id),
        lang::REQUEST_DEVICE_ERROR,
    )?;

    // Send request.
    let request_sender = error_with_info(
        env,
        device_sender.request(&route),
        lang::REQUEST_SENDER_ERROR,
    )?;

    // Obtain response.
    if ids.is_empty() {
        // Send request.
        error_with_info(
            env,
            request_sender.send().await,
            lang::REQUEST_SENDER_DEFAULT_PARAMS_ERROR,
        )
    } else {
        // Create parameters.
        let parameters = create_parameters(env, &ids, &names, values)?;
        // Send request with parameters.
        error_with_info(
            env,
            request_sender.send_with_parameters(&parameters).await,
            lang::REQUEST_SENDER_PARAMS_ERROR,
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
    let response = send_controller_request(&env, &controller, request).await?;

    // TODO: Add responses to response log.
    //
    // Check response kind.
    match response {
        Response::OkBody(response) => {
            error_with_info(&env, response.parse_body().await, lang::RESPONSE_OK_ERROR)?;
        }
        Response::SerialBody(response) => {
            error_with_info(
                &env,
                response.parse_body::<Value>().await,
                lang::RESPONSE_SERIAL_ERROR,
            )?;
        }
        Response::InfoBody(_response) => {}
        // TODO: How to treat a skip response because of privacy here. Add to
        // response log.
        Response::Skipped => todo!("Add skipped response to response log"),
        Response::StreamBody(_) => {
            return Err(Error::description_page(
                &env,
                lang::RESPONSE_WRONG_STREAM_ERROR,
            ))
        }
    }

    // Redirect to index
    Ok(Redirect::to(layout::INDEX_ROUTE))
}
