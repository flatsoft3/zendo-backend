// use std::sync::Arc;
use crate::config::AppConfig;
use crate::events::events_bus::EventsBus;

#[derive(Clone)]
pub struct AppState {
 pub config: AppConfig,
 pub db_pool: sqlx::PgPool,
 pub events_bus: EventsBus,
}