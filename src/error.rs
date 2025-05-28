use axum::{
    extract::{Json, State},
    http::{StatusCode, Uri},
    response::{Html, IntoResponse, Response},
};

use minijinja::Environment;

use serde::Serialize;

use crate::language::lang;
use crate::AppState;

pub(crate) fn error_with_info<T, E: std::error::Error>(
    env: &Environment<'static>,
    res: Result<T, E>,
    description: &str,
) -> Result<T, Error> {
    res.map_err(|e| {
        #[cfg(feature = "logging")]
        tracing::error!("{} ~> {e}", lang::REQUEST_ERROR);
        Error::error_page(env, description, e)
    })
}

pub(crate) async fn missing_assets() -> Error {
    #[cfg(feature = "logging")]
    tracing::error!(
        "{} ~> Failed to load the `assets` directory",
        lang::REQUEST_ERROR
    );
    Error::json_description("Failed to load the `assets` directory")
}

pub(crate) async fn missing_route(State(state): State<AppState>, uri: Uri) -> Error {
    let error = format!("No route for {uri}");
    #[cfg(feature = "logging")]
    tracing::error!("{} ~> {error}", lang::REQUEST_ERROR);
    Error::description_page(&state.env, &error)
}

#[derive(Serialize)]
struct JsonError<'a> {
    // Error description.
    description: &'a str,
    // Information about an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<String>,
}

impl<'a> JsonError<'a> {
    fn with_description(description: &'a str) -> Self {
        Self {
            description: description,
            info: None,
        }
    }

    fn with_description_error(description: &'a str, info: impl std::error::Error) -> Self {
        Self {
            description: description,
            info: Some(info.to_string()),
        }
    }
}

pub(crate) struct Error(Response);

impl Error {
    pub(crate) fn description_page(env: &Environment<'static>, description: &str) -> Self {
        let template = match env.get_template("error") {
            Ok(template) => template,
            Err(e) => return Self::json_error(description, e),
        };

        Self((StatusCode::INTERNAL_SERVER_ERROR, Html("ciao".to_string())).into_response())
    }

    fn error_page(
        env: &Environment<'static>,
        description: &str,
        info: impl std::error::Error,
    ) -> Self {
        let template = match env.get_template("error") {
            Ok(template) => template,
            Err(e) => return Self::json_error(description, e),
        };

        Self((StatusCode::INTERNAL_SERVER_ERROR, Html("ciao".to_string())).into_response())
    }

    fn json_error(description: &str, error: impl std::error::Error) -> Self {
        Self(
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JsonError::with_description_error(description, error)),
            )
                .into_response(),
        )
    }

    fn json_description(description: &str) -> Self {
        Self(
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JsonError::with_description(description)),
            )
                .into_response(),
        )
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.0
    }
}
