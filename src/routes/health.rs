use axum::{extract::State, response::IntoResponse};
use crate::state::AppState;

pub async fn health_check(
    State(state): State<AppState>,
) -> impl IntoResponse {
    format!("{}, is OK", state.config.app_name)
}