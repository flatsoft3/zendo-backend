// use std::fmt::format;

use std::fs;

use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

// use axum::Router;
use zendo::state::AppState;

use chrono::Local;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use zendo::config::AppConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let log_dir = Path::new("/var/tmp/log/zendo");
    fs::create_dir_all(log_dir).expect("Failed to create log directory");

    let log_file = std::fs::File::create(
        log_dir.join(format!("zendo-{}.log", Local::now().format("%Y-%m-%d"),)),
    )
    .expect("Failed to create log file");

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        // .with_env_filter(EnvFilter::new("info"))
        .with_writer(log_file)
        .init();

    let config = AppConfig::load();

    info!(
        app = %config.app_name,
        env = %config.app_env,
        "Configuration loaded"
    );

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState {
        config: config.clone(),
        db_pool: db,
    };

    // let app = Router::new().merge(routes::router()).with_state(state);
    let app = axum::Router::new()
        .merge(zendo::routes::router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.app_port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!(
        "🚀 {} running in {} mode on http://{}",
        config.app_name, config.app_env, addr
    );

    info!(address = %addr, "server started");

    axum::serve(listener, app).await.unwrap();
}
async fn _root() -> &'static str {
    "Hello, Zendo App in Rust"
}
