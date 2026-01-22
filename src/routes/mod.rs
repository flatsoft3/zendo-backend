use axum::{routing::get, Router};

use crate::state::AppState;

pub mod health;
pub mod user_routes;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health-check", get(health::health_check))
        .route("/users/find-by-id", get(user_routes::find_by_id))
}