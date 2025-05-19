use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;

use crate::language::lang;

pub(crate) fn error_with_info<T, E: std::error::Error>(
    res: Result<T, E>,
    description: &str,
) -> Result<T, Error> {
    res.map_err(|e| {
        #[cfg(feature = "logging")]
        tracing::error!("{} ~> {e}", lang::REQUEST_ERROR);
        Error::with_description_error(description, e)
    })
}

pub(crate) async fn missing_assets() -> Error {
    Error::with_description("Failed to load the `assets` directory")
}

#[derive(Serialize)]
pub(crate) struct Error {
    // Error description.
    pub description: String,
    // Information about an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
}

impl Error {
    fn with_description(description: &str) -> Self {
        Self {
            description: description.into(),
            info: None,
        }
    }

    fn with_description_error(description: &str, info: impl std::error::Error) -> Self {
        Self {
            description: description.into(),
            info: Some(info.to_string()),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}
