use axum::{routing::get, Router};

use crate::state::AppState;

pub mod health;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health-check", get(health::health_check))
}