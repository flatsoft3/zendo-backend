use axum::{extract::State, response::IntoResponse};
use crate::state::AppState;
use tracing::info;

pub async fn health_check(
    State(state): State<AppState>,
) -> impl IntoResponse {
    info!("health-check route is called");

   let result =  sqlx::query("SELECT 1")
        .execute(&state.db_pool)
        .await
        .expect("Failed to execute query");

    format!("{}, is OK, {}", state.config.app_name, result.rows_affected())
}