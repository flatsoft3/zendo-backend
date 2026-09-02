use crate::common::services::email::email_service::EmailService;
// use std::sync::Arc;
use crate::config::AppConfig;
use crate::events::events_bus::EventsBus;

#[derive(Clone)]
pub struct CommonServices {
   pub email: EmailService
}
#[derive(Clone)]
pub struct AppState {
 pub config: AppConfig,
 pub db_pool: sqlx::PgPool,
 pub events_bus: EventsBus,
 pub common_services: CommonServices
}