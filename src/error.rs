use serde::Serialize;

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Serialize)]
pub(crate) struct Error {
    // Error description.
    pub description: String,
    // Information about an error.
    pub info: Option<String>,
}

impl Error {
    pub(crate) fn with_description(description: &str) -> Self {
        Self {
            description: description.into(),
            info: None,
        }
    }

    pub(crate) fn with_description_error(description: &str, info: impl std::error::Error) -> Self {
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
