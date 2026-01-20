mod config;
mod routes;
mod state;

use std::fmt::format;

use axum::Router;
use state::AppState;

use crate::config::AppConfig;

#[tokio::main]
async fn main() {
    let config = AppConfig::load();

    let state = AppState {
        config: config.clone(),
    };

    let app = Router::new().merge(routes::router()).with_state(state);

    let addr = format!("0.0.0.0:{}", config.app_port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!(
        "🚀 {} running in {} mode on http://{}",
        config.app_name, config.app_env, addr
    );

    axum::serve(listener, app).await.unwrap();
}
async fn _root() -> &'static str {
    "Hello, Zendo App in Rust"
}
