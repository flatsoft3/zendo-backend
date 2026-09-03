// use std::fmt::format;

use std::fs;

use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use zendo::common::services::email::email_service::EmailService;
use zendo::state::CommonServices;
// use axum::Router;
use zendo::{db::postgres, state::AppState};

use chrono::Local;
use std::path::Path;
use zendo::config::AppConfig;
use zendo::events::events_bus::EventsBus;
use zendo::events::listeners::{
    create_main_wallet_after_user_registered, send_welcome_email_after_user_registered,
};
use zendo::events::user_registered::UserRegisteredEventBus;

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

    //events
    let user_registered_event_bus = UserRegisteredEventBus::new();

    //a general wrapper for all events, for easy pub/sub, everywhere
    let events_bus: EventsBus = EventsBus::new(user_registered_event_bus);

    let email_service =
        EmailService::new(config.smtp_config.clone()).expect("Failed to create new email service");

    let redis_client =
        redis::Client::open(config.clone().redis_url).expect("Failed to create Redis client");

    let state = AppState {
        config: config.clone(),
        db_pool: postgres::get_connection(&config).await,
        events_bus,
        common_services: CommonServices {
            email: email_service,
        },
        redis_client: redis_client,
    };

    //spawn event listeners
    tokio::spawn(
        create_main_wallet_after_user_registered::listen_to_user_registered_event(state.clone()),
    );
    tokio::spawn(
        send_welcome_email_after_user_registered::listen_to_user_registered_event(state.clone()),
    );

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
