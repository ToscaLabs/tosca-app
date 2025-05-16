// TODO: Saves the responses in a collector and presents them in the log page

use axum::extract::State;

use crate::AppState;

pub(crate) async fn response_log(State(state): State<AppState>) {}
