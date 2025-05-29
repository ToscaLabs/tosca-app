use axum::{
    extract::{Json, State},
    http::{StatusCode, Uri},
    response::{Html, IntoResponse, Response},
};

use minijinja::{context, Environment, Value};

use serde::Serialize;

use crate::language::lang;
use crate::layout;
use crate::AppState;

const ASSETS_ERROR: &str = "Failed to load the `assets` directory";

pub(crate) fn error_with_info<T, E: std::error::Error>(
    env: &Environment<'static>,
    res: Result<T, E>,
    description: &str,
) -> Result<T, Error> {
    res.map_err(|e| print_error(&e.to_string(), Error::error_page(env, description, e)))
}

pub(crate) async fn missing_assets() -> Error {
    print_error(ASSETS_ERROR, Error::json_description(ASSETS_ERROR))
}

pub(crate) async fn missing_route(State(state): State<AppState>, uri: Uri) -> Error {
    let error = format!("No route for {uri}");
    print_error(&error, Error::description_page(&state.env, &error))
}

fn print_error(description: &str, error: Error) -> Error {
    #[cfg(feature = "logging")]
    tracing::error!("{} ~> {description}", lang::REQUEST_ERROR);
    error
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
            description,
            info: None,
        }
    }

    fn with_description_error(description: &'a str, info: impl std::error::Error) -> Self {
        Self {
            description,
            info: Some(info.to_string()),
        }
    }
}

pub(crate) struct Error(Response);

impl Error {
    pub(crate) fn description_page(env: &Environment<'static>, description: &str) -> Self {
        Self::render_template(
            env,
            context! {
                title => "Ascot Controller",
                navbar => layout::NAVBAR,
                description => description,
                goto_message => "Go to devices",
                index => "/",
                footer => layout::footer(),
            },
        )
    }

    fn error_page(
        env: &Environment<'static>,
        description: &str,
        info: impl std::error::Error,
    ) -> Self {
        Self::render_template(
            env,
            context! {
                title => "Ascot Controller",
                navbar => layout::NAVBAR,
                description => description,
                error_message => info.to_string(),
                goto_message => "Go to devices",
                index => "/",
                footer => layout::footer(),
            },
        )
    }

    fn render_template(env: &Environment<'static>, context: Value) -> Self {
        let template = match env.get_template("error") {
            Ok(template) => template,
            Err(e) => return Self::minijinja_error("Error in loading the `error` template", e),
        };

        let rendered = match template.render(context) {
            Ok(rendered) => rendered,
            Err(e) => return Self::minijinja_error("Error in rendering the `error` template", e),
        };

        Self((StatusCode::INTERNAL_SERVER_ERROR, Html(rendered)).into_response())
    }

    fn minijinja_error(description: &str, error: minijinja::Error) -> Self {
        Self::json_error(description, error)
    }

    fn json_error(description: &str, error: impl std::error::Error) -> Self {
        Self::description_response(JsonError::with_description_error(description, error))
    }

    fn json_description(description: &str) -> Self {
        Self::description_response(JsonError::with_description(description))
    }

    fn description_response(json_error: JsonError) -> Self {
        Self((StatusCode::INTERNAL_SERVER_ERROR, Json(json_error)).into_response())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.0
    }
}
